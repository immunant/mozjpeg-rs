use ::libc;

pub use crate::jconfig_h::JPEG_LIB_VERSION;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
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
pub use crate::src::jcmarker::jinit_marker_writer;
pub use crate::src::jcmaster::c_pass_type;
pub use crate::src::jcmaster::huff_opt_pass;
pub use crate::src::jcmaster::main_pass;
pub use crate::src::jcmaster::my_comp_master;
pub use crate::src::jcmaster::output_pass;
pub use crate::src::jcmaster::trellis_pass;
pub use crate::src::jcomapi::jpeg_abort;
pub use crate::src::jcomapi::jpeg_destroy;
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
pub use crate::src::jmemmgr::jinit_memory_mgr;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
use crate::stdlib::memset;
pub use crate::stdlib::C2RustUnnamed_0;
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
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut version: libc::c_int,
    mut structsize: crate::stddef_h::size_t,
) {
    let mut i: libc::c_int = 0;
    /* Guard against version mismatches between library and caller. */
    (*cinfo).mem = crate::stddef_h::NULL as *mut crate::jpeglib_h::jpeg_memory_mgr; /* so jpeg_destroy knows mem mgr not called */
    if version != crate::jconfig_h::JPEG_LIB_VERSION {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_LIB_VERSION as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = 62 as libc::c_int;
        (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = version;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if structsize
        != ::std::mem::size_of::<crate::jpeglib_h::jpeg_compress_struct>() as libc::c_ulong
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_STRUCT_SIZE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] =
            ::std::mem::size_of::<crate::jpeglib_h::jpeg_compress_struct>() as libc::c_ulong
                as libc::c_int;
        (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = structsize as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* For debugging purposes, we zero the whole master structure.
     * But the application has already set the err pointer, and may have set
     * client_data, so we have to save and restore those fields.
     * Note: if application hasn't set client_data, tools like Purify may
     * complain here.
     */
    let mut err: *mut crate::jpeglib_h::jpeg_error_mgr = (*cinfo).err; /* ignore Purify complaint here */
    let mut client_data: *mut libc::c_void = (*cinfo).client_data;
    crate::stdlib::memset(
        cinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::jpeglib_h::jpeg_compress_struct>() as libc::c_ulong,
    );
    (*cinfo).err = err;
    (*cinfo).client_data = client_data;
    (*cinfo).is_decompressor = crate::jmorecfg_h::FALSE;
    /* Initialize a memory manager instance for this object */
    crate::src::jmemmgr::jinit_memory_mgr(cinfo as crate::jpeglib_h::j_common_ptr);
    /* Zero out pointers to permanent structures. */
    (*cinfo).progress = crate::stddef_h::NULL as *mut crate::jpeglib_h::jpeg_progress_mgr; /* in case application forgets */
    (*cinfo).dest = crate::stddef_h::NULL as *mut crate::jpeglib_h::jpeg_destination_mgr;
    (*cinfo).comp_info = crate::stddef_h::NULL as *mut crate::jpeglib_h::jpeg_component_info;
    i = 0 as libc::c_int;
    while i < crate::jpeglib_h::NUM_QUANT_TBLS {
        (*cinfo).quant_tbl_ptrs[i as usize] =
            crate::stddef_h::NULL as *mut crate::jpeglib_h::JQUANT_TBL;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < crate::jpeglib_h::NUM_HUFF_TBLS {
        (*cinfo).dc_huff_tbl_ptrs[i as usize] =
            crate::stddef_h::NULL as *mut crate::jpeglib_h::JHUFF_TBL;
        (*cinfo).ac_huff_tbl_ptrs[i as usize] =
            crate::stddef_h::NULL as *mut crate::jpeglib_h::JHUFF_TBL;
        i += 1
    }
    (*cinfo).script_space = crate::stddef_h::NULL as *mut crate::jpeglib_h::jpeg_scan_info;
    (*cinfo).input_gamma = 1.0f64;
    /* OK, I'm ready */
    (*cinfo).global_state = crate::jpegint_h::CSTATE_START;
    /* The master struct is used to store extension parameters, so we allocate it
     * here.  It is later reallocated by jinit_c_master_control().
     */
    (*cinfo).master = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_PERMANENT,
        ::std::mem::size_of::<crate::src::jcmaster::my_comp_master>() as libc::c_ulong,
    ) as *mut crate::jpegint_h::jpeg_comp_master;
    crate::stdlib::memset(
        (*cinfo).master as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::src::jcmaster::my_comp_master>() as libc::c_ulong,
    );
    (*(*cinfo).master).compress_profile = crate::jpeglib_h::JCP_MAX_COMPRESSION as libc::c_int;
}
/*
 * Destruction of a JPEG compression object
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_destroy_compress(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    crate::src::jcomapi::jpeg_destroy(cinfo as crate::jpeglib_h::j_common_ptr);
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

pub unsafe extern "C" fn jpeg_abort_compress(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    crate::src::jcomapi::jpeg_abort(cinfo as crate::jpeglib_h::j_common_ptr);
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
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut suppress: crate::jmorecfg_h::boolean,
) {
    let mut i: libc::c_int = 0;
    let mut qtbl: *mut crate::jpeglib_h::JQUANT_TBL = 0 as *mut crate::jpeglib_h::JQUANT_TBL;
    let mut htbl: *mut crate::jpeglib_h::JHUFF_TBL = 0 as *mut crate::jpeglib_h::JHUFF_TBL;
    i = 0 as libc::c_int;
    while i < crate::jpeglib_h::NUM_QUANT_TBLS {
        qtbl = (*cinfo).quant_tbl_ptrs[i as usize];
        if !qtbl.is_null() {
            (*qtbl).sent_table = suppress
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < crate::jpeglib_h::NUM_HUFF_TBLS {
        htbl = (*cinfo).dc_huff_tbl_ptrs[i as usize];
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

pub unsafe extern "C" fn jpeg_finish_compress(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    let mut iMCU_row: crate::jmorecfg_h::JDIMENSION = 0;
    if (*cinfo).global_state == crate::jpegint_h::CSTATE_SCANNING
        || (*cinfo).global_state == crate::jpegint_h::CSTATE_RAW_OK
    {
        /* Terminate first pass */
        if (*cinfo).next_scanline < (*cinfo).image_height {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_TOO_LITTLE_DATA as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        Some(
            (*(*cinfo).master)
                .finish_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    } else if (*cinfo).global_state != crate::jpegint_h::CSTATE_WRCOEFS {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Perform any remaining passes */
    while (*(*cinfo).master).is_last_pass == 0 {
        Some(
            (*(*cinfo).master)
                .prepare_for_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
        iMCU_row = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
        while iMCU_row < (*cinfo).total_iMCU_rows {
            if !(*cinfo).progress.is_null() {
                (*(*cinfo).progress).pass_counter = iMCU_row as libc::c_long;
                (*(*cinfo).progress).pass_limit = (*cinfo).total_iMCU_rows as libc::c_long;
                Some(
                    (*(*cinfo).progress)
                        .progress_monitor
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
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
                crate::stddef_h::NULL as *mut libc::c_void as crate::jpeglib_h::JSAMPIMAGE,
            ) == 0
            {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_CANT_SUSPEND as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            iMCU_row = iMCU_row.wrapping_add(1)
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
    crate::src::jcomapi::jpeg_abort(cinfo as crate::jpeglib_h::j_common_ptr);
}
/*
 * Write a special marker.
 * This is only recommended for writing COM or APPn markers.
 * Must be called after jpeg_start_compress() and before
 * first call to jpeg_write_scanlines() or jpeg_write_raw_data().
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_write_marker(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut marker: libc::c_int,
    mut dataptr: *const crate::jmorecfg_h::JOCTET,
    mut datalen: libc::c_uint,
) {
    let mut write_marker_byte: Option<
        unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr, _: libc::c_int) -> (),
    > = None; /* copy for speed */
    if (*cinfo).next_scanline != 0 as libc::c_int as libc::c_uint
        || (*cinfo).global_state != crate::jpegint_h::CSTATE_SCANNING
            && (*cinfo).global_state != crate::jpegint_h::CSTATE_RAW_OK
            && (*cinfo).global_state != crate::jpegint_h::CSTATE_WRCOEFS
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
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
        datalen = datalen.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        Some(write_marker_byte.expect("non-null function pointer"))
            .expect("non-null function pointer")(cinfo, *dataptr as libc::c_int);
        dataptr = dataptr.offset(1)
    }
}
/* Same, but piecemeal. */
#[no_mangle]

pub unsafe extern "C" fn jpeg_write_m_header(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut marker: libc::c_int,
    mut datalen: libc::c_uint,
) {
    if (*cinfo).next_scanline != 0 as libc::c_int as libc::c_uint
        || (*cinfo).global_state != crate::jpegint_h::CSTATE_SCANNING
            && (*cinfo).global_state != crate::jpegint_h::CSTATE_RAW_OK
            && (*cinfo).global_state != crate::jpegint_h::CSTATE_WRCOEFS
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
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
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut val: libc::c_int,
) {
    Some(
        (*(*cinfo).marker)
            .write_marker_byte
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, val);
}
/*
 * jpeglib.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1998, Thomas G. Lane.
 * Modified 2002-2009 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009-2011, 2013-2014, 2016-2017, D. R. Commander.
 * Copyright (C) 2015, Google, Inc.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file defines the application interface for the JPEG library.
 * Most applications using the library need only include this file,
 * and perhaps jerror.h if they want to know the exact error codes.
 */
/*
 * First we include the configuration files that record how this
 * installation of the JPEG library is set up.  jconfig.h can be
 * generated automatically for many systems.  jmorecfg.h contains
 * manual configuration options that most people need not worry about.
 */
/* in case jinclude.h already did */
/* Various constants determining the sizes of things.
 * All of these are specified by the JPEG standard, so don't change them
 * if you want to be compatible.
 */
/* The basic DCT block is 8x8 samples */
/* DCTSIZE squared; # of elements in a block */
/* Quantization tables are numbered 0..3 */
/* Huffman tables are numbered 0..3 */
/* Arith-coding tables are numbered 0..15 */
/* JPEG limit on # of components in one scan */
/* JPEG limit on sampling factors */
/* Unfortunately, some bozo at Adobe saw no reason to be bound by the standard;
 * the PostScript DCT filter can emit files with many more than 10 blocks/MCU.
 * If you happen to run across such a file, you can up D_MAX_BLOCKS_IN_MCU
 * to handle it.  We even let you do this from the jconfig.h file.  However,
 * we strongly discourage changing C_MAX_BLOCKS_IN_MCU; just because Adobe
 * sometimes emits noncompliant files doesn't mean you should too.
 */
/* compressor's limit on blocks per MCU */
/* decompressor's limit on blocks per MCU */
/* Data structures for images (arrays of samples and of DCT coefficients).
 */
/* ptr to one image row of pixel samples. */
/* ptr to some rows (a 2-D sample array) */
/* a 3-D sample array: top index is color */
/* one block of coefficients */
/* pointer to one row of coefficient blocks */
/* a 2-D array of coefficient blocks */
/* a 3-D array of coefficient blocks */
/* useful in a couple of places */
/* Types for JPEG compression parameters and working tables. */
/* DCT coefficient quantization tables. */
/* This array gives the coefficient quantizers in natural array order
 * (not the zigzag order in which they are stored in a JPEG DQT marker).
 * CAUTION: IJG versions prior to v6a kept this array in zigzag order.
 */
/* quantization step for each coefficient */
/* This field is used only during compression.  It's initialized FALSE when
 * the table is created, and set TRUE when it's been output to the file.
 * You could suppress output of a table by setting this to TRUE.
 * (See jpeg_suppress_tables for an example.)
 */
/* TRUE when table has been output */
/* Huffman coding tables. */
/* These two fields directly represent the contents of a JPEG DHT marker */
/* bits[k] = # of symbols with codes of */
/* length k bits; bits[0] is unused */
/* The symbols, in order of incr code length */
/* This field is used only during compression.  It's initialized FALSE when
 * the table is created, and set TRUE when it's been output to the file.
 * You could suppress output of a table by setting this to TRUE.
 * (See jpeg_suppress_tables for an example.)
 */
/* TRUE when table has been output */
/* Basic info about one component (color channel). */
/* These values are fixed over the whole image. */
/* For compression, they must be supplied by parameter setup; */
/* for decompression, they are read from the SOF marker. */
/* identifier for this component (0..255) */
/* its index in SOF or cinfo->comp_info[] */
/* horizontal sampling factor (1..4) */
/* vertical sampling factor (1..4) */
/* quantization table selector (0..3) */
/* These values may vary between scans. */
/* For compression, they must be supplied by parameter setup; */
/* for decompression, they are read from the SOS marker. */
/* The decompressor output side may not use these variables. */
/* DC entropy table selector (0..3) */
/* AC entropy table selector (0..3) */
/* Remaining fields should be treated as private by applications. */
/* These values are computed during compression or decompression startup: */
/* Component's size in DCT blocks.
 * Any dummy blocks added to complete an MCU are not counted; therefore
 * these values do not depend on whether a scan is interleaved or not.
 */
/* Size of a DCT block in samples.  Always DCTSIZE for compression.
 * For decompression this is the size of the output from one DCT block,
 * reflecting any scaling we choose to apply during the IDCT step.
 * Values from 1 to 16 are supported.
 * Note that different components may receive different IDCT scalings.
 */
/* The downsampled dimensions are the component's actual, unpadded number
 * of samples at the main buffer (preprocessing/compression interface), thus
 * downsampled_width = ceil(image_width * Hi/Hmax)
 * and similarly for height.  For decompression, IDCT scaling is included, so
 * downsampled_width = ceil(image_width * Hi/Hmax * DCT_[h_]scaled_size/DCTSIZE)
 */
/* actual width in samples */
/* actual height in samples */
/* This flag is used only for decompression.  In cases where some of the
 * components will be ignored (eg grayscale output from YCbCr image),
 * we can skip most computations for the unused components.
 */
/* do we need the value of this component? */
/* These values are computed before starting a scan of the component. */
/* The decompressor output side may not use these variables. */
/* number of blocks per MCU, horizontally */
/* number of blocks per MCU, vertically */
/* MCU_width * MCU_height */
/* MCU width in samples, MCU_width*DCT_[h_]scaled_size */
/* # of non-dummy blocks across in last MCU */
/* # of non-dummy blocks down in last MCU */
/* Saved quantization table for component; NULL if none yet saved.
 * See jdinput.c comments about the need for this information.
 * This field is currently used only for decompression.
 */
/* Private per-component storage for DCT or IDCT subsystem. */
/* The script for encoding a multiple-scan file is an array of these: */
/* number of components encoded in this scan */
/* their SOF/comp_info[] indexes */
/* progressive JPEG spectral selection parms */
/* progressive JPEG successive approx. parms */
/* The decompressor can save APPn and COM markers in a list of these: */
/* next in list, or NULL */
/* marker code: JPEG_COM, or JPEG_APP0+n */
/* # bytes of data in the file */
/* # bytes of data saved at data[] */
/* the data contained in the marker */
/* the marker length word is not counted in data_length or original_length */
/* Known color spaces. */
/* error/unspecified */
/* monochrome */
/* red/green/blue as specified by the RGB_RED,
RGB_GREEN, RGB_BLUE, and RGB_PIXELSIZE macros */
/* Y/Cb/Cr (also known as YUV) */
/* C/M/Y/K */
/* Y/Cb/Cr/K */
/* red/green/blue */
/* red/green/blue/x */
/* blue/green/red */
/* blue/green/red/x */
/* x/blue/green/red */
/* x/red/green/blue */
/* When out_color_space it set to JCS_EXT_RGBX, JCS_EXT_BGRX, JCS_EXT_XBGR,
or JCS_EXT_XRGB during decompression, the X byte is undefined, and in
order to ensure the best performance, libjpeg-turbo can set that byte to
whatever value it wishes.  Use the following colorspace constants to
ensure that the X byte is set to 0xFF, so that it can be interpreted as an
opaque alpha channel. */
/* red/green/blue/alpha */
/* blue/green/red/alpha */
/* alpha/blue/green/red */
/* alpha/red/green/blue */
/* 5-bit red/6-bit green/5-bit blue */
/* DCT/IDCT algorithm options. */
/* slow but accurate integer algorithm */
/* faster, less accurate integer method */
/* floating-point: accurate, fast on fast HW */
/* may be overridden in jconfig.h */
/* may be overridden in jconfig.h */
/* Dithering options for decompression. */
/* no dithering */
/* simple ordered dither */
/* Floyd-Steinberg error diffusion dither */
/* These 32-bit GUIDs and the corresponding jpeg_*_get_*_param()/
 * jpeg_*_set_*_param() functions allow for extending the libjpeg API without
 * breaking backward ABI compatibility.  The actual parameters are stored in
 * the opaque jpeg_comp_master and jpeg_decomp_master structs.
 */
/* Boolean extension parameters */
/* TRUE=optimize progressive coding scans */
/* TRUE=use trellis quantization */
/* TRUE=use trellis quant for DC coefficient */
/* TRUE=optimize for sequences of EOB */
/* TRUE=use lambda weighting table */
/* TRUE=use scans in trellis optimization */
/* TRUE=optimize quant table in trellis loop */
/* TRUE=preprocess input to reduce ringing of edges on white background */
/* Floating point parameters */
/* Integer parameters */
/* compression profile */
/* splitting point for frequency in trellis quantization */
/* number of trellis loops */
/* base quantization table index */
/* DC scan optimization mode */
/* Values for the JINT_COMPRESS_PROFILE parameter (32-bit GUIDs) */
/* best compression ratio (progressive, all mozjpeg extensions) */
/* libjpeg[-turbo] defaults (baseline, no mozjpeg extensions) */
/* Common fields between JPEG compression and decompression master structs. */
/* Error handler module */
/* Memory manager module */
/* Progress monitor, or NULL if none */
/* Available for use by application */
/* So common code can tell which is which */
/* For checking call sequence validity */
/* Routines that are to be used by both halves of the library are declared
 * to receive a pointer to this structure.  There are no actual instances of
 * jpeg_common_struct, only of jpeg_compress_struct and jpeg_decompress_struct.
 */
/* Fields common to both master struct types */
/* Additional fields follow in an actual jpeg_compress_struct or
 * jpeg_decompress_struct.  All three structs must agree on these
 * initial fields!  (This would be a lot cleaner in C++.)
 */
/* Master record for a compression instance */
/* Fields shared with jpeg_decompress_struct */
/* Destination for compressed data */
/* Description of source image --- these fields must be filled in by
 * outer application before starting compression.  in_color_space must
 * be correct before you can even call jpeg_set_defaults().
 */
/* input image width */
/* input image height */
/* # of color components in input image */
/* colorspace of input image */
/* image gamma of input image */
/* Compression parameters --- these fields must be set before calling
 * jpeg_start_compress().  We recommend calling jpeg_set_defaults() to
 * initialize everything to reasonable defaults, then changing anything
 * the application specifically wants to change.  That way you won't get
 * burnt when new parameters are added.  Also note that there are several
 * helper routines to simplify changing parameters.
 */
/* bits of precision in image data */
/* # of color components in JPEG image */
/* colorspace of JPEG image */
/* comp_info[i] describes component that appears i'th in SOF */
/* ptrs to coefficient quantization tables, or NULL if not defined,
 * and corresponding scale factors (percentage, initialized 100).
 */
/* ptrs to Huffman coding tables, or NULL if not defined */
/* L values for DC arith-coding tables */
/* U values for DC arith-coding tables */
/* Kx values for AC arith-coding tables */
/* # of entries in scan_info array */
/* script for multi-scan file, or NULL */
/* The default value of scan_info is NULL, which causes a single-scan
 * sequential JPEG file to be emitted.  To create a multi-scan file,
 * set num_scans and scan_info to point to an array of scan definitions.
 */
/* TRUE=caller supplies downsampled data */
/* TRUE=arithmetic coding, FALSE=Huffman */
/* TRUE=optimize entropy encoding parms */
/* TRUE=first samples are cosited */
/* 1..100, or 0 for no input smoothing */
/* DCT algorithm selector */
/* The restart interval can be specified in absolute MCUs by setting
 * restart_interval, or in MCU rows by setting restart_in_rows
 * (in which case the correct restart_interval will be figured
 * for each scan).
 */
/* MCUs per restart, or 0 for no restart */
/* if > 0, MCU rows per restart interval */
/* Parameters controlling emission of special markers. */
/* should a JFIF marker be written? */
/* What to write for the JFIF version number */
/* These three values are not used by the JPEG code, merely copied */
/* into the JFIF APP0 marker.  density_unit can be 0 for unknown, */
/* 1 for dots/inch, or 2 for dots/cm.  Note that the pixel aspect */
/* ratio is defined by X_density/Y_density even when density_unit=0. */
/* JFIF code for pixel size units */
/* Horizontal pixel density */
/* Vertical pixel density */
/* should an Adobe marker be written? */
/* State variable: index of next scanline to be written to
 * jpeg_write_scanlines().  Application may use this to control its
 * processing loop, e.g., "while (next_scanline < image_height)".
 */
/* 0 .. image_height-1  */
/* Remaining fields are known throughout compressor, but generally
 * should not be touched by a surrounding application.
 */
/*
 * These fields are computed during compression startup
 */
/* TRUE if scan script uses progressive mode */
/* largest h_samp_factor */
/* largest v_samp_factor */
/* # of iMCU rows to be input to coef ctlr */
/* The coefficient controller receives data in units of MCU rows as defined
 * for fully interleaved scans (whether the JPEG file is interleaved or not).
 * There are v_samp_factor * DCTSIZE sample rows of each component in an
 * "iMCU" (interleaved MCU) row.
 */
/*
 * These fields are valid during any one scan.
 * They describe the components and MCUs actually appearing in the scan.
 */
/* # of JPEG components in this scan */
/* *cur_comp_info[i] describes component that appears i'th in SOS */
/* # of MCUs across the image */
/* # of MCU rows in the image */
/* # of DCT blocks per MCU */
/* MCU_membership[i] is index in cur_comp_info of component owning */
/* i'th block in an MCU */
/* progressive JPEG parameters for scan */
/*
 * Links to compression subobjects (methods and private variables of modules)
 */
/* workspace for jpeg_simple_progression */
/* Master record for a decompression instance */
/* Fields shared with jpeg_compress_struct */
/* Source of compressed data */
/* Basic description of image --- filled in by jpeg_read_header(). */
/* Application may inspect these values to decide how to process image. */
/* nominal image width (from SOF marker) */
/* nominal image height */
/* # of color components in JPEG image */
/* colorspace of JPEG image */
/* Decompression processing parameters --- these fields must be set before
 * calling jpeg_start_decompress().  Note that jpeg_read_header() initializes
 * them to default values.
 */
/* colorspace for output */
/* fraction by which to scale image */
/* image gamma wanted in output */
/* TRUE=multiple output passes */
/* TRUE=downsampled data wanted */
/* IDCT algorithm selector */
/* TRUE=apply fancy upsampling */
/* TRUE=apply interblock smoothing */
/* TRUE=colormapped output wanted */
/* the following are ignored if not quantize_colors: */
/* type of color dithering to use */
/* TRUE=use two-pass color quantization */
/* max # colors to use in created colormap */
/* these are significant only in buffered-image mode: */
/* enable future use of 1-pass quantizer */
/* enable future use of external colormap */
/* enable future use of 2-pass quantizer */
/* Description of actual output image that will be returned to application.
 * These fields are computed by jpeg_start_decompress().
 * You can also use jpeg_calc_output_dimensions() to determine these values
 * in advance of calling jpeg_start_decompress().
 */
/* scaled image width */
/* scaled image height */
/* # of color components in out_color_space */
/* # of color components returned */
/* output_components is 1 (a colormap index) when quantizing colors;
 * otherwise it equals out_color_components.
 */
/* min recommended height of scanline buffer */
/* If the buffer passed to jpeg_read_scanlines() is less than this many rows
 * high, space and time will be wasted due to unnecessary data copying.
 * Usually rec_outbuf_height will be 1 or 2, at most 4.
 */
/* When quantizing colors, the output colormap is described by these fields.
 * The application can supply a colormap by setting colormap non-NULL before
 * calling jpeg_start_decompress; otherwise a colormap is created during
 * jpeg_start_decompress or jpeg_start_output.
 * The map has out_color_components rows and actual_number_of_colors columns.
 */
/* number of entries in use */
/* The color map as a 2-D pixel array */
/* State variables: these variables indicate the progress of decompression.
 * The application may examine these but must not modify them.
 */
/* Row index of next scanline to be read from jpeg_read_scanlines().
 * Application may use this to control its processing loop, e.g.,
 * "while (output_scanline < output_height)".
 */
/* 0 .. output_height-1  */
/* Current input scan number and number of iMCU rows completed in scan.
 * These indicate the progress of the decompressor input side.
 */
/* Number of SOS markers seen so far */
/* Number of iMCU rows completed */
/* The "output scan number" is the notional scan being displayed by the
 * output side.  The decompressor will not allow output scan/row number
 * to get ahead of input scan/row, but it can fall arbitrarily far behind.
 */
/* Nominal scan number being displayed */
/* Number of iMCU rows read */
/* Current progression status.  coef_bits[c][i] indicates the precision
 * with which component c's DCT coefficient i (in zigzag order) is known.
 * It is -1 when no data has yet been received, otherwise it is the point
 * transform (shift) value for the most recent scan of the coefficient
 * (thus, 0 at completion of the progression).
 * This pointer is NULL when reading a non-progressive file.
 */
/* -1 or current Al value for each coef */
/* Internal JPEG parameters --- the application usually need not look at
 * these fields.  Note that the decompressor output side may not use
 * any parameters that can change between scans.
 */
/* Quantization and Huffman tables are carried forward across input
 * datastreams when processing abbreviated JPEG datastreams.
 */
/* ptrs to coefficient quantization tables, or NULL if not defined */
/* ptrs to Huffman coding tables, or NULL if not defined */
/* These parameters are never carried across datastreams, since they
 * are given in SOF/SOS markers or defined to be reset by SOI.
 */
/* bits of precision in image data */
/* comp_info[i] describes component that appears i'th in SOF */
/* TRUE if SOFn specifies progressive mode */
/* TRUE=arithmetic coding, FALSE=Huffman */
/* L values for DC arith-coding tables */
/* U values for DC arith-coding tables */
/* Kx values for AC arith-coding tables */
/* MCUs per restart interval, or 0 for no restart */
/* These fields record data obtained from optional markers recognized by
 * the JPEG library.
 */
/* TRUE iff a JFIF APP0 marker was found */
/* Data copied from JFIF marker; only valid if saw_JFIF_marker is TRUE: */
/* JFIF version number */
/* JFIF code for pixel size units */
/* Horizontal pixel density */
/* Vertical pixel density */
/* TRUE iff an Adobe APP14 marker was found */
/* Color transform code from Adobe marker */
/* TRUE=first samples are cosited */
/* Aside from the specific data retained from APPn markers known to the
 * library, the uninterpreted contents of any or all APPn and COM markers
 * can be saved in a list for examination by the application.
 */
/* Head of list of saved markers */
/* Remaining fields are known throughout decompressor, but generally
 * should not be touched by a surrounding application.
 */
/*
 * These fields are computed during decompression startup
 */
/* largest h_samp_factor */
/* largest v_samp_factor */
/* smallest DCT_scaled_size of any component */
/* # of iMCU rows in image */
/* The coefficient controller's input and output progress is measured in
 * units of "iMCU" (interleaved MCU) rows.  These are the same as MCU rows
 * in fully interleaved JPEG scans, but are used whether the scan is
 * interleaved or not.  We define an iMCU row as v_samp_factor DCT block
 * rows of each component.  Therefore, the IDCT output contains
 * v_samp_factor*DCT_[v_]scaled_size sample rows of a component per iMCU row.
 */
/* table for fast range-limiting */
/*
 * These fields are valid during any one scan.
 * They describe the components and MCUs actually appearing in the scan.
 * Note that the decompressor output side must not use these fields.
 */
/* # of JPEG components in this scan */
/* *cur_comp_info[i] describes component that appears i'th in SOS */
/* # of MCUs across the image */
/* # of MCU rows in the image */
/* # of DCT blocks per MCU */
/* MCU_membership[i] is index in cur_comp_info of component owning */
/* i'th block in an MCU */
/* progressive JPEG parameters for scan */
/* This field is shared between entropy decoder and marker parser.
 * It is either zero or the code of a JPEG marker that has been
 * read from the data source, but has not yet been processed.
 */
/*
 * Links to decompression subobjects (methods, private variables of modules)
 */
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

pub unsafe extern "C" fn jpeg_write_tables(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    if (*cinfo).global_state != crate::jpegint_h::CSTATE_START {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* (Re)initialize error mgr and destination modules */
    Some(
        (*(*cinfo).err)
            .reset_error_mgr
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    Some(
        (*(*cinfo).dest)
            .init_destination
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    /* Initialize the marker writer ... bit of a crock to do it here. */
    crate::src::jcmarker::jinit_marker_writer(cinfo);
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
