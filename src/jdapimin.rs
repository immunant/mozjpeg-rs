




































































































































































































































use crate::stdlib::memset;use libc::{c_void, c_ulong, c_int, self};pub use crate::jpegint_h::{inverse_DCT_method_ptr, jinit_input_controller,
                           jinit_marker_reader, jinit_memory_mgr,
                           JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT,
                           JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE, J_BUF_MODE};pub use crate::jpeglib_h::{j_common_ptr, j_decompress_ptr, jpeg_abort,
                           jpeg_color_deconverter, jpeg_color_quantizer,
                           jpeg_common_struct, jpeg_component_info,
                           jpeg_d_coef_controller, jpeg_d_main_controller,
                           jpeg_d_post_controller, jpeg_decomp_master,
                           jpeg_decompress_struct, jpeg_destroy,
                           jpeg_entropy_decoder, jpeg_error_mgr,
                           jpeg_idct_method, jpeg_idct_method_selector,
                           jpeg_input_controller, jpeg_inverse_dct,
                           jpeg_marker_parser_method, jpeg_marker_reader,
                           jpeg_marker_struct, jpeg_memory_mgr,
                           jpeg_progress_mgr, jpeg_saved_marker_ptr,
                           jpeg_source_mgr, jpeg_upsampler,
                           jvirt_barray_control, jvirt_barray_ptr,
                           jvirt_sarray_control, jvirt_sarray_ptr,
                           C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY,
                           JBLOCKROW, JCOEFPTR, JCS_CMYK, JCS_EXT_ABGR,
                           JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA,
                           JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA,
                           JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB,
                           JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN,
                           JCS_YCCK, JDCT_DEFAULT, JDCT_FLOAT, JDCT_IFAST,
                           JDCT_ISLOW, JDITHER_FS, JDITHER_NONE,
                           JDITHER_ORDERED, JHUFF_TBL, JPEG_HEADER_OK,
                           JPEG_HEADER_TABLES_ONLY, JPOOL_PERMANENT,
                           JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW,
                           J_COLOR_SPACE, J_DCT_METHOD, J_DITHER_MODE,
                           NUM_HUFF_TBLS, NUM_QUANT_TBLS};pub use crate::jmorecfg_h::{boolean, FALSE, JCOEF, JDIMENSION, JOCTET,
                            JSAMPLE, TRUE, UINT16, UINT8};pub use crate::stddef_h::{size_t, NULL};pub use crate::jconfig_h::JPEG_LIB_VERSION;pub use super::jdmaster::my_decomp_master;pub use super::jerror::{C2RustUnnamed_3, JERR_ARITH_NOTIMPL,
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
/*
 * jdapimin.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1998, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2016, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains application interface code for the decompression half
 * of the JPEG library.  These are the "minimum" API routines that may be
 * needed in either the normal full-decompression case or the
 * transcoding-only case.
 *
 * Most of the routines intended to be called directly by an application
 * are in this file or in jdapistd.c.  But also see jcomapi.c for routines
 * shared by compression and decompression, and jdtrans.c for the transcoding
 * case.
 */
/*
 * Initialization of a JPEG decompression object.
 * The error manager must already be set up (in case memory manager fails).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_CreateDecompress(
    mut cinfo: j_decompress_ptr,
    mut version: c_int,
    mut structsize: size_t,
) {
     
    /* Guard against version mismatches between library and caller. */
    (*cinfo).mem = NULL as *mut jpeg_memory_mgr; /* so jpeg_destroy knows mem mgr not called */
    if version != JPEG_LIB_VERSION {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_LIB_VERSION as c_int;
        (*(*cinfo).err).msg_parm.i[0] = 62i32;
        (*(*cinfo).err).msg_parm.i[1] = version;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if structsize
        != ::std::mem::size_of::<jpeg_decompress_struct>() as c_ulong
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_STRUCT_SIZE as c_int;
        (*(*cinfo).err).msg_parm.i[0] =  ::std::mem::size_of::<
            jpeg_decompress_struct,
        >() as c_int;
        (*(*cinfo).err).msg_parm.i[1] = structsize as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* For debugging purposes, we zero the whole master structure.
     * But the application has already set the err pointer, and may have set
     * client_data, so we have to save and restore those fields.
     * Note: if application hasn't set client_data, tools like Purify may
     * complain here.
     */
    let mut err: *mut jpeg_error_mgr = (*cinfo).err; /* ignore Purify complaint here */
    let mut client_data: *mut c_void = (*cinfo).client_data;
    memset(
        cinfo as *mut c_void,
        0i32,
        ::std::mem::size_of::<jpeg_decompress_struct>() as c_ulong,
    );
    (*cinfo).err = err;
    (*cinfo).client_data = client_data;
    (*cinfo).is_decompressor = TRUE;
    /* Initialize a memory manager instance for this object */
    jinit_memory_mgr(cinfo as j_common_ptr);
    /* Zero out pointers to permanent structures. */
    (*cinfo).progress = NULL as *mut jpeg_progress_mgr;
    (*cinfo).src = NULL as *mut jpeg_source_mgr;
     let mut i:   c_int =  0i32;
    while i < NUM_QUANT_TBLS {
        (*cinfo).quant_tbl_ptrs[i as usize] =
            NULL as *mut JQUANT_TBL;
        i += 1
    }
    i = 0i32;
    while i < NUM_HUFF_TBLS {
        (*cinfo).dc_huff_tbl_ptrs[i as usize] =
            NULL as *mut JHUFF_TBL;
        (*cinfo).ac_huff_tbl_ptrs[i as usize] =
            NULL as *mut JHUFF_TBL;
        i += 1
    }
    /* Initialize marker processor so application can override methods
     * for COM, APPn markers before calling jpeg_read_header.
     */
    (*cinfo).marker_list = NULL as jpeg_saved_marker_ptr;
    jinit_marker_reader(cinfo);
    /* And initialize the overall input controller. */
    jinit_input_controller(cinfo);
    /* OK, I'm ready */
    (*cinfo).global_state = 200i32;
    /* The master struct is used to store extension parameters, so we allocate it
     * here.
     */
    (*cinfo).master = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_PERMANENT,
        ::std::mem::size_of::<super::jdmaster::my_decomp_master>() as c_ulong,
    ) as *mut jpeg_decomp_master;
    memset(
        (*cinfo).master as *mut c_void,
        0i32,
        ::std::mem::size_of::<super::jdmaster::my_decomp_master>() as c_ulong,
    );
}
/*
 * Destruction of a JPEG decompression object
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_destroy_decompress(mut cinfo: j_decompress_ptr) {
    jpeg_destroy(cinfo as j_common_ptr);
    /* use common routine */
}
/*
 * Abort processing of a JPEG decompression operation,
 * but don't destroy the object itself.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_abort_decompress(mut cinfo: j_decompress_ptr) {
    jpeg_abort(cinfo as j_common_ptr);
    /* use common routine */
}
/*
 * Set default decompression parameters.
 */

unsafe extern "C" fn default_decompress_parms(mut cinfo: j_decompress_ptr) {
    /* Guess the input colorspace, and set output colorspace accordingly. */
    /* (Wish JPEG committee had provided a real way to specify this...) */
    /* Note application may override our guesses. */
    match (*cinfo).num_components {
        1 => {
            (*cinfo).jpeg_color_space = JCS_GRAYSCALE;
            (*cinfo).out_color_space = JCS_GRAYSCALE
        }
        3 => {
            if (*cinfo).saw_JFIF_marker != 0 {
                (*cinfo).jpeg_color_space = JCS_YCbCr
            /* JFIF implies YCbCr */
            } else if (*cinfo).saw_Adobe_marker != 0 {
                match (*cinfo).Adobe_transform as c_int {
                    0 => (*cinfo).jpeg_color_space = JCS_RGB,
                    1 => (*cinfo).jpeg_color_space = JCS_YCbCr,
                    _ => {
                        (*(*cinfo).err).msg_code =
                            super::jerror::JWRN_ADOBE_XFORM as c_int; /* assume it's YCbCr */
                        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).Adobe_transform as c_int;
                        Some(
                            (*(*cinfo).err)
                                .emit_message
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as j_common_ptr,
                            -1i32,
                        );
                        (*cinfo).jpeg_color_space = JCS_YCbCr
                    }
                }
            } else {
                /* Saw no special markers, try to guess from the component IDs */
                let mut cid0: c_int = (*(*cinfo).comp_info.offset(0)).component_id; /* assume JFIF w/out marker */
                let mut cid1: c_int = (*(*cinfo).comp_info.offset(1)).component_id; /* ASCII 'R', 'G', 'B' */
                let mut cid2: c_int = (*(*cinfo).comp_info.offset(2)).component_id;
                if cid0 == 1i32 && cid1 == 2i32 && cid2 == 3i32 {
                    (*cinfo).jpeg_color_space = JCS_YCbCr
                } else if cid0 == 82i32 && cid1 == 71i32 && cid2 == 66i32 {
                    (*cinfo).jpeg_color_space = JCS_RGB
                } else {
                    let mut _mp: *mut c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
                    *_mp.offset(0) = cid0;
                    *_mp.offset(1) = cid1;
                    *_mp.offset(2) = cid2;
                    (*(*cinfo).err).msg_code = super::jerror::JTRC_UNKNOWN_IDS as c_int;
                    Some(
                        (*(*cinfo).err)
                            .emit_message
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr,
                        1i32,
                    );
                    (*cinfo).jpeg_color_space = JCS_YCbCr
                    /* assume it's YCbCr */
                }
            }
            /* Always guess RGB is proper output colorspace. */
            (*cinfo).out_color_space = JCS_RGB
        }
        4 => {
            if (*cinfo).saw_Adobe_marker != 0 {
                match (*cinfo).Adobe_transform as c_int {
                    0 => (*cinfo).jpeg_color_space = JCS_CMYK,
                    2 => (*cinfo).jpeg_color_space = JCS_YCCK,
                    _ => {
                        (*(*cinfo).err).msg_code =
                            super::jerror::JWRN_ADOBE_XFORM as c_int; /* assume it's YCCK */
                        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).Adobe_transform as c_int;
                        Some(
                            (*(*cinfo).err)
                                .emit_message
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as j_common_ptr,
                            -1i32,
                        );
                        (*cinfo).jpeg_color_space = JCS_YCCK
                    }
                }
            } else {
                /* No special markers, assume straight CMYK. */
                (*cinfo).jpeg_color_space = JCS_CMYK
            }
            (*cinfo).out_color_space = JCS_CMYK
        }
        _ => {
            (*cinfo).jpeg_color_space = JCS_UNKNOWN;
            (*cinfo).out_color_space = JCS_UNKNOWN
        }
    }
    /* Set defaults for other decompression parameters. */
    (*cinfo).scale_num = 1u32; /* 1:1 scaling */
    (*cinfo).scale_denom = 1u32;
    (*cinfo).output_gamma = 1.0f64;
    (*cinfo).buffered_image = FALSE;
    (*cinfo).raw_data_out = FALSE;
    (*cinfo).dct_method = JDCT_DEFAULT as J_DCT_METHOD;
    (*cinfo).do_fancy_upsampling = TRUE;
    (*cinfo).do_block_smoothing = TRUE;
    (*cinfo).quantize_colors = FALSE;
    /* We set these in case application only sets quantize_colors. */
    (*cinfo).dither_mode = JDITHER_FS;
    (*cinfo).two_pass_quantize = TRUE;
    (*cinfo).desired_number_of_colors = 256i32;
    (*cinfo).colormap = NULL as JSAMPARRAY;
    /* Initialize for no mode change in buffered-image mode. */
    (*cinfo).enable_1pass_quant = FALSE;
    (*cinfo).enable_external_quant = FALSE;
    (*cinfo).enable_2pass_quant = FALSE;
}
/*
 * Decompression startup: read start of JPEG datastream to see what's there.
 * Need only initialize JPEG object and supply a data source before calling.
 *
 * This routine will read as far as the first SOS marker (ie, actual start of
 * compressed data), and will save all tables and parameters in the JPEG
 * object.  It will also initialize the decompression parameters to default
 * values, and finally return JPEG_HEADER_OK.  On return, the application may
 * adjust the decompression parameters and then call jpeg_start_decompress.
 * (Or, if the application only wanted to determine the image parameters,
 * the data need not be decompressed.  In that case, call jpeg_abort or
 * jpeg_destroy to release any temporary space.)
 * If an abbreviated (tables only) datastream is presented, the routine will
 * return JPEG_HEADER_TABLES_ONLY upon reaching EOI.  The application may then
 * re-use the JPEG object to read the abbreviated image datastream(s).
 * It is unnecessary (but OK) to call jpeg_abort in this case.
 * The JPEG_SUSPENDED return code only occurs if the data source module
 * requests suspension of the decompressor.  In this case the application
 * should load more source data and then re-call jpeg_read_header to resume
 * processing.
 * If a non-suspending data source is used and require_image is TRUE, then the
 * return code need not be inspected since only JPEG_HEADER_OK is possible.
 *
 * This routine is now just a front end to jpeg_consume_input, with some
 * extra error checking.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_read_header(
    mut cinfo: j_decompress_ptr,
    mut require_image: boolean,
) -> c_int {
     
    if (*cinfo).global_state != 200i32 && (*cinfo).global_state != 201i32 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
     let mut retcode:   c_int =  jpeg_consume_input(cinfo);
    match retcode {
        1 => retcode = JPEG_HEADER_OK,
        2 => {
            if require_image != 0 {
                /* Complain if application wanted an image */
                (*(*cinfo).err).msg_code = super::jerror::JERR_NO_IMAGE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            /* Reset to start state; it would be safer to require the application to
             * call jpeg_abort, but we can't change it now for compatibility reasons.
             * A side effect is to free any temporary memory (there shouldn't be any).
             */
            jpeg_abort(cinfo as j_common_ptr); /* sets state = DSTATE_START */
            retcode = JPEG_HEADER_TABLES_ONLY
        }
        0 | _ => {}
    }
    return retcode;
}
/*
 * Consume data in advance of what the decompressor requires.
 * This can be called at any time once the decompressor object has
 * been created and a data source has been set up.
 *
 * This routine is essentially a state machine that handles a couple
 * of critical state-transition actions, namely initial setup and
 * transition from header scanning to ready-for-start_decompress.
 * All the actual input is done via the input controller's consume_input
 * method.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_consume_input(
    mut cinfo: j_decompress_ptr,
) -> c_int {
    
     let mut retcode:  c_int =  0i32; let mut current_block_10:  u64;
    /* NB: every possible DSTATE value should be listed in this switch */
    match (*cinfo).global_state {
        200 => {
            /* Start-of-datastream actions: reset appropriate modules */
            Some(
                (*(*cinfo).inputctl)
                    .reset_input_controller
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
            /* Initialize application's data source module */
            Some(
                (*(*cinfo).src)
                    .init_source
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
            (*cinfo).global_state = 201i32;
            current_block_10 = 7481093856908124627;
        }
        201 => {
            current_block_10 = 7481093856908124627;
        }
        202 => {
            /* Can't advance past first SOS until start_decompress is called */
            retcode = 1i32;
            current_block_10 = 7149356873433890176;
        }
        203 | 204 | 205 | 206 | 207 | 208 | 210 => {
            retcode = Some(
                (*(*cinfo).inputctl)
                    .consume_input
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
            current_block_10 = 7149356873433890176;
        }
        _ => {
            (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_STATE as c_int;
            (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
            current_block_10 = 7149356873433890176;
        }
    }
    match current_block_10 {
        7481093856908124627 =>
        /*FALLTHROUGH*/
        {
            retcode = Some(
                (*(*cinfo).inputctl)
                    .consume_input
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
            if retcode == 1i32 {
                /* Found SOS, prepare to decompress */
                /* Set up default parameters based on header data */
                default_decompress_parms(cinfo);
                /* Set global state: ready for start_decompress */
                (*cinfo).global_state = 202i32
            }
        }
        _ => {}
    }
    return retcode;
}
/*
 * Have we finished reading the input file?
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_input_complete(
    cinfo: j_decompress_ptr,
) -> boolean {
    /* Check for valid jpeg object */
    if (*cinfo).global_state < 200i32 || (*cinfo).global_state > 210i32 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    return (*(*cinfo).inputctl).eoi_reached;
}
/*
 * Is there more than one scan?
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_has_multiple_scans(
    cinfo: j_decompress_ptr,
) -> boolean {
    /* Only valid after jpeg_read_header completes */
    if (*cinfo).global_state < 202i32 || (*cinfo).global_state > 210i32 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    return (*(*cinfo).inputctl).has_multiple_scans;
}
/*
 * Finish JPEG decompression.
 *
 * This will normally just verify the file trailer and release temp storage.
 *
 * Returns FALSE if suspended.  The return value need be inspected only if
 * a suspending data source is used.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_finish_decompress(
    mut cinfo: j_decompress_ptr,
) -> boolean {
    if ((*cinfo).global_state == 205i32 || (*cinfo).global_state == 206i32)
        && (*cinfo).buffered_image == 0
    {
        /* Terminate final pass of non-buffered mode */
        if (*cinfo).output_scanline < (*cinfo).output_height {
            (*(*cinfo).err).msg_code = super::jerror::JERR_TOO_LITTLE_DATA as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        Some(
            (*(*cinfo).master)
                .finish_output_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
        (*cinfo).global_state = 210i32
    } else if (*cinfo).global_state == 207i32 {
        /* Finishing after a buffered-image operation */
        (*cinfo).global_state = 210i32
    } else if (*cinfo).global_state != 210i32 {
        /* STOPPING = repeat call after a suspension, anything else is error */
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Read until EOI */
    while (*(*cinfo).inputctl).eoi_reached == 0 {
        if Some(
            (*(*cinfo).inputctl)
                .consume_input
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0i32
        {
            return FALSE;
        }
        /* Suspend, come back later */
    }
    /* Do final cleanup */
    Some(
        (*(*cinfo).src)
            .term_source
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    /* We can use jpeg_abort to release memory and reset global_state */
    jpeg_abort(cinfo as j_common_ptr);
    return TRUE;
}
