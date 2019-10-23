use libc;

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
pub use crate::jpeglib_h::jpeg_color_deconverter;
pub use crate::jpeglib_h::jpeg_color_quantizer;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_d_coef_controller;
pub use crate::jpeglib_h::jpeg_d_main_controller;
pub use crate::jpeglib_h::jpeg_d_post_controller;
pub use crate::jpeglib_h::jpeg_decomp_master;
pub use crate::jpeglib_h::jpeg_decompress_struct;
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
pub use crate::src::jdcoefct::my_coef_controller;
pub use crate::src::jdcoefct::my_coef_ptr;
pub use crate::src::jdcoefct::start_iMCU_row;
pub use crate::src::jdmainct::my_main_controller;
pub use crate::src::jdmainct::my_main_ptr;
pub use crate::src::jdmainct::set_wraparound_pointers;
pub use crate::src::jdmainct::CTX_PREPARE_FOR_IMCU;
pub use crate::src::jdsample::my_upsample_ptr;
pub use crate::src::jdsample::my_upsampler;
pub use crate::src::jdsample::upsample1_ptr;
pub use crate::src::jerror::C2RustUnnamed_3;
pub use crate::src::jerror::JERR_ARITH_NOTIMPL;
pub use crate::src::jerror::JERR_BAD_ALIGN_TYPE;
pub use crate::src::jerror::JERR_BAD_ALLOC_CHUNK;
pub use crate::src::jerror::JERR_BAD_BUFFER_MODE;
pub use crate::src::jerror::JERR_BAD_COMPONENT_ID;
pub use crate::src::jerror::JERR_BAD_CROP_SPEC;
pub use crate::src::jerror::JERR_BAD_DCTSIZE;
pub use crate::src::jerror::JERR_BAD_DCT_COEF;
pub use crate::src::jerror::JERR_BAD_HUFF_TABLE;
pub use crate::src::jerror::JERR_BAD_IN_COLORSPACE;
pub use crate::src::jerror::JERR_BAD_J_COLORSPACE;
pub use crate::src::jerror::JERR_BAD_LENGTH;
pub use crate::src::jerror::JERR_BAD_LIB_VERSION;
pub use crate::src::jerror::JERR_BAD_MCU_SIZE;
pub use crate::src::jerror::JERR_BAD_PARAM;
pub use crate::src::jerror::JERR_BAD_PARAM_VALUE;
pub use crate::src::jerror::JERR_BAD_POOL_ID;
pub use crate::src::jerror::JERR_BAD_PRECISION;
pub use crate::src::jerror::JERR_BAD_PROGRESSION;
pub use crate::src::jerror::JERR_BAD_PROG_SCRIPT;
pub use crate::src::jerror::JERR_BAD_SAMPLING;
pub use crate::src::jerror::JERR_BAD_SCAN_SCRIPT;
pub use crate::src::jerror::JERR_BAD_STATE;
pub use crate::src::jerror::JERR_BAD_STRUCT_SIZE;
pub use crate::src::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use crate::src::jerror::JERR_BUFFER_SIZE;
pub use crate::src::jerror::JERR_CANT_SUSPEND;
pub use crate::src::jerror::JERR_CCIR601_NOTIMPL;
pub use crate::src::jerror::JERR_COMPONENT_COUNT;
pub use crate::src::jerror::JERR_CONVERSION_NOTIMPL;
pub use crate::src::jerror::JERR_DAC_INDEX;
pub use crate::src::jerror::JERR_DAC_VALUE;
pub use crate::src::jerror::JERR_DHT_INDEX;
pub use crate::src::jerror::JERR_DQT_INDEX;
pub use crate::src::jerror::JERR_EMPTY_IMAGE;
pub use crate::src::jerror::JERR_EMS_READ;
pub use crate::src::jerror::JERR_EMS_WRITE;
pub use crate::src::jerror::JERR_EOI_EXPECTED;
pub use crate::src::jerror::JERR_FILE_READ;
pub use crate::src::jerror::JERR_FILE_WRITE;
pub use crate::src::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use crate::src::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use crate::src::jerror::JERR_HUFF_MISSING_CODE;
pub use crate::src::jerror::JERR_IMAGE_TOO_BIG;
pub use crate::src::jerror::JERR_INPUT_EMPTY;
pub use crate::src::jerror::JERR_INPUT_EOF;
pub use crate::src::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use crate::src::jerror::JERR_MISSING_DATA;
pub use crate::src::jerror::JERR_MODE_CHANGE;
pub use crate::src::jerror::JERR_NOTIMPL;
pub use crate::src::jerror::JERR_NOT_COMPILED;
pub use crate::src::jerror::JERR_NO_BACKING_STORE;
pub use crate::src::jerror::JERR_NO_HUFF_TABLE;
pub use crate::src::jerror::JERR_NO_IMAGE;
pub use crate::src::jerror::JERR_NO_QUANT_TABLE;
pub use crate::src::jerror::JERR_NO_SOI;
pub use crate::src::jerror::JERR_OUT_OF_MEMORY;
pub use crate::src::jerror::JERR_QUANT_COMPONENTS;
pub use crate::src::jerror::JERR_QUANT_FEW_COLORS;
pub use crate::src::jerror::JERR_QUANT_MANY_COLORS;
pub use crate::src::jerror::JERR_SOF_DUPLICATE;
pub use crate::src::jerror::JERR_SOF_NO_SOS;
pub use crate::src::jerror::JERR_SOF_UNSUPPORTED;
pub use crate::src::jerror::JERR_SOI_DUPLICATE;
pub use crate::src::jerror::JERR_SOS_NO_SOF;
pub use crate::src::jerror::JERR_TFILE_CREATE;
pub use crate::src::jerror::JERR_TFILE_READ;
pub use crate::src::jerror::JERR_TFILE_SEEK;
pub use crate::src::jerror::JERR_TFILE_WRITE;
pub use crate::src::jerror::JERR_TOO_LITTLE_DATA;
pub use crate::src::jerror::JERR_UNKNOWN_MARKER;
pub use crate::src::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use crate::src::jerror::JERR_VIRTUAL_BUG;
pub use crate::src::jerror::JERR_WIDTH_OVERFLOW;
pub use crate::src::jerror::JERR_XMS_READ;
pub use crate::src::jerror::JERR_XMS_WRITE;
pub use crate::src::jerror::JMSG_COPYRIGHT;
pub use crate::src::jerror::JMSG_LASTMSGCODE;
pub use crate::src::jerror::JMSG_NOMESSAGE;
pub use crate::src::jerror::JMSG_VERSION;
pub use crate::src::jerror::JTRC_16BIT_TABLES;
pub use crate::src::jerror::JTRC_ADOBE;
pub use crate::src::jerror::JTRC_APP0;
pub use crate::src::jerror::JTRC_APP14;
pub use crate::src::jerror::JTRC_DAC;
pub use crate::src::jerror::JTRC_DHT;
pub use crate::src::jerror::JTRC_DQT;
pub use crate::src::jerror::JTRC_DRI;
pub use crate::src::jerror::JTRC_EMS_CLOSE;
pub use crate::src::jerror::JTRC_EMS_OPEN;
pub use crate::src::jerror::JTRC_EOI;
pub use crate::src::jerror::JTRC_HUFFBITS;
pub use crate::src::jerror::JTRC_JFIF;
pub use crate::src::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use crate::src::jerror::JTRC_JFIF_EXTENSION;
pub use crate::src::jerror::JTRC_JFIF_THUMBNAIL;
pub use crate::src::jerror::JTRC_MISC_MARKER;
pub use crate::src::jerror::JTRC_PARMLESS_MARKER;
pub use crate::src::jerror::JTRC_QUANTVALS;
pub use crate::src::jerror::JTRC_QUANT_3_NCOLORS;
pub use crate::src::jerror::JTRC_QUANT_NCOLORS;
pub use crate::src::jerror::JTRC_QUANT_SELECTED;
pub use crate::src::jerror::JTRC_RECOVERY_ACTION;
pub use crate::src::jerror::JTRC_RST;
pub use crate::src::jerror::JTRC_SMOOTH_NOTIMPL;
pub use crate::src::jerror::JTRC_SOF;
pub use crate::src::jerror::JTRC_SOF_COMPONENT;
pub use crate::src::jerror::JTRC_SOI;
pub use crate::src::jerror::JTRC_SOS;
pub use crate::src::jerror::JTRC_SOS_COMPONENT;
pub use crate::src::jerror::JTRC_SOS_PARAMS;
pub use crate::src::jerror::JTRC_TFILE_CLOSE;
pub use crate::src::jerror::JTRC_TFILE_OPEN;
pub use crate::src::jerror::JTRC_THUMB_JPEG;
pub use crate::src::jerror::JTRC_THUMB_PALETTE;
pub use crate::src::jerror::JTRC_THUMB_RGB;
pub use crate::src::jerror::JTRC_UNKNOWN_IDS;
pub use crate::src::jerror::JTRC_XMS_CLOSE;
pub use crate::src::jerror::JTRC_XMS_OPEN;
pub use crate::src::jerror::JWRN_ADOBE_XFORM;
pub use crate::src::jerror::JWRN_BOGUS_ICC;
pub use crate::src::jerror::JWRN_BOGUS_PROGRESSION;
pub use crate::src::jerror::JWRN_EXTRANEOUS_DATA;
pub use crate::src::jerror::JWRN_HIT_MARKER;
pub use crate::src::jerror::JWRN_HUFF_BAD_CODE;
pub use crate::src::jerror::JWRN_JFIF_MAJOR;
pub use crate::src::jerror::JWRN_JPEG_EOF;
pub use crate::src::jerror::JWRN_MUST_RESYNC;
pub use crate::src::jerror::JWRN_NOT_SEQUENTIAL;
pub use crate::src::jerror::JWRN_TOO_MUCH_DATA;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
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
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean {
    if (*cinfo).global_state == crate::jpegint_h::DSTATE_READY {
        /* First call: initialize master control, select active modules */
        crate::jpegint_h::jinit_master_decompress(cinfo);
        if (*cinfo).buffered_image != 0 {
            /* No more work here; expecting jpeg_start_output next */
            (*cinfo).global_state = crate::jpegint_h::DSTATE_BUFIMAGE;
            return crate::jmorecfg_h::TRUE;
        }
        (*cinfo).global_state = crate::jpegint_h::DSTATE_PRELOAD
    }
    if (*cinfo).global_state == crate::jpegint_h::DSTATE_PRELOAD {
        /* If file has multiple scans, absorb them all into the coef buffer */
        if (*(*cinfo).inputctl).has_multiple_scans != 0 {
            loop {
                 
                /* Call progress monitor hook if present */
                if !(*cinfo).progress.is_null() {
                    Some(
                        (*(*cinfo).progress)
                            .progress_monitor
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr
                    );
                }
                 let mut retcode:   libc::c_int =
     Some(
                    (*(*cinfo).inputctl)
                        .consume_input
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
                if retcode == crate::jpeglib_h::JPEG_SUSPENDED {
                    return crate::jmorecfg_h::FALSE;
                }
                if retcode == crate::jpeglib_h::JPEG_REACHED_EOI {
                    break;
                }
                /* Advance progress counter if appropriate */
                if !(*cinfo).progress.is_null()
                    && (retcode == crate::jpeglib_h::JPEG_ROW_COMPLETED
                        || retcode == crate::jpeglib_h::JPEG_REACHED_SOS)
                {
                    (*(*cinfo).progress).pass_counter += 1;
                    if (*(*cinfo).progress).pass_counter >= (*(*cinfo).progress).pass_limit {
                        /* jdmaster underestimated number of scans; ratchet up one scan */
                        (*(*cinfo).progress).pass_limit += (*cinfo).total_iMCU_rows as libc::c_long
                    }
                }
            }
            /* D_MULTISCAN_FILES_SUPPORTED */
        }
        (*cinfo).output_scan_number = (*cinfo).input_scan_number
    } else if (*cinfo).global_state != crate::jpegint_h::DSTATE_PRESCAN {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Perform any dummy output passes, and set up for the final pass */
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
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean {
    if (*cinfo).global_state != crate::jpegint_h::DSTATE_PRESCAN {
        /* First call: do pass setup */
        Some(
            (*(*cinfo).master)
                .prepare_for_output_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
        (*cinfo).output_scanline = 0u32;
        (*cinfo).global_state = crate::jpegint_h::DSTATE_PRESCAN
    }
    /* Loop over any required dummy passes */
    while (*(*cinfo).master).is_dummy_pass != 0 {
        /* Crank through the dummy pass */
        while (*cinfo).output_scanline < (*cinfo).output_height {
             
            /* No progress made, must suspend */
            if !(*cinfo).progress.is_null() {
                (*(*cinfo).progress).pass_counter = (*cinfo).output_scanline as libc::c_long;
                (*(*cinfo).progress).pass_limit = (*cinfo).output_height as libc::c_long;
                Some(
                    (*(*cinfo).progress)
                        .progress_monitor
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
             let mut last_scanline:   crate::jmorecfg_h::JDIMENSION =
     (*cinfo).output_scanline;
            Some(
                (*(*cinfo).main)
                    .process_data
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                
                crate::stddef_h::NULL as crate::jpeglib_h::JSAMPARRAY,
                &mut (*cinfo).output_scanline,
                0u32,
            );
            if (*cinfo).output_scanline == last_scanline {
                return crate::jmorecfg_h::FALSE;
            }
        }
        /* Call progress monitor hook if present */
        /* Process some data */
        /* QUANT_2PASS_SUPPORTED */
        Some(
            (*(*cinfo).master)
                .finish_output_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
        Some(
            (*(*cinfo).master)
                .prepare_for_output_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
        (*cinfo).output_scanline = 0u32
    }
    /* Finish up dummy pass, and set up for another one */
    /* Ready for application to drive output pass through
     * jpeg_read_scanlines or jpeg_read_raw_data.
     */
    (*cinfo).global_state = if (*cinfo).raw_data_out != 0 {
        crate::jpegint_h::DSTATE_RAW_OK
    } else {
        crate::jpegint_h::DSTATE_SCANNING
    };
    return crate::jmorecfg_h::TRUE;
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
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut xoffset: *mut crate::jmorecfg_h::JDIMENSION,
    mut width: *mut crate::jmorecfg_h::JDIMENSION,
) {
    
    
    
      let mut align:  libc::c_int =  0;  
    let mut reinit_upsampler: crate::jmorecfg_h::boolean = crate::jmorecfg_h::FALSE;
    
    if (*cinfo).global_state != crate::jpegint_h::DSTATE_SCANNING
        || (*cinfo).output_scanline != 0u32
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if xoffset.is_null() || width.is_null() {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_CROP_SPEC as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* xoffset and width must fall within the output image dimensions. */
    if *width == 0u32 || *xoffset + *width > (*cinfo).output_width {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_WIDTH_OVERFLOW as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* No need to do anything if the caller wants the entire width. */
    if *width == (*cinfo).output_width {
        return;
    }
    /* Ensuring the proper alignment of xoffset is tricky.  At minimum, it
     * must align with an MCU boundary, because:
     *
     *   (1) The IDCT is performed in blocks, and it is not feasible to modify
     *       the algorithm so that it can transform partial blocks.
     *   (2) Because of the SIMD extensions, any input buffer passed to the
     *       upsampling and color conversion routines must be aligned to the
     *       SIMD word size (for instance, 128-bit in the case of SSE2.)  The
     *       easiest way to accomplish this without copying data is to ensure
     *       that upsampling and color conversion begin at the start of the
     *       first MCU column that will be inverse transformed.
     *
     * In practice, we actually impose a stricter alignment requirement.  We
     * require that xoffset be a multiple of the maximum MCU column width of all
     * of the components (the "iMCU column width.")  This is to simplify the
     * single-pass decompression case, allowing us to use the same MCU column
     * width for all of the components.
     */
    if (*cinfo).comps_in_scan == 1i32 && (*cinfo).num_components == 1i32 {
        align = (*cinfo).min_DCT_scaled_size
    } else {
        align = (*cinfo).min_DCT_scaled_size * (*cinfo).max_h_samp_factor
    }
     let mut input_xoffset:   crate::jmorecfg_h::JDIMENSION =  *xoffset;
    *xoffset =  input_xoffset / align as libc::c_uint * align as libc::c_uint;
    /* Adjust the width so that the right edge of the output image is as
     * requested (only the left edge is altered.)  It is important that calling
     * programs check this value after this function returns, so that they can
     * allocate an output buffer with the appropriate size.
     */
    *width = *width + input_xoffset - *xoffset;
    (*cinfo).output_width = *width;
    /* Set the first and last iMCU columns that we must decompress.  These values
     * will be used in single-scan decompressions.
     */
    (*(*cinfo).master).first_iMCU_col =
        (*xoffset as libc::c_long
            / align as libc::c_long) as crate::jmorecfg_h::JDIMENSION;
    (*(*cinfo).master).last_iMCU_col = crate::jpegint_h::jdiv_round_up(
        (*xoffset + (*cinfo).output_width) as libc::c_long,
        align as libc::c_long,
    ) as crate::jmorecfg_h::JDIMENSION - 1u32;
    
     let mut ci:   libc::c_int =  0i32; let mut compptr:   *mut crate::jpeglib_h::jpeg_component_info =
     (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
         let mut hsf: libc::c_int =
            if (*cinfo).comps_in_scan == 1i32 && (*cinfo).num_components == 1i32 {
                1i32
            } else {
                (*compptr).h_samp_factor
            };
         let mut orig_downsampled_width:   libc::c_int =
     (*compptr).downsampled_width as libc::c_int;
        (*compptr).downsampled_width = crate::jpegint_h::jdiv_round_up(
            (
            (*cinfo)
                .output_width * (*compptr).h_samp_factor as libc::c_uint) as libc::c_long,
            (*cinfo).max_h_samp_factor as libc::c_long,
        ) as crate::jmorecfg_h::JDIMENSION;
        if (*compptr).downsampled_width < 2u32 && orig_downsampled_width >= 2i32 {
            reinit_upsampler = crate::jmorecfg_h::TRUE
        }
        /* Set the first and last iMCU columns that we must decompress.  These
         * values will be used in multi-scan decompressions.
         */
        (*(*cinfo).master).first_MCU_col[ci as usize] =
            ((((*xoffset * hsf as libc::c_uint))) as libc::c_long
                / align as libc::c_long) as crate::jmorecfg_h::JDIMENSION;
        (*(*cinfo).master).last_MCU_col[ci as usize] = crate::jpegint_h::jdiv_round_up(
            ((*xoffset + (*cinfo).output_width) * hsf as libc::c_uint) as libc::c_long,
            align as libc::c_long,
        ) as crate::jmorecfg_h::JDIMENSION - 1u32;
        ci += 1;
        compptr = compptr.offset(1)
    }
    if reinit_upsampler != 0 {
        (*(*cinfo).master).jinit_upsampler_no_alloc = crate::jmorecfg_h::TRUE;
        crate::jpegint_h::jinit_upsampler(cinfo);
        (*(*cinfo).master).jinit_upsampler_no_alloc = crate::jmorecfg_h::FALSE
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
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut scanlines: crate::jpeglib_h::JSAMPARRAY,
    mut max_lines: crate::jmorecfg_h::JDIMENSION,
) -> crate::jmorecfg_h::JDIMENSION {
     
    if (*cinfo).global_state != crate::jpegint_h::DSTATE_SCANNING {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if (*cinfo).output_scanline >= (*cinfo).output_height {
        (*(*cinfo).err).msg_code = crate::src::jerror::JWRN_TOO_MUCH_DATA as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, -1i32);
        return 0u32;
    }
    /* Call progress monitor hook if present */
    if !(*cinfo).progress.is_null() {
        (*(*cinfo).progress).pass_counter = (*cinfo).output_scanline as libc::c_long;
        (*(*cinfo).progress).pass_limit = (*cinfo).output_height as libc::c_long;
        Some(
            (*(*cinfo).progress)
                .progress_monitor
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
     let mut row_ctr:   crate::jmorecfg_h::JDIMENSION =  0u32;
    Some(
        (*(*cinfo).main)
            .process_data
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, scanlines, &mut row_ctr, max_lines);
    (*cinfo).output_scanline = (*cinfo).output_scanline + row_ctr;
    return row_ctr;
}
/* Dummy color convert function used by jpeg_skip_scanlines() */

unsafe extern "C" fn noop_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
}
/* Dummy quantize function used by jpeg_skip_scanlines() */

unsafe extern "C" fn noop_quantize(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
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
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut num_lines: crate::jmorecfg_h::JDIMENSION,
) {
     
    let mut color_convert: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: libc::c_int,
        ) -> (),
    > = ::std::mem::transmute::<
        libc::intptr_t,
        Option<
            unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::jpeglib_h::JSAMPIMAGE,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jpeglib_h::JSAMPARRAY,
                _: libc::c_int,
            ) -> (),
        >,
    >(crate::stddef_h::NULL as libc::intptr_t);
    let mut color_quantize: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: libc::c_int,
        ) -> (),
    > = ::std::mem::transmute::<
        libc::intptr_t,
        Option<
            unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::jpeglib_h::JSAMPARRAY,
                _: crate::jpeglib_h::JSAMPARRAY,
                _: libc::c_int,
            ) -> (),
        >,
    >(crate::stddef_h::NULL as libc::intptr_t);
    if !(*cinfo).cconvert.is_null() && (*(*cinfo).cconvert).color_convert.is_some() {
        color_convert = (*(*cinfo).cconvert).color_convert;
        (*(*cinfo).cconvert).color_convert = Some(
            noop_convert
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                    _: crate::jpeglib_h::JSAMPIMAGE,
                    _: crate::jmorecfg_h::JDIMENSION,
                    _: crate::jpeglib_h::JSAMPARRAY,
                    _: libc::c_int,
                ) -> (),
        )
    }
    if !(*cinfo).cquantize.is_null() && (*(*cinfo).cquantize).color_quantize.is_some() {
        color_quantize = (*(*cinfo).cquantize).color_quantize;
        (*(*cinfo).cquantize).color_quantize = Some(
            noop_quantize
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                    _: crate::jpeglib_h::JSAMPARRAY,
                    _: crate::jpeglib_h::JSAMPARRAY,
                    _: libc::c_int,
                ) -> (),
        )
    }
     let mut n:   crate::jmorecfg_h::JDIMENSION =  0u32;
    while n < num_lines {
        jpeg_read_scanlines(
            cinfo,
            crate::stddef_h::NULL as crate::jpeglib_h::JSAMPARRAY,
            1u32,
        );
        n =  n + 1
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
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut rows: crate::jmorecfg_h::JDIMENSION,
) {
     
    let mut main_ptr: crate::src::jdmainct::my_main_ptr =
        (*cinfo).main as crate::src::jdmainct::my_main_ptr;
    /* Increment the counter to the next row group after the skipped rows. */
    (*main_ptr).rowgroup_ctr = (*main_ptr).rowgroup_ctr +
    rows / (*cinfo).max_v_samp_factor as libc::c_uint;
    /* Partially skipping a row group would involve modifying the internal state
     * of the upsampler, so read the remaining rows into a dummy buffer instead.
     */
     let mut rows_left:   crate::jmorecfg_h::JDIMENSION =
      rows % (*cinfo).max_v_samp_factor as libc::c_uint;
    (*cinfo).output_scanline =
        (*cinfo).output_scanline + (rows - rows_left);
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
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut num_lines: crate::jmorecfg_h::JDIMENSION,
) -> crate::jmorecfg_h::JDIMENSION {
         let mut lines_to_skip:  crate::jmorecfg_h::JDIMENSION =  0; let mut main_ptr: crate::src::jdmainct::my_main_ptr =
        (*cinfo).main as crate::src::jdmainct::my_main_ptr;
    let mut coef: crate::src::jdcoefct::my_coef_ptr =
        (*cinfo).coef as crate::src::jdcoefct::my_coef_ptr;
    let mut upsample: crate::src::jdsample::my_upsample_ptr =
        (*cinfo).upsample as crate::src::jdsample::my_upsample_ptr;
    
    
    
    
    
    
    
    
    if (*cinfo).global_state != crate::jpegint_h::DSTATE_SCANNING {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Do not skip past the bottom of the image. */
    if  (*cinfo).output_scanline + num_lines >= (*cinfo).output_height {
        (*cinfo).output_scanline = (*cinfo).output_height;
        Some(
            (*(*cinfo).inputctl)
                .finish_input_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
        (*(*cinfo).inputctl).eoi_reached = crate::jmorecfg_h::TRUE;
        return  (*cinfo)
            .output_height - (*cinfo).output_scanline;
    }
    if num_lines == 0u32 {
        return 0u32;
    }
    
    
     let mut lines_per_iMCU_row:   crate::jmorecfg_h::JDIMENSION =
     ((*cinfo).min_DCT_scaled_size * (*cinfo).max_v_samp_factor)
        as crate::jmorecfg_h::JDIMENSION; let mut lines_left_in_iMCU_row:   crate::jmorecfg_h::JDIMENSION =
     ( lines_per_iMCU_row - (*cinfo).output_scanline % lines_per_iMCU_row) %
    lines_per_iMCU_row; let mut lines_after_iMCU_row:   crate::jmorecfg_h::JDIMENSION =
      num_lines - lines_left_in_iMCU_row;
    /* Skip the lines remaining in the current iMCU row.  When upsampling
     * requires context rows, we need the previous and next rows in order to read
     * the current row.  This adds some complexity.
     */
    if (*(*cinfo).upsample).need_context_rows != 0 {
        /* If the skipped lines would not move us past the current iMCU row, we
         * read the lines and ignore them.  There might be a faster way of doing
         * this, but we are facing increasing complexity for diminishing returns.
         * The increasing complexity would be a by-product of meddling with the
         * state machine used to skip context rows.  Near the end of an iMCU row,
         * the next iMCU row may have already been entropy-decoded.  In this unique
         * case, we will read the next iMCU row if we cannot skip past it as well.
         */
        if num_lines <  lines_left_in_iMCU_row + 1u32
            || lines_left_in_iMCU_row <= 1u32
                && (*main_ptr).buffer_full != 0
                && lines_after_iMCU_row <  lines_per_iMCU_row + 1u32
        {
            read_and_discard_scanlines(cinfo, num_lines);
            return num_lines;
        }
        /* If the next iMCU row has already been entropy-decoded, make sure that
         * we do not skip too far.
         */
        if lines_left_in_iMCU_row <= 1u32 && (*main_ptr).buffer_full != 0 {
            (*cinfo).output_scanline = (*cinfo).output_scanline +
    (lines_left_in_iMCU_row + lines_per_iMCU_row);
            lines_after_iMCU_row = lines_after_iMCU_row - lines_per_iMCU_row
        } else {
            (*cinfo).output_scanline = (*cinfo).output_scanline + lines_left_in_iMCU_row
        }
        /* If we have just completed the first block, adjust the buffer pointers */
        if (*main_ptr).iMCU_row_ctr == 0u32
            || (*main_ptr).iMCU_row_ctr == 1u32
                && lines_left_in_iMCU_row > 2u32
        {
            crate::src::jdmainct::set_wraparound_pointers(cinfo);
        }
        (*main_ptr).buffer_full = crate::jmorecfg_h::FALSE;
        (*main_ptr).rowgroup_ctr = 0u32;
        (*main_ptr).context_state = crate::src::jdmainct::CTX_PREPARE_FOR_IMCU;
        (*upsample).next_row_out = (*cinfo).max_v_samp_factor;
        (*upsample).rows_to_go =  (*cinfo)
            .output_height - (*cinfo).output_scanline
    } else if num_lines < lines_left_in_iMCU_row {
        increment_simple_rowgroup_ctr(cinfo, num_lines);
        return num_lines;
    } else {
        (*cinfo).output_scanline =
            (*cinfo).output_scanline + lines_left_in_iMCU_row;
        (*main_ptr).buffer_full = crate::jmorecfg_h::FALSE;
        (*main_ptr).rowgroup_ctr = 0u32;
        (*upsample).next_row_out = (*cinfo).max_v_samp_factor;
        (*upsample).rows_to_go =  (*cinfo)
            .output_height - (*cinfo).output_scanline
    }
    /* Skipping is much simpler when context rows are not required. */
    /* Calculate how many full iMCU rows we can skip. */
    if (*(*cinfo).upsample).need_context_rows != 0 {
        lines_to_skip = ( lines_after_iMCU_row - 1u32) / lines_per_iMCU_row *
    lines_per_iMCU_row
    } else {
        lines_to_skip =  lines_after_iMCU_row / lines_per_iMCU_row * lines_per_iMCU_row
    }
    /* Calculate the number of lines that remain to be skipped after skipping all
     * of the full iMCU rows that we can.  We will not read these lines unless we
     * have to.
     */
     let mut lines_to_read:   crate::jmorecfg_h::JDIMENSION =
      lines_after_iMCU_row - lines_to_skip;
    /* For images requiring multiple scans (progressive, non-interleaved, etc.),
     * all of the entropy decoding occurs in jpeg_start_decompress(), assuming
     * that the input data source is non-suspending.  This makes skipping easy.
     */
    if (*(*cinfo).inputctl).has_multiple_scans != 0 {
        if (*(*cinfo).upsample).need_context_rows != 0 {
            (*cinfo).output_scanline = (*cinfo).output_scanline + lines_to_skip;
            (*cinfo).output_iMCU_row = (*cinfo).output_iMCU_row + lines_to_skip / lines_per_iMCU_row;
            (*main_ptr).iMCU_row_ctr = (*main_ptr).iMCU_row_ctr + lines_to_skip / lines_per_iMCU_row;
            /* It is complex to properly move to the middle of a context block, so
             * read the remaining lines instead of skipping them.
             */
            read_and_discard_scanlines(cinfo, lines_to_read);
        } else {
            (*cinfo).output_scanline = (*cinfo).output_scanline + lines_to_skip;
            (*cinfo).output_iMCU_row = (*cinfo).output_iMCU_row + lines_to_skip / lines_per_iMCU_row;
            increment_simple_rowgroup_ctr(cinfo, lines_to_read);
        }
        (*upsample).rows_to_go =  (*cinfo)
            .output_height - (*cinfo).output_scanline;
        return num_lines;
    }
     let mut i:   crate::jmorecfg_h::JDIMENSION =  0u32;
    while i < lines_to_skip {
          let mut y:   libc::c_int =  0i32;
        while y < (*coef).MCU_rows_per_iMCU_row {
              let mut x:   crate::jmorecfg_h::JDIMENSION =  0u32;
            while x < (*cinfo).MCUs_per_row {
                /* Calling decode_mcu() with a NULL pointer causes it to discard the
                 * decoded coefficients.  This is ~5% faster for large subsets, but
                 * it's tough to tell a difference for smaller images.
                 */
                Some(
                    (*(*cinfo).entropy)
                        .decode_mcu
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo,
                    crate::stddef_h::NULL as *mut crate::jpeglib_h::JBLOCKROW,
                );
                x =  x + 1
            }
            y += 1
        }
        (*cinfo).input_iMCU_row =  (*cinfo).input_iMCU_row + 1;
        (*cinfo).output_iMCU_row =  (*cinfo).output_iMCU_row + 1;
        if (*cinfo).input_iMCU_row < (*cinfo).total_iMCU_rows {
            crate::src::jdcoefct::start_iMCU_row(cinfo);
        } else {
            Some(
                (*(*cinfo).inputctl)
                    .finish_input_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
        }
        i = i + lines_per_iMCU_row
    }
    (*cinfo).output_scanline = (*cinfo).output_scanline + lines_to_skip;
    if (*(*cinfo).upsample).need_context_rows != 0 {
        /* Context-based upsampling keeps track of iMCU rows. */
        (*main_ptr).iMCU_row_ctr = (*main_ptr).iMCU_row_ctr + lines_to_skip / lines_per_iMCU_row;
        /* It is complex to properly move to the middle of a context block, so
         * read the remaining lines instead of skipping them.
         */
        read_and_discard_scanlines(cinfo, lines_to_read);
    } else {
        increment_simple_rowgroup_ctr(cinfo, lines_to_read);
    }
    /* Since skipping lines involves skipping the upsampling step, the value of
     * "rows_to_go" will become invalid unless we set it here.  NOTE: This is a
     * bit odd, since "rows_to_go" seems to be redundantly keeping track of
     * output_scanline.
     */
    (*upsample).rows_to_go =  (*cinfo)
        .output_height - (*cinfo).output_scanline;
    /* Always skip the requested number of lines. */
    return num_lines;
}
/*
 * Alternate entry point to read raw data.
 * Processes exactly one iMCU row per call, unless suspended.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_read_raw_data(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut data: crate::jpeglib_h::JSAMPIMAGE,
    mut max_lines: crate::jmorecfg_h::JDIMENSION,
) -> crate::jmorecfg_h::JDIMENSION {
     
    if (*cinfo).global_state != crate::jpegint_h::DSTATE_RAW_OK {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if (*cinfo).output_scanline >= (*cinfo).output_height {
        (*(*cinfo).err).msg_code = crate::src::jerror::JWRN_TOO_MUCH_DATA as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, -1i32);
        return 0u32;
    }
    /* Call progress monitor hook if present */
    if !(*cinfo).progress.is_null() {
        (*(*cinfo).progress).pass_counter = (*cinfo).output_scanline as libc::c_long;
        (*(*cinfo).progress).pass_limit = (*cinfo).output_height as libc::c_long;
        Some(
            (*(*cinfo).progress)
                .progress_monitor
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
     let mut lines_per_iMCU_row:   crate::jmorecfg_h::JDIMENSION =
     ((*cinfo).max_v_samp_factor * (*cinfo).min_DCT_scaled_size)
        as crate::jmorecfg_h::JDIMENSION;
    if max_lines < lines_per_iMCU_row {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BUFFER_SIZE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Decompress directly into user's buffer. */
    if Some(
        (*(*cinfo).coef)
            .decompress_data
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, data)
        == 0
    {
        return 0u32;
    } /* suspension forced, can do nothing more */
    /* OK, we processed one iMCU row. */
    (*cinfo).output_scanline =
        (*cinfo).output_scanline + lines_per_iMCU_row;
    return lines_per_iMCU_row;
}
/* Additional entry points for buffered-image mode. */
/*
 * Initialize for an output pass in buffered-image mode.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_start_output(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut scan_number: libc::c_int,
) -> crate::jmorecfg_h::boolean {
    if (*cinfo).global_state != crate::jpegint_h::DSTATE_BUFIMAGE
        && (*cinfo).global_state != crate::jpegint_h::DSTATE_PRESCAN
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Limit scan number to valid range */
    if scan_number <= 0i32 {
        scan_number = 1i32
    }
    if (*(*cinfo).inputctl).eoi_reached != 0 && scan_number > (*cinfo).input_scan_number {
        scan_number = (*cinfo).input_scan_number
    }
    (*cinfo).output_scan_number = scan_number;
    /* Perform any dummy output passes, and set up for the real pass */
    return output_pass_setup(cinfo);
}
/* Method pointers */
/* Limit on memory allocation for this JPEG object.  (Note that this is
 * merely advisory, not a guaranteed maximum; it only affects the space
 * used for virtual-array buffers.)  May be changed by outer application
 * after creating the JPEG object.
 */
/* Maximum allocation request accepted by alloc_large. */
/* Routine signature for application-supplied marker processing methods.
 * Need not pass marker code since it is stored in cinfo->unread_marker.
 */
/* Originally, this macro was used as a way of defining function prototypes
 * for both modern compilers as well as older compilers that did not support
 * prototype parameters.  libjpeg-turbo has never supported these older,
 * non-ANSI compilers, but the macro is still included because there is some
 * software out there that uses it.
 */
/* Default error-management setup */
/* Initialization of JPEG compression objects.
 * jpeg_create_compress() and jpeg_create_decompress() are the exported
 * names that applications should call.  These expand to calls on
 * jpeg_CreateCompress and jpeg_CreateDecompress with additional information
 * passed for version mismatch checking.
 * NB: you must set up the error-manager BEFORE calling jpeg_create_xxx.
 */
/* Destruction of JPEG compression objects */
/* Standard data source and destination managers: stdio streams. */
/* Caller is responsible for opening the file before and closing after. */
/* Data source and destination managers: memory buffers. */
/* Default parameter setup for compression */
/* Compression parameter setup aids */
/* Main entry points for compression */
/* Replaces jpeg_write_scanlines when writing raw downsampled data. */
/* Write a special marker.  See libjpeg.txt concerning safe usage. */
/* Same, but piecemeal. */
/* Alternate compression function: just write an abbreviated table file */
/* Write ICC profile.  See libjpeg.txt for usage information. */
/* Decompression startup: read start of JPEG datastream to see what's there */
/* Return value is one of: */
/* Suspended due to lack of input data */
/* Found valid image datastream */
/* Found valid table-specs-only datastream */
/* If you pass require_image = TRUE (normal case), you need not check for
 * a TABLES_ONLY return code; an abbreviated file will cause an error exit.
 * JPEG_SUSPENDED is only possible if you use a data source module that can
 * give a suspension return (the stdio source module doesn't).
 */
/* Main entry points for decompression */
/* Replaces jpeg_read_scanlines when reading raw downsampled data. */
/* Additional entry points for buffered-image mode. */
/*
 * Finish up after an output pass in buffered-image mode.
 *
 * Returns FALSE if suspended.  The return value need be inspected only if
 * a suspending data source is used.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_finish_output(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean {
    if ((*cinfo).global_state == crate::jpegint_h::DSTATE_SCANNING
        || (*cinfo).global_state == crate::jpegint_h::DSTATE_RAW_OK)
        && (*cinfo).buffered_image != 0
    {
        /* Terminate this pass. */
        /* We do not require the whole pass to have been completed. */
        Some(
            (*(*cinfo).master)
                .finish_output_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
        (*cinfo).global_state = crate::jpegint_h::DSTATE_BUFPOST
    } else if (*cinfo).global_state != crate::jpegint_h::DSTATE_BUFPOST {
        /* BUFPOST = repeat call after a suspension, anything else is error */
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Read markers looking for SOS or EOI */
    while (*cinfo).input_scan_number <= (*cinfo).output_scan_number
        && (*(*cinfo).inputctl).eoi_reached == 0
    {
        if Some(
            (*(*cinfo).inputctl)
                .consume_input
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == crate::jpeglib_h::JPEG_SUSPENDED
        {
            return crate::jmorecfg_h::FALSE;
        }
        /* Suspend, come back later */
    }
    (*cinfo).global_state = crate::jpegint_h::DSTATE_BUFIMAGE;
    return crate::jmorecfg_h::TRUE;
}
/* D_MULTISCAN_FILES_SUPPORTED */
