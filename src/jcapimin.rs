use libc::c_uint;use libc::c_ulong;use libc::c_int;use libc::c_long;use libc::c_void;use libc;

pub use crate::jconfig_h::JPEG_LIB_VERSION;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::jinit_marker_writer;
pub use crate::jpegint_h::jinit_memory_mgr;
pub use crate::jpegint_h::CSTATE_RAW_OK;
pub use crate::jpegint_h::CSTATE_SCANNING;
pub use crate::jpegint_h::CSTATE_START;
pub use crate::jpegint_h::CSTATE_WRCOEFS;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_abort;
pub use crate::jpeglib_h::jpeg_c_coef_controller;
pub use crate::jpeglib_h::jpeg_c_main_controller;
pub use crate::jpeglib_h::jpeg_c_prep_controller;
pub use crate::jpeglib_h::jpeg_color_converter;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_comp_master;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_destroy;
pub use crate::jpeglib_h::jpeg_downsampler;
pub use crate::jpeglib_h::jpeg_entropy_encoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_forward_dct;
pub use crate::jpeglib_h::jpeg_marker_writer;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_1;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JCP_FASTEST;
pub use crate::jpeglib_h::JCP_MAX_COMPRESSION;
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
pub use crate::jpeglib_h::JPOOL_PERMANENT;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::NUM_HUFF_TBLS;
pub use crate::jpeglib_h::NUM_QUANT_TBLS;
pub use super::jcmaster::c_pass_type;
pub use super::jcmaster::huff_opt_pass;
pub use super::jcmaster::main_pass;
pub use super::jcmaster::my_comp_master;
pub use super::jcmaster::output_pass;
pub use super::jcmaster::trellis_pass;
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
 * jcapimin.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1998, Thomas G. Lane.
 * Modified 2003-2010 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2014, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains application interface code for the compression half
 * of the JPEG library.  These are the "minimum" API routines that may be
 * needed in either the normal full-compression case or the transcoding-only
 * case.
 *
 * Most of the routines intended to be called directly by an application
 * are in this file or in jcapistd.c.  But also see jcparam.c for
 * parameter-setup helper routines, jcomapi.c for routines shared by
 * compression and decompression, and jctrans.c for the transcoding case.
 */
/*
 * Initialization of a JPEG compression object.
 * The error manager must already be set up (in case memory manager fails).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_CreateCompress(
    mut cinfo: j_compress_ptr,
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
        != ::std::mem::size_of::<jpeg_compress_struct>() as c_ulong
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_STRUCT_SIZE as c_int;
        (*(*cinfo).err).msg_parm.i[0] =
            
            ::std::mem::size_of::<jpeg_compress_struct>()
                as c_int;
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
        ::std::mem::size_of::<jpeg_compress_struct>() as c_ulong,
    );
    (*cinfo).err = err;
    (*cinfo).client_data = client_data;
    (*cinfo).is_decompressor = FALSE;
    /* Initialize a memory manager instance for this object */
    jinit_memory_mgr(cinfo as j_common_ptr);
    /* Zero out pointers to permanent structures. */
    (*cinfo).progress = NULL as *mut jpeg_progress_mgr; /* in case application forgets */
    (*cinfo).dest = NULL as *mut jpeg_destination_mgr;
    (*cinfo).comp_info = NULL as *mut jpeg_component_info;
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
    (*cinfo).script_space = NULL as *mut jpeg_scan_info;
    (*cinfo).input_gamma = 1.0f64;
    /* OK, I'm ready */
    (*cinfo).global_state = CSTATE_START;
    /* The master struct is used to store extension parameters, so we allocate it
     * here.  It is later reallocated by jinit_c_master_control().
     */
    (*cinfo).master = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_PERMANENT,
        ::std::mem::size_of::<super::jcmaster::my_comp_master>() as c_ulong,
    ) as *mut jpeg_comp_master;
    memset(
        (*cinfo).master as *mut c_void,
        0i32,
        ::std::mem::size_of::<super::jcmaster::my_comp_master>() as c_ulong,
    );
    (*(*cinfo).master).compress_profile = JCP_MAX_COMPRESSION as c_int;
}
/*
 * Destruction of a JPEG compression object
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_destroy_compress(mut cinfo: j_compress_ptr) {
    jpeg_destroy(cinfo as j_common_ptr);
    /* use common routine */
}
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
/* Return value is one of: */
/* #define JPEG_SUSPENDED       0    Suspended due to lack of input data */
/* Reached start of new scan */
/* Reached end of image */
/* Completed one iMCU row */
/* Completed last iMCU row of a scan */
/* Precalculate output dimensions for current decompression parameters. */
/* Control saving of COM and APPn markers into marker_list. */
/* Install a special processing method for COM or APPn markers. */
/* Read or write raw DCT coefficients --- useful for lossless transcoding. */
/* If you choose to abort compression or decompression before completing
 * jpeg_finish_(de)compress, then you need to clean up to release memory,
 * temporary files, etc.  You can just call jpeg_destroy_(de)compress
 * if you're done with the JPEG object, but if you want to clean it up and
 * reuse it, call this:
 */
/*
 * Abort processing of a JPEG compression operation,
 * but don't destroy the object itself.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_abort_compress(mut cinfo: j_compress_ptr) {
    jpeg_abort(cinfo as j_common_ptr);
    /* use common routine */
}
/*
 * Forcibly suppress or un-suppress all quantization and Huffman tables.
 * Marks all currently defined tables as already written (if suppress)
 * or not written (if !suppress).  This will control whether they get emitted
 * by a subsequent jpeg_start_compress call.
 *
 * This routine is exported for use by applications that want to produce
 * abbreviated JPEG datastreams.  It logically belongs in jcparam.c, but
 * since it is called by jpeg_start_compress, we put it here --- otherwise
 * jcparam.o would be linked whether the application used it or not.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_suppress_tables(
    mut cinfo: j_compress_ptr,
    mut suppress: boolean,
) {
    
    
     
     let mut i:   c_int =  0i32;
    while i < NUM_QUANT_TBLS {
          let mut qtbl:   *mut JQUANT_TBL =
     (*cinfo).quant_tbl_ptrs[i as usize];
        if !qtbl.is_null() {
            (*qtbl).sent_table = suppress
        }
        i += 1
    }
    i = 0i32;
    while i < NUM_HUFF_TBLS {
          let mut htbl:   *mut JHUFF_TBL =
     (*cinfo).dc_huff_tbl_ptrs[i as usize];
        if !htbl.is_null() {
            (*htbl).sent_table = suppress
        }
        htbl = (*cinfo).ac_huff_tbl_ptrs[i as usize];
        if !htbl.is_null() {
            (*htbl).sent_table = suppress
        }
        i += 1
    }
}
/*
 * Finish JPEG compression.
 *
 * If a multipass operating mode was selected, this may do a great deal of
 * work including most of the actual output.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_finish_compress(mut cinfo: j_compress_ptr) {
    
    if (*cinfo).global_state == CSTATE_SCANNING
        || (*cinfo).global_state == CSTATE_RAW_OK
    {
        /* Terminate first pass */
        if (*cinfo).next_scanline < (*cinfo).image_height {
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
                .finish_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    } else if (*cinfo).global_state != CSTATE_WRCOEFS {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Perform any remaining passes */
    while (*(*cinfo).master).is_last_pass == 0 {
         Some(
            (*(*cinfo).master)
                .prepare_for_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
         let mut iMCU_row:   JDIMENSION =  0u32;
        while iMCU_row < (*cinfo).total_iMCU_rows {
            if !(*cinfo).progress.is_null() {
                (*(*cinfo).progress).pass_counter = iMCU_row as c_long;
                (*(*cinfo).progress).pass_limit = (*cinfo).total_iMCU_rows as c_long;
                Some(
                    (*(*cinfo).progress)
                        .progress_monitor
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            /* We bypass the main controller and invoke coef controller directly;
             * all work is being done from the coefficient buffer.
             */
            if Some(
                (*(*cinfo).coef)
                    .compress_data
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                
                NULL as JSAMPIMAGE,
            ) == 0
            {
                (*(*cinfo).err).msg_code = super::jerror::JERR_CANT_SUSPEND as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            iMCU_row +=  1
        }
        Some(
            (*(*cinfo).master)
                .finish_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    /* Write EOI, do final cleanup */
    Some(
        (*(*cinfo).marker)
            .write_file_trailer
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    Some(
        (*(*cinfo).dest)
            .term_destination
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    /* We can use jpeg_abort to release memory and reset global_state */
    jpeg_abort(cinfo as j_common_ptr);
}
/*
 * Write a special marker.
 * This is only recommended for writing COM or APPn markers.
 * Must be called after jpeg_start_compress() and before
 * first call to jpeg_write_scanlines() or jpeg_write_raw_data().
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_write_marker(
    mut cinfo: j_compress_ptr,
    mut marker: c_int,
    mut dataptr: *const JOCTET,
    mut datalen: c_uint,
) {
    let mut write_marker_byte: Option<
        unsafe extern "C" fn(_: j_compress_ptr, _: c_int) -> (),
    > = None; /* copy for speed */
    if (*cinfo).next_scanline != 0u32
        || (*cinfo).global_state != CSTATE_SCANNING
            && (*cinfo).global_state != CSTATE_RAW_OK
            && (*cinfo).global_state != CSTATE_WRCOEFS
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    Some(
        (*(*cinfo).marker)
            .write_marker_header
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, marker, datalen);
    write_marker_byte = (*(*cinfo).marker).write_marker_byte;
    loop {
        let fresh0 = datalen;
        datalen -=  1;
        if !(fresh0 != 0) {
            break;
        }
        Some(write_marker_byte.expect("non-null function pointer"))
            .expect("non-null function pointer")(cinfo, *dataptr as c_int);
        dataptr = dataptr.offset(1)
    }
}
/* Same, but piecemeal. */
#[no_mangle]

pub unsafe extern "C" fn jpeg_write_m_header(
    mut cinfo: j_compress_ptr,
    mut marker: c_int,
    mut datalen: c_uint,
) {
    if (*cinfo).next_scanline != 0u32
        || (*cinfo).global_state != CSTATE_SCANNING
            && (*cinfo).global_state != CSTATE_RAW_OK
            && (*cinfo).global_state != CSTATE_WRCOEFS
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    Some(
        (*(*cinfo).marker)
            .write_marker_header
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, marker, datalen);
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_write_m_byte(
    mut cinfo: j_compress_ptr,
    mut val: c_int,
) {
    Some(
        (*(*cinfo).marker)
            .write_marker_byte
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, val);
}
/* "Object" declarations for JPEG modules that may be supplied or called
 * directly by the surrounding application.
 * As with all objects in the JPEG library, these structs only define the
 * publicly visible methods and state variables of a module.  Additional
 * private fields may exist after the public ones.
 */
/* Error handler object */
/* Error exit handler: does not return to caller */
/* Conditionally emit a trace or warning message */
/* Routine that actually outputs a trace or error message */
/* Format a message string for the most recent JPEG error or message */
/* recommended size of format_message buffer */
/* Reset error state variables at start of a new image */
/* The message ID code and any parameters are saved here.
 * A message can have one string parameter or up to 8 int parameters.
 */
/* Standard state variables for error facility */
/* max msg_level that will be displayed */
/* For recoverable corrupt-data errors, we emit a warning message,
 * but keep going unless emit_message chooses to abort.  emit_message
 * should count warnings in num_warnings.  The surrounding application
 * can check for bad data by seeing if num_warnings is nonzero at the
 * end of processing.
 */
/* number of corrupt-data warnings */
/* These fields point to the table(s) of error message strings.
 * An application can change the table pointer to switch to a different
 * message list (typically, to change the language in which errors are
 * reported).  Some applications may wish to add additional error codes
 * that will be handled by the JPEG library error mechanism; the second
 * table pointer is used for this purpose.
 *
 * First table includes all errors generated by JPEG library itself.
 * Error code 0 is reserved for a "no such error string" message.
 */
/* Library errors */
/* Table contains strings 0..last_jpeg_message */
/* Second table can be added by application (see cjpeg/djpeg for example).
 * It contains strings numbered first_addon_message..last_addon_message.
 */
/* Non-library errors */
/* code for first string in addon table */
/* code for last string in addon table */
/* Progress monitor object */
/* work units completed in this pass */
/* total number of work units in this pass */
/* passes completed so far */
/* total number of passes expected */
/* Data destination object for compression */
/* => next byte to write in buffer */
/* # of byte spaces remaining in buffer */
/* Data source object for decompression */
/* => next byte to read from buffer */
/* # of bytes remaining in buffer */
/* Memory manager object.
 * Allocates "small" objects (a few K total), "large" objects (tens of K),
 * and "really big" objects (virtual arrays with backing store if needed).
 * The memory manager does not allow individual objects to be freed; rather,
 * each created object is assigned to a pool, and whole pools can be freed
 * at once.  This is faster and more convenient than remembering exactly what
 * to free, especially where malloc()/free() are not too speedy.
 * NB: alloc routines never return NULL.  They exit to error_exit if not
 * successful.
 */
/* lasts until master record is destroyed */
/* lasts until done with image/datastream */
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
/*
 * Alternate compression function: just write an abbreviated table file.
 * Before calling this, all parameters and a data destination must be set up.
 *
 * To produce a pair of files containing abbreviated tables and abbreviated
 * image data, one would proceed as follows:
 *
 *              initialize JPEG object
 *              set JPEG parameters
 *              set destination to table file
 *              jpeg_write_tables(cinfo);
 *              set destination to image file
 *              jpeg_start_compress(cinfo, FALSE);
 *              write data...
 *              jpeg_finish_compress(cinfo);
 *
 * jpeg_write_tables has the side effect of marking all tables written
 * (same as jpeg_suppress_tables(..., TRUE)).  Thus a subsequent start_compress
 * will not re-emit the tables unless it is passed write_all_tables=TRUE.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_write_tables(mut cinfo: j_compress_ptr) {
    if (*cinfo).global_state != CSTATE_START {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* (Re)initialize error mgr and destination modules */
    Some(
        (*(*cinfo).err)
            .reset_error_mgr
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as j_common_ptr);
    Some(
        (*(*cinfo).dest)
            .init_destination
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    /* Initialize the marker writer ... bit of a crock to do it here. */
    jinit_marker_writer(cinfo);
    /* Write them tables! */
    Some(
        (*(*cinfo).marker)
            .write_tables_only
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    /* And clean up. */
    Some(
        (*(*cinfo).dest)
            .term_destination
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    /*
     * In library releases up through v6a, we called jpeg_abort() here to free
     * any working memory allocated by the destination manager and marker
     * writer.  Some applications had a problem with that: they allocated space
     * of their own from the library memory manager, and didn't want it to go
     * away during write_tables.  So now we do nothing.  This will cause a
     * memory leak if an app calls write_tables repeatedly without doing a full
     * compression cycle or otherwise resetting the JPEG object.  However, that
     * seems less bad than unexpectedly freeing memory in the normal case.
     * An app that prefers the old behavior can call jpeg_abort for itself after
     * each call to jpeg_write_tables().
     */
}
