pub use crate::jchuff::c_derived_tbl;
pub use crate::jchuff::jpeg_make_c_derived_tbl;
pub use crate::jchuff::quantize_trellis;
pub use crate::jerror::C2RustUnnamed_3;
pub use crate::jerror::JERR_ARITH_NOTIMPL;
pub use crate::jerror::JERR_BAD_ALIGN_TYPE;
pub use crate::jerror::JERR_BAD_ALLOC_CHUNK;
pub use crate::jerror::JERR_BAD_BUFFER_MODE;
pub use crate::jerror::JERR_BAD_COMPONENT_ID;
pub use crate::jerror::JERR_BAD_CROP_SPEC;
pub use crate::jerror::JERR_BAD_DCTSIZE;
pub use crate::jerror::JERR_BAD_DCT_COEF;
pub use crate::jerror::JERR_BAD_HUFF_TABLE;
pub use crate::jerror::JERR_BAD_IN_COLORSPACE;
pub use crate::jerror::JERR_BAD_J_COLORSPACE;
pub use crate::jerror::JERR_BAD_LENGTH;
pub use crate::jerror::JERR_BAD_LIB_VERSION;
pub use crate::jerror::JERR_BAD_MCU_SIZE;
pub use crate::jerror::JERR_BAD_PARAM;
pub use crate::jerror::JERR_BAD_PARAM_VALUE;
pub use crate::jerror::JERR_BAD_POOL_ID;
pub use crate::jerror::JERR_BAD_PRECISION;
pub use crate::jerror::JERR_BAD_PROGRESSION;
pub use crate::jerror::JERR_BAD_PROG_SCRIPT;
pub use crate::jerror::JERR_BAD_SAMPLING;
pub use crate::jerror::JERR_BAD_SCAN_SCRIPT;
pub use crate::jerror::JERR_BAD_STATE;
pub use crate::jerror::JERR_BAD_STRUCT_SIZE;
pub use crate::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use crate::jerror::JERR_BUFFER_SIZE;
pub use crate::jerror::JERR_CANT_SUSPEND;
pub use crate::jerror::JERR_CCIR601_NOTIMPL;
pub use crate::jerror::JERR_COMPONENT_COUNT;
pub use crate::jerror::JERR_CONVERSION_NOTIMPL;
pub use crate::jerror::JERR_DAC_INDEX;
pub use crate::jerror::JERR_DAC_VALUE;
pub use crate::jerror::JERR_DHT_INDEX;
pub use crate::jerror::JERR_DQT_INDEX;
pub use crate::jerror::JERR_EMPTY_IMAGE;
pub use crate::jerror::JERR_EMS_READ;
pub use crate::jerror::JERR_EMS_WRITE;
pub use crate::jerror::JERR_EOI_EXPECTED;
pub use crate::jerror::JERR_FILE_READ;
pub use crate::jerror::JERR_FILE_WRITE;
pub use crate::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use crate::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use crate::jerror::JERR_HUFF_MISSING_CODE;
pub use crate::jerror::JERR_IMAGE_TOO_BIG;
pub use crate::jerror::JERR_INPUT_EMPTY;
pub use crate::jerror::JERR_INPUT_EOF;
pub use crate::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use crate::jerror::JERR_MISSING_DATA;
pub use crate::jerror::JERR_MODE_CHANGE;
pub use crate::jerror::JERR_NOTIMPL;
pub use crate::jerror::JERR_NOT_COMPILED;
pub use crate::jerror::JERR_NO_BACKING_STORE;
pub use crate::jerror::JERR_NO_HUFF_TABLE;
pub use crate::jerror::JERR_NO_IMAGE;
pub use crate::jerror::JERR_NO_QUANT_TABLE;
pub use crate::jerror::JERR_NO_SOI;
pub use crate::jerror::JERR_OUT_OF_MEMORY;
pub use crate::jerror::JERR_QUANT_COMPONENTS;
pub use crate::jerror::JERR_QUANT_FEW_COLORS;
pub use crate::jerror::JERR_QUANT_MANY_COLORS;
pub use crate::jerror::JERR_SOF_DUPLICATE;
pub use crate::jerror::JERR_SOF_NO_SOS;
pub use crate::jerror::JERR_SOF_UNSUPPORTED;
pub use crate::jerror::JERR_SOI_DUPLICATE;
pub use crate::jerror::JERR_SOS_NO_SOF;
pub use crate::jerror::JERR_TFILE_CREATE;
pub use crate::jerror::JERR_TFILE_READ;
pub use crate::jerror::JERR_TFILE_SEEK;
pub use crate::jerror::JERR_TFILE_WRITE;
pub use crate::jerror::JERR_TOO_LITTLE_DATA;
pub use crate::jerror::JERR_UNKNOWN_MARKER;
pub use crate::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use crate::jerror::JERR_VIRTUAL_BUG;
pub use crate::jerror::JERR_WIDTH_OVERFLOW;
pub use crate::jerror::JERR_XMS_READ;
pub use crate::jerror::JERR_XMS_WRITE;
pub use crate::jerror::JMSG_COPYRIGHT;
pub use crate::jerror::JMSG_LASTMSGCODE;
pub use crate::jerror::JMSG_NOMESSAGE;
pub use crate::jerror::JMSG_VERSION;
pub use crate::jerror::JTRC_16BIT_TABLES;
pub use crate::jerror::JTRC_ADOBE;
pub use crate::jerror::JTRC_APP0;
pub use crate::jerror::JTRC_APP14;
pub use crate::jerror::JTRC_DAC;
pub use crate::jerror::JTRC_DHT;
pub use crate::jerror::JTRC_DQT;
pub use crate::jerror::JTRC_DRI;
pub use crate::jerror::JTRC_EMS_CLOSE;
pub use crate::jerror::JTRC_EMS_OPEN;
pub use crate::jerror::JTRC_EOI;
pub use crate::jerror::JTRC_HUFFBITS;
pub use crate::jerror::JTRC_JFIF;
pub use crate::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use crate::jerror::JTRC_JFIF_EXTENSION;
pub use crate::jerror::JTRC_JFIF_THUMBNAIL;
pub use crate::jerror::JTRC_MISC_MARKER;
pub use crate::jerror::JTRC_PARMLESS_MARKER;
pub use crate::jerror::JTRC_QUANTVALS;
pub use crate::jerror::JTRC_QUANT_3_NCOLORS;
pub use crate::jerror::JTRC_QUANT_NCOLORS;
pub use crate::jerror::JTRC_QUANT_SELECTED;
pub use crate::jerror::JTRC_RECOVERY_ACTION;
pub use crate::jerror::JTRC_RST;
pub use crate::jerror::JTRC_SMOOTH_NOTIMPL;
pub use crate::jerror::JTRC_SOF;
pub use crate::jerror::JTRC_SOF_COMPONENT;
pub use crate::jerror::JTRC_SOI;
pub use crate::jerror::JTRC_SOS;
pub use crate::jerror::JTRC_SOS_COMPONENT;
pub use crate::jerror::JTRC_SOS_PARAMS;
pub use crate::jerror::JTRC_TFILE_CLOSE;
pub use crate::jerror::JTRC_TFILE_OPEN;
pub use crate::jerror::JTRC_THUMB_JPEG;
pub use crate::jerror::JTRC_THUMB_PALETTE;
pub use crate::jerror::JTRC_THUMB_RGB;
pub use crate::jerror::JTRC_UNKNOWN_IDS;
pub use crate::jerror::JTRC_XMS_CLOSE;
pub use crate::jerror::JTRC_XMS_OPEN;
pub use crate::jerror::JWRN_ADOBE_XFORM;
pub use crate::jerror::JWRN_BOGUS_ICC;
pub use crate::jerror::JWRN_BOGUS_PROGRESSION;
pub use crate::jerror::JWRN_EXTRANEOUS_DATA;
pub use crate::jerror::JWRN_HIT_MARKER;
pub use crate::jerror::JWRN_HUFF_BAD_CODE;
pub use crate::jerror::JWRN_JFIF_MAJOR;
pub use crate::jerror::JWRN_JPEG_EOF;
pub use crate::jerror::JWRN_MUST_RESYNC;
pub use crate::jerror::JWRN_NOT_SEQUENTIAL;
pub use crate::jerror::JWRN_TOO_MUCH_DATA;
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
pub use crate::jpegint_h::jround_up;
pub use crate::jpegint_h::jzero_far;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
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
pub use crate::jpeglib_h::C_MAX_BLOCKS_IN_MCU;
pub use crate::jpeglib_h::DCTSIZE;
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
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
use libc;
use libc::c_int;
use libc::c_long;
use libc::c_uint;
use libc::c_ulong;
use libc::c_void;
pub type my_coef_ptr = *mut my_coef_controller;
/*
 * jccoefct.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1997, Thomas G. Lane.
 * It was modified by The libjpeg-turbo Project to include only code and
 * information relevant to libjpeg-turbo.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains the coefficient buffer controller for compression.
 * This controller is the top level of the JPEG compressor proper.
 * The coefficient buffer lies between forward-DCT and entropy encoding steps.
 */
/* We use a full-image coefficient buffer when doing Huffman optimization,
 * and also for writing multiple-scan JPEG files.  In all cases, the DCT
 * step is run during the first pass, and subsequent passes need only read
 * the buffered coefficients.
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
    pub MCU_buffer: [JBLOCKROW; 10],
    pub whole_image: [jvirt_barray_ptr; 10],
    pub whole_image_uq: [jvirt_barray_ptr; 10],
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
    (*coef).iMCU_row_num = 0i32 as JDIMENSION;
    start_iMCU_row(cinfo);
    match pass_mode as c_uint {
        0 => {
            if !(*coef).whole_image[0usize].is_null() {
                (*(*cinfo).err).msg_code = JERR_BAD_BUFFER_MODE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            (*coef).pub_0.compress_data = Some(
                compress_data as unsafe extern "C" fn(_: j_compress_ptr, _: JSAMPIMAGE) -> boolean,
            )
        }
        3 => {
            if (*coef).whole_image[0usize].is_null() {
                (*(*cinfo).err).msg_code = JERR_BAD_BUFFER_MODE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            (*coef).pub_0.compress_data = Some(
                compress_first_pass
                    as unsafe extern "C" fn(_: j_compress_ptr, _: JSAMPIMAGE) -> boolean,
            )
        }
        2 => {
            if (*coef).whole_image[0usize].is_null() {
                (*(*cinfo).err).msg_code = JERR_BAD_BUFFER_MODE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            (*coef).pub_0.compress_data = Some(
                compress_output
                    as unsafe extern "C" fn(_: j_compress_ptr, _: JSAMPIMAGE) -> boolean,
            )
        }
        4 => {
            if (*coef).whole_image[0usize].is_null() {
                (*(*cinfo).err).msg_code = JERR_BAD_BUFFER_MODE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            (*coef).pub_0.compress_data = Some(
                compress_trellis_pass
                    as unsafe extern "C" fn(_: j_compress_ptr, _: JSAMPIMAGE) -> boolean,
            )
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_BAD_BUFFER_MODE as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    };
}
/* Forward declarations */
/*
 * Process some data in the single-pass case.
 * We process the equivalent of one fully interleaved MCU row ("iMCU" row)
 * per call, ie, v_samp_factor block rows for each component in the image.
 * Returns TRUE if the iMCU row is completed, FALSE if suspended.
 *
 * NB: input_buf contains a plane for each component in image,
 * which we index according to the component's SOF position.
 */
unsafe extern "C" fn compress_data(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPIMAGE,
) -> boolean {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    /* index of current MCU within row */
    let mut MCU_col_num: JDIMENSION = 0;
    let mut last_MCU_col: JDIMENSION = (*cinfo).MCUs_per_row.wrapping_sub(1i32 as c_uint);
    let mut last_iMCU_row: JDIMENSION = (*cinfo).total_iMCU_rows.wrapping_sub(1i32 as c_uint);
    let mut blkn: c_int = 0;
    let mut bi: c_int = 0;
    let mut ci: c_int = 0;
    let mut yindex: c_int = 0;
    let mut yoffset: c_int = 0;
    let mut blockcnt: c_int = 0;
    let mut ypos: JDIMENSION = 0;
    let mut xpos: JDIMENSION = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    yoffset = (*coef).MCU_vert_offset;
    while yoffset < (*coef).MCU_rows_per_iMCU_row {
        MCU_col_num = (*coef).mcu_ctr;
        while MCU_col_num <= last_MCU_col {
            blkn = 0i32;
            ci = 0i32;
            while ci < (*cinfo).comps_in_scan {
                compptr = (*cinfo).cur_comp_info[ci as usize];
                blockcnt = if MCU_col_num < last_MCU_col {
                    (*compptr).MCU_width
                } else {
                    (*compptr).last_col_width
                };
                xpos = MCU_col_num.wrapping_mul((*compptr).MCU_sample_width as c_uint);
                ypos = (yoffset * DCTSIZE) as JDIMENSION;
                yindex = 0i32;
                while yindex < (*compptr).MCU_height {
                    if (*coef).iMCU_row_num < last_iMCU_row
                        || yoffset + yindex < (*compptr).last_row_height
                    {
                        (*(*cinfo).fdct)
                            .forward_DCT
                            .expect("non-null function pointer")(
                            cinfo,
                            compptr,
                            *input_buf.offset((*compptr).component_index as isize),
                            (*coef).MCU_buffer[blkn as usize],
                            ypos,
                            xpos,
                            blockcnt as JDIMENSION,
                            NULL as JBLOCKROW,
                        );
                        if blockcnt < (*compptr).MCU_width {
                            jzero_far(
                                (*coef).MCU_buffer[(blkn + blockcnt) as usize] as *mut c_void,
                                (((*compptr).MCU_width - blockcnt) as c_ulong)
                                    .wrapping_mul(::std::mem::size_of::<JBLOCK>() as c_ulong),
                            );
                            bi = blockcnt;
                            while bi < (*compptr).MCU_width {
                                (*(*coef).MCU_buffer[(blkn + bi) as usize].offset(0isize))
                                    [0usize] = (*(*coef).MCU_buffer[(blkn + bi - 1i32) as usize]
                                    .offset(0isize))[0usize];
                                bi += 1
                            }
                        }
                    } else {
                        jzero_far(
                            (*coef).MCU_buffer[blkn as usize] as *mut c_void,
                            ((*compptr).MCU_width as c_ulong).wrapping_mul(::std::mem::size_of::<
                                JBLOCK,
                            >(
                            )
                                as c_ulong),
                        );
                        bi = 0i32;
                        while bi < (*compptr).MCU_width {
                            (*(*coef).MCU_buffer[(blkn + bi) as usize].offset(0isize))[0usize] =
                                (*(*coef).MCU_buffer[(blkn - 1i32) as usize].offset(0isize))
                                    [0usize];
                            bi += 1
                        }
                    }
                    blkn += (*compptr).MCU_width;
                    ypos = (ypos as c_uint).wrapping_add(DCTSIZE as c_uint) as JDIMENSION
                        as JDIMENSION;
                    yindex += 1
                }
                ci += 1
            }
            if 0 == (*(*cinfo).entropy)
                .encode_mcu
                .expect("non-null function pointer")(
                cinfo, (*coef).MCU_buffer.as_mut_ptr()
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
 * Process some data in the first pass of a multi-pass case.
 * We process the equivalent of one fully interleaved MCU row ("iMCU" row)
 * per call, ie, v_samp_factor block rows for each component in the image.
 * This amount of data is read from the source buffer, DCT'd and quantized,
 * and saved into the virtual arrays.  We also generate suitable dummy blocks
 * as needed at the right and lower edges.  (The dummy blocks are constructed
 * in the virtual arrays, which have been padded appropriately.)  This makes
 * it possible for subsequent passes not to worry about real vs. dummy blocks.
 *
 * We must also emit the data to the entropy encoder.  This is conveniently
 * done by calling compress_output() after we've loaded the current strip
 * of the virtual arrays.
 *
 * NB: input_buf contains a plane for each component in image.  All
 * components are DCT'd and loaded into the virtual arrays in this pass.
 * However, it may be that only a subset of the components are emitted to
 * the entropy encoder during this first pass; be careful about looking
 * at the scan-dependent variables (MCU dimensions, etc).
 */
unsafe extern "C" fn compress_first_pass(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPIMAGE,
) -> boolean {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    let mut last_iMCU_row: JDIMENSION = (*cinfo).total_iMCU_rows.wrapping_sub(1i32 as c_uint);
    let mut blocks_across: JDIMENSION = 0;
    let mut MCUs_across: JDIMENSION = 0;
    let mut MCUindex: JDIMENSION = 0;
    let mut bi: c_int = 0;
    let mut ci: c_int = 0;
    let mut h_samp_factor: c_int = 0;
    let mut block_row: c_int = 0;
    let mut block_rows: c_int = 0;
    let mut ndummy: c_int = 0;
    let mut lastDC: JCOEF = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut thisblockrow: JBLOCKROW = 0 as *mut JBLOCK;
    let mut lastblockrow: JBLOCKROW = 0 as *mut JBLOCK;
    let mut buffer_dst: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        buffer = (*(*cinfo).mem)
            .access_virt_barray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*coef).whole_image[ci as usize],
            (*coef)
                .iMCU_row_num
                .wrapping_mul((*compptr).v_samp_factor as c_uint),
            (*compptr).v_samp_factor as JDIMENSION,
            TRUE,
        );
        buffer_dst = (*(*cinfo).mem)
            .access_virt_barray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*coef).whole_image_uq[ci as usize],
            (*coef)
                .iMCU_row_num
                .wrapping_mul((*compptr).v_samp_factor as c_uint),
            (*compptr).v_samp_factor as JDIMENSION,
            TRUE,
        );
        if (*coef).iMCU_row_num < last_iMCU_row {
            block_rows = (*compptr).v_samp_factor
        } else {
            block_rows = (*compptr)
                .height_in_blocks
                .wrapping_rem((*compptr).v_samp_factor as c_uint) as c_int;
            if block_rows == 0i32 {
                block_rows = (*compptr).v_samp_factor
            }
        }
        blocks_across = (*compptr).width_in_blocks;
        h_samp_factor = (*compptr).h_samp_factor;
        ndummy = blocks_across.wrapping_rem(h_samp_factor as c_uint) as c_int;
        if ndummy > 0i32 {
            ndummy = h_samp_factor - ndummy
        }
        block_row = 0i32;
        while block_row < block_rows {
            thisblockrow = *buffer.offset(block_row as isize);
            (*(*cinfo).fdct)
                .forward_DCT
                .expect("non-null function pointer")(
                cinfo,
                compptr,
                *input_buf.offset(ci as isize),
                thisblockrow,
                (block_row * DCTSIZE) as JDIMENSION,
                0i32 as JDIMENSION,
                blocks_across,
                *buffer_dst.offset(block_row as isize),
            );
            if ndummy > 0i32 {
                thisblockrow = thisblockrow.offset(blocks_across as isize);
                jzero_far(
                    thisblockrow as *mut c_void,
                    (ndummy as c_ulong).wrapping_mul(::std::mem::size_of::<JBLOCK>() as c_ulong),
                );
                lastDC = (*thisblockrow.offset(-1i32 as isize))[0usize];
                bi = 0i32;
                while bi < ndummy {
                    (*thisblockrow.offset(bi as isize))[0usize] = lastDC;
                    bi += 1
                }
            }
            block_row += 1
        }
        if (*coef).iMCU_row_num == last_iMCU_row {
            blocks_across = (blocks_across as c_uint).wrapping_add(ndummy as c_uint) as JDIMENSION
                as JDIMENSION;
            MCUs_across = blocks_across.wrapping_div(h_samp_factor as c_uint);
            block_row = block_rows;
            while block_row < (*compptr).v_samp_factor {
                thisblockrow = *buffer.offset(block_row as isize);
                lastblockrow = *buffer.offset((block_row - 1i32) as isize);
                jzero_far(
                    thisblockrow as *mut c_void,
                    (blocks_across as c_ulong)
                        .wrapping_mul(::std::mem::size_of::<JBLOCK>() as c_ulong),
                );
                MCUindex = 0i32 as JDIMENSION;
                while MCUindex < MCUs_across {
                    lastDC = (*lastblockrow.offset((h_samp_factor - 1i32) as isize))[0usize];
                    bi = 0i32;
                    while bi < h_samp_factor {
                        (*thisblockrow.offset(bi as isize))[0usize] = lastDC;
                        bi += 1
                    }
                    thisblockrow = thisblockrow.offset(h_samp_factor as isize);
                    lastblockrow = lastblockrow.offset(h_samp_factor as isize);
                    MCUindex = MCUindex.wrapping_add(1)
                }
                block_row += 1
            }
        }
        ci += 1;
        compptr = compptr.offset(1isize)
    }
    return compress_output(cinfo, input_buf);
}
unsafe extern "C" fn compress_trellis_pass(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPIMAGE,
) -> boolean {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    let mut last_iMCU_row: JDIMENSION = (*cinfo).total_iMCU_rows.wrapping_sub(1i32 as c_uint);
    let mut blocks_across: JDIMENSION = 0;
    let mut MCUs_across: JDIMENSION = 0;
    let mut MCUindex: JDIMENSION = 0;
    let mut bi: c_int = 0;
    let mut ci: c_int = 0;
    let mut h_samp_factor: c_int = 0;
    let mut block_row: c_int = 0;
    let mut block_rows: c_int = 0;
    let mut ndummy: c_int = 0;
    let mut lastDC: JCOEF = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut thisblockrow: JBLOCKROW = 0 as *mut JBLOCK;
    let mut lastblockrow: JBLOCKROW = 0 as *mut JBLOCK;
    let mut buffer_dst: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    ci = 0i32;
    while ci < (*cinfo).comps_in_scan {
        let mut dctbl_data: c_derived_tbl = c_derived_tbl {
            ehufco: [0; 256],
            ehufsi: [0; 256],
        };
        let mut dctbl: *mut c_derived_tbl = &mut dctbl_data;
        let mut actbl_data: c_derived_tbl = c_derived_tbl {
            ehufco: [0; 256],
            ehufsi: [0; 256],
        };
        let mut actbl: *mut c_derived_tbl = &mut actbl_data;
        compptr = (*cinfo).cur_comp_info[ci as usize];
        jpeg_make_c_derived_tbl(cinfo, TRUE, (*compptr).dc_tbl_no, &mut dctbl);
        jpeg_make_c_derived_tbl(cinfo, FALSE, (*compptr).ac_tbl_no, &mut actbl);
        buffer = (*(*cinfo).mem)
            .access_virt_barray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*coef).whole_image[(*compptr).component_index as usize],
            (*coef)
                .iMCU_row_num
                .wrapping_mul((*compptr).v_samp_factor as c_uint),
            (*compptr).v_samp_factor as JDIMENSION,
            TRUE,
        );
        buffer_dst = (*(*cinfo).mem)
            .access_virt_barray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*coef).whole_image_uq[(*compptr).component_index as usize],
            (*coef)
                .iMCU_row_num
                .wrapping_mul((*compptr).v_samp_factor as c_uint),
            (*compptr).v_samp_factor as JDIMENSION,
            TRUE,
        );
        if (*coef).iMCU_row_num < last_iMCU_row {
            block_rows = (*compptr).v_samp_factor
        } else {
            block_rows = (*compptr)
                .height_in_blocks
                .wrapping_rem((*compptr).v_samp_factor as c_uint) as c_int;
            if block_rows == 0i32 {
                block_rows = (*compptr).v_samp_factor
            }
        }
        blocks_across = (*compptr).width_in_blocks;
        h_samp_factor = (*compptr).h_samp_factor;
        ndummy = blocks_across.wrapping_rem(h_samp_factor as c_uint) as c_int;
        if ndummy > 0i32 {
            ndummy = h_samp_factor - ndummy
        }
        lastDC = 0i32 as JCOEF;
        block_row = 0i32;
        while block_row < block_rows {
            thisblockrow = *buffer.offset(block_row as isize);
            lastblockrow = if block_row > 0i32 {
                *buffer.offset((block_row - 1i32) as isize)
            } else {
                NULL as JBLOCKROW
            };
            quantize_trellis(
                cinfo,
                dctbl,
                actbl,
                thisblockrow,
                *buffer_dst.offset(block_row as isize),
                blocks_across,
                (*cinfo).quant_tbl_ptrs[(*compptr).quant_tbl_no as usize],
                (*(*cinfo).master).norm_src[(*compptr).quant_tbl_no as usize].as_mut_ptr(),
                (*(*cinfo).master).norm_coef[(*compptr).quant_tbl_no as usize].as_mut_ptr(),
                &mut lastDC,
                lastblockrow,
                *buffer_dst.offset((block_row - 1i32) as isize),
            );
            if ndummy > 0i32 {
                thisblockrow = thisblockrow.offset(blocks_across as isize);
                jzero_far(
                    thisblockrow as *mut c_void,
                    (ndummy as c_ulong).wrapping_mul(::std::mem::size_of::<JBLOCK>() as c_ulong),
                );
                lastDC = (*thisblockrow.offset(-1i32 as isize))[0usize];
                bi = 0i32;
                while bi < ndummy {
                    (*thisblockrow.offset(bi as isize))[0usize] = lastDC;
                    bi += 1
                }
            }
            block_row += 1
        }
        if (*coef).iMCU_row_num == last_iMCU_row {
            blocks_across = (blocks_across as c_uint).wrapping_add(ndummy as c_uint) as JDIMENSION
                as JDIMENSION;
            MCUs_across = blocks_across.wrapping_div(h_samp_factor as c_uint);
            block_row = block_rows;
            while block_row < (*compptr).v_samp_factor {
                thisblockrow = *buffer.offset(block_row as isize);
                lastblockrow = *buffer.offset((block_row - 1i32) as isize);
                jzero_far(
                    thisblockrow as *mut c_void,
                    (blocks_across as c_ulong)
                        .wrapping_mul(::std::mem::size_of::<JBLOCK>() as c_ulong),
                );
                MCUindex = 0i32 as JDIMENSION;
                while MCUindex < MCUs_across {
                    lastDC = (*lastblockrow.offset((h_samp_factor - 1i32) as isize))[0usize];
                    bi = 0i32;
                    while bi < h_samp_factor {
                        (*thisblockrow.offset(bi as isize))[0usize] = lastDC;
                        bi += 1
                    }
                    thisblockrow = thisblockrow.offset(h_samp_factor as isize);
                    lastblockrow = lastblockrow.offset(h_samp_factor as isize);
                    MCUindex = MCUindex.wrapping_add(1)
                }
                block_row += 1
            }
        }
        ci += 1
    }
    return compress_output(cinfo, input_buf);
}
/*
 * Process some data in subsequent passes of a multi-pass case.
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
                .encode_mcu
                .expect("non-null function pointer")(
                cinfo, (*coef).MCU_buffer.as_mut_ptr()
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
/* FULL_COEF_BUFFER_SUPPORTED */
/*
 * Initialize coefficient buffer controller.
 */
#[no_mangle]
pub unsafe extern "C" fn jinit_c_coef_controller(
    mut cinfo: j_compress_ptr,
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
    (*cinfo).coef = coef as *mut jpeg_c_coef_controller;
    (*coef).pub_0.start_pass =
        Some(start_pass_coef as unsafe extern "C" fn(_: j_compress_ptr, _: J_BUF_MODE) -> ());
    if 0 != need_full_buffer {
        let mut ci: c_int = 0;
        let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
        ci = 0i32;
        compptr = (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
            (*coef).whole_image[ci as usize] = (*(*cinfo).mem)
                .request_virt_barray
                .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                JPOOL_IMAGE,
                FALSE,
                jround_up(
                    (*compptr).width_in_blocks as c_long,
                    (*compptr).h_samp_factor as c_long,
                ) as JDIMENSION,
                jround_up(
                    (*compptr).height_in_blocks as c_long,
                    (*compptr).v_samp_factor as c_long,
                ) as JDIMENSION,
                (*compptr).v_samp_factor as JDIMENSION,
            );
            (*coef).whole_image_uq[ci as usize] = (*(*cinfo).mem)
                .request_virt_barray
                .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                JPOOL_IMAGE,
                FALSE,
                jround_up(
                    (*compptr).width_in_blocks as c_long,
                    (*compptr).h_samp_factor as c_long,
                ) as JDIMENSION,
                jround_up(
                    (*compptr).height_in_blocks as c_long,
                    (*compptr).v_samp_factor as c_long,
                ) as JDIMENSION,
                (*compptr).v_samp_factor as JDIMENSION,
            );
            ci += 1;
            compptr = compptr.offset(1isize)
        }
    } else {
        let mut buffer: JBLOCKROW = 0 as *mut JBLOCK;
        let mut i: c_int = 0;
        buffer = (*(*cinfo).mem)
            .alloc_large
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (C_MAX_BLOCKS_IN_MCU as c_ulong)
                .wrapping_mul(::std::mem::size_of::<JBLOCK>() as c_ulong),
        ) as JBLOCKROW;
        i = 0i32;
        while i < C_MAX_BLOCKS_IN_MCU {
            (*coef).MCU_buffer[i as usize] = buffer.offset(i as isize);
            i += 1
        }
        (*coef).whole_image[0usize] = NULL as jvirt_barray_ptr
    };
}
