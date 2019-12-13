// =============== BEGIN jcmaster_h ================
pub type my_master_ptr = *mut crate::src::jcmaster::my_comp_master;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_comp_master {
    pub pub_0: crate::jpegint_h::jpeg_comp_master,
    pub pass_type: crate::src::jcmaster::c_pass_type,
    pub pass_number: libc::c_int,
    pub total_passes: libc::c_int,
    pub scan_number: libc::c_int,
    pub pass_number_scan_opt_base: libc::c_int,
    pub scan_buffer: [*mut libc::c_uchar; 64],
    pub scan_size: [libc::c_ulong; 64],
    pub actual_Al: [libc::c_int; 64],
    pub best_cost: libc::c_ulong,
    pub best_freq_split_idx_luma: libc::c_int,
    pub best_freq_split_idx_chroma: libc::c_int,
    pub best_Al_luma: libc::c_int,
    pub best_Al_chroma: libc::c_int,
    pub interleave_chroma_dc: crate::jmorecfg_h::boolean,
    pub saved_dest: *mut crate::jpeglib_h::jpeg_destination_mgr,
    pub jpeg_version: *const libc::c_char,
}

pub type c_pass_type = libc::c_uint;

pub const trellis_pass: crate::src::jcmaster::c_pass_type = 3;

pub const output_pass: crate::src::jcmaster::c_pass_type = 2;

pub const huff_opt_pass: crate::src::jcmaster::c_pass_type = 1;

pub const main_pass: crate::src::jcmaster::c_pass_type = 0;
use ::libc;

pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::jconfig_h::BITS_IN_JSAMPLE;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JPEG_MAX_DIMENSION;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::MAX_COMPONENTS;
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
pub use crate::jpeglib_h::DCTSIZE2;
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
pub use crate::jpeglib_h::MAX_COMPS_IN_SCAN;
pub use crate::jpeglib_h::MAX_SAMP_FACTOR;
pub use crate::jpeglib_h::NUM_QUANT_TBLS;
pub use crate::src::jdatadst::jpeg_mem_dest_internal;
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
pub use crate::src::jutils::jdiv_round_up;
use crate::stdlib::fprintf;
use crate::stdlib::free;
use crate::stdlib::memcpy;
use crate::stdlib::stderr;
pub use crate::stdlib::C2RustUnnamed_0;
/*
 * jcmaster.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * Modified 2003-2010 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2010, 2016, 2018, D. R. Commander.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains master control logic for the JPEG compressor.
 * These routines are concerned with parameter validation, initial setup,
 * and inter-pass control (determining the number of passes and the work
 * to be done in each pass).
 */
/*
 * Support routines that do various essential calculations.
 */

unsafe extern "C" fn initial_setup(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut transcode_only: crate::jmorecfg_h::boolean,
)
/* Do computations that are needed before master selection phase */
{
    let mut ci: libc::c_int = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    let mut samplesperrow: libc::c_long = 0;
    let mut jd_samplesperrow: crate::jmorecfg_h::JDIMENSION = 0;
    /* Sanity check on image dimensions */
    if (*cinfo).image_height <= 0 as libc::c_int as libc::c_uint
        || (*cinfo).image_width <= 0 as libc::c_int as libc::c_uint
        || (*cinfo).num_components <= 0 as libc::c_int
        || (*cinfo).input_components <= 0 as libc::c_int
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_EMPTY_IMAGE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Make sure image isn't bigger than I can handle */
    if (*cinfo).image_height as libc::c_long > crate::jmorecfg_h::JPEG_MAX_DIMENSION
        || (*cinfo).image_width as libc::c_long > crate::jmorecfg_h::JPEG_MAX_DIMENSION
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_IMAGE_TOO_BIG as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] =
            65500 as libc::c_long as libc::c_uint as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Width of an input scanline must be representable as JDIMENSION. */
    samplesperrow =
        (*cinfo).image_width as libc::c_long * (*cinfo).input_components as libc::c_long;
    jd_samplesperrow = samplesperrow as crate::jmorecfg_h::JDIMENSION;
    if jd_samplesperrow as libc::c_long != samplesperrow {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_WIDTH_OVERFLOW as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* For now, precision must match compiled-in value... */
    if (*cinfo).data_precision != crate::jconfig_h::BITS_IN_JSAMPLE {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_PRECISION as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*cinfo).data_precision;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Check that number of components won't exceed internal array sizes */
    if (*cinfo).num_components > crate::jmorecfg_h::MAX_COMPONENTS {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_COMPONENT_COUNT as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*cinfo).num_components;
        (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = 10 as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Compute maximum sampling factors; check factor validity */
    (*cinfo).max_h_samp_factor = 1 as libc::c_int;
    (*cinfo).max_v_samp_factor = 1 as libc::c_int;
    ci = 0 as libc::c_int;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        if (*compptr).h_samp_factor <= 0 as libc::c_int
            || (*compptr).h_samp_factor > crate::jpeglib_h::MAX_SAMP_FACTOR
            || (*compptr).v_samp_factor <= 0 as libc::c_int
            || (*compptr).v_samp_factor > crate::jpeglib_h::MAX_SAMP_FACTOR
        {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_SAMPLING as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        (*cinfo).max_h_samp_factor = if (*cinfo).max_h_samp_factor > (*compptr).h_samp_factor {
            (*cinfo).max_h_samp_factor
        } else {
            (*compptr).h_samp_factor
        };
        (*cinfo).max_v_samp_factor = if (*cinfo).max_v_samp_factor > (*compptr).v_samp_factor {
            (*cinfo).max_v_samp_factor
        } else {
            (*compptr).v_samp_factor
        };
        ci += 1;
        compptr = compptr.offset(1)
    }
    /* Compute dimensions of components */
    ci = 0 as libc::c_int;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Fill in the correct component_index value; don't rely on application */
        (*compptr).component_index = ci;
        /* For compression, we never do DCT scaling. */
        (*compptr).DCT_scaled_size = crate::jpeglib_h::DCTSIZE;
        /* Size in DCT blocks */
        (*compptr).width_in_blocks = crate::src::jutils::jdiv_round_up(
            (*cinfo).image_width as libc::c_long * (*compptr).h_samp_factor as libc::c_long,
            ((*cinfo).max_h_samp_factor * crate::jpeglib_h::DCTSIZE) as libc::c_long,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*compptr).height_in_blocks = crate::src::jutils::jdiv_round_up(
            (*cinfo).image_height as libc::c_long * (*compptr).v_samp_factor as libc::c_long,
            ((*cinfo).max_v_samp_factor * crate::jpeglib_h::DCTSIZE) as libc::c_long,
        ) as crate::jmorecfg_h::JDIMENSION;
        /* Size in samples */
        (*compptr).downsampled_width = crate::src::jutils::jdiv_round_up(
            (*cinfo).image_width as libc::c_long * (*compptr).h_samp_factor as libc::c_long,
            (*cinfo).max_h_samp_factor as libc::c_long,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*compptr).downsampled_height = crate::src::jutils::jdiv_round_up(
            (*cinfo).image_height as libc::c_long * (*compptr).v_samp_factor as libc::c_long,
            (*cinfo).max_v_samp_factor as libc::c_long,
        ) as crate::jmorecfg_h::JDIMENSION;
        /* Mark component needed (this flag isn't actually used for compression) */
        (*compptr).component_needed = crate::jmorecfg_h::TRUE;
        ci += 1;
        compptr = compptr.offset(1)
    }
    /* Compute number of fully interleaved MCU rows (number of times that
     * main controller will call coefficient controller).
     */
    (*cinfo).total_iMCU_rows = crate::src::jutils::jdiv_round_up(
        (*cinfo).image_height as libc::c_long,
        ((*cinfo).max_v_samp_factor * crate::jpeglib_h::DCTSIZE) as libc::c_long,
    ) as crate::jmorecfg_h::JDIMENSION;
}

unsafe extern "C" fn validate_script(mut cinfo: crate::jpeglib_h::j_compress_ptr)
/* Verify that the scan script in cinfo->scan_info[] is valid; also
 * determine whether it uses progressive JPEG, and set cinfo->progressive_mode.
 */
{
    let mut scanptr: *const crate::jpeglib_h::jpeg_scan_info =
        0 as *const crate::jpeglib_h::jpeg_scan_info;
    let mut scanno: libc::c_int = 0;
    let mut ncomps: libc::c_int = 0;
    let mut ci: libc::c_int = 0;
    let mut coefi: libc::c_int = 0;
    let mut thisi: libc::c_int = 0;
    let mut Ss: libc::c_int = 0;
    let mut Se: libc::c_int = 0;
    let mut Ah: libc::c_int = 0;
    let mut Al: libc::c_int = 0;
    let mut component_sent: [crate::jmorecfg_h::boolean; 10] = [0; 10];
    let mut last_bitpos_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut last_bitpos: [[libc::c_int; 64]; 10] = [[0; 64]; 10];
    /* -1 until that coefficient has been seen; then last Al for it */
    if (*(*cinfo).master).optimize_scans != 0 {
        (*cinfo).progressive_mode = crate::jmorecfg_h::TRUE;
        /* When we optimize scans, there is redundancy in the scan list
         * and this function will fail. Therefore skip all this checking
         */
        return;
    }
    if (*cinfo).num_scans <= 0 as libc::c_int {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_SCAN_SCRIPT as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = 0 as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* For sequential JPEG, all scans must have Ss=0, Se=DCTSIZE2-1;
     * for progressive JPEG, no scan can have this.
     */
    scanptr = (*cinfo).scan_info;
    if (*scanptr).Ss != 0 as libc::c_int
        || (*scanptr).Se != crate::jpeglib_h::DCTSIZE2 - 1 as libc::c_int
    {
        (*cinfo).progressive_mode = crate::jmorecfg_h::TRUE;
        last_bitpos_ptr = &mut *(*last_bitpos.as_mut_ptr().offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut libc::c_int;
        ci = 0 as libc::c_int;
        while ci < (*cinfo).num_components {
            coefi = 0 as libc::c_int;
            while coefi < crate::jpeglib_h::DCTSIZE2 {
                let fresh0 = last_bitpos_ptr;
                last_bitpos_ptr = last_bitpos_ptr.offset(1);
                *fresh0 = -(1 as libc::c_int);
                coefi += 1
            }
            ci += 1
        }
    } else {
        (*cinfo).progressive_mode = crate::jmorecfg_h::FALSE;
        ci = 0 as libc::c_int;
        while ci < (*cinfo).num_components {
            component_sent[ci as usize] = crate::jmorecfg_h::FALSE;
            ci += 1
        }
    }
    scanno = 1 as libc::c_int;
    while scanno <= (*cinfo).num_scans {
        /* Validate component indexes */
        ncomps = (*scanptr).comps_in_scan;
        if ncomps <= 0 as libc::c_int || ncomps > crate::jpeglib_h::MAX_COMPS_IN_SCAN {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_COMPONENT_COUNT as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = ncomps;
            (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = 4 as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        ci = 0 as libc::c_int;
        while ci < ncomps {
            thisi = (*scanptr).component_index[ci as usize];
            if thisi < 0 as libc::c_int || thisi >= (*cinfo).num_components {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_SCAN_SCRIPT as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = scanno;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            /* Components must appear in SOF order within each scan */
            if ci > 0 as libc::c_int
                && thisi <= (*scanptr).component_index[(ci - 1 as libc::c_int) as usize]
            {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_SCAN_SCRIPT as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = scanno;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            ci += 1
        }
        /* Validate progression parameters */
        Ss = (*scanptr).Ss;
        Se = (*scanptr).Se;
        Ah = (*scanptr).Ah;
        Al = (*scanptr).Al;
        if (*cinfo).progressive_mode != 0 {
            /* Rec. ITU-T T.81 | ISO/IEC 10918-1 simply gives the ranges 0..13 for Ah
             * and Al, but that seems wrong: the upper bound ought to depend on data
             * precision.  Perhaps they really meant 0..N+1 for N-bit precision.
             * Here we allow 0..10 for 8-bit data; Al larger than 10 results in
             * out-of-range reconstructed DC values during the first DC scan,
             * which might cause problems for some decoders.
             */
            if Ss < 0 as libc::c_int
                || Ss >= crate::jpeglib_h::DCTSIZE2
                || Se < Ss
                || Se >= crate::jpeglib_h::DCTSIZE2
                || Ah < 0 as libc::c_int
                || Ah > MAX_AH_AL
                || Al < 0 as libc::c_int
                || Al > MAX_AH_AL
            {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_PROG_SCRIPT as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = scanno;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            if Ss == 0 as libc::c_int {
                if Se != 0 as libc::c_int {
                    /* DC and AC together not OK */
                    (*(*cinfo).err).msg_code =
                        crate::src::jerror::JERR_BAD_PROG_SCRIPT as libc::c_int;
                    (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = scanno;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr
                    );
                }
            } else if ncomps != 1 as libc::c_int {
                /* AC scans must be for only one component */
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_PROG_SCRIPT as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = scanno;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            ci = 0 as libc::c_int;
            while ci < ncomps {
                last_bitpos_ptr = &mut *(*last_bitpos
                    .as_mut_ptr()
                    .offset(*(*scanptr).component_index.as_ptr().offset(ci as isize) as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize)
                    as *mut libc::c_int;
                if Ss != 0 as libc::c_int
                    && *last_bitpos_ptr.offset(0 as libc::c_int as isize) < 0 as libc::c_int
                {
                    /* AC without prior DC scan */
                    (*(*cinfo).err).msg_code =
                        crate::src::jerror::JERR_BAD_PROG_SCRIPT as libc::c_int;
                    (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = scanno;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr
                    );
                }
                coefi = Ss;
                while coefi <= Se {
                    if *last_bitpos_ptr.offset(coefi as isize) < 0 as libc::c_int {
                        /* first scan of this coefficient */
                        if Ah != 0 as libc::c_int {
                            (*(*cinfo).err).msg_code =
                                crate::src::jerror::JERR_BAD_PROG_SCRIPT as libc::c_int;
                            (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = scanno;
                            Some(
                                (*(*cinfo).err)
                                    .error_exit
                                    .expect("non-null function pointer"),
                            )
                            .expect("non-null function pointer")(
                                cinfo as crate::jpeglib_h::j_common_ptr,
                            );
                        }
                    } else if Ah != *last_bitpos_ptr.offset(coefi as isize)
                        || Al != Ah - 1 as libc::c_int
                    {
                        (*(*cinfo).err).msg_code =
                            crate::src::jerror::JERR_BAD_PROG_SCRIPT as libc::c_int;
                        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = scanno;
                        Some(
                            (*(*cinfo).err)
                                .error_exit
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as crate::jpeglib_h::j_common_ptr,
                        );
                    }
                    *last_bitpos_ptr.offset(coefi as isize) = Al;
                    coefi += 1
                }
                ci += 1
            }
        } else {
            /* not first scan */
            /* For sequential JPEG, all progression parameters must be these: */
            if Ss != 0 as libc::c_int
                || Se != crate::jpeglib_h::DCTSIZE2 - 1 as libc::c_int
                || Ah != 0 as libc::c_int
                || Al != 0 as libc::c_int
            {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_PROG_SCRIPT as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = scanno;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            /* Make sure components are not sent twice */
            ci = 0 as libc::c_int;
            while ci < ncomps {
                thisi = (*scanptr).component_index[ci as usize];
                if component_sent[thisi as usize] != 0 {
                    (*(*cinfo).err).msg_code =
                        crate::src::jerror::JERR_BAD_SCAN_SCRIPT as libc::c_int;
                    (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = scanno;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr
                    );
                }
                component_sent[thisi as usize] = crate::jmorecfg_h::TRUE;
                ci += 1
            }
        }
        scanptr = scanptr.offset(1);
        scanno += 1
    }
    /* Now verify that everything got sent. */
    if (*cinfo).progressive_mode != 0 {
        /* For progressive mode, we only check that at least some DC data
         * got sent for each component; the spec does not require that all bits
         * of all coefficients be transmitted.  Would it be wiser to enforce
         * transmission of all coefficient bits??
         */
        ci = 0 as libc::c_int;
        while ci < (*cinfo).num_components {
            if last_bitpos[ci as usize][0 as libc::c_int as usize] < 0 as libc::c_int {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_MISSING_DATA as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            ci += 1
        }
    } else {
        ci = 0 as libc::c_int;
        while ci < (*cinfo).num_components {
            if component_sent[ci as usize] == 0 {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_MISSING_DATA as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            ci += 1
        }
    };
}

pub const MAX_AH_AL: libc::c_int = 10 as libc::c_int;
/* C_MULTISCAN_FILES_SUPPORTED */

unsafe extern "C" fn select_scan_parameters(mut cinfo: crate::jpeglib_h::j_compress_ptr)
/* Set up the scan parameters for the current scan */
{
    let mut ci: libc::c_int = 0;
    let mut master: crate::src::jcmaster::my_master_ptr =
        (*cinfo).master as crate::src::jcmaster::my_master_ptr;
    if (*master).pass_number < (*master).pass_number_scan_opt_base {
        (*cinfo).comps_in_scan = 1 as libc::c_int;
        if (*(*cinfo).master).use_scans_in_trellis != 0 {
            (*cinfo).cur_comp_info[0 as libc::c_int as usize] = &mut *(*cinfo).comp_info.offset(
                ((*master).pass_number / (4 as libc::c_int * (*(*cinfo).master).trellis_num_loops))
                    as isize,
            )
                as *mut crate::jpeglib_h::jpeg_component_info;
            (*cinfo).Ss = if ((*master).pass_number % 4 as libc::c_int) < 2 as libc::c_int {
                1 as libc::c_int
            } else {
                ((*(*cinfo).master).trellis_freq_split) + 1 as libc::c_int
            };
            (*cinfo).Se = if ((*master).pass_number % 4 as libc::c_int) < 2 as libc::c_int {
                (*(*cinfo).master).trellis_freq_split
            } else {
                (crate::jpeglib_h::DCTSIZE2) - 1 as libc::c_int
            }
        } else {
            (*cinfo).cur_comp_info[0 as libc::c_int as usize] = &mut *(*cinfo).comp_info.offset(
                ((*master).pass_number / (2 as libc::c_int * (*(*cinfo).master).trellis_num_loops))
                    as isize,
            )
                as *mut crate::jpeglib_h::jpeg_component_info;
            (*cinfo).Ss = 1 as libc::c_int;
            (*cinfo).Se = crate::jpeglib_h::DCTSIZE2 - 1 as libc::c_int
        }
    } else if !(*cinfo).scan_info.is_null() {
        /* Prepare for current scan --- the script is already validated */
        let mut scanptr: *const crate::jpeglib_h::jpeg_scan_info =
            (*cinfo).scan_info.offset((*master).scan_number as isize);
        (*cinfo).comps_in_scan = (*scanptr).comps_in_scan;
        ci = 0 as libc::c_int;
        while ci < (*scanptr).comps_in_scan {
            (*cinfo).cur_comp_info[ci as usize] = &mut *(*cinfo)
                .comp_info
                .offset(*(*scanptr).component_index.as_ptr().offset(ci as isize) as isize)
                as *mut crate::jpeglib_h::jpeg_component_info;
            ci += 1
        }
        (*cinfo).Ss = (*scanptr).Ss;
        (*cinfo).Se = (*scanptr).Se;
        (*cinfo).Ah = (*scanptr).Ah;
        (*cinfo).Al = (*scanptr).Al;
        if (*(*cinfo).master).optimize_scans != 0 {
            /* luma frequency split passes */
            if (*master).scan_number
                >= (*(*cinfo).master).num_scans_luma_dc
                    + 3 as libc::c_int * (*(*cinfo).master).Al_max_luma
                    + 2 as libc::c_int
                && (*master).scan_number < (*(*cinfo).master).num_scans_luma
            {
                (*cinfo).Al = (*master).best_Al_luma
            }
            /* chroma frequency split passes */
            if (*master).scan_number
                >= (*(*cinfo).master).num_scans_luma
                    + (*(*cinfo).master).num_scans_chroma_dc
                    + (6 as libc::c_int * (*(*cinfo).master).Al_max_chroma + 4 as libc::c_int)
                && (*master).scan_number < (*cinfo).num_scans
            {
                (*cinfo).Al = (*master).best_Al_chroma
            }
        }
        /* save value for later retrieval during printout of scans */
        (*master).actual_Al[(*master).scan_number as usize] = (*cinfo).Al
    } else {
        /* Prepare for single sequential-JPEG scan containing all components */
        if (*cinfo).num_components > crate::jpeglib_h::MAX_COMPS_IN_SCAN {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_COMPONENT_COUNT as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*cinfo).num_components;
            (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = 4 as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        (*cinfo).comps_in_scan = (*cinfo).num_components;
        ci = 0 as libc::c_int;
        while ci < (*cinfo).num_components {
            (*cinfo).cur_comp_info[ci as usize] = &mut *(*cinfo).comp_info.offset(ci as isize)
                as *mut crate::jpeglib_h::jpeg_component_info;
            ci += 1
        }
        (*cinfo).Ss = 0 as libc::c_int;
        (*cinfo).Se = crate::jpeglib_h::DCTSIZE2 - 1 as libc::c_int;
        (*cinfo).Ah = 0 as libc::c_int;
        (*cinfo).Al = 0 as libc::c_int
    };
}

unsafe extern "C" fn per_scan_setup(mut cinfo: crate::jpeglib_h::j_compress_ptr)
/* Do computations that are needed before processing a JPEG scan */
/* cinfo->comps_in_scan and cinfo->cur_comp_info[] are already set */
{
    let mut ci: libc::c_int = 0;
    let mut mcublks: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    if (*cinfo).comps_in_scan == 1 as libc::c_int {
        /* Noninterleaved (single-component) scan */
        compptr = (*cinfo).cur_comp_info[0 as libc::c_int as usize];
        /* Overall image size in MCUs */
        (*cinfo).MCUs_per_row = (*compptr).width_in_blocks;
        (*cinfo).MCU_rows_in_scan = (*compptr).height_in_blocks;
        /* For noninterleaved scan, always one block per MCU */
        (*compptr).MCU_width = 1 as libc::c_int;
        (*compptr).MCU_height = 1 as libc::c_int;
        (*compptr).MCU_blocks = 1 as libc::c_int;
        (*compptr).MCU_sample_width = crate::jpeglib_h::DCTSIZE;
        (*compptr).last_col_width = 1 as libc::c_int;
        /* For noninterleaved scans, it is convenient to define last_row_height
         * as the number of block rows present in the last iMCU row.
         */
        tmp = (*compptr)
            .height_in_blocks
            .wrapping_rem((*compptr).v_samp_factor as libc::c_uint) as libc::c_int;
        if tmp == 0 as libc::c_int {
            tmp = (*compptr).v_samp_factor
        }
        (*compptr).last_row_height = tmp;
        /* Prepare array describing MCU composition */
        (*cinfo).blocks_in_MCU = 1 as libc::c_int;
        (*cinfo).MCU_membership[0 as libc::c_int as usize] = 0 as libc::c_int
    } else {
        /* Interleaved (multi-component) scan */
        if (*cinfo).comps_in_scan <= 0 as libc::c_int
            || (*cinfo).comps_in_scan > crate::jpeglib_h::MAX_COMPS_IN_SCAN
        {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_COMPONENT_COUNT as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*cinfo).comps_in_scan;
            (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = 4 as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        /* Overall image size in MCUs */
        (*cinfo).MCUs_per_row = crate::src::jutils::jdiv_round_up(
            (*cinfo).image_width as libc::c_long,
            ((*cinfo).max_h_samp_factor * crate::jpeglib_h::DCTSIZE) as libc::c_long,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).MCU_rows_in_scan = crate::src::jutils::jdiv_round_up(
            (*cinfo).image_height as libc::c_long,
            ((*cinfo).max_v_samp_factor * crate::jpeglib_h::DCTSIZE) as libc::c_long,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).blocks_in_MCU = 0 as libc::c_int;
        ci = 0 as libc::c_int;
        while ci < (*cinfo).comps_in_scan {
            compptr = (*cinfo).cur_comp_info[ci as usize];
            /* Sampling factors give # of blocks of component in each MCU */
            (*compptr).MCU_width = (*compptr).h_samp_factor;
            (*compptr).MCU_height = (*compptr).v_samp_factor;
            (*compptr).MCU_blocks = (*compptr).MCU_width * (*compptr).MCU_height;
            (*compptr).MCU_sample_width = (*compptr).MCU_width * crate::jpeglib_h::DCTSIZE;
            /* Figure number of non-dummy blocks in last MCU column & row */
            tmp = (*compptr)
                .width_in_blocks
                .wrapping_rem((*compptr).MCU_width as libc::c_uint)
                as libc::c_int;
            if tmp == 0 as libc::c_int {
                tmp = (*compptr).MCU_width
            }
            (*compptr).last_col_width = tmp;
            tmp = (*compptr)
                .height_in_blocks
                .wrapping_rem((*compptr).MCU_height as libc::c_uint)
                as libc::c_int;
            if tmp == 0 as libc::c_int {
                tmp = (*compptr).MCU_height
            }
            (*compptr).last_row_height = tmp;
            /* Prepare array describing MCU composition */
            mcublks = (*compptr).MCU_blocks;
            if (*cinfo).blocks_in_MCU + mcublks > crate::jpeglib_h::C_MAX_BLOCKS_IN_MCU {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_MCU_SIZE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            loop {
                let fresh1 = mcublks;
                mcublks = mcublks - 1;
                if !(fresh1 > 0 as libc::c_int) {
                    break;
                }
                let fresh2 = (*cinfo).blocks_in_MCU;
                (*cinfo).blocks_in_MCU = (*cinfo).blocks_in_MCU + 1;
                (*cinfo).MCU_membership[fresh2 as usize] = ci
            }
            ci += 1
        }
    }
    /* Convert restart specified in rows to actual MCU count. */
    /* Note that count must fit in 16 bits, so we provide limiting. */
    if (*cinfo).restart_in_rows > 0 as libc::c_int {
        let mut nominal: libc::c_long =
            (*cinfo).restart_in_rows as libc::c_long * (*cinfo).MCUs_per_row as libc::c_long;
        (*cinfo).restart_interval = if nominal < 65535 as libc::c_long {
            nominal
        } else {
            65535 as libc::c_long
        } as libc::c_uint
    };
}
/*
 * Per-pass setup.
 * This is called at the beginning of each pass.  We determine which modules
 * will be active during this pass and give them appropriate start_pass calls.
 * We also set is_last_pass to indicate whether any more passes will be
 * required.
 */

unsafe extern "C" fn prepare_for_pass(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    let mut master: crate::src::jcmaster::my_master_ptr =
        (*cinfo).master as crate::src::jcmaster::my_master_ptr;
    (*(*cinfo).master).trellis_passes =
        ((*master).pass_number < (*master).pass_number_scan_opt_base) as libc::c_int;
    let mut current_block_54: u64;
    match (*master).pass_type as libc::c_uint {
        0 => {
            /* Initial pass: will collect input data, and do either Huffman
             * optimization or data output for the first scan.
             */
            select_scan_parameters(cinfo);
            per_scan_setup(cinfo);
            if (*cinfo).raw_data_in == 0 {
                Some(
                    (*(*cinfo).cconvert)
                        .start_pass
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
                Some(
                    (*(*cinfo).downsample)
                        .start_pass
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
                Some(
                    (*(*cinfo).prep)
                        .start_pass
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo, crate::jpegint_h::JBUF_PASS_THRU
                );
            }
            Some(
                (*(*cinfo).fdct)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
            Some(
                (*(*cinfo).entropy)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                (((*cinfo).optimize_coding != 0 || (*(*cinfo).master).trellis_quant != 0)
                    && (*cinfo).arith_code == 0) as libc::c_int,
            );
            Some(
                (*(*cinfo).coef)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                if (*master).total_passes > 1 as libc::c_int {
                    crate::jpegint_h::JBUF_SAVE_AND_PASS as libc::c_int
                } else {
                    crate::jpegint_h::JBUF_PASS_THRU as libc::c_int
                } as crate::jpegint_h::J_BUF_MODE,
            );
            Some(
                (*(*cinfo).main)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo, crate::jpegint_h::JBUF_PASS_THRU
            );
            if (*cinfo).optimize_coding != 0 || (*(*cinfo).master).trellis_quant != 0 {
                /* No immediate data output; postpone writing frame/scan headers */
                (*master).pub_0.call_pass_startup = crate::jmorecfg_h::FALSE
            } else {
                /* Will write frame/scan headers at first jpeg_write_scanlines call */
                (*master).pub_0.call_pass_startup = crate::jmorecfg_h::TRUE
            }
            current_block_54 = 2500484646272006982;
        }
        1 => {
            /* Do Huffman optimization for a scan after the first one. */
            select_scan_parameters(cinfo);
            per_scan_setup(cinfo);
            if (*cinfo).Ss != 0 as libc::c_int
                || (*cinfo).Ah == 0 as libc::c_int
                || (*cinfo).arith_code != 0
            {
                Some(
                    (*(*cinfo).entropy)
                        .start_pass
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo, crate::jmorecfg_h::TRUE);
                Some(
                    (*(*cinfo).coef)
                        .start_pass
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo, crate::jpegint_h::JBUF_CRANK_DEST
                );
                (*master).pub_0.call_pass_startup = crate::jmorecfg_h::FALSE;
                current_block_54 = 2500484646272006982;
            } else {
                /* Special case: Huffman DC refinement scans need no Huffman table
                 * and therefore we can skip the optimization pass for them.
                 */
                (*master).pass_type = crate::src::jcmaster::output_pass;
                (*master).pass_number += 1;
                current_block_54 = 12235309447780442549;
            }
        }
        2 => {
            current_block_54 = 12235309447780442549;
        }
        3 => {
            if (*master).pass_number
                % ((*cinfo).num_components
                    * (if (*(*cinfo).master).use_scans_in_trellis != 0 {
                        4 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }))
                == 1 as libc::c_int
                && (*(*cinfo).master).trellis_q_opt != 0
            {
                let mut i: libc::c_int = 0;
                let mut j: libc::c_int = 0;
                i = 0 as libc::c_int;
                while i < crate::jpeglib_h::NUM_QUANT_TBLS {
                    j = 1 as libc::c_int;
                    while j < crate::jpeglib_h::DCTSIZE2 {
                        (*(*cinfo).master).norm_src[i as usize][j as usize] = 0.0f64;
                        (*(*cinfo).master).norm_coef[i as usize][j as usize] = 0.0f64;
                        j += 1
                    }
                    i += 1
                }
            }
            Some(
                (*(*cinfo).entropy)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo, ((*cinfo).arith_code == 0) as libc::c_int
            );
            Some(
                (*(*cinfo).coef)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo, crate::jpegint_h::JBUF_REQUANT);
            (*master).pub_0.call_pass_startup = crate::jmorecfg_h::FALSE;
            current_block_54 = 2500484646272006982;
        }
        _ => {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_NOT_COMPILED as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
            current_block_54 = 2500484646272006982;
        }
    }
    match current_block_54 {
        12235309447780442549 =>
        /*FALLTHROUGH*/
        /* Do a data-output pass. */
        /* We need not repeat per-scan setup if prior optimization pass did it. */
        {
            if (*cinfo).optimize_coding == 0 {
                select_scan_parameters(cinfo);
                per_scan_setup(cinfo);
            }
            if (*(*cinfo).master).optimize_scans != 0 {
                (*master).saved_dest = (*cinfo).dest;
                (*cinfo).dest =
                    crate::stddef_h::NULL as *mut crate::jpeglib_h::jpeg_destination_mgr;
                (*master).scan_size[(*master).scan_number as usize] =
                    0 as libc::c_int as libc::c_ulong;
                crate::src::jdatadst::jpeg_mem_dest_internal(
                    cinfo,
                    &mut *(*master)
                        .scan_buffer
                        .as_mut_ptr()
                        .offset((*master).scan_number as isize),
                    &mut *(*master)
                        .scan_size
                        .as_mut_ptr()
                        .offset((*master).scan_number as isize),
                    crate::jpeglib_h::JPOOL_IMAGE,
                );
                Some(
                    (*(*cinfo).dest)
                        .init_destination
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
            }
            Some(
                (*(*cinfo).entropy)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo, crate::jmorecfg_h::FALSE);
            Some(
                (*(*cinfo).coef)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo, crate::jpegint_h::JBUF_CRANK_DEST
            );
            /* We emit frame/scan headers now */
            if (*master).scan_number == 0 as libc::c_int {
                Some(
                    (*(*cinfo).marker)
                        .write_frame_header
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
            }
            Some(
                (*(*cinfo).marker)
                    .write_scan_header
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
            (*master).pub_0.call_pass_startup = crate::jmorecfg_h::FALSE
        }
        _ => {}
    }
    (*master).pub_0.is_last_pass =
        ((*master).pass_number == (*master).total_passes - 1 as libc::c_int) as libc::c_int;
    /* Set up progress monitor's pass info if present */
    if !(*cinfo).progress.is_null() {
        (*(*cinfo).progress).completed_passes = (*master).pass_number;
        (*(*cinfo).progress).total_passes = (*master).total_passes
    };
}
/*
 * Special start-of-pass hook.
 * This is called by jpeg_write_scanlines if call_pass_startup is TRUE.
 * In single-pass processing, we need this hook because we don't want to
 * write frame/scan headers during jpeg_start_compress; we want to let the
 * application write COM markers etc. between jpeg_start_compress and the
 * jpeg_write_scanlines loop.
 * In multi-pass processing, this routine is not used.
 */

unsafe extern "C" fn pass_startup(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    (*(*cinfo).master).call_pass_startup = crate::jmorecfg_h::FALSE; /* reset flag so call only once */
    Some(
        (*(*cinfo).marker)
            .write_frame_header
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    Some(
        (*(*cinfo).marker)
            .write_scan_header
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
}

unsafe extern "C" fn copy_buffer(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut scan_idx: libc::c_int,
) {
    let mut master: crate::src::jcmaster::my_master_ptr =
        (*cinfo).master as crate::src::jcmaster::my_master_ptr;
    let mut size: libc::c_ulong = (*master).scan_size[scan_idx as usize];
    let mut src: *mut libc::c_uchar = (*master).scan_buffer[scan_idx as usize];
    let mut i: libc::c_int = 0;
    if (*(*cinfo).err).trace_level > 0 as libc::c_int {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"SCAN \x00" as *const u8 as *const libc::c_char,
        );
        i = 0 as libc::c_int;
        while i < (*(*cinfo).scan_info.offset(scan_idx as isize)).comps_in_scan {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s%d\x00" as *const u8 as *const libc::c_char,
                if i == 0 as libc::c_int {
                    b"\x00" as *const u8 as *const libc::c_char
                } else {
                    b",\x00" as *const u8 as *const libc::c_char
                },
                (*(*cinfo).scan_info.offset(scan_idx as isize)).component_index[i as usize],
            );
            i += 1
        }
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b": %d %d\x00" as *const u8 as *const libc::c_char,
            (*(*cinfo).scan_info.offset(scan_idx as isize)).Ss,
            (*(*cinfo).scan_info.offset(scan_idx as isize)).Se,
        );
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b" %d %d\x00" as *const u8 as *const libc::c_char,
            (*(*cinfo).scan_info.offset(scan_idx as isize)).Ah,
            (*master).actual_Al[scan_idx as usize],
        );
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    while size >= (*(*cinfo).dest).free_in_buffer {
        crate::stdlib::memcpy(
            (*(*cinfo).dest).next_output_byte as *mut libc::c_void,
            src as *const libc::c_void,
            (*(*cinfo).dest).free_in_buffer,
        );
        src = src.offset((*(*cinfo).dest).free_in_buffer as isize);
        size = size.wrapping_sub((*(*cinfo).dest).free_in_buffer);
        (*(*cinfo).dest).next_output_byte = (*(*cinfo).dest)
            .next_output_byte
            .offset((*(*cinfo).dest).free_in_buffer as isize);
        (*(*cinfo).dest).free_in_buffer = 0 as libc::c_int as crate::stddef_h::size_t;
        if Some(
            (*(*cinfo).dest)
                .empty_output_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_UNSUPPORTED_SUSPEND as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
    }
    crate::stdlib::memcpy(
        (*(*cinfo).dest).next_output_byte as *mut libc::c_void,
        src as *const libc::c_void,
        size,
    );
    (*(*cinfo).dest).next_output_byte = (*(*cinfo).dest).next_output_byte.offset(size as isize);
    (*(*cinfo).dest).free_in_buffer = ((*(*cinfo).dest).free_in_buffer as libc::c_ulong)
        .wrapping_sub(size) as crate::stddef_h::size_t
        as crate::stddef_h::size_t;
}

unsafe extern "C" fn select_scans(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut next_scan_number: libc::c_int,
) {
    let mut master: crate::src::jcmaster::my_master_ptr =
        (*cinfo).master as crate::src::jcmaster::my_master_ptr;
    let mut base_scan_idx: libc::c_int = 0 as libc::c_int;
    let mut luma_freq_split_scan_start: libc::c_int = (*(*cinfo).master).num_scans_luma_dc
        + 3 as libc::c_int * (*(*cinfo).master).Al_max_luma
        + 2 as libc::c_int;
    let mut chroma_freq_split_scan_start: libc::c_int = (*(*cinfo).master).num_scans_luma
        + (*(*cinfo).master).num_scans_chroma_dc
        + (6 as libc::c_int * (*(*cinfo).master).Al_max_chroma + 4 as libc::c_int);
    let mut passes_per_scan: libc::c_int = if (*cinfo).optimize_coding != 0 {
        2 as libc::c_int
    } else {
        1 as libc::c_int
    };
    if next_scan_number > 1 as libc::c_int && next_scan_number <= luma_freq_split_scan_start {
        if (next_scan_number - 1 as libc::c_int) % 3 as libc::c_int == 2 as libc::c_int {
            let mut Al: libc::c_int = (next_scan_number - 1 as libc::c_int) / 3 as libc::c_int;
            let mut i: libc::c_int = 0;
            let mut cost: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
            cost = cost
                .wrapping_add((*master).scan_size[(next_scan_number - 2 as libc::c_int) as usize]);
            cost = cost
                .wrapping_add((*master).scan_size[(next_scan_number - 1 as libc::c_int) as usize]);
            i = 0 as libc::c_int;
            while i < Al {
                cost = cost.wrapping_add(
                    (*master).scan_size[(3 as libc::c_int + 3 as libc::c_int * i) as usize],
                );
                i += 1
            }
            if Al == 0 as libc::c_int || cost < (*master).best_cost {
                (*master).best_cost = cost;
                (*master).best_Al_luma = Al
            } else {
                (*master).scan_number = luma_freq_split_scan_start - 1 as libc::c_int;
                (*master).pass_number = passes_per_scan * ((*master).scan_number + 1 as libc::c_int)
                    - 1 as libc::c_int
                    + (*master).pass_number_scan_opt_base
            }
        }
    } else if next_scan_number > luma_freq_split_scan_start
        && next_scan_number <= (*(*cinfo).master).num_scans_luma
    {
        if next_scan_number == luma_freq_split_scan_start + 1 as libc::c_int {
            (*master).best_freq_split_idx_luma = 0 as libc::c_int;
            (*master).best_cost =
                (*master).scan_size[(next_scan_number - 1 as libc::c_int) as usize]
        } else if (next_scan_number - luma_freq_split_scan_start) % 2 as libc::c_int
            == 1 as libc::c_int
        {
            let mut idx: libc::c_int =
                next_scan_number - luma_freq_split_scan_start >> 1 as libc::c_int;
            let mut cost_0: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
            cost_0 = cost_0
                .wrapping_add((*master).scan_size[(next_scan_number - 2 as libc::c_int) as usize]);
            cost_0 = cost_0
                .wrapping_add((*master).scan_size[(next_scan_number - 1 as libc::c_int) as usize]);
            if cost_0 < (*master).best_cost {
                (*master).best_cost = cost_0;
                (*master).best_freq_split_idx_luma = idx
            }
            /* if after testing first 3, no split is the best, don't search further */
            if idx == 2 as libc::c_int && (*master).best_freq_split_idx_luma == 0 as libc::c_int
                || idx == 3 as libc::c_int && (*master).best_freq_split_idx_luma != 2 as libc::c_int
                || idx == 4 as libc::c_int && (*master).best_freq_split_idx_luma != 4 as libc::c_int
            {
                (*master).scan_number = (*(*cinfo).master).num_scans_luma - 1 as libc::c_int;
                (*master).pass_number =
                    passes_per_scan * ((*master).scan_number + 1 as libc::c_int) - 1 as libc::c_int
                        + (*master).pass_number_scan_opt_base;
                (*master).pub_0.is_last_pass = ((*master).pass_number
                    == (*master).total_passes - 1 as libc::c_int)
                    as libc::c_int
            }
        }
    } else if (*cinfo).num_scans > (*(*cinfo).master).num_scans_luma {
        if next_scan_number
            == (*(*cinfo).master).num_scans_luma + (*(*cinfo).master).num_scans_chroma_dc
        {
            base_scan_idx = (*(*cinfo).master).num_scans_luma;
            (*master).interleave_chroma_dc = ((*master).scan_size[base_scan_idx as usize]
                <= (*master).scan_size[(base_scan_idx + 1 as libc::c_int) as usize]
                    .wrapping_add((*master).scan_size[(base_scan_idx + 2 as libc::c_int) as usize]))
                as libc::c_int
        } else if next_scan_number
            > (*(*cinfo).master).num_scans_luma + (*(*cinfo).master).num_scans_chroma_dc
            && next_scan_number <= chroma_freq_split_scan_start
        {
            base_scan_idx =
                (*(*cinfo).master).num_scans_luma + (*(*cinfo).master).num_scans_chroma_dc;
            if (next_scan_number - base_scan_idx) % 6 as libc::c_int == 4 as libc::c_int {
                let mut Al_0: libc::c_int = (next_scan_number - base_scan_idx) / 6 as libc::c_int;
                let mut i_0: libc::c_int = 0;
                let mut cost_1: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
                cost_1 = cost_1.wrapping_add(
                    (*master).scan_size[(next_scan_number - 4 as libc::c_int) as usize],
                );
                cost_1 = cost_1.wrapping_add(
                    (*master).scan_size[(next_scan_number - 3 as libc::c_int) as usize],
                );
                cost_1 = cost_1.wrapping_add(
                    (*master).scan_size[(next_scan_number - 2 as libc::c_int) as usize],
                );
                cost_1 = cost_1.wrapping_add(
                    (*master).scan_size[(next_scan_number - 1 as libc::c_int) as usize],
                );
                i_0 = 0 as libc::c_int;
                while i_0 < Al_0 {
                    cost_1 = cost_1.wrapping_add(
                        (*master).scan_size
                            [(base_scan_idx + 4 as libc::c_int + 6 as libc::c_int * i_0) as usize],
                    );
                    cost_1 = cost_1.wrapping_add(
                        (*master).scan_size
                            [(base_scan_idx + 5 as libc::c_int + 6 as libc::c_int * i_0) as usize],
                    );
                    i_0 += 1
                }
                if Al_0 == 0 as libc::c_int || cost_1 < (*master).best_cost {
                    (*master).best_cost = cost_1;
                    (*master).best_Al_chroma = Al_0
                } else {
                    (*master).scan_number = chroma_freq_split_scan_start - 1 as libc::c_int;
                    (*master).pass_number = passes_per_scan
                        * ((*master).scan_number + 1 as libc::c_int)
                        - 1 as libc::c_int
                        + (*master).pass_number_scan_opt_base
                }
            }
        } else if next_scan_number > chroma_freq_split_scan_start
            && next_scan_number <= (*cinfo).num_scans
        {
            if next_scan_number == chroma_freq_split_scan_start + 2 as libc::c_int {
                (*master).best_freq_split_idx_chroma = 0 as libc::c_int;
                (*master).best_cost =
                    (*master).scan_size[(next_scan_number - 2 as libc::c_int) as usize];
                (*master).best_cost = (*master).best_cost.wrapping_add(
                    (*master).scan_size[(next_scan_number - 1 as libc::c_int) as usize],
                )
            } else if (next_scan_number - chroma_freq_split_scan_start) % 4 as libc::c_int
                == 2 as libc::c_int
            {
                let mut idx_0: libc::c_int =
                    next_scan_number - chroma_freq_split_scan_start >> 2 as libc::c_int;
                let mut cost_2: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
                cost_2 = cost_2.wrapping_add(
                    (*master).scan_size[(next_scan_number - 4 as libc::c_int) as usize],
                );
                cost_2 = cost_2.wrapping_add(
                    (*master).scan_size[(next_scan_number - 3 as libc::c_int) as usize],
                );
                cost_2 = cost_2.wrapping_add(
                    (*master).scan_size[(next_scan_number - 2 as libc::c_int) as usize],
                );
                cost_2 = cost_2.wrapping_add(
                    (*master).scan_size[(next_scan_number - 1 as libc::c_int) as usize],
                );
                if cost_2 < (*master).best_cost {
                    (*master).best_cost = cost_2;
                    (*master).best_freq_split_idx_chroma = idx_0
                }
                /* if after testing first 3, no split is the best, don't search further */
                if idx_0 == 2 as libc::c_int
                    && (*master).best_freq_split_idx_chroma == 0 as libc::c_int
                    || idx_0 == 3 as libc::c_int
                        && (*master).best_freq_split_idx_chroma != 2 as libc::c_int
                    || idx_0 == 4 as libc::c_int
                        && (*master).best_freq_split_idx_chroma != 4 as libc::c_int
                {
                    (*master).scan_number = (*cinfo).num_scans - 1 as libc::c_int;
                    (*master).pass_number = passes_per_scan
                        * ((*master).scan_number + 1 as libc::c_int)
                        - 1 as libc::c_int
                        + (*master).pass_number_scan_opt_base;
                    (*master).pub_0.is_last_pass = ((*master).pass_number
                        == (*master).total_passes - 1 as libc::c_int)
                        as libc::c_int
                }
            }
        }
    }
    if (*master).scan_number == (*cinfo).num_scans - 1 as libc::c_int {
        let mut i_1: libc::c_int = 0;
        let mut Al_1: libc::c_int = 0;
        let mut min_Al: libc::c_int = if (*master).best_Al_luma < (*master).best_Al_chroma {
            (*master).best_Al_luma
        } else {
            (*master).best_Al_chroma
        };
        copy_buffer(cinfo, 0 as libc::c_int);
        if (*cinfo).num_scans > (*(*cinfo).master).num_scans_luma
            && (*(*cinfo).master).dc_scan_opt_mode != 0 as libc::c_int
        {
            base_scan_idx = (*(*cinfo).master).num_scans_luma;
            if (*master).interleave_chroma_dc != 0
                && (*(*cinfo).master).dc_scan_opt_mode != 1 as libc::c_int
            {
                copy_buffer(cinfo, base_scan_idx);
            } else {
                copy_buffer(cinfo, base_scan_idx + 1 as libc::c_int);
                copy_buffer(cinfo, base_scan_idx + 2 as libc::c_int);
            }
        }
        if (*master).best_freq_split_idx_luma == 0 as libc::c_int {
            copy_buffer(cinfo, luma_freq_split_scan_start);
        } else {
            copy_buffer(
                cinfo,
                luma_freq_split_scan_start
                    + 2 as libc::c_int * ((*master).best_freq_split_idx_luma - 1 as libc::c_int)
                    + 1 as libc::c_int,
            );
            copy_buffer(
                cinfo,
                luma_freq_split_scan_start
                    + 2 as libc::c_int * ((*master).best_freq_split_idx_luma - 1 as libc::c_int)
                    + 2 as libc::c_int,
            );
        }
        /* copy the LSB refinements as well */
        Al_1 = (*master).best_Al_luma - 1 as libc::c_int;
        while Al_1 >= min_Al {
            copy_buffer(cinfo, 3 as libc::c_int + 3 as libc::c_int * Al_1);
            Al_1 -= 1
        }
        if (*cinfo).num_scans > (*(*cinfo).master).num_scans_luma {
            if (*master).best_freq_split_idx_chroma == 0 as libc::c_int {
                copy_buffer(cinfo, chroma_freq_split_scan_start);
                copy_buffer(cinfo, chroma_freq_split_scan_start + 1 as libc::c_int);
            } else {
                copy_buffer(
                    cinfo,
                    chroma_freq_split_scan_start
                        + 4 as libc::c_int
                            * ((*master).best_freq_split_idx_chroma - 1 as libc::c_int)
                        + 2 as libc::c_int,
                );
                copy_buffer(
                    cinfo,
                    chroma_freq_split_scan_start
                        + 4 as libc::c_int
                            * ((*master).best_freq_split_idx_chroma - 1 as libc::c_int)
                        + 3 as libc::c_int,
                );
                copy_buffer(
                    cinfo,
                    chroma_freq_split_scan_start
                        + 4 as libc::c_int
                            * ((*master).best_freq_split_idx_chroma - 1 as libc::c_int)
                        + 4 as libc::c_int,
                );
                copy_buffer(
                    cinfo,
                    chroma_freq_split_scan_start
                        + 4 as libc::c_int
                            * ((*master).best_freq_split_idx_chroma - 1 as libc::c_int)
                        + 5 as libc::c_int,
                );
            }
            base_scan_idx =
                (*(*cinfo).master).num_scans_luma + (*(*cinfo).master).num_scans_chroma_dc;
            Al_1 = (*master).best_Al_chroma - 1 as libc::c_int;
            while Al_1 >= min_Al {
                copy_buffer(
                    cinfo,
                    base_scan_idx + 6 as libc::c_int * Al_1 + 4 as libc::c_int,
                );
                copy_buffer(
                    cinfo,
                    base_scan_idx + 6 as libc::c_int * Al_1 + 5 as libc::c_int,
                );
                Al_1 -= 1
            }
        }
        Al_1 = min_Al - 1 as libc::c_int;
        while Al_1 >= 0 as libc::c_int {
            copy_buffer(cinfo, 3 as libc::c_int + 3 as libc::c_int * Al_1);
            if (*cinfo).num_scans > (*(*cinfo).master).num_scans_luma {
                copy_buffer(
                    cinfo,
                    base_scan_idx + 6 as libc::c_int * Al_1 + 4 as libc::c_int,
                );
                copy_buffer(
                    cinfo,
                    base_scan_idx + 6 as libc::c_int * Al_1 + 5 as libc::c_int,
                );
            }
            Al_1 -= 1
        }
        /* free the memory allocated for buffers */
        i_1 = 0 as libc::c_int;
        while i_1 < (*cinfo).num_scans {
            if !(*master).scan_buffer[i_1 as usize].is_null() {
                crate::stdlib::free((*master).scan_buffer[i_1 as usize] as *mut libc::c_void);
            }
            i_1 += 1
        }
    };
}
/*
 * Finish up at end of pass.
 */

unsafe extern "C" fn finish_pass_master(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    let mut master: crate::src::jcmaster::my_master_ptr =
        (*cinfo).master as crate::src::jcmaster::my_master_ptr;
    /* The entropy coder always needs an end-of-pass call,
     * either to analyze statistics or to flush its output buffer.
     */
    Some(
        (*(*cinfo).entropy)
            .finish_pass
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    /* Update state for next pass */
    match (*master).pass_type as libc::c_uint {
        0 => {
            /* next pass is either output of scan 0 (after optimization)
             * or output of scan 1 (if no optimization).
             */
            if (*(*cinfo).master).trellis_quant != 0 {
                (*master).pass_type = crate::src::jcmaster::trellis_pass
            } else {
                (*master).pass_type = crate::src::jcmaster::output_pass;
                if (*cinfo).optimize_coding == 0 {
                    (*master).scan_number += 1
                }
            }
        }
        1 => {
            /* next pass is always output of current scan */
            (*master).pass_type =
                if (*master).pass_number < (*master).pass_number_scan_opt_base - 1 as libc::c_int {
                    crate::src::jcmaster::trellis_pass as libc::c_int
                } else {
                    crate::src::jcmaster::output_pass as libc::c_int
                } as crate::src::jcmaster::c_pass_type
        }
        2 => {
            /* next pass is either optimization or output of next scan */
            if (*cinfo).optimize_coding != 0 {
                (*master).pass_type = crate::src::jcmaster::huff_opt_pass
            }
            if (*(*cinfo).master).optimize_scans != 0 {
                Some(
                    (*(*cinfo).dest)
                        .term_destination
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
                (*cinfo).dest = (*master).saved_dest;
                select_scans(cinfo, (*master).scan_number + 1 as libc::c_int);
            }
            (*master).scan_number += 1
        }
        3 => {
            if (*cinfo).optimize_coding != 0 {
                (*master).pass_type = crate::src::jcmaster::huff_opt_pass
            } else {
                (*master).pass_type = if (*master).pass_number
                    < (*master).pass_number_scan_opt_base - 1 as libc::c_int
                {
                    crate::src::jcmaster::trellis_pass as libc::c_int
                } else {
                    crate::src::jcmaster::output_pass as libc::c_int
                } as crate::src::jcmaster::c_pass_type
            }
            if ((*master).pass_number + 1 as libc::c_int)
                % ((*cinfo).num_components
                    * (if (*(*cinfo).master).use_scans_in_trellis != 0 {
                        4 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }))
                == 0 as libc::c_int
                && (*(*cinfo).master).trellis_q_opt != 0
            {
                let mut i: libc::c_int = 0;
                let mut j: libc::c_int = 0;
                i = 0 as libc::c_int;
                while i < crate::jpeglib_h::NUM_QUANT_TBLS {
                    j = 1 as libc::c_int;
                    while j < crate::jpeglib_h::DCTSIZE2 {
                        if (*(*cinfo).master).norm_coef[i as usize][j as usize] != 0.0f64 {
                            let mut q: libc::c_int =
                                ((*(*cinfo).master).norm_src[i as usize][j as usize]
                                    / (*(*cinfo).master).norm_coef[i as usize][j as usize]
                                    + 0.5f64) as libc::c_int;
                            if q > 254 as libc::c_int {
                                q = 254 as libc::c_int
                            }
                            if q < 1 as libc::c_int {
                                q = 1 as libc::c_int
                            }
                            (*(*cinfo).quant_tbl_ptrs[i as usize]).quantval[j as usize] =
                                q as crate::jmorecfg_h::UINT16
                        }
                        j += 1
                    }
                    i += 1
                }
            }
        }
        _ => {}
    }
    (*master).pass_number += 1;
}
/*
 * Initialize master compression control.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_c_master_control(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut transcode_only: crate::jmorecfg_h::boolean,
) {
    let mut master: crate::src::jcmaster::my_master_ptr =
        (*cinfo).master as crate::src::jcmaster::my_master_ptr;
    (*master).pub_0.prepare_for_pass =
        Some(prepare_for_pass as unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ());
    (*master).pub_0.pass_startup =
        Some(pass_startup as unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ());
    (*master).pub_0.finish_pass =
        Some(finish_pass_master as unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ());
    (*master).pub_0.is_last_pass = crate::jmorecfg_h::FALSE;
    (*master).pub_0.call_pass_startup = crate::jmorecfg_h::FALSE;
    /* Validate parameters, determine derived values */
    initial_setup(cinfo, transcode_only); /* assume default tables no good for progressive mode */
    if !(*cinfo).scan_info.is_null() {
        validate_script(cinfo);
    } else {
        (*cinfo).progressive_mode = crate::jmorecfg_h::FALSE;
        (*cinfo).num_scans = 1 as libc::c_int
    }
    if (*cinfo).progressive_mode != 0 && (*cinfo).arith_code == 0 {
        /*  TEMPORARY HACK ??? */
        (*cinfo).optimize_coding = crate::jmorecfg_h::TRUE
    }
    /* Initialize my private state */
    if transcode_only != 0 {
        /* no main pass in transcoding */
        if (*cinfo).optimize_coding != 0 {
            (*master).pass_type = crate::src::jcmaster::huff_opt_pass
        } else {
            (*master).pass_type = crate::src::jcmaster::output_pass
        }
    } else {
        /* for normal compression, first pass is always this type: */
        (*master).pass_type = crate::src::jcmaster::main_pass
    }
    (*master).scan_number = 0 as libc::c_int;
    (*master).pass_number = 0 as libc::c_int;
    if (*cinfo).optimize_coding != 0 {
        (*master).total_passes = (*cinfo).num_scans * 2 as libc::c_int
    } else {
        (*master).total_passes = (*cinfo).num_scans
    }
    (*master).jpeg_version =
        b"mozjpeg version 4.0.0 (build 20191212)\x00" as *const u8 as *const libc::c_char;
    (*master).pass_number_scan_opt_base = 0 as libc::c_int;
    if (*(*cinfo).master).trellis_quant != 0 {
        if (*cinfo).optimize_coding != 0 {
            (*master).pass_number_scan_opt_base = (if (*(*cinfo).master).use_scans_in_trellis != 0 {
                4 as libc::c_int
            } else {
                2 as libc::c_int
            }) * (*cinfo).num_components
                * (*(*cinfo).master).trellis_num_loops
        } else {
            (*master).pass_number_scan_opt_base = (if (*(*cinfo).master).use_scans_in_trellis != 0 {
                2 as libc::c_int
            } else {
                1 as libc::c_int
            }) * (*cinfo).num_components
                * (*(*cinfo).master).trellis_num_loops
                + 1 as libc::c_int
        }
        (*master).total_passes += (*master).pass_number_scan_opt_base
    }
    if (*(*cinfo).master).optimize_scans != 0 {
        let mut i: libc::c_int = 0;
        (*master).best_Al_chroma = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*cinfo).num_scans {
            (*master).scan_buffer[i as usize] = crate::stddef_h::NULL as *mut libc::c_uchar;
            i += 1
        }
    };
}
