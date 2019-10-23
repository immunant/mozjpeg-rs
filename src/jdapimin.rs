use libc::c_void;use libc::c_ulong;use libc::c_int;use libc;

pub use crate::jconfig_h::JPEG_LIB_VERSION;
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
pub use crate::jpegint_h::jinit_input_controller;
pub use crate::jpegint_h::jinit_marker_reader;
pub use crate::jpegint_h::jinit_memory_mgr;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_abort;
pub use crate::jpeglib_h::jpeg_color_deconverter;
pub use crate::jpeglib_h::jpeg_color_quantizer;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_d_coef_controller;
pub use crate::jpeglib_h::jpeg_d_main_controller;
pub use crate::jpeglib_h::jpeg_d_post_controller;
pub use crate::jpeglib_h::jpeg_decomp_master;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_destroy;
pub use crate::jpeglib_h::jpeg_entropy_decoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_idct_method;
pub use crate::jpeglib_h::jpeg_idct_method_selector;
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
pub use crate::jpeglib_h::JDCT_DEFAULT;
pub use crate::jpeglib_h::JDCT_FLOAT;
pub use crate::jpeglib_h::JDCT_IFAST;
pub use crate::jpeglib_h::JDCT_ISLOW;
pub use crate::jpeglib_h::JDITHER_FS;
pub use crate::jpeglib_h::JDITHER_NONE;
pub use crate::jpeglib_h::JDITHER_ORDERED;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JPEG_HEADER_OK;
pub use crate::jpeglib_h::JPEG_HEADER_TABLES_ONLY;
pub use crate::jpeglib_h::JPOOL_PERMANENT;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::jpeglib_h::NUM_HUFF_TBLS;
pub use crate::jpeglib_h::NUM_QUANT_TBLS;
pub use super::jdmaster::my_decomp_master;
pub use super::jerror::C2RustUnnamed_3;
pub use super::jerror::JERR_ARITH_NOTIMPL;
pub use super::jerror::JERR_BAD_ALIGN_TYPE;
pub use super::jerror::JERR_BAD_ALLOC_CHUNK;
pub use super::jerror::JERR_BAD_BUFFER_MODE;
pub use super::jerror::JERR_BAD_COMPONENT_ID;
pub use super::jerror::JERR_BAD_CROP_SPEC;
pub use super::jerror::JERR_BAD_DCTSIZE;
pub use super::jerror::JERR_BAD_DCT_COEF;
pub use super::jerror::JERR_BAD_HUFF_TABLE;
pub use super::jerror::JERR_BAD_IN_COLORSPACE;
pub use super::jerror::JERR_BAD_J_COLORSPACE;
pub use super::jerror::JERR_BAD_LENGTH;
pub use super::jerror::JERR_BAD_LIB_VERSION;
pub use super::jerror::JERR_BAD_MCU_SIZE;
pub use super::jerror::JERR_BAD_PARAM;
pub use super::jerror::JERR_BAD_PARAM_VALUE;
pub use super::jerror::JERR_BAD_POOL_ID;
pub use super::jerror::JERR_BAD_PRECISION;
pub use super::jerror::JERR_BAD_PROGRESSION;
pub use super::jerror::JERR_BAD_PROG_SCRIPT;
pub use super::jerror::JERR_BAD_SAMPLING;
pub use super::jerror::JERR_BAD_SCAN_SCRIPT;
pub use super::jerror::JERR_BAD_STATE;
pub use super::jerror::JERR_BAD_STRUCT_SIZE;
pub use super::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use super::jerror::JERR_BUFFER_SIZE;
pub use super::jerror::JERR_CANT_SUSPEND;
pub use super::jerror::JERR_CCIR601_NOTIMPL;
pub use super::jerror::JERR_COMPONENT_COUNT;
pub use super::jerror::JERR_CONVERSION_NOTIMPL;
pub use super::jerror::JERR_DAC_INDEX;
pub use super::jerror::JERR_DAC_VALUE;
pub use super::jerror::JERR_DHT_INDEX;
pub use super::jerror::JERR_DQT_INDEX;
pub use super::jerror::JERR_EMPTY_IMAGE;
pub use super::jerror::JERR_EMS_READ;
pub use super::jerror::JERR_EMS_WRITE;
pub use super::jerror::JERR_EOI_EXPECTED;
pub use super::jerror::JERR_FILE_READ;
pub use super::jerror::JERR_FILE_WRITE;
pub use super::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use super::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use super::jerror::JERR_HUFF_MISSING_CODE;
pub use super::jerror::JERR_IMAGE_TOO_BIG;
pub use super::jerror::JERR_INPUT_EMPTY;
pub use super::jerror::JERR_INPUT_EOF;
pub use super::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use super::jerror::JERR_MISSING_DATA;
pub use super::jerror::JERR_MODE_CHANGE;
pub use super::jerror::JERR_NOTIMPL;
pub use super::jerror::JERR_NOT_COMPILED;
pub use super::jerror::JERR_NO_BACKING_STORE;
pub use super::jerror::JERR_NO_HUFF_TABLE;
pub use super::jerror::JERR_NO_IMAGE;
pub use super::jerror::JERR_NO_QUANT_TABLE;
pub use super::jerror::JERR_NO_SOI;
pub use super::jerror::JERR_OUT_OF_MEMORY;
pub use super::jerror::JERR_QUANT_COMPONENTS;
pub use super::jerror::JERR_QUANT_FEW_COLORS;
pub use super::jerror::JERR_QUANT_MANY_COLORS;
pub use super::jerror::JERR_SOF_DUPLICATE;
pub use super::jerror::JERR_SOF_NO_SOS;
pub use super::jerror::JERR_SOF_UNSUPPORTED;
pub use super::jerror::JERR_SOI_DUPLICATE;
pub use super::jerror::JERR_SOS_NO_SOF;
pub use super::jerror::JERR_TFILE_CREATE;
pub use super::jerror::JERR_TFILE_READ;
pub use super::jerror::JERR_TFILE_SEEK;
pub use super::jerror::JERR_TFILE_WRITE;
pub use super::jerror::JERR_TOO_LITTLE_DATA;
pub use super::jerror::JERR_UNKNOWN_MARKER;
pub use super::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use super::jerror::JERR_VIRTUAL_BUG;
pub use super::jerror::JERR_WIDTH_OVERFLOW;
pub use super::jerror::JERR_XMS_READ;
pub use super::jerror::JERR_XMS_WRITE;
pub use super::jerror::JMSG_COPYRIGHT;
pub use super::jerror::JMSG_LASTMSGCODE;
pub use super::jerror::JMSG_NOMESSAGE;
pub use super::jerror::JMSG_VERSION;
pub use super::jerror::JTRC_16BIT_TABLES;
pub use super::jerror::JTRC_ADOBE;
pub use super::jerror::JTRC_APP0;
pub use super::jerror::JTRC_APP14;
pub use super::jerror::JTRC_DAC;
pub use super::jerror::JTRC_DHT;
pub use super::jerror::JTRC_DQT;
pub use super::jerror::JTRC_DRI;
pub use super::jerror::JTRC_EMS_CLOSE;
pub use super::jerror::JTRC_EMS_OPEN;
pub use super::jerror::JTRC_EOI;
pub use super::jerror::JTRC_HUFFBITS;
pub use super::jerror::JTRC_JFIF;
pub use super::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use super::jerror::JTRC_JFIF_EXTENSION;
pub use super::jerror::JTRC_JFIF_THUMBNAIL;
pub use super::jerror::JTRC_MISC_MARKER;
pub use super::jerror::JTRC_PARMLESS_MARKER;
pub use super::jerror::JTRC_QUANTVALS;
pub use super::jerror::JTRC_QUANT_3_NCOLORS;
pub use super::jerror::JTRC_QUANT_NCOLORS;
pub use super::jerror::JTRC_QUANT_SELECTED;
pub use super::jerror::JTRC_RECOVERY_ACTION;
pub use super::jerror::JTRC_RST;
pub use super::jerror::JTRC_SMOOTH_NOTIMPL;
pub use super::jerror::JTRC_SOF;
pub use super::jerror::JTRC_SOF_COMPONENT;
pub use super::jerror::JTRC_SOI;
pub use super::jerror::JTRC_SOS;
pub use super::jerror::JTRC_SOS_COMPONENT;
pub use super::jerror::JTRC_SOS_PARAMS;
pub use super::jerror::JTRC_TFILE_CLOSE;
pub use super::jerror::JTRC_TFILE_OPEN;
pub use super::jerror::JTRC_THUMB_JPEG;
pub use super::jerror::JTRC_THUMB_PALETTE;
pub use super::jerror::JTRC_THUMB_RGB;
pub use super::jerror::JTRC_UNKNOWN_IDS;
pub use super::jerror::JTRC_XMS_CLOSE;
pub use super::jerror::JTRC_XMS_OPEN;
pub use super::jerror::JWRN_ADOBE_XFORM;
pub use super::jerror::JWRN_BOGUS_ICC;
pub use super::jerror::JWRN_BOGUS_PROGRESSION;
pub use super::jerror::JWRN_EXTRANEOUS_DATA;
pub use super::jerror::JWRN_HIT_MARKER;
pub use super::jerror::JWRN_HUFF_BAD_CODE;
pub use super::jerror::JWRN_JFIF_MAJOR;
pub use super::jerror::JWRN_JPEG_EOF;
pub use super::jerror::JWRN_MUST_RESYNC;
pub use super::jerror::JWRN_NOT_SEQUENTIAL;
pub use super::jerror::JWRN_TOO_MUCH_DATA;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
use crate::stdlib::memset;
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
