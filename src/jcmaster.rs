pub use crate::jerror::C2RustUnnamed_4;
pub use crate::jpeglib_h::C2RustUnnamed_3;
use libc::c_char;
use libc::c_int;
use libc::c_long;
use libc::c_uchar;
use libc::c_uint;
use libc::c_ulong;
use libc::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_comp_master {
    pub pub_0: jpeg_comp_master,
    pub pass_type: c_pass_type,
    pub pass_number: c_int,
    pub total_passes: c_int,
    pub scan_number: c_int,
    pub pass_number_scan_opt_base: c_int,
    pub scan_buffer: [*mut c_uchar; 64],
    pub scan_size: [c_ulong; 64],
    pub actual_Al: [c_int; 64],
    pub best_cost: c_ulong,
    pub best_freq_split_idx_luma: c_int,
    pub best_freq_split_idx_chroma: c_int,
    pub best_Al_luma: c_int,
    pub best_Al_chroma: c_int,
    pub interleave_chroma_dc: boolean,
    pub saved_dest: *mut jpeg_destination_mgr,
    pub jpeg_version: *const c_char,
}
/*
 * jcmaster.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains the master control structures for the JPEG compressor.
 */
/* Private state */
pub type c_pass_type = c_uint;
pub use crate::jpegint_h::jpeg_comp_master;
use libc;

pub use crate::jmorecfg_h::boolean;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub type my_master_ptr = *mut my_comp_master;
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
pub use crate::jpegint_h::jpeg_downsampler;
pub use crate::jpegint_h::jpeg_entropy_encoder;
pub use crate::jpegint_h::jpeg_forward_dct;
pub use crate::jpegint_h::jpeg_marker_writer;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;
/* trellis quantization pass */
pub const trellis_pass: c_pass_type = 3;
/* data output pass */
pub const output_pass: c_pass_type = 2;
/* Huffman code optimization pass */
pub const huff_opt_pass: c_pass_type = 1;
/* input data, also do first output step */
pub const main_pass: c_pass_type = 0;
pub use crate::jconfig_h::BITS_IN_JSAMPLE;
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
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JPEG_MAX_DIMENSION;
pub use crate::jmorecfg_h::MAX_COMPONENTS;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jpegint_h::jdiv_round_up;
pub use crate::jpegint_h::jpeg_mem_dest_internal;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::C_MAX_BLOCKS_IN_MCU;
pub use crate::jpeglib_h::DCTSIZE;
pub use crate::jpeglib_h::DCTSIZE2;
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
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::MAX_COMPS_IN_SCAN;
pub use crate::jpeglib_h::MAX_SAMP_FACTOR;
pub use crate::jpeglib_h::NUM_QUANT_TBLS;
pub use crate::stddef_h::NULL;
use crate::stdlib::fprintf;
use crate::stdlib::free;
use crate::stdlib::memcpy;
use crate::stdlib::stderr;
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
unsafe extern "C" fn initial_setup(mut cinfo: j_compress_ptr, mut transcode_only: boolean) {
    let mut ci: c_int = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut samplesperrow: c_long = 0;
    let mut jd_samplesperrow: JDIMENSION = 0;
    if (*cinfo).image_height <= 0i32 as c_uint
        || (*cinfo).image_width <= 0i32 as c_uint
        || (*cinfo).num_components <= 0i32
        || (*cinfo).input_components <= 0i32
    {
        (*(*cinfo).err).msg_code = JERR_EMPTY_IMAGE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*cinfo).image_height as c_long > JPEG_MAX_DIMENSION
        || (*cinfo).image_width as c_long > JPEG_MAX_DIMENSION
    {
        (*(*cinfo).err).msg_code = JERR_IMAGE_TOO_BIG as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = 65500i64 as c_uint as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    samplesperrow = (*cinfo).image_width as c_long * (*cinfo).input_components as c_long;
    jd_samplesperrow = samplesperrow as JDIMENSION;
    if jd_samplesperrow as c_long != samplesperrow {
        (*(*cinfo).err).msg_code = JERR_WIDTH_OVERFLOW as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*cinfo).data_precision != BITS_IN_JSAMPLE {
        (*(*cinfo).err).msg_code = JERR_BAD_PRECISION as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).data_precision;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*cinfo).num_components > MAX_COMPONENTS {
        (*(*cinfo).err).msg_code = JERR_COMPONENT_COUNT as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).num_components;
        (*(*cinfo).err).msg_parm.i[1usize] = 10i32;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*cinfo).max_h_samp_factor = 1i32;
    (*cinfo).max_v_samp_factor = 1i32;
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        if (*compptr).h_samp_factor <= 0i32
            || (*compptr).h_samp_factor > MAX_SAMP_FACTOR
            || (*compptr).v_samp_factor <= 0i32
            || (*compptr).v_samp_factor > MAX_SAMP_FACTOR
        {
            (*(*cinfo).err).msg_code = JERR_BAD_SAMPLING as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
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
        compptr = compptr.offset(1isize)
    }
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        (*compptr).component_index = ci;
        (*compptr).DCT_scaled_size = DCTSIZE;
        (*compptr).width_in_blocks = jdiv_round_up(
            (*cinfo).image_width as c_long * (*compptr).h_samp_factor as c_long,
            ((*cinfo).max_h_samp_factor * DCTSIZE) as c_long,
        ) as JDIMENSION;
        (*compptr).height_in_blocks = jdiv_round_up(
            (*cinfo).image_height as c_long * (*compptr).v_samp_factor as c_long,
            ((*cinfo).max_v_samp_factor * DCTSIZE) as c_long,
        ) as JDIMENSION;
        (*compptr).downsampled_width = jdiv_round_up(
            (*cinfo).image_width as c_long * (*compptr).h_samp_factor as c_long,
            (*cinfo).max_h_samp_factor as c_long,
        ) as JDIMENSION;
        (*compptr).downsampled_height = jdiv_round_up(
            (*cinfo).image_height as c_long * (*compptr).v_samp_factor as c_long,
            (*cinfo).max_v_samp_factor as c_long,
        ) as JDIMENSION;
        (*compptr).component_needed = TRUE;
        ci += 1;
        compptr = compptr.offset(1isize)
    }
    (*cinfo).total_iMCU_rows = jdiv_round_up(
        (*cinfo).image_height as c_long,
        ((*cinfo).max_v_samp_factor * DCTSIZE) as c_long,
    ) as JDIMENSION;
}
unsafe extern "C" fn validate_script(mut cinfo: j_compress_ptr) {
    let mut scanptr: *const jpeg_scan_info = 0 as *const jpeg_scan_info;
    let mut scanno: c_int = 0;
    let mut ncomps: c_int = 0;
    let mut ci: c_int = 0;
    let mut coefi: c_int = 0;
    let mut thisi: c_int = 0;
    let mut Ss: c_int = 0;
    let mut Se: c_int = 0;
    let mut Ah: c_int = 0;
    let mut Al: c_int = 0;
    let mut component_sent: [boolean; 10] = [0; 10];
    let mut last_bitpos_ptr: *mut c_int = 0 as *mut c_int;
    let mut last_bitpos: [[c_int; 64]; 10] = [[0; 64]; 10];
    if 0 != (*(*cinfo).master).optimize_scans {
        (*cinfo).progressive_mode = TRUE;
        return;
    }
    if (*cinfo).num_scans <= 0i32 {
        (*(*cinfo).err).msg_code = JERR_BAD_SCAN_SCRIPT as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = 0i32;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    scanptr = (*cinfo).scan_info;
    if (*scanptr).Ss != 0i32 || (*scanptr).Se != DCTSIZE2 - 1i32 {
        (*cinfo).progressive_mode = TRUE;
        last_bitpos_ptr = &mut *(*last_bitpos.as_mut_ptr().offset(0isize))
            .as_mut_ptr()
            .offset(0isize) as *mut c_int;
        ci = 0i32;
        while ci < (*cinfo).num_components {
            coefi = 0i32;
            while coefi < DCTSIZE2 {
                let fresh0 = last_bitpos_ptr;
                last_bitpos_ptr = last_bitpos_ptr.offset(1);
                *fresh0 = -1i32;
                coefi += 1
            }
            ci += 1
        }
    } else {
        (*cinfo).progressive_mode = FALSE;
        ci = 0i32;
        while ci < (*cinfo).num_components {
            component_sent[ci as usize] = FALSE;
            ci += 1
        }
    }
    scanno = 1i32;
    while scanno <= (*cinfo).num_scans {
        ncomps = (*scanptr).comps_in_scan;
        if ncomps <= 0i32 || ncomps > MAX_COMPS_IN_SCAN {
            (*(*cinfo).err).msg_code = JERR_COMPONENT_COUNT as c_int;
            (*(*cinfo).err).msg_parm.i[0usize] = ncomps;
            (*(*cinfo).err).msg_parm.i[1usize] = 4i32;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        ci = 0i32;
        while ci < ncomps {
            thisi = (*scanptr).component_index[ci as usize];
            if thisi < 0i32 || thisi >= (*cinfo).num_components {
                (*(*cinfo).err).msg_code = JERR_BAD_SCAN_SCRIPT as c_int;
                (*(*cinfo).err).msg_parm.i[0usize] = scanno;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if ci > 0i32 && thisi <= (*scanptr).component_index[(ci - 1i32) as usize] {
                (*(*cinfo).err).msg_code = JERR_BAD_SCAN_SCRIPT as c_int;
                (*(*cinfo).err).msg_parm.i[0usize] = scanno;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            ci += 1
        }
        Ss = (*scanptr).Ss;
        Se = (*scanptr).Se;
        Ah = (*scanptr).Ah;
        Al = (*scanptr).Al;
        if 0 != (*cinfo).progressive_mode {
            if Ss < 0i32
                || Ss >= DCTSIZE2
                || Se < Ss
                || Se >= DCTSIZE2
                || Ah < 0i32
                || Ah > MAX_AH_AL
                || Al < 0i32
                || Al > MAX_AH_AL
            {
                (*(*cinfo).err).msg_code = JERR_BAD_PROG_SCRIPT as c_int;
                (*(*cinfo).err).msg_parm.i[0usize] = scanno;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if Ss == 0i32 {
                if Se != 0i32 {
                    (*(*cinfo).err).msg_code = JERR_BAD_PROG_SCRIPT as c_int;
                    (*(*cinfo).err).msg_parm.i[0usize] = scanno;
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer")(
                        cinfo as j_common_ptr
                    );
                }
            } else if ncomps != 1i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_PROG_SCRIPT as c_int;
                (*(*cinfo).err).msg_parm.i[0usize] = scanno;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            ci = 0i32;
            while ci < ncomps {
                last_bitpos_ptr = &mut *(*last_bitpos
                    .as_mut_ptr()
                    .offset(*(*scanptr).component_index.as_ptr().offset(ci as isize) as isize))
                .as_mut_ptr()
                .offset(0isize) as *mut c_int;
                if Ss != 0i32 && *last_bitpos_ptr.offset(0isize) < 0i32 {
                    (*(*cinfo).err).msg_code = JERR_BAD_PROG_SCRIPT as c_int;
                    (*(*cinfo).err).msg_parm.i[0usize] = scanno;
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer")(
                        cinfo as j_common_ptr
                    );
                }
                coefi = Ss;
                while coefi <= Se {
                    if *last_bitpos_ptr.offset(coefi as isize) < 0i32 {
                        if Ah != 0i32 {
                            (*(*cinfo).err).msg_code = JERR_BAD_PROG_SCRIPT as c_int;
                            (*(*cinfo).err).msg_parm.i[0usize] = scanno;
                            (*(*cinfo).err)
                                .error_exit
                                .expect("non-null function pointer")(
                                cinfo as j_common_ptr
                            );
                        }
                    } else if Ah != *last_bitpos_ptr.offset(coefi as isize) || Al != Ah - 1i32 {
                        (*(*cinfo).err).msg_code = JERR_BAD_PROG_SCRIPT as c_int;
                        (*(*cinfo).err).msg_parm.i[0usize] = scanno;
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer")(
                            cinfo as j_common_ptr
                        );
                    }
                    *last_bitpos_ptr.offset(coefi as isize) = Al;
                    coefi += 1
                }
                ci += 1
            }
        } else {
            if Ss != 0i32 || Se != DCTSIZE2 - 1i32 || Ah != 0i32 || Al != 0i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_PROG_SCRIPT as c_int;
                (*(*cinfo).err).msg_parm.i[0usize] = scanno;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            ci = 0i32;
            while ci < ncomps {
                thisi = (*scanptr).component_index[ci as usize];
                if 0 != component_sent[thisi as usize] {
                    (*(*cinfo).err).msg_code = JERR_BAD_SCAN_SCRIPT as c_int;
                    (*(*cinfo).err).msg_parm.i[0usize] = scanno;
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer")(
                        cinfo as j_common_ptr
                    );
                }
                component_sent[thisi as usize] = TRUE;
                ci += 1
            }
        }
        scanptr = scanptr.offset(1isize);
        scanno += 1
    }
    if 0 != (*cinfo).progressive_mode {
        ci = 0i32;
        while ci < (*cinfo).num_components {
            if last_bitpos[ci as usize][0usize] < 0i32 {
                (*(*cinfo).err).msg_code = JERR_MISSING_DATA as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            ci += 1
        }
    } else {
        ci = 0i32;
        while ci < (*cinfo).num_components {
            if 0 == component_sent[ci as usize] {
                (*(*cinfo).err).msg_code = JERR_MISSING_DATA as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            ci += 1
        }
    };
}
/* Rec. ITU-T T.81 | ISO/IEC 10918-1 simply gives the ranges 0..13 for Ah
 * and Al, but that seems wrong: the upper bound ought to depend on data
 * precision.  Perhaps they really meant 0..N+1 for N-bit precision.
 * Here we allow 0..10 for 8-bit data; Al larger than 10 results in
 * out-of-range reconstructed DC values during the first DC scan,
 * which might cause problems for some decoders.
 */
pub const MAX_AH_AL: c_int = 10i32;
/* C_MULTISCAN_FILES_SUPPORTED */
unsafe extern "C" fn select_scan_parameters(mut cinfo: j_compress_ptr) {
    let mut ci: c_int = 0;
    let mut master: my_master_ptr = (*cinfo).master as my_master_ptr;
    if (*master).pass_number < (*master).pass_number_scan_opt_base {
        (*cinfo).comps_in_scan = 1i32;
        if 0 != (*(*cinfo).master).use_scans_in_trellis {
            (*cinfo).cur_comp_info[0usize] = &mut *(*cinfo).comp_info.offset(
                ((*master).pass_number / (4i32 * (*(*cinfo).master).trellis_num_loops)) as isize,
            ) as *mut jpeg_component_info;
            (*cinfo).Ss = if (*master).pass_number % 4i32 < 2i32 {
                1i32
            } else {
                (*(*cinfo).master).trellis_freq_split + 1i32
            };
            (*cinfo).Se = if (*master).pass_number % 4i32 < 2i32 {
                (*(*cinfo).master).trellis_freq_split
            } else {
                DCTSIZE2 - 1i32
            }
        } else {
            (*cinfo).cur_comp_info[0usize] = &mut *(*cinfo).comp_info.offset(
                ((*master).pass_number / (2i32 * (*(*cinfo).master).trellis_num_loops)) as isize,
            ) as *mut jpeg_component_info;
            (*cinfo).Ss = 1i32;
            (*cinfo).Se = DCTSIZE2 - 1i32
        }
    } else if !(*cinfo).scan_info.is_null() {
        let mut scanptr: *const jpeg_scan_info =
            (*cinfo).scan_info.offset((*master).scan_number as isize);
        (*cinfo).comps_in_scan = (*scanptr).comps_in_scan;
        ci = 0i32;
        while ci < (*scanptr).comps_in_scan {
            (*cinfo).cur_comp_info[ci as usize] = &mut *(*cinfo)
                .comp_info
                .offset(*(*scanptr).component_index.as_ptr().offset(ci as isize) as isize)
                as *mut jpeg_component_info;
            ci += 1
        }
        (*cinfo).Ss = (*scanptr).Ss;
        (*cinfo).Se = (*scanptr).Se;
        (*cinfo).Ah = (*scanptr).Ah;
        (*cinfo).Al = (*scanptr).Al;
        if 0 != (*(*cinfo).master).optimize_scans {
            if (*master).scan_number
                >= (*(*cinfo).master).num_scans_luma_dc
                    + 3i32 * (*(*cinfo).master).Al_max_luma
                    + 2i32
                && (*master).scan_number < (*(*cinfo).master).num_scans_luma
            {
                (*cinfo).Al = (*master).best_Al_luma
            }
            if (*master).scan_number
                >= (*(*cinfo).master).num_scans_luma
                    + (*(*cinfo).master).num_scans_chroma_dc
                    + (6i32 * (*(*cinfo).master).Al_max_chroma + 4i32)
                && (*master).scan_number < (*cinfo).num_scans
            {
                (*cinfo).Al = (*master).best_Al_chroma
            }
        }
        (*master).actual_Al[(*master).scan_number as usize] = (*cinfo).Al
    } else {
        if (*cinfo).num_components > MAX_COMPS_IN_SCAN {
            (*(*cinfo).err).msg_code = JERR_COMPONENT_COUNT as c_int;
            (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).num_components;
            (*(*cinfo).err).msg_parm.i[1usize] = 4i32;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        (*cinfo).comps_in_scan = (*cinfo).num_components;
        ci = 0i32;
        while ci < (*cinfo).num_components {
            (*cinfo).cur_comp_info[ci as usize] =
                &mut *(*cinfo).comp_info.offset(ci as isize) as *mut jpeg_component_info;
            ci += 1
        }
        (*cinfo).Ss = 0i32;
        (*cinfo).Se = DCTSIZE2 - 1i32;
        (*cinfo).Ah = 0i32;
        (*cinfo).Al = 0i32
    };
}
unsafe extern "C" fn per_scan_setup(mut cinfo: j_compress_ptr) {
    let mut ci: c_int = 0;
    let mut mcublks: c_int = 0;
    let mut tmp: c_int = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    if (*cinfo).comps_in_scan == 1i32 {
        compptr = (*cinfo).cur_comp_info[0usize];
        (*cinfo).MCUs_per_row = (*compptr).width_in_blocks;
        (*cinfo).MCU_rows_in_scan = (*compptr).height_in_blocks;
        (*compptr).MCU_width = 1i32;
        (*compptr).MCU_height = 1i32;
        (*compptr).MCU_blocks = 1i32;
        (*compptr).MCU_sample_width = DCTSIZE;
        (*compptr).last_col_width = 1i32;
        tmp = (*compptr)
            .height_in_blocks
            .wrapping_rem((*compptr).v_samp_factor as c_uint) as c_int;
        if tmp == 0i32 {
            tmp = (*compptr).v_samp_factor
        }
        (*compptr).last_row_height = tmp;
        (*cinfo).blocks_in_MCU = 1i32;
        (*cinfo).MCU_membership[0usize] = 0i32
    } else {
        if (*cinfo).comps_in_scan <= 0i32 || (*cinfo).comps_in_scan > MAX_COMPS_IN_SCAN {
            (*(*cinfo).err).msg_code = JERR_COMPONENT_COUNT as c_int;
            (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).comps_in_scan;
            (*(*cinfo).err).msg_parm.i[1usize] = 4i32;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        (*cinfo).MCUs_per_row = jdiv_round_up(
            (*cinfo).image_width as c_long,
            ((*cinfo).max_h_samp_factor * DCTSIZE) as c_long,
        ) as JDIMENSION;
        (*cinfo).MCU_rows_in_scan = jdiv_round_up(
            (*cinfo).image_height as c_long,
            ((*cinfo).max_v_samp_factor * DCTSIZE) as c_long,
        ) as JDIMENSION;
        (*cinfo).blocks_in_MCU = 0i32;
        ci = 0i32;
        while ci < (*cinfo).comps_in_scan {
            compptr = (*cinfo).cur_comp_info[ci as usize];
            (*compptr).MCU_width = (*compptr).h_samp_factor;
            (*compptr).MCU_height = (*compptr).v_samp_factor;
            (*compptr).MCU_blocks = (*compptr).MCU_width * (*compptr).MCU_height;
            (*compptr).MCU_sample_width = (*compptr).MCU_width * DCTSIZE;
            tmp = (*compptr)
                .width_in_blocks
                .wrapping_rem((*compptr).MCU_width as c_uint) as c_int;
            if tmp == 0i32 {
                tmp = (*compptr).MCU_width
            }
            (*compptr).last_col_width = tmp;
            tmp = (*compptr)
                .height_in_blocks
                .wrapping_rem((*compptr).MCU_height as c_uint) as c_int;
            if tmp == 0i32 {
                tmp = (*compptr).MCU_height
            }
            (*compptr).last_row_height = tmp;
            mcublks = (*compptr).MCU_blocks;
            if (*cinfo).blocks_in_MCU + mcublks > C_MAX_BLOCKS_IN_MCU {
                (*(*cinfo).err).msg_code = JERR_BAD_MCU_SIZE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            loop {
                let fresh1 = mcublks;
                mcublks = mcublks - 1;
                if !(fresh1 > 0i32) {
                    break;
                }
                let fresh2 = (*cinfo).blocks_in_MCU;
                (*cinfo).blocks_in_MCU = (*cinfo).blocks_in_MCU + 1;
                (*cinfo).MCU_membership[fresh2 as usize] = ci
            }
            ci += 1
        }
    }
    if (*cinfo).restart_in_rows > 0i32 {
        let mut nominal: c_long =
            (*cinfo).restart_in_rows as c_long * (*cinfo).MCUs_per_row as c_long;
        (*cinfo).restart_interval = (if nominal < 65535i64 {
            nominal
        } else {
            65535i64
        }) as c_uint
    };
}
/*
 * Per-pass setup.
 * This is called at the beginning of each pass.  We determine which modules
 * will be active during this pass and give them appropriate start_pass calls.
 * We also set is_last_pass to indicate whether any more passes will be
 * required.
 */
unsafe extern "C" fn prepare_for_pass(mut cinfo: j_compress_ptr) {
    let mut master: my_master_ptr = (*cinfo).master as my_master_ptr;
    (*(*cinfo).master).trellis_passes =
        ((*master).pass_number < (*master).pass_number_scan_opt_base) as c_int;
    let mut current_block_54: u64;
    match (*master).pass_type as c_uint {
        0 => {
            select_scan_parameters(cinfo);
            per_scan_setup(cinfo);
            if 0 == (*cinfo).raw_data_in {
                (*(*cinfo).cconvert)
                    .start_pass
                    .expect("non-null function pointer")(cinfo);
                (*(*cinfo).downsample)
                    .start_pass
                    .expect("non-null function pointer")(cinfo);
                (*(*cinfo).prep)
                    .start_pass
                    .expect("non-null function pointer")(cinfo, JBUF_PASS_THRU);
            }
            (*(*cinfo).fdct)
                .start_pass
                .expect("non-null function pointer")(cinfo);
            (*(*cinfo).entropy)
                .start_pass
                .expect("non-null function pointer")(
                cinfo,
                ((0 != (*cinfo).optimize_coding || 0 != (*(*cinfo).master).trellis_quant)
                    && 0 == (*cinfo).arith_code) as c_int,
            );
            (*(*cinfo).coef)
                .start_pass
                .expect("non-null function pointer")(
                cinfo,
                (if (*master).total_passes > 1i32 {
                    JBUF_SAVE_AND_PASS as c_int
                } else {
                    JBUF_PASS_THRU as c_int
                }) as J_BUF_MODE,
            );
            (*(*cinfo).main)
                .start_pass
                .expect("non-null function pointer")(cinfo, JBUF_PASS_THRU);
            if 0 != (*cinfo).optimize_coding || 0 != (*(*cinfo).master).trellis_quant {
                (*master).pub_0.call_pass_startup = FALSE
            } else {
                (*master).pub_0.call_pass_startup = TRUE
            }
            current_block_54 = 2500484646272006982;
        }
        1 => {
            select_scan_parameters(cinfo);
            per_scan_setup(cinfo);
            if (*cinfo).Ss != 0i32 || (*cinfo).Ah == 0i32 || 0 != (*cinfo).arith_code {
                (*(*cinfo).entropy)
                    .start_pass
                    .expect("non-null function pointer")(cinfo, TRUE);
                (*(*cinfo).coef)
                    .start_pass
                    .expect("non-null function pointer")(cinfo, JBUF_CRANK_DEST);
                (*master).pub_0.call_pass_startup = FALSE;
                current_block_54 = 2500484646272006982;
            } else {
                (*master).pass_type = output_pass;
                (*master).pass_number += 1;
                /*FALLTHROUGH*/
                current_block_54 = 12235309447780442549;
            }
        }
        2 => {
            current_block_54 = 12235309447780442549;
        }
        3 => {
            if (*master).pass_number
                % ((*cinfo).num_components
                    * (if 0 != (*(*cinfo).master).use_scans_in_trellis {
                        4i32
                    } else {
                        2i32
                    }))
                == 1i32
                && 0 != (*(*cinfo).master).trellis_q_opt
            {
                let mut i: c_int = 0;
                let mut j: c_int = 0;
                i = 0i32;
                while i < NUM_QUANT_TBLS {
                    j = 1i32;
                    while j < DCTSIZE2 {
                        (*(*cinfo).master).norm_src[i as usize][j as usize] = 0.0f64;
                        (*(*cinfo).master).norm_coef[i as usize][j as usize] = 0.0f64;
                        j += 1
                    }
                    i += 1
                }
            }
            (*(*cinfo).entropy)
                .start_pass
                .expect("non-null function pointer")(
                cinfo, (0 == (*cinfo).arith_code) as c_int
            );
            (*(*cinfo).coef)
                .start_pass
                .expect("non-null function pointer")(cinfo, JBUF_REQUANT);
            (*master).pub_0.call_pass_startup = FALSE;
            current_block_54 = 2500484646272006982;
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_NOT_COMPILED as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            current_block_54 = 2500484646272006982;
        }
    }
    match current_block_54 {
        12235309447780442549 => {
            if 0 == (*cinfo).optimize_coding {
                select_scan_parameters(cinfo);
                per_scan_setup(cinfo);
            }
            if 0 != (*(*cinfo).master).optimize_scans {
                (*master).saved_dest = (*cinfo).dest;
                (*cinfo).dest = NULL as *mut jpeg_destination_mgr;
                (*master).scan_size[(*master).scan_number as usize] = 0i32 as c_ulong;
                jpeg_mem_dest_internal(
                    cinfo,
                    &mut *(*master)
                        .scan_buffer
                        .as_mut_ptr()
                        .offset((*master).scan_number as isize),
                    &mut *(*master)
                        .scan_size
                        .as_mut_ptr()
                        .offset((*master).scan_number as isize),
                    JPOOL_IMAGE,
                );
                (*(*cinfo).dest)
                    .init_destination
                    .expect("non-null function pointer")(cinfo);
            }
            (*(*cinfo).entropy)
                .start_pass
                .expect("non-null function pointer")(cinfo, FALSE);
            (*(*cinfo).coef)
                .start_pass
                .expect("non-null function pointer")(cinfo, JBUF_CRANK_DEST);
            if (*master).scan_number == 0i32 {
                (*(*cinfo).marker)
                    .write_frame_header
                    .expect("non-null function pointer")(cinfo);
            }
            (*(*cinfo).marker)
                .write_scan_header
                .expect("non-null function pointer")(cinfo);
            (*master).pub_0.call_pass_startup = FALSE
        }
        _ => {}
    }
    (*master).pub_0.is_last_pass =
        ((*master).pass_number == (*master).total_passes - 1i32) as c_int;
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
unsafe extern "C" fn pass_startup(mut cinfo: j_compress_ptr) {
    (*(*cinfo).master).call_pass_startup = FALSE;
    (*(*cinfo).marker)
        .write_frame_header
        .expect("non-null function pointer")(cinfo);
    (*(*cinfo).marker)
        .write_scan_header
        .expect("non-null function pointer")(cinfo);
}
unsafe extern "C" fn copy_buffer(mut cinfo: j_compress_ptr, mut scan_idx: c_int) {
    let mut master: my_master_ptr = (*cinfo).master as my_master_ptr;
    let mut size: c_ulong = (*master).scan_size[scan_idx as usize];
    let mut src: *mut c_uchar = (*master).scan_buffer[scan_idx as usize];
    let mut i: c_int = 0;
    if (*(*cinfo).err).trace_level > 0i32 {
        fprintf(stderr, b"SCAN \x00" as *const u8 as *const c_char);
        i = 0i32;
        while i < (*(*cinfo).scan_info.offset(scan_idx as isize)).comps_in_scan {
            fprintf(
                stderr,
                b"%s%d\x00" as *const u8 as *const c_char,
                if i == 0i32 {
                    b"\x00" as *const u8 as *const c_char
                } else {
                    b",\x00" as *const u8 as *const c_char
                },
                (*(*cinfo).scan_info.offset(scan_idx as isize)).component_index[i as usize],
            );
            i += 1
        }
        fprintf(
            stderr,
            b": %d %d\x00" as *const u8 as *const c_char,
            (*(*cinfo).scan_info.offset(scan_idx as isize)).Ss,
            (*(*cinfo).scan_info.offset(scan_idx as isize)).Se,
        );
        fprintf(
            stderr,
            b" %d %d\x00" as *const u8 as *const c_char,
            (*(*cinfo).scan_info.offset(scan_idx as isize)).Ah,
            (*master).actual_Al[scan_idx as usize],
        );
        fprintf(stderr, b"\n\x00" as *const u8 as *const c_char);
    }
    while size >= (*(*cinfo).dest).free_in_buffer {
        memcpy(
            (*(*cinfo).dest).next_output_byte as *mut c_void,
            src as *const c_void,
            (*(*cinfo).dest).free_in_buffer,
        );
        src = src.offset((*(*cinfo).dest).free_in_buffer as isize);
        size = size.wrapping_sub((*(*cinfo).dest).free_in_buffer);
        (*(*cinfo).dest).next_output_byte = (*(*cinfo).dest)
            .next_output_byte
            .offset((*(*cinfo).dest).free_in_buffer as isize);
        (*(*cinfo).dest).free_in_buffer = 0i32 as size_t;
        if 0 == (*(*cinfo).dest)
            .empty_output_buffer
            .expect("non-null function pointer")(cinfo)
        {
            (*(*cinfo).err).msg_code = JERR_UNSUPPORTED_SUSPEND as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    }
    memcpy(
        (*(*cinfo).dest).next_output_byte as *mut c_void,
        src as *const c_void,
        size,
    );
    (*(*cinfo).dest).next_output_byte = (*(*cinfo).dest).next_output_byte.offset(size as isize);
    (*(*cinfo).dest).free_in_buffer =
        ((*(*cinfo).dest).free_in_buffer as c_ulong).wrapping_sub(size) as size_t as size_t;
}
unsafe extern "C" fn select_scans(mut cinfo: j_compress_ptr, mut next_scan_number: c_int) {
    let mut master: my_master_ptr = (*cinfo).master as my_master_ptr;
    let mut base_scan_idx: c_int = 0i32;
    let mut luma_freq_split_scan_start: c_int =
        (*(*cinfo).master).num_scans_luma_dc + 3i32 * (*(*cinfo).master).Al_max_luma + 2i32;
    let mut chroma_freq_split_scan_start: c_int = (*(*cinfo).master).num_scans_luma
        + (*(*cinfo).master).num_scans_chroma_dc
        + (6i32 * (*(*cinfo).master).Al_max_chroma + 4i32);
    let mut passes_per_scan: c_int = if 0 != (*cinfo).optimize_coding {
        2i32
    } else {
        1i32
    };
    if next_scan_number > 1i32 && next_scan_number <= luma_freq_split_scan_start {
        if (next_scan_number - 1i32) % 3i32 == 2i32 {
            let mut Al: c_int = (next_scan_number - 1i32) / 3i32;
            let mut i: c_int = 0;
            let mut cost: c_ulong = 0i32 as c_ulong;
            cost = cost.wrapping_add((*master).scan_size[(next_scan_number - 2i32) as usize]);
            cost = cost.wrapping_add((*master).scan_size[(next_scan_number - 1i32) as usize]);
            i = 0i32;
            while i < Al {
                cost = cost.wrapping_add((*master).scan_size[(3i32 + 3i32 * i) as usize]);
                i += 1
            }
            if Al == 0i32 || cost < (*master).best_cost {
                (*master).best_cost = cost;
                (*master).best_Al_luma = Al
            } else {
                (*master).scan_number = luma_freq_split_scan_start - 1i32;
                (*master).pass_number = passes_per_scan * ((*master).scan_number + 1i32) - 1i32
                    + (*master).pass_number_scan_opt_base
            }
        }
    } else if next_scan_number > luma_freq_split_scan_start
        && next_scan_number <= (*(*cinfo).master).num_scans_luma
    {
        if next_scan_number == luma_freq_split_scan_start + 1i32 {
            (*master).best_freq_split_idx_luma = 0i32;
            (*master).best_cost = (*master).scan_size[(next_scan_number - 1i32) as usize]
        } else if (next_scan_number - luma_freq_split_scan_start) % 2i32 == 1i32 {
            let mut idx: c_int = next_scan_number - luma_freq_split_scan_start >> 1i32;
            let mut cost_0: c_ulong = 0i32 as c_ulong;
            cost_0 = cost_0.wrapping_add((*master).scan_size[(next_scan_number - 2i32) as usize]);
            cost_0 = cost_0.wrapping_add((*master).scan_size[(next_scan_number - 1i32) as usize]);
            if cost_0 < (*master).best_cost {
                (*master).best_cost = cost_0;
                (*master).best_freq_split_idx_luma = idx
            }
            if idx == 2i32 && (*master).best_freq_split_idx_luma == 0i32
                || idx == 3i32 && (*master).best_freq_split_idx_luma != 2i32
                || idx == 4i32 && (*master).best_freq_split_idx_luma != 4i32
            {
                (*master).scan_number = (*(*cinfo).master).num_scans_luma - 1i32;
                (*master).pass_number = passes_per_scan * ((*master).scan_number + 1i32) - 1i32
                    + (*master).pass_number_scan_opt_base;
                (*master).pub_0.is_last_pass =
                    ((*master).pass_number == (*master).total_passes - 1i32) as c_int
            }
        }
    } else if (*cinfo).num_scans > (*(*cinfo).master).num_scans_luma {
        if next_scan_number
            == (*(*cinfo).master).num_scans_luma + (*(*cinfo).master).num_scans_chroma_dc
        {
            base_scan_idx = (*(*cinfo).master).num_scans_luma;
            (*master).interleave_chroma_dc = ((*master).scan_size[base_scan_idx as usize]
                <= (*master).scan_size[(base_scan_idx + 1i32) as usize]
                    .wrapping_add((*master).scan_size[(base_scan_idx + 2i32) as usize]))
                as c_int
        } else if next_scan_number
            > (*(*cinfo).master).num_scans_luma + (*(*cinfo).master).num_scans_chroma_dc
            && next_scan_number <= chroma_freq_split_scan_start
        {
            base_scan_idx =
                (*(*cinfo).master).num_scans_luma + (*(*cinfo).master).num_scans_chroma_dc;
            if (next_scan_number - base_scan_idx) % 6i32 == 4i32 {
                let mut Al_0: c_int = (next_scan_number - base_scan_idx) / 6i32;
                let mut i_0: c_int = 0;
                let mut cost_1: c_ulong = 0i32 as c_ulong;
                cost_1 =
                    cost_1.wrapping_add((*master).scan_size[(next_scan_number - 4i32) as usize]);
                cost_1 =
                    cost_1.wrapping_add((*master).scan_size[(next_scan_number - 3i32) as usize]);
                cost_1 =
                    cost_1.wrapping_add((*master).scan_size[(next_scan_number - 2i32) as usize]);
                cost_1 =
                    cost_1.wrapping_add((*master).scan_size[(next_scan_number - 1i32) as usize]);
                i_0 = 0i32;
                while i_0 < Al_0 {
                    cost_1 = cost_1.wrapping_add(
                        (*master).scan_size[(base_scan_idx + 4i32 + 6i32 * i_0) as usize],
                    );
                    cost_1 = cost_1.wrapping_add(
                        (*master).scan_size[(base_scan_idx + 5i32 + 6i32 * i_0) as usize],
                    );
                    i_0 += 1
                }
                if Al_0 == 0i32 || cost_1 < (*master).best_cost {
                    (*master).best_cost = cost_1;
                    (*master).best_Al_chroma = Al_0
                } else {
                    (*master).scan_number = chroma_freq_split_scan_start - 1i32;
                    (*master).pass_number = passes_per_scan * ((*master).scan_number + 1i32) - 1i32
                        + (*master).pass_number_scan_opt_base
                }
            }
        } else if next_scan_number > chroma_freq_split_scan_start
            && next_scan_number <= (*cinfo).num_scans
        {
            if next_scan_number == chroma_freq_split_scan_start + 2i32 {
                (*master).best_freq_split_idx_chroma = 0i32;
                (*master).best_cost = (*master).scan_size[(next_scan_number - 2i32) as usize];
                (*master).best_cost = (*master)
                    .best_cost
                    .wrapping_add((*master).scan_size[(next_scan_number - 1i32) as usize])
            } else if (next_scan_number - chroma_freq_split_scan_start) % 4i32 == 2i32 {
                let mut idx_0: c_int = next_scan_number - chroma_freq_split_scan_start >> 2i32;
                let mut cost_2: c_ulong = 0i32 as c_ulong;
                cost_2 =
                    cost_2.wrapping_add((*master).scan_size[(next_scan_number - 4i32) as usize]);
                cost_2 =
                    cost_2.wrapping_add((*master).scan_size[(next_scan_number - 3i32) as usize]);
                cost_2 =
                    cost_2.wrapping_add((*master).scan_size[(next_scan_number - 2i32) as usize]);
                cost_2 =
                    cost_2.wrapping_add((*master).scan_size[(next_scan_number - 1i32) as usize]);
                if cost_2 < (*master).best_cost {
                    (*master).best_cost = cost_2;
                    (*master).best_freq_split_idx_chroma = idx_0
                }
                if idx_0 == 2i32 && (*master).best_freq_split_idx_chroma == 0i32
                    || idx_0 == 3i32 && (*master).best_freq_split_idx_chroma != 2i32
                    || idx_0 == 4i32 && (*master).best_freq_split_idx_chroma != 4i32
                {
                    (*master).scan_number = (*cinfo).num_scans - 1i32;
                    (*master).pass_number = passes_per_scan * ((*master).scan_number + 1i32) - 1i32
                        + (*master).pass_number_scan_opt_base;
                    (*master).pub_0.is_last_pass =
                        ((*master).pass_number == (*master).total_passes - 1i32) as c_int
                }
            }
        }
    }
    if (*master).scan_number == (*cinfo).num_scans - 1i32 {
        let mut i_1: c_int = 0;
        let mut Al_1: c_int = 0;
        let mut min_Al: c_int = if (*master).best_Al_luma < (*master).best_Al_chroma {
            (*master).best_Al_luma
        } else {
            (*master).best_Al_chroma
        };
        copy_buffer(cinfo, 0i32);
        if (*cinfo).num_scans > (*(*cinfo).master).num_scans_luma
            && (*(*cinfo).master).dc_scan_opt_mode != 0i32
        {
            base_scan_idx = (*(*cinfo).master).num_scans_luma;
            if 0 != (*master).interleave_chroma_dc && (*(*cinfo).master).dc_scan_opt_mode != 1i32 {
                copy_buffer(cinfo, base_scan_idx);
            } else {
                copy_buffer(cinfo, base_scan_idx + 1i32);
                copy_buffer(cinfo, base_scan_idx + 2i32);
            }
        }
        if (*master).best_freq_split_idx_luma == 0i32 {
            copy_buffer(cinfo, luma_freq_split_scan_start);
        } else {
            copy_buffer(
                cinfo,
                luma_freq_split_scan_start
                    + 2i32 * ((*master).best_freq_split_idx_luma - 1i32)
                    + 1i32,
            );
            copy_buffer(
                cinfo,
                luma_freq_split_scan_start
                    + 2i32 * ((*master).best_freq_split_idx_luma - 1i32)
                    + 2i32,
            );
        }
        Al_1 = (*master).best_Al_luma - 1i32;
        while Al_1 >= min_Al {
            copy_buffer(cinfo, 3i32 + 3i32 * Al_1);
            Al_1 -= 1
        }
        if (*cinfo).num_scans > (*(*cinfo).master).num_scans_luma {
            if (*master).best_freq_split_idx_chroma == 0i32 {
                copy_buffer(cinfo, chroma_freq_split_scan_start);
                copy_buffer(cinfo, chroma_freq_split_scan_start + 1i32);
            } else {
                copy_buffer(
                    cinfo,
                    chroma_freq_split_scan_start
                        + 4i32 * ((*master).best_freq_split_idx_chroma - 1i32)
                        + 2i32,
                );
                copy_buffer(
                    cinfo,
                    chroma_freq_split_scan_start
                        + 4i32 * ((*master).best_freq_split_idx_chroma - 1i32)
                        + 3i32,
                );
                copy_buffer(
                    cinfo,
                    chroma_freq_split_scan_start
                        + 4i32 * ((*master).best_freq_split_idx_chroma - 1i32)
                        + 4i32,
                );
                copy_buffer(
                    cinfo,
                    chroma_freq_split_scan_start
                        + 4i32 * ((*master).best_freq_split_idx_chroma - 1i32)
                        + 5i32,
                );
            }
            base_scan_idx =
                (*(*cinfo).master).num_scans_luma + (*(*cinfo).master).num_scans_chroma_dc;
            Al_1 = (*master).best_Al_chroma - 1i32;
            while Al_1 >= min_Al {
                copy_buffer(cinfo, base_scan_idx + 6i32 * Al_1 + 4i32);
                copy_buffer(cinfo, base_scan_idx + 6i32 * Al_1 + 5i32);
                Al_1 -= 1
            }
        }
        Al_1 = min_Al - 1i32;
        while Al_1 >= 0i32 {
            copy_buffer(cinfo, 3i32 + 3i32 * Al_1);
            if (*cinfo).num_scans > (*(*cinfo).master).num_scans_luma {
                copy_buffer(cinfo, base_scan_idx + 6i32 * Al_1 + 4i32);
                copy_buffer(cinfo, base_scan_idx + 6i32 * Al_1 + 5i32);
            }
            Al_1 -= 1
        }
        i_1 = 0i32;
        while i_1 < (*cinfo).num_scans {
            if !(*master).scan_buffer[i_1 as usize].is_null() {
                free((*master).scan_buffer[i_1 as usize] as *mut c_void);
            }
            i_1 += 1
        }
    };
}
/*
 * Finish up at end of pass.
 */
unsafe extern "C" fn finish_pass_master(mut cinfo: j_compress_ptr) {
    let mut master: my_master_ptr = (*cinfo).master as my_master_ptr;
    (*(*cinfo).entropy)
        .finish_pass
        .expect("non-null function pointer")(cinfo);
    match (*master).pass_type as c_uint {
        0 => {
            if 0 != (*(*cinfo).master).trellis_quant {
                (*master).pass_type = trellis_pass
            } else {
                (*master).pass_type = output_pass;
                if 0 == (*cinfo).optimize_coding {
                    (*master).scan_number += 1
                }
            }
        }
        1 => {
            (*master).pass_type =
                (if (*master).pass_number < (*master).pass_number_scan_opt_base - 1i32 {
                    trellis_pass as c_int
                } else {
                    output_pass as c_int
                }) as c_pass_type
        }
        2 => {
            if 0 != (*cinfo).optimize_coding {
                (*master).pass_type = huff_opt_pass
            }
            if 0 != (*(*cinfo).master).optimize_scans {
                (*(*cinfo).dest)
                    .term_destination
                    .expect("non-null function pointer")(cinfo);
                (*cinfo).dest = (*master).saved_dest;
                select_scans(cinfo, (*master).scan_number + 1i32);
            }
            (*master).scan_number += 1
        }
        3 => {
            if 0 != (*cinfo).optimize_coding {
                (*master).pass_type = huff_opt_pass
            } else {
                (*master).pass_type =
                    (if (*master).pass_number < (*master).pass_number_scan_opt_base - 1i32 {
                        trellis_pass as c_int
                    } else {
                        output_pass as c_int
                    }) as c_pass_type
            }
            if ((*master).pass_number + 1i32)
                % ((*cinfo).num_components
                    * (if 0 != (*(*cinfo).master).use_scans_in_trellis {
                        4i32
                    } else {
                        2i32
                    }))
                == 0i32
                && 0 != (*(*cinfo).master).trellis_q_opt
            {
                let mut i: c_int = 0;
                let mut j: c_int = 0;
                i = 0i32;
                while i < NUM_QUANT_TBLS {
                    j = 1i32;
                    while j < DCTSIZE2 {
                        if (*(*cinfo).master).norm_coef[i as usize][j as usize] != 0.0f64 {
                            let mut q: c_int = ((*(*cinfo).master).norm_src[i as usize][j as usize]
                                / (*(*cinfo).master).norm_coef[i as usize][j as usize]
                                + 0.5f64) as c_int;
                            if q > 254i32 {
                                q = 254i32
                            }
                            if q < 1i32 {
                                q = 1i32
                            }
                            (*(*cinfo).quant_tbl_ptrs[i as usize]).quantval[j as usize] =
                                q as UINT16
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
    mut cinfo: j_compress_ptr,
    mut transcode_only: boolean,
) {
    let mut master: my_master_ptr = (*cinfo).master as my_master_ptr;
    (*master).pub_0.prepare_for_pass =
        Some(prepare_for_pass as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    (*master).pub_0.pass_startup =
        Some(pass_startup as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    (*master).pub_0.finish_pass =
        Some(finish_pass_master as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    (*master).pub_0.is_last_pass = FALSE;
    (*master).pub_0.call_pass_startup = FALSE;
    initial_setup(cinfo, transcode_only);
    if !(*cinfo).scan_info.is_null() {
        validate_script(cinfo);
    } else {
        (*cinfo).progressive_mode = FALSE;
        (*cinfo).num_scans = 1i32
    }
    if 0 != (*cinfo).progressive_mode && 0 == (*cinfo).arith_code {
        (*cinfo).optimize_coding = TRUE
    }
    if 0 != transcode_only {
        if 0 != (*cinfo).optimize_coding {
            (*master).pass_type = huff_opt_pass
        } else {
            (*master).pass_type = output_pass
        }
    } else {
        (*master).pass_type = main_pass
    }
    (*master).scan_number = 0i32;
    (*master).pass_number = 0i32;
    if 0 != (*cinfo).optimize_coding {
        (*master).total_passes = (*cinfo).num_scans * 2i32
    } else {
        (*master).total_passes = (*cinfo).num_scans
    }
    (*master).jpeg_version =
        b"mozjpeg version 4.0.0 (build 20190626)\x00" as *const u8 as *const c_char;
    (*master).pass_number_scan_opt_base = 0i32;
    if 0 != (*(*cinfo).master).trellis_quant {
        if 0 != (*cinfo).optimize_coding {
            (*master).pass_number_scan_opt_base = (if 0 != (*(*cinfo).master).use_scans_in_trellis {
                4i32
            } else {
                2i32
            }) * (*cinfo).num_components
                * (*(*cinfo).master).trellis_num_loops
        } else {
            (*master).pass_number_scan_opt_base = (if 0 != (*(*cinfo).master).use_scans_in_trellis {
                2i32
            } else {
                1i32
            }) * (*cinfo).num_components
                * (*(*cinfo).master).trellis_num_loops
                + 1i32
        }
        (*master).total_passes += (*master).pass_number_scan_opt_base
    }
    if 0 != (*(*cinfo).master).optimize_scans {
        let mut i: c_int = 0;
        (*master).best_Al_chroma = 0i32;
        i = 0i32;
        while i < (*cinfo).num_scans {
            (*master).scan_buffer[i as usize] = NULL as *mut c_uchar;
            i += 1
        }
    };
}
