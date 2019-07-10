use libc::c_void;use libc::c_int;use libc::c_uint;use libc::c_long;use libc::intptr_t;pub use crate::jdcoefct::my_coef_controller;
pub use crate::jdcoefct::my_coef_ptr;
pub use crate::jdcoefct::start_iMCU_row;
pub use crate::jdmainct::my_main_controller;
pub use crate::jdmainct::my_main_ptr;
pub use crate::jdmainct::set_wraparound_pointers;
pub use crate::jdmainct::CTX_PREPARE_FOR_IMCU;
pub use crate::jdsample::my_upsample_ptr;
pub use crate::jdsample::my_upsampler;
pub use crate::jdsample::upsample1_ptr;
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
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jdiv_round_up;
pub use crate::jpegint_h::jinit_master_decompress;
pub use crate::jpegint_h::jinit_upsampler;
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
pub use crate::jpegint_h::DSTATE_BUFIMAGE;
pub use crate::jpegint_h::DSTATE_BUFPOST;
pub use crate::jpegint_h::DSTATE_PRELOAD;
pub use crate::jpegint_h::DSTATE_PRESCAN;
pub use crate::jpegint_h::DSTATE_RAW_OK;
pub use crate::jpegint_h::DSTATE_READY;
pub use crate::jpegint_h::DSTATE_SCANNING;
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
pub use crate::jpeglib_h::JPEG_REACHED_EOI;
pub use crate::jpeglib_h::JPEG_REACHED_SOS;
pub use crate::jpeglib_h::JPEG_ROW_COMPLETED;
pub use crate::jpeglib_h::JPEG_SUSPENDED;
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
/* Found valid image datastream */
/* Found valid table-specs-only datastream */
/* If you pass require_image = TRUE (normal case), you need not check for
 * a TABLES_ONLY return code; an abbreviated file will cause an error exit.
 * JPEG_SUSPENDED is only possible if you use a data source module that can
 * give a suspension return (the stdio source module doesn't).
 */
/* Main entry points for decompression */
/*
 * Decompression initialization.
 * jpeg_read_header must be completed before calling this.
 *
 * If a multipass operating mode was selected, this will do all but the
 * last pass, and thus may take a great deal of time.
 *
 * Returns FALSE if suspended.  The return value need be inspected only if
 * a suspending data source is used.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_start_decompress(
    mut cinfo: j_decompress_ptr,
) -> boolean {
    if (*cinfo).global_state == DSTATE_READY {
        jinit_master_decompress(cinfo);
        if 0 != (*cinfo).buffered_image {
            (*cinfo).global_state = DSTATE_BUFIMAGE;
            return TRUE;
        }
        (*cinfo).global_state = DSTATE_PRELOAD
    }
    if (*cinfo).global_state == DSTATE_PRELOAD {
        if 0 != (*(*cinfo).inputctl).has_multiple_scans {
            loop {
                let mut retcode: c_int = 0;
                if !(*cinfo).progress.is_null() {
                    (*(*cinfo).progress)
                        .progress_monitor
                        .expect("non-null function pointer")(
                        cinfo as j_common_ptr,
                    );
                }
                retcode = (*(*cinfo).inputctl)
                    .consume_input
                    .expect("non-null function pointer")(cinfo);
                if retcode == JPEG_SUSPENDED {
                    return FALSE;
                }
                if retcode == JPEG_REACHED_EOI {
                    break;
                }
                if !(*cinfo).progress.is_null()
                    && (retcode == JPEG_ROW_COMPLETED
                        || retcode == JPEG_REACHED_SOS)
                {
                    (*(*cinfo).progress).pass_counter += 1;
                    if (*(*cinfo).progress).pass_counter >= (*(*cinfo).progress).pass_limit {
                        (*(*cinfo).progress).pass_limit += (*cinfo).total_iMCU_rows as c_long
                    }
                }
            }
        }
        (*cinfo).output_scan_number = (*cinfo).input_scan_number
    } else if (*cinfo).global_state != DSTATE_PRESCAN {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    return output_pass_setup(cinfo);
}
/*
 * jdapistd.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2010, 2015-2018, D. R. Commander.
 * Copyright (C) 2015, Google, Inc.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains application interface code for the decompression half
 * of the JPEG library.  These are the "standard" API routines that are
 * used in the normal full-decompression case.  They are not used by a
 * transcoding-only application.  Note that if an application links in
 * jpeg_start_decompress, it will end up linking in the entire decompressor.
 * We thus must separate this file from jdapimin.c to avoid linking the
 * whole decompression library into a transcoder.
 */
/* Forward declarations */
/*
 * Set up for an output pass, and perform any dummy pass(es) needed.
 * Common subroutine for jpeg_start_decompress and jpeg_start_output.
 * Entry: global_state = DSTATE_PRESCAN only if previously suspended.
 * Exit: If done, returns TRUE and sets global_state for proper output mode.
 *       If suspended, returns FALSE and sets global_state = DSTATE_PRESCAN.
 */
unsafe extern "C" fn output_pass_setup(
    mut cinfo: j_decompress_ptr,
) -> boolean {
    if (*cinfo).global_state != DSTATE_PRESCAN {
        (*(*cinfo).master)
            .prepare_for_output_pass
            .expect("non-null function pointer")(cinfo);
        (*cinfo).output_scanline = 0i32 as JDIMENSION;
        (*cinfo).global_state = DSTATE_PRESCAN
    }
    while 0 != (*(*cinfo).master).is_dummy_pass {
        while (*cinfo).output_scanline < (*cinfo).output_height {
            let mut last_scanline: JDIMENSION = 0;
            if !(*cinfo).progress.is_null() {
                (*(*cinfo).progress).pass_counter = (*cinfo).output_scanline as c_long;
                (*(*cinfo).progress).pass_limit = (*cinfo).output_height as c_long;
                (*(*cinfo).progress)
                    .progress_monitor
                    .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            last_scanline = (*cinfo).output_scanline;
            (*(*cinfo).main)
                .process_data
                .expect("non-null function pointer")(
                cinfo,
                NULL as *mut c_void as JSAMPARRAY,
                &mut (*cinfo).output_scanline,
                0i32 as JDIMENSION,
            );
            if (*cinfo).output_scanline == last_scanline {
                return FALSE;
            }
        }
        (*(*cinfo).master)
            .finish_output_pass
            .expect("non-null function pointer")(cinfo);
        (*(*cinfo).master)
            .prepare_for_output_pass
            .expect("non-null function pointer")(cinfo);
        (*cinfo).output_scanline = 0i32 as JDIMENSION
    }
    (*cinfo).global_state = if 0 != (*cinfo).raw_data_out {
        DSTATE_RAW_OK
    } else {
        DSTATE_SCANNING
    };
    return TRUE;
}
/*
 * Enable partial scanline decompression
 *
 * Must be called after jpeg_start_decompress() and before any calls to
 * jpeg_read_scanlines() or jpeg_skip_scanlines().
 *
 * Refer to libjpeg.txt for more information.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_crop_scanline(
    mut cinfo: j_decompress_ptr,
    mut xoffset: *mut JDIMENSION,
    mut width: *mut JDIMENSION,
) {
    let mut ci: c_int = 0;
    let mut align: c_int = 0;
    let mut orig_downsampled_width: c_int = 0;
    let mut input_xoffset: JDIMENSION = 0;
    let mut reinit_upsampler: boolean = FALSE;
    let mut compptr: *mut jpeg_component_info =
        0 as *mut jpeg_component_info;
    if (*cinfo).global_state != DSTATE_SCANNING
        || (*cinfo).output_scanline != 0i32 as c_uint
    {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if xoffset.is_null() || width.is_null() {
        (*(*cinfo).err).msg_code = JERR_BAD_CROP_SPEC as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if *width == 0i32 as c_uint || (*xoffset).wrapping_add(*width) > (*cinfo).output_width {
        (*(*cinfo).err).msg_code = JERR_WIDTH_OVERFLOW as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if *width == (*cinfo).output_width {
        return;
    }
    if (*cinfo).comps_in_scan == 1i32 && (*cinfo).num_components == 1i32 {
        align = (*cinfo).min_DCT_scaled_size
    } else {
        align = (*cinfo).min_DCT_scaled_size * (*cinfo).max_h_samp_factor
    }
    input_xoffset = *xoffset;
    *xoffset = input_xoffset
        .wrapping_div(align as c_uint)
        .wrapping_mul(align as c_uint);
    *width = (*width).wrapping_add(input_xoffset).wrapping_sub(*xoffset);
    (*cinfo).output_width = *width;
    (*(*cinfo).master).first_iMCU_col =
        (*xoffset as c_long as JDIMENSION as c_long
            / align as c_long) as JDIMENSION;
    (*(*cinfo).master).last_iMCU_col = (jdiv_round_up(
        (*xoffset).wrapping_add((*cinfo).output_width) as c_long,
        align as c_long,
    ) as JDIMENSION)
        .wrapping_sub(1i32 as c_uint);
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        let mut hsf: c_int =
            if (*cinfo).comps_in_scan == 1i32 && (*cinfo).num_components == 1i32 {
                1i32
            } else {
                (*compptr).h_samp_factor
            };
        orig_downsampled_width = (*compptr).downsampled_width as c_int;
        (*compptr).downsampled_width = jdiv_round_up(
            (*cinfo)
                .output_width
                .wrapping_mul((*compptr).h_samp_factor as c_uint) as c_long,
            (*cinfo).max_h_samp_factor as c_long,
        ) as JDIMENSION;
        if (*compptr).downsampled_width < 2i32 as c_uint && orig_downsampled_width >= 2i32 {
            reinit_upsampler = TRUE
        }
        (*(*cinfo).master).first_MCU_col[ci as usize] =
            ((*xoffset).wrapping_mul(hsf as c_uint) as c_long
                as JDIMENSION as c_long
                / align as c_long) as JDIMENSION;
        (*(*cinfo).master).last_MCU_col[ci as usize] = (jdiv_round_up(
            (*xoffset)
                .wrapping_add((*cinfo).output_width)
                .wrapping_mul(hsf as c_uint) as c_long,
            align as c_long,
        ) as JDIMENSION)
            .wrapping_sub(1i32 as c_uint);
        ci += 1;
        compptr = compptr.offset(1isize)
    }
    if 0 != reinit_upsampler {
        (*(*cinfo).master).jinit_upsampler_no_alloc = TRUE;
        jinit_upsampler(cinfo);
        (*(*cinfo).master).jinit_upsampler_no_alloc = FALSE
    };
}
/*
 * Read some scanlines of data from the JPEG decompressor.
 *
 * The return value will be the number of lines actually read.
 * This may be less than the number requested in several cases,
 * including bottom of image, data source suspension, and operating
 * modes that emit multiple scanlines at a time.
 *
 * Note: we warn about excess calls to jpeg_read_scanlines() since
 * this likely signals an application programmer error.  However,
 * an oversize buffer (max_lines > scanlines remaining) is not an error.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_read_scanlines(
    mut cinfo: j_decompress_ptr,
    mut scanlines: JSAMPARRAY,
    mut max_lines: JDIMENSION,
) -> JDIMENSION {
    let mut row_ctr: JDIMENSION = 0;
    if (*cinfo).global_state != DSTATE_SCANNING {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*cinfo).output_scanline >= (*cinfo).output_height {
        (*(*cinfo).err).msg_code = JWRN_TOO_MUCH_DATA as c_int;
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer")(
            cinfo as j_common_ptr, -1i32
        );
        return 0i32 as JDIMENSION;
    }
    if !(*cinfo).progress.is_null() {
        (*(*cinfo).progress).pass_counter = (*cinfo).output_scanline as c_long;
        (*(*cinfo).progress).pass_limit = (*cinfo).output_height as c_long;
        (*(*cinfo).progress)
            .progress_monitor
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    row_ctr = 0i32 as JDIMENSION;
    (*(*cinfo).main)
        .process_data
        .expect("non-null function pointer")(cinfo, scanlines, &mut row_ctr, max_lines);
    (*cinfo).output_scanline = ((*cinfo).output_scanline as c_uint).wrapping_add(row_ctr)
        as JDIMENSION
        as JDIMENSION;
    return row_ctr;
}
/* Dummy color convert function used by jpeg_skip_scanlines() */
unsafe extern "C" fn noop_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
}
/* Dummy quantize function used by jpeg_skip_scanlines() */
unsafe extern "C" fn noop_quantize(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
}
/*
 * In some cases, it is best to call jpeg_read_scanlines() and discard the
 * output, rather than skipping the scanlines, because this allows us to
 * maintain the internal state of the context-based upsampler.  In these cases,
 * we set up and tear down a dummy color converter in order to avoid valgrind
 * errors and to achieve the best possible performance.
 */
unsafe extern "C" fn read_and_discard_scanlines(
    mut cinfo: j_decompress_ptr,
    mut num_lines: JDIMENSION,
) {
    let mut n: JDIMENSION = 0;
    let mut color_convert: Option<
        unsafe extern "C" fn(
            _: j_decompress_ptr,
            _: JSAMPIMAGE,
            _: JDIMENSION,
            _: JSAMPARRAY,
            _: c_int,
        ) -> (),
    > = ::std::mem::transmute::<
        intptr_t,
        Option<
            unsafe extern "C" fn(
                _: j_decompress_ptr,
                _: JSAMPIMAGE,
                _: JDIMENSION,
                _: JSAMPARRAY,
                _: c_int,
            ) -> (),
        >,
    >(NULL as intptr_t);
    let mut color_quantize: Option<
        unsafe extern "C" fn(
            _: j_decompress_ptr,
            _: JSAMPARRAY,
            _: JSAMPARRAY,
            _: c_int,
        ) -> (),
    > = ::std::mem::transmute::<
        intptr_t,
        Option<
            unsafe extern "C" fn(
                _: j_decompress_ptr,
                _: JSAMPARRAY,
                _: JSAMPARRAY,
                _: c_int,
            ) -> (),
        >,
    >(NULL as intptr_t);
    if !(*cinfo).cconvert.is_null() && (*(*cinfo).cconvert).color_convert.is_some() {
        color_convert = (*(*cinfo).cconvert).color_convert;
        (*(*cinfo).cconvert).color_convert = Some(
            noop_convert
                as unsafe extern "C" fn(
                    _: j_decompress_ptr,
                    _: JSAMPIMAGE,
                    _: JDIMENSION,
                    _: JSAMPARRAY,
                    _: c_int,
                ) -> (),
        )
    }
    if !(*cinfo).cquantize.is_null() && (*(*cinfo).cquantize).color_quantize.is_some() {
        color_quantize = (*(*cinfo).cquantize).color_quantize;
        (*(*cinfo).cquantize).color_quantize = Some(
            noop_quantize
                as unsafe extern "C" fn(
                    _: j_decompress_ptr,
                    _: JSAMPARRAY,
                    _: JSAMPARRAY,
                    _: c_int,
                ) -> (),
        )
    }
    n = 0i32 as JDIMENSION;
    while n < num_lines {
        jpeg_read_scanlines(
            cinfo,
            NULL as JSAMPARRAY,
            1i32 as JDIMENSION,
        );
        n = n.wrapping_add(1)
    }
    if color_convert.is_some() {
        (*(*cinfo).cconvert).color_convert = color_convert
    }
    if color_quantize.is_some() {
        (*(*cinfo).cquantize).color_quantize = color_quantize
    };
}
/*
 * Called by jpeg_skip_scanlines().  This partially skips a decompress block by
 * incrementing the rowgroup counter.
 */
unsafe extern "C" fn increment_simple_rowgroup_ctr(
    mut cinfo: j_decompress_ptr,
    mut rows: JDIMENSION,
) {
    let mut rows_left: JDIMENSION = 0;
    let mut main_ptr: my_main_ptr = (*cinfo).main as my_main_ptr;
    (*main_ptr).rowgroup_ctr = ((*main_ptr).rowgroup_ctr as c_uint)
        .wrapping_add(rows.wrapping_div((*cinfo).max_v_samp_factor as c_uint))
        as JDIMENSION
        as JDIMENSION;
    rows_left = rows.wrapping_rem((*cinfo).max_v_samp_factor as c_uint);
    (*cinfo).output_scanline =
        ((*cinfo).output_scanline as c_uint).wrapping_add(rows.wrapping_sub(rows_left))
            as JDIMENSION as JDIMENSION;
    read_and_discard_scanlines(cinfo, rows_left);
}
/*
 * Skips some scanlines of data from the JPEG decompressor.
 *
 * The return value will be the number of lines actually skipped.  If skipping
 * num_lines would move beyond the end of the image, then the actual number of
 * lines remaining in the image is returned.  Otherwise, the return value will
 * be equal to num_lines.
 *
 * Refer to libjpeg.txt for more information.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_skip_scanlines(
    mut cinfo: j_decompress_ptr,
    mut num_lines: JDIMENSION,
) -> JDIMENSION {
    let mut main_ptr: my_main_ptr = (*cinfo).main as my_main_ptr;
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    let mut upsample: my_upsample_ptr =
        (*cinfo).upsample as my_upsample_ptr;
    let mut i: JDIMENSION = 0;
    let mut x: JDIMENSION = 0;
    let mut y: c_int = 0;
    let mut lines_per_iMCU_row: JDIMENSION = 0;
    let mut lines_left_in_iMCU_row: JDIMENSION = 0;
    let mut lines_after_iMCU_row: JDIMENSION = 0;
    let mut lines_to_skip: JDIMENSION = 0;
    let mut lines_to_read: JDIMENSION = 0;
    if (*cinfo).global_state != DSTATE_SCANNING {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*cinfo).output_scanline.wrapping_add(num_lines) >= (*cinfo).output_height {
        (*cinfo).output_scanline = (*cinfo).output_height;
        (*(*cinfo).inputctl)
            .finish_input_pass
            .expect("non-null function pointer")(cinfo);
        (*(*cinfo).inputctl).eoi_reached = TRUE;
        return (*cinfo)
            .output_height
            .wrapping_sub((*cinfo).output_scanline);
    }
    if num_lines == 0i32 as c_uint {
        return 0i32 as JDIMENSION;
    }
    lines_per_iMCU_row = ((*cinfo).min_DCT_scaled_size * (*cinfo).max_v_samp_factor)
        as JDIMENSION;
    lines_left_in_iMCU_row = lines_per_iMCU_row
        .wrapping_sub((*cinfo).output_scanline.wrapping_rem(lines_per_iMCU_row))
        .wrapping_rem(lines_per_iMCU_row);
    lines_after_iMCU_row = num_lines.wrapping_sub(lines_left_in_iMCU_row);
    if 0 != (*(*cinfo).upsample).need_context_rows {
        if num_lines < lines_left_in_iMCU_row.wrapping_add(1i32 as c_uint)
            || lines_left_in_iMCU_row <= 1i32 as c_uint
                && 0 != (*main_ptr).buffer_full
                && lines_after_iMCU_row < lines_per_iMCU_row.wrapping_add(1i32 as c_uint)
        {
            read_and_discard_scanlines(cinfo, num_lines);
            return num_lines;
        }
        if lines_left_in_iMCU_row <= 1i32 as c_uint && 0 != (*main_ptr).buffer_full {
            (*cinfo).output_scanline = ((*cinfo).output_scanline as c_uint)
                .wrapping_add(lines_left_in_iMCU_row.wrapping_add(lines_per_iMCU_row))
                as JDIMENSION
                as JDIMENSION;
            lines_after_iMCU_row = (lines_after_iMCU_row as c_uint)
                .wrapping_sub(lines_per_iMCU_row)
                as JDIMENSION
                as JDIMENSION
        } else {
            (*cinfo).output_scanline = ((*cinfo).output_scanline as c_uint)
                .wrapping_add(lines_left_in_iMCU_row)
                as JDIMENSION
                as JDIMENSION
        }
        if (*main_ptr).iMCU_row_ctr == 0i32 as c_uint
            || (*main_ptr).iMCU_row_ctr == 1i32 as c_uint
                && lines_left_in_iMCU_row > 2i32 as c_uint
        {
            set_wraparound_pointers(cinfo);
        }
        (*main_ptr).buffer_full = FALSE;
        (*main_ptr).rowgroup_ctr = 0i32 as JDIMENSION;
        (*main_ptr).context_state = CTX_PREPARE_FOR_IMCU;
        (*upsample).next_row_out = (*cinfo).max_v_samp_factor;
        (*upsample).rows_to_go = (*cinfo)
            .output_height
            .wrapping_sub((*cinfo).output_scanline)
    } else if num_lines < lines_left_in_iMCU_row {
        increment_simple_rowgroup_ctr(cinfo, num_lines);
        return num_lines;
    } else {
        (*cinfo).output_scanline =
            ((*cinfo).output_scanline as c_uint).wrapping_add(lines_left_in_iMCU_row)
                as JDIMENSION as JDIMENSION;
        (*main_ptr).buffer_full = FALSE;
        (*main_ptr).rowgroup_ctr = 0i32 as JDIMENSION;
        (*upsample).next_row_out = (*cinfo).max_v_samp_factor;
        (*upsample).rows_to_go = (*cinfo)
            .output_height
            .wrapping_sub((*cinfo).output_scanline)
    }
    if 0 != (*(*cinfo).upsample).need_context_rows {
        lines_to_skip = lines_after_iMCU_row
            .wrapping_sub(1i32 as c_uint)
            .wrapping_div(lines_per_iMCU_row)
            .wrapping_mul(lines_per_iMCU_row)
    } else {
        lines_to_skip = lines_after_iMCU_row
            .wrapping_div(lines_per_iMCU_row)
            .wrapping_mul(lines_per_iMCU_row)
    }
    lines_to_read = lines_after_iMCU_row.wrapping_sub(lines_to_skip);
    if 0 != (*(*cinfo).inputctl).has_multiple_scans {
        if 0 != (*(*cinfo).upsample).need_context_rows {
            (*cinfo).output_scanline = ((*cinfo).output_scanline as c_uint)
                .wrapping_add(lines_to_skip)
                as JDIMENSION
                as JDIMENSION;
            (*cinfo).output_iMCU_row = ((*cinfo).output_iMCU_row as c_uint)
                .wrapping_add(lines_to_skip.wrapping_div(lines_per_iMCU_row))
                as JDIMENSION
                as JDIMENSION;
            (*main_ptr).iMCU_row_ctr = ((*main_ptr).iMCU_row_ctr as c_uint)
                .wrapping_add(lines_to_skip.wrapping_div(lines_per_iMCU_row))
                as JDIMENSION
                as JDIMENSION;
            read_and_discard_scanlines(cinfo, lines_to_read);
        } else {
            (*cinfo).output_scanline = ((*cinfo).output_scanline as c_uint)
                .wrapping_add(lines_to_skip)
                as JDIMENSION
                as JDIMENSION;
            (*cinfo).output_iMCU_row = ((*cinfo).output_iMCU_row as c_uint)
                .wrapping_add(lines_to_skip.wrapping_div(lines_per_iMCU_row))
                as JDIMENSION
                as JDIMENSION;
            increment_simple_rowgroup_ctr(cinfo, lines_to_read);
        }
        (*upsample).rows_to_go = (*cinfo)
            .output_height
            .wrapping_sub((*cinfo).output_scanline);
        return num_lines;
    }
    i = 0i32 as JDIMENSION;
    while i < lines_to_skip {
        y = 0i32;
        while y < (*coef).MCU_rows_per_iMCU_row {
            x = 0i32 as JDIMENSION;
            while x < (*cinfo).MCUs_per_row {
                (*(*cinfo).entropy)
                    .decode_mcu
                    .expect("non-null function pointer")(
                    cinfo,
                    NULL as *mut JBLOCKROW,
                );
                x = x.wrapping_add(1)
            }
            y += 1
        }
        (*cinfo).input_iMCU_row = (*cinfo).input_iMCU_row.wrapping_add(1);
        (*cinfo).output_iMCU_row = (*cinfo).output_iMCU_row.wrapping_add(1);
        if (*cinfo).input_iMCU_row < (*cinfo).total_iMCU_rows {
            start_iMCU_row(cinfo);
        } else {
            (*(*cinfo).inputctl)
                .finish_input_pass
                .expect("non-null function pointer")(cinfo);
        }
        i = (i as c_uint).wrapping_add(lines_per_iMCU_row) as JDIMENSION
            as JDIMENSION
    }
    (*cinfo).output_scanline = ((*cinfo).output_scanline as c_uint)
        .wrapping_add(lines_to_skip) as JDIMENSION
        as JDIMENSION;
    if 0 != (*(*cinfo).upsample).need_context_rows {
        (*main_ptr).iMCU_row_ctr = ((*main_ptr).iMCU_row_ctr as c_uint)
            .wrapping_add(lines_to_skip.wrapping_div(lines_per_iMCU_row))
            as JDIMENSION
            as JDIMENSION;
        read_and_discard_scanlines(cinfo, lines_to_read);
    } else {
        increment_simple_rowgroup_ctr(cinfo, lines_to_read);
    }
    (*upsample).rows_to_go = (*cinfo)
        .output_height
        .wrapping_sub((*cinfo).output_scanline);
    return num_lines;
}
/* Replaces jpeg_read_scanlines when reading raw downsampled data. */
/*
 * Alternate entry point to read raw data.
 * Processes exactly one iMCU row per call, unless suspended.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_read_raw_data(
    mut cinfo: j_decompress_ptr,
    mut data: JSAMPIMAGE,
    mut max_lines: JDIMENSION,
) -> JDIMENSION {
    let mut lines_per_iMCU_row: JDIMENSION = 0;
    if (*cinfo).global_state != DSTATE_RAW_OK {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*cinfo).output_scanline >= (*cinfo).output_height {
        (*(*cinfo).err).msg_code = JWRN_TOO_MUCH_DATA as c_int;
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer")(
            cinfo as j_common_ptr, -1i32
        );
        return 0i32 as JDIMENSION;
    }
    if !(*cinfo).progress.is_null() {
        (*(*cinfo).progress).pass_counter = (*cinfo).output_scanline as c_long;
        (*(*cinfo).progress).pass_limit = (*cinfo).output_height as c_long;
        (*(*cinfo).progress)
            .progress_monitor
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    lines_per_iMCU_row = ((*cinfo).max_v_samp_factor * (*cinfo).min_DCT_scaled_size)
        as JDIMENSION;
    if max_lines < lines_per_iMCU_row {
        (*(*cinfo).err).msg_code = JERR_BUFFER_SIZE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if 0 == (*(*cinfo).coef)
        .decompress_data
        .expect("non-null function pointer")(cinfo, data)
    {
        return 0i32 as JDIMENSION;
    }
    (*cinfo).output_scanline =
        ((*cinfo).output_scanline as c_uint).wrapping_add(lines_per_iMCU_row)
            as JDIMENSION as JDIMENSION;
    return lines_per_iMCU_row;
}
/* Additional entry points for buffered-image mode. */
/*
 * Initialize for an output pass in buffered-image mode.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_start_output(
    mut cinfo: j_decompress_ptr,
    mut scan_number: c_int,
) -> boolean {
    if (*cinfo).global_state != DSTATE_BUFIMAGE
        && (*cinfo).global_state != DSTATE_PRESCAN
    {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if scan_number <= 0i32 {
        scan_number = 1i32
    }
    if 0 != (*(*cinfo).inputctl).eoi_reached && scan_number > (*cinfo).input_scan_number {
        scan_number = (*cinfo).input_scan_number
    }
    (*cinfo).output_scan_number = scan_number;
    return output_pass_setup(cinfo);
}
/*
 * Finish up after an output pass in buffered-image mode.
 *
 * Returns FALSE if suspended.  The return value need be inspected only if
 * a suspending data source is used.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_finish_output(
    mut cinfo: j_decompress_ptr,
) -> boolean {
    if ((*cinfo).global_state == DSTATE_SCANNING
        || (*cinfo).global_state == DSTATE_RAW_OK)
        && 0 != (*cinfo).buffered_image
    {
        (*(*cinfo).master)
            .finish_output_pass
            .expect("non-null function pointer")(cinfo);
        (*cinfo).global_state = DSTATE_BUFPOST
    } else if (*cinfo).global_state != DSTATE_BUFPOST {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    while (*cinfo).input_scan_number <= (*cinfo).output_scan_number
        && 0 == (*(*cinfo).inputctl).eoi_reached
    {
        if (*(*cinfo).inputctl)
            .consume_input
            .expect("non-null function pointer")(cinfo)
            == JPEG_SUSPENDED
        {
            return FALSE;
        }
    }
    (*cinfo).global_state = DSTATE_BUFIMAGE;
    return TRUE;
}
