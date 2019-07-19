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
pub use crate::jmorecfg_h::{
    boolean, FALSE, JCOEF, JDIMENSION, JOCTET, JSAMPLE, MAX_COMPONENTS, TRUE, UINT16, UINT8,
};
pub use crate::jpegint_h::{
    inverse_DCT_method_ptr, jinit_c_master_control, jinit_huff_encoder, jinit_marker_writer,
    jinit_phuff_encoder, jpeg_c_coef_controller, jpeg_c_main_controller, jpeg_c_prep_controller,
    jpeg_color_converter, jpeg_color_deconverter, jpeg_color_quantizer, jpeg_comp_master,
    jpeg_d_coef_controller, jpeg_d_main_controller, jpeg_d_post_controller, jpeg_decomp_master,
    jpeg_downsampler, jpeg_entropy_decoder, jpeg_entropy_encoder, jpeg_forward_dct,
    jpeg_input_controller, jpeg_inverse_dct, jpeg_marker_reader, jpeg_marker_writer,
    jpeg_upsampler, jzero_far, CSTATE_START, CSTATE_WRCOEFS, JBUF_CRANK_DEST, JBUF_PASS_THRU,
    JBUF_REQUANT, JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_compress_ptr, j_decompress_ptr, jpeg_alloc_quant_table, jpeg_common_struct,
    jpeg_component_info, jpeg_compress_struct, jpeg_decompress_struct, jpeg_destination_mgr,
    jpeg_error_mgr, jpeg_marker_parser_method, jpeg_marker_struct, jpeg_memory_mgr,
    jpeg_progress_mgr, jpeg_saved_marker_ptr, jpeg_scan_info, jpeg_set_colorspace,
    jpeg_set_defaults, jpeg_source_mgr, jpeg_suppress_tables, jvirt_barray_control,
    jvirt_barray_ptr, jvirt_sarray_control, jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr,
    C_MAX_BLOCKS_IN_MCU, DCTSIZE2, JBLOCK, JBLOCKARRAY, JBLOCKROW, JCOEFPTR, JCS_CMYK,
    JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA,
    JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN,
    JCS_YCCK, JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JDITHER_FS, JDITHER_NONE, JDITHER_ORDERED,
    JHUFF_TBL, JPOOL_IMAGE, JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE,
    J_DCT_METHOD, J_DITHER_MODE, NUM_QUANT_TBLS,
};
pub use crate::stddef_h::{size_t, NULL};
use crate::stdlib::memcpy;
use libc::{self, c_int, c_uint, c_ulong, c_void};
pub type my_coef_ptr = *mut my_coef_controller;
/*
 * The rest of this file is a special implementation of the coefficient
 * buffer controller.  This is similar to jccoefct.c, but it handles only
 * output from presupplied virtual arrays.  Furthermore, we generate any
 * dummy padding blocks on-the-fly rather than expecting them to be present
 * in the arrays.
 */
/* Private buffer controller object */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_coef_controller {
    pub pub_0: jpeg_c_coef_controller,
    pub iMCU_row_num: JDIMENSION,
    pub mcu_ctr: JDIMENSION,
    pub MCU_vert_offset: c_int,
    pub MCU_rows_per_iMCU_row: c_int,
    pub whole_image: *mut jvirt_barray_ptr,
    pub dummy_buffer: [JBLOCKROW; 10],
}
/*
 * Compression initialization for writing raw-coefficient data.
 * Before calling this, all parameters and a data destination must be set up.
 * Call jpeg_finish_compress() to actually write the data.
 *
 * The number of passed virtual arrays must match cinfo->num_components.
 * Note that the virtual arrays need not be filled or even realized at
 * the time write_coefficients is called; indeed, if the virtual arrays
 * were requested from this compression object's memory manager, they
 * typically will be realized during this routine and filled afterwards.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_write_coefficients(
    mut cinfo: j_compress_ptr,
    mut coef_arrays: *mut jvirt_barray_ptr,
) {
    if (*(*cinfo).master).num_scans_luma == 0i32 {
        (*(*cinfo).master).optimize_scans = FALSE
    }
    if (*cinfo).global_state != CSTATE_START {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    jpeg_suppress_tables(cinfo, FALSE);
    (*(*cinfo).err)
        .reset_error_mgr
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    (*(*cinfo).dest)
        .init_destination
        .expect("non-null function pointer")(cinfo);
    transencode_master_selection(cinfo, coef_arrays);
    (*cinfo).next_scanline = 0i32 as JDIMENSION;
    (*cinfo).global_state = CSTATE_WRCOEFS;
}
/*
 * Initialize the compression object with default parameters,
 * then copy from the source object all parameters needed for lossless
 * transcoding.  Parameters that can be varied without loss (such as
 * scan script and Huffman optimization) are left in their default states.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_copy_critical_parameters(
    srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
) {
    let mut qtblptr: *mut *mut JQUANT_TBL = 0 as *mut *mut JQUANT_TBL;
    let mut incomp: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut outcomp: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut c_quant: *mut JQUANT_TBL = 0 as *mut JQUANT_TBL;
    let mut slot_quant: *mut JQUANT_TBL = 0 as *mut JQUANT_TBL;
    let mut tblno: c_int = 0;
    let mut ci: c_int = 0;
    let mut coefi: c_int = 0;
    if (*dstinfo).global_state != CSTATE_START {
        (*(*dstinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*dstinfo).err).msg_parm.i[0usize] = (*dstinfo).global_state;
        (*(*dstinfo).err)
            .error_exit
            .expect("non-null function pointer")(dstinfo as j_common_ptr);
    }
    (*dstinfo).image_width = (*srcinfo).image_width;
    (*dstinfo).image_height = (*srcinfo).image_height;
    (*dstinfo).input_components = (*srcinfo).num_components;
    (*dstinfo).in_color_space = (*srcinfo).jpeg_color_space;
    jpeg_set_defaults(dstinfo);
    (*(*dstinfo).master).trellis_quant = FALSE;
    jpeg_set_colorspace(dstinfo, (*srcinfo).jpeg_color_space);
    (*dstinfo).data_precision = (*srcinfo).data_precision;
    (*dstinfo).CCIR601_sampling = (*srcinfo).CCIR601_sampling;
    tblno = 0i32;
    while tblno < NUM_QUANT_TBLS {
        if !(*srcinfo).quant_tbl_ptrs[tblno as usize].is_null() {
            qtblptr = &mut *(*dstinfo)
                .quant_tbl_ptrs
                .as_mut_ptr()
                .offset(tblno as isize) as *mut *mut JQUANT_TBL;
            if (*qtblptr).is_null() {
                *qtblptr = jpeg_alloc_quant_table(dstinfo as j_common_ptr)
            }
            memcpy(
                (**qtblptr).quantval.as_mut_ptr() as *mut c_void,
                (*(*srcinfo).quant_tbl_ptrs[tblno as usize])
                    .quantval
                    .as_mut_ptr() as *const c_void,
                ::std::mem::size_of::<[UINT16; 64]>() as c_ulong,
            );
            (**qtblptr).sent_table = FALSE
        }
        tblno += 1
    }
    (*dstinfo).num_components = (*srcinfo).num_components;
    if (*dstinfo).num_components < 1i32 || (*dstinfo).num_components > MAX_COMPONENTS {
        (*(*dstinfo).err).msg_code = JERR_COMPONENT_COUNT as c_int;
        (*(*dstinfo).err).msg_parm.i[0usize] = (*dstinfo).num_components;
        (*(*dstinfo).err).msg_parm.i[1usize] = 10i32;
        (*(*dstinfo).err)
            .error_exit
            .expect("non-null function pointer")(dstinfo as j_common_ptr);
    }
    ci = 0i32;
    incomp = (*srcinfo).comp_info;
    outcomp = (*dstinfo).comp_info;
    while ci < (*dstinfo).num_components {
        (*outcomp).component_id = (*incomp).component_id;
        (*outcomp).h_samp_factor = (*incomp).h_samp_factor;
        (*outcomp).v_samp_factor = (*incomp).v_samp_factor;
        (*outcomp).quant_tbl_no = (*incomp).quant_tbl_no;
        tblno = (*outcomp).quant_tbl_no;
        if tblno < 0i32
            || tblno >= NUM_QUANT_TBLS
            || (*srcinfo).quant_tbl_ptrs[tblno as usize].is_null()
        {
            (*(*dstinfo).err).msg_code = JERR_NO_QUANT_TABLE as c_int;
            (*(*dstinfo).err).msg_parm.i[0usize] = tblno;
            (*(*dstinfo).err)
                .error_exit
                .expect("non-null function pointer")(dstinfo as j_common_ptr);
        }
        slot_quant = (*srcinfo).quant_tbl_ptrs[tblno as usize];
        c_quant = (*incomp).quant_table;
        if !c_quant.is_null() {
            coefi = 0i32;
            while coefi < DCTSIZE2 {
                if (*c_quant).quantval[coefi as usize] as c_int
                    != (*slot_quant).quantval[coefi as usize] as c_int
                {
                    (*(*dstinfo).err).msg_code = JERR_MISMATCHED_QUANT_TABLE as c_int;
                    (*(*dstinfo).err).msg_parm.i[0usize] = tblno;
                    (*(*dstinfo).err)
                        .error_exit
                        .expect("non-null function pointer")(
                        dstinfo as j_common_ptr
                    );
                }
                coefi += 1
            }
        }
        ci += 1;
        incomp = incomp.offset(1isize);
        outcomp = outcomp.offset(1isize)
    }
    if 0 != (*srcinfo).saw_JFIF_marker {
        if (*srcinfo).JFIF_major_version as c_int == 1i32 {
            (*dstinfo).JFIF_major_version = (*srcinfo).JFIF_major_version;
            (*dstinfo).JFIF_minor_version = (*srcinfo).JFIF_minor_version
        }
        (*dstinfo).density_unit = (*srcinfo).density_unit;
        (*dstinfo).X_density = (*srcinfo).X_density;
        (*dstinfo).Y_density = (*srcinfo).Y_density
    };
}
/*
 * jctrans.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1995-1998, Thomas G. Lane.
 * Modified 2000-2009 by Guido Vollbeding.
 * It was modified by The libjpeg-turbo Project to include only code relevant
 * to libjpeg-turbo.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains library routines for transcoding compression,
 * that is, writing raw DCT coefficient arrays to an output JPEG file.
 * The routines in jcapimin.c will also be needed by a transcoder.
 */
/* Forward declarations */
/*
 * Master selection of compression modules for transcoding.
 * This substitutes for jcinit.c's initialization of the full compressor.
 */
unsafe extern "C" fn transencode_master_selection(
    mut cinfo: j_compress_ptr,
    mut coef_arrays: *mut jvirt_barray_ptr,
) {
    (*cinfo).input_components = 1i32;
    jinit_c_master_control(cinfo, TRUE);
    if 0 != (*cinfo).arith_code {
        (*(*cinfo).err).msg_code = JERR_ARITH_NOTIMPL as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    } else if 0 != (*cinfo).progressive_mode {
        jinit_phuff_encoder(cinfo);
    } else {
        jinit_huff_encoder(cinfo);
    }
    transencode_coef_controller(cinfo, coef_arrays);
    jinit_marker_writer(cinfo);
    (*(*cinfo).mem)
        .realize_virt_arrays
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    (*(*cinfo).marker)
        .write_file_header
        .expect("non-null function pointer")(cinfo);
}
unsafe extern "C" fn start_iMCU_row(mut cinfo: j_compress_ptr) {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    if (*cinfo).comps_in_scan > 1i32 {
        (*coef).MCU_rows_per_iMCU_row = 1i32
    } else if (*coef).iMCU_row_num < (*cinfo).total_iMCU_rows.wrapping_sub(1i32 as c_uint) {
        (*coef).MCU_rows_per_iMCU_row = (*(*cinfo).cur_comp_info[0usize]).v_samp_factor
    } else {
        (*coef).MCU_rows_per_iMCU_row = (*(*cinfo).cur_comp_info[0usize]).last_row_height
    }
    (*coef).mcu_ctr = 0i32 as JDIMENSION;
    (*coef).MCU_vert_offset = 0i32;
}
/*
 * Initialize for a processing pass.
 */
unsafe extern "C" fn start_pass_coef(mut cinfo: j_compress_ptr, mut pass_mode: J_BUF_MODE) {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    if pass_mode as c_uint != JBUF_CRANK_DEST as c_int as c_uint {
        (*(*cinfo).err).msg_code = JERR_BAD_BUFFER_MODE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*coef).iMCU_row_num = 0i32 as JDIMENSION;
    start_iMCU_row(cinfo);
}
/*
 * Process some data.
 * We process the equivalent of one fully interleaved MCU row ("iMCU" row)
 * per call, ie, v_samp_factor block rows for each component in the scan.
 * The data is obtained from the virtual arrays and fed to the entropy coder.
 * Returns TRUE if the iMCU row is completed, FALSE if suspended.
 *
 * NB: input_buf is ignored; it is likely to be a NULL pointer.
 */
unsafe extern "C" fn compress_output(
    mut cinfo: j_compress_ptr,
    mut _input_buf: JSAMPIMAGE,
) -> boolean {
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
    let mut blockcnt: c_int = 0;
    let mut start_col: JDIMENSION = 0;
    let mut buffer: [JBLOCKARRAY; 4] = [0 as *mut JBLOCKROW; 4];
    let mut MCU_buffer: [JBLOCKROW; 10] = [0 as *mut JBLOCK; 10];
    let mut buffer_ptr: JBLOCKROW = 0 as *mut JBLOCK;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    ci = 0i32;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        buffer[ci as usize] = (*(*cinfo).mem)
            .access_virt_barray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            *(*coef)
                .whole_image
                .offset((*compptr).component_index as isize),
            (*coef)
                .iMCU_row_num
                .wrapping_mul((*compptr).v_samp_factor as c_uint),
            (*compptr).v_samp_factor as JDIMENSION,
            FALSE,
        );
        ci += 1
    }
    yoffset = (*coef).MCU_vert_offset;
    while yoffset < (*coef).MCU_rows_per_iMCU_row {
        MCU_col_num = (*coef).mcu_ctr;
        while MCU_col_num < (*cinfo).MCUs_per_row {
            blkn = 0i32;
            ci = 0i32;
            while ci < (*cinfo).comps_in_scan {
                compptr = (*cinfo).cur_comp_info[ci as usize];
                start_col = MCU_col_num.wrapping_mul((*compptr).MCU_width as c_uint);
                blockcnt = if MCU_col_num < last_MCU_col {
                    (*compptr).MCU_width
                } else {
                    (*compptr).last_col_width
                };
                yindex = 0i32;
                while yindex < (*compptr).MCU_height {
                    if (*coef).iMCU_row_num < last_iMCU_row
                        || yindex + yoffset < (*compptr).last_row_height
                    {
                        buffer_ptr = (*buffer[ci as usize].offset((yindex + yoffset) as isize))
                            .offset(start_col as isize);
                        xindex = 0i32;
                        while xindex < blockcnt {
                            let fresh1 = blkn;
                            blkn = blkn + 1;
                            let fresh0 = buffer_ptr;
                            buffer_ptr = buffer_ptr.offset(1);
                            MCU_buffer[fresh1 as usize] = fresh0;
                            xindex += 1
                        }
                    } else {
                        xindex = 0i32
                    }
                    while xindex < (*compptr).MCU_width {
                        MCU_buffer[blkn as usize] = (*coef).dummy_buffer[blkn as usize];
                        (*MCU_buffer[blkn as usize].offset(0isize))[0usize] =
                            (*MCU_buffer[(blkn - 1i32) as usize].offset(0isize))[0usize];
                        blkn += 1;
                        xindex += 1
                    }
                    yindex += 1
                }
                ci += 1
            }
            if 0 == (*(*cinfo).entropy)
                .encode_mcu
                .expect("non-null function pointer")(
                cinfo, MCU_buffer.as_mut_ptr()
            ) {
                (*coef).MCU_vert_offset = yoffset;
                (*coef).mcu_ctr = MCU_col_num;
                return FALSE;
            }
            MCU_col_num = MCU_col_num.wrapping_add(1)
        }
        (*coef).mcu_ctr = 0i32 as JDIMENSION;
        yoffset += 1
    }
    (*coef).iMCU_row_num = (*coef).iMCU_row_num.wrapping_add(1);
    start_iMCU_row(cinfo);
    return TRUE;
}
/*
 * Initialize coefficient buffer controller.
 *
 * Each passed coefficient array must be the right size for that
 * coefficient: width_in_blocks wide and height_in_blocks high,
 * with unitheight at least v_samp_factor.
 */
unsafe extern "C" fn transencode_coef_controller(
    mut cinfo: j_compress_ptr,
    mut coef_arrays: *mut jvirt_barray_ptr,
) {
    let mut coef: my_coef_ptr = 0 as *mut my_coef_controller;
    let mut buffer: JBLOCKROW = 0 as *mut JBLOCK;
    let mut i: c_int = 0;
    coef = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<my_coef_controller>() as c_ulong,
    ) as my_coef_ptr;
    (*cinfo).coef = coef as *mut jpeg_c_coef_controller;
    (*coef).pub_0.start_pass =
        Some(start_pass_coef as unsafe extern "C" fn(_: j_compress_ptr, _: J_BUF_MODE) -> ());
    (*coef).pub_0.compress_data =
        Some(compress_output as unsafe extern "C" fn(_: j_compress_ptr, _: JSAMPIMAGE) -> boolean);
    (*coef).whole_image = coef_arrays;
    buffer = (*(*cinfo).mem)
        .alloc_large
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (C_MAX_BLOCKS_IN_MCU as c_ulong).wrapping_mul(::std::mem::size_of::<JBLOCK>() as c_ulong),
    ) as JBLOCKROW;
    jzero_far(
        buffer as *mut c_void,
        (C_MAX_BLOCKS_IN_MCU as c_ulong).wrapping_mul(::std::mem::size_of::<JBLOCK>() as c_ulong),
    );
    i = 0i32;
    while i < C_MAX_BLOCKS_IN_MCU {
        (*coef).dummy_buffer[i as usize] = buffer.offset(i as isize);
        i += 1
    }
}
