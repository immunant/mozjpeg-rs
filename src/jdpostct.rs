use libc::c_uint;use libc::c_int;use libc::c_void;use libc::c_ulong;use libc::c_long;pub use crate::jerror::C2RustUnnamed_3;
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
pub use crate::jpegint_h::inverse_DCT_method_ptr;
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
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
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
pub type my_post_ptr = *mut my_post_controller;
/*
 * jdpostct.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * It was modified by The libjpeg-turbo Project to include only code relevant
 * to libjpeg-turbo.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains the decompression postprocessing controller.
 * This controller manages the upsampling, color conversion, and color
 * quantization/reduction steps; specifically, it controls the buffering
 * between upsample/color conversion and color quantization/reduction.
 *
 * If no color quantization/reduction is required, then this module has no
 * work to do, and it just hands off to the upsample/color conversion code.
 * An integrated upsample/convert/quantize process would replace this module
 * entirely.
 */
/* Private buffer controller object */
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
unsafe extern "C" fn start_pass_dpost(
    mut cinfo: j_decompress_ptr,
    mut pass_mode: J_BUF_MODE,
) {
    let mut post: my_post_ptr = (*cinfo).post as my_post_ptr;
    match pass_mode as c_uint {
        0 => {
            if 0 != (*cinfo).quantize_colors {
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
                if (*post).buffer.is_null() {
                    (*post).buffer = (*(*cinfo).mem)
                        .access_virt_sarray
                        .expect("non-null function pointer")(
                        cinfo as j_common_ptr,
                        (*post).whole_image,
                        0i32 as JDIMENSION,
                        (*post).strip_height,
                        TRUE,
                    )
                }
            } else {
                (*post).pub_0.post_process_data = (*(*cinfo).upsample).upsample
            }
        }
        3 => {
            if (*post).whole_image.is_null() {
                (*(*cinfo).err).msg_code = JERR_BAD_BUFFER_MODE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
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
            if (*post).whole_image.is_null() {
                (*(*cinfo).err).msg_code = JERR_BAD_BUFFER_MODE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
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
            (*(*cinfo).err).msg_code = JERR_BAD_BUFFER_MODE as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
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
    max_rows = out_rows_avail.wrapping_sub(*out_row_ctr);
    if max_rows > (*post).strip_height {
        max_rows = (*post).strip_height
    }
    num_rows = 0i32 as JDIMENSION;
    (*(*cinfo).upsample)
        .upsample
        .expect("non-null function pointer")(
        cinfo,
        input_buf,
        in_row_group_ctr,
        in_row_groups_avail,
        (*post).buffer,
        &mut num_rows,
        max_rows,
    );
    (*(*cinfo).cquantize)
        .color_quantize
        .expect("non-null function pointer")(
        cinfo,
        (*post).buffer,
        output_buf.offset(*out_row_ctr as isize),
        num_rows as c_int,
    );
    *out_row_ctr = (*out_row_ctr as c_uint).wrapping_add(num_rows)
        as JDIMENSION as JDIMENSION;
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
    if (*post).next_row == 0i32 as c_uint {
        (*post).buffer = (*(*cinfo).mem)
            .access_virt_sarray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*post).whole_image,
            (*post).starting_row,
            (*post).strip_height,
            TRUE,
        )
    }
    old_next_row = (*post).next_row;
    (*(*cinfo).upsample)
        .upsample
        .expect("non-null function pointer")(
        cinfo,
        input_buf,
        in_row_group_ctr,
        in_row_groups_avail,
        (*post).buffer,
        &mut (*post).next_row,
        (*post).strip_height,
    );
    if (*post).next_row > old_next_row {
        num_rows = (*post).next_row.wrapping_sub(old_next_row);
        (*(*cinfo).cquantize)
            .color_quantize
            .expect("non-null function pointer")(
            cinfo,
            (*post).buffer.offset(old_next_row as isize),
            NULL as *mut c_void as JSAMPARRAY,
            num_rows as c_int,
        );
        *out_row_ctr = (*out_row_ctr as c_uint).wrapping_add(num_rows)
            as JDIMENSION as JDIMENSION
    }
    if (*post).next_row >= (*post).strip_height {
        (*post).starting_row =
            ((*post).starting_row as c_uint).wrapping_add((*post).strip_height)
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
    if (*post).next_row == 0i32 as c_uint {
        (*post).buffer = (*(*cinfo).mem)
            .access_virt_sarray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*post).whole_image,
            (*post).starting_row,
            (*post).strip_height,
            FALSE,
        )
    }
    num_rows = (*post).strip_height.wrapping_sub((*post).next_row);
    max_rows = out_rows_avail.wrapping_sub(*out_row_ctr);
    if num_rows > max_rows {
        num_rows = max_rows
    }
    max_rows = (*cinfo).output_height.wrapping_sub((*post).starting_row);
    if num_rows > max_rows {
        num_rows = max_rows
    }
    (*(*cinfo).cquantize)
        .color_quantize
        .expect("non-null function pointer")(
        cinfo,
        (*post).buffer.offset((*post).next_row as isize),
        output_buf.offset(*out_row_ctr as isize),
        num_rows as c_int,
    );
    *out_row_ctr = (*out_row_ctr as c_uint).wrapping_add(num_rows)
        as JDIMENSION as JDIMENSION;
    (*post).next_row = ((*post).next_row as c_uint).wrapping_add(num_rows)
        as JDIMENSION as JDIMENSION;
    if (*post).next_row >= (*post).strip_height {
        (*post).starting_row =
            ((*post).starting_row as c_uint).wrapping_add((*post).strip_height)
                as JDIMENSION as JDIMENSION;
        (*post).next_row = 0i32 as JDIMENSION
    };
}
/* QUANT_2PASS_SUPPORTED */
/*
 * Initialize postprocessing controller.
 */
#[no_mangle]
pub unsafe extern "C" fn jinit_d_post_controller(
    mut cinfo: j_decompress_ptr,
    mut need_full_buffer: boolean,
) {
    let mut post: my_post_ptr = 0 as *mut my_post_controller;
    post = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<my_post_controller>() as c_ulong,
    ) as my_post_ptr;
    (*cinfo).post = post as *mut jpeg_d_post_controller;
    (*post).pub_0.start_pass = Some(
        start_pass_dpost
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
                _: J_BUF_MODE,
            ) -> (),
    );
    (*post).whole_image = NULL as jvirt_sarray_ptr;
    (*post).buffer = NULL as JSAMPARRAY;
    if 0 != (*cinfo).quantize_colors {
        (*post).strip_height = (*cinfo).max_v_samp_factor as JDIMENSION;
        if 0 != need_full_buffer {
            (*post).whole_image = (*(*cinfo).mem)
                .request_virt_sarray
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
        } else {
            (*post).buffer = (*(*cinfo).mem)
                .alloc_sarray
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
