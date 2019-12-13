// =============== BEGIN jdmainct_h ================

/*
 * jdmainct.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 */

/* Private buffer controller object */

/* public fields */

/* Pointer to allocated workspace (M or M+2 row groups). */

/* Have we gotten an iMCU row from decoder? */

/* counts row groups output to postprocessor */

/* Remaining fields are only used in the context case. */

/* These are the master pointers to the funny-order pointer lists. */

/* pointers to weird pointer lists */

/* indicates which pointer set is now in use */

/* process_data state machine status */

/* row groups available to postprocessor */

/* counts iMCU rows to detect image top/bot */
pub type my_main_ptr = *mut crate::src::jdmainct::my_main_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_main_controller {
    pub pub_0: crate::jpegint_h::jpeg_d_main_controller,
    pub buffer: [crate::jpeglib_h::JSAMPARRAY; 10],
    pub buffer_full: crate::jmorecfg_h::boolean,
    pub rowgroup_ctr: crate::jmorecfg_h::JDIMENSION,
    pub xbuffer: [crate::jpeglib_h::JSAMPIMAGE; 2],
    pub whichptr: libc::c_int,
    pub context_state: libc::c_int,
    pub rowgroups_avail: crate::jmorecfg_h::JDIMENSION,
    pub iMCU_row_ctr: crate::jmorecfg_h::JDIMENSION,
}
/* context_state values: */

pub const CTX_PREPARE_FOR_IMCU: libc::c_int = 0 as libc::c_int;
use ::libc;

#[c2rust::header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/jdmainct.h:20"]
pub mod jdmainct_h {

    /* feeding iMCU to postprocessor */
    /* feeding postponed row group */

    pub unsafe extern "C" fn set_wraparound_pointers(
        mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    )
    /* Set up the "wraparound" pointers at top and bottom of the pointer lists.
     * This changes the pointer list state from top-of-image to the normal state.
     */
    {
        let mut main_ptr: crate::src::jdmainct::my_main_ptr =
            (*cinfo).main as crate::src::jdmainct::my_main_ptr; /* height of a row group of component */
        let mut ci: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut rgroup: libc::c_int = 0;
        let mut M: libc::c_int = (*cinfo).min_DCT_scaled_size;
        let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
            0 as *mut crate::jpeglib_h::jpeg_component_info;
        let mut xbuf0: crate::jpeglib_h::JSAMPARRAY = 0 as *mut crate::jpeglib_h::JSAMPROW;
        let mut xbuf1: crate::jpeglib_h::JSAMPARRAY = 0 as *mut crate::jpeglib_h::JSAMPROW;
        ci = 0 as libc::c_int;
        compptr = (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
            rgroup = (*compptr).v_samp_factor * (*compptr).DCT_scaled_size
                / (*cinfo).min_DCT_scaled_size;
            xbuf0 = *(*main_ptr).xbuffer[0 as libc::c_int as usize].offset(ci as isize);
            xbuf1 = *(*main_ptr).xbuffer[1 as libc::c_int as usize].offset(ci as isize);
            i = 0 as libc::c_int;
            while i < rgroup {
                let ref mut fresh0 = *xbuf0.offset((i - rgroup) as isize);
                *fresh0 = *xbuf0.offset((rgroup * (M + 1 as libc::c_int) + i) as isize);
                let ref mut fresh1 = *xbuf1.offset((i - rgroup) as isize);
                *fresh1 = *xbuf1.offset((rgroup * (M + 1 as libc::c_int) + i) as isize);
                let ref mut fresh2 = *xbuf0.offset((rgroup * (M + 2 as libc::c_int) + i) as isize);
                *fresh2 = *xbuf0.offset(i as isize);
                let ref mut fresh3 = *xbuf1.offset((rgroup * (M + 2 as libc::c_int) + i) as isize);
                *fresh3 = *xbuf1.offset(i as isize);
                i += 1
            }
            ci += 1;
            compptr = compptr.offset(1)
        }
    }

    use crate::jpeglib_h::jpeg_component_info;
    use crate::jpeglib_h::JSAMPARRAY;
    use crate::jpeglib_h::JSAMPROW;
}

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
pub use crate::src::jdmainct::jdmainct_h::set_wraparound_pointers;
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
pub use crate::stdlib::C2RustUnnamed_0;

unsafe extern "C" fn alloc_funny_pointers(mut cinfo: crate::jpeglib_h::j_decompress_ptr)
/* Allocate space for the funny pointer lists.
 * This is done only once, not once per pass.
 */
{
    let mut main_ptr: crate::src::jdmainct::my_main_ptr =
        (*cinfo).main as crate::src::jdmainct::my_main_ptr;
    let mut ci: libc::c_int = 0;
    let mut rgroup: libc::c_int = 0;
    let mut M: libc::c_int = (*cinfo).min_DCT_scaled_size;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    let mut xbuf: crate::jpeglib_h::JSAMPARRAY = 0 as *mut crate::jpeglib_h::JSAMPROW;
    /* Get top-level space for component array pointers.
     * We alloc both arrays with one call to save a few cycles.
     */
    (*main_ptr).xbuffer[0 as libc::c_int as usize] = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        (((*cinfo).num_components * 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::jpeglib_h::JSAMPARRAY>() as libc::c_ulong),
    ) as crate::jpeglib_h::JSAMPIMAGE; /* height of a row group of component */
    (*main_ptr).xbuffer[1 as libc::c_int as usize] =
        (*main_ptr).xbuffer[0 as libc::c_int as usize].offset((*cinfo).num_components as isize);
    ci = 0 as libc::c_int;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        rgroup =
            (*compptr).v_samp_factor * (*compptr).DCT_scaled_size / (*cinfo).min_DCT_scaled_size;
        /* Get space for pointer lists --- M+4 row groups in each list.
         * We alloc both pointer lists with one call to save a few cycles.
         */
        xbuf = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            ((2 as libc::c_int * (rgroup * (M + 4 as libc::c_int))) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::jpeglib_h::JSAMPROW>() as libc::c_ulong),
        ) as crate::jpeglib_h::JSAMPARRAY; /* want one row group at negative offsets */
        xbuf = xbuf.offset(rgroup as isize);
        let ref mut fresh4 = *(*main_ptr).xbuffer[0 as libc::c_int as usize].offset(ci as isize);
        *fresh4 = xbuf;
        xbuf = xbuf.offset((rgroup * (M + 4 as libc::c_int)) as isize);
        let ref mut fresh5 = *(*main_ptr).xbuffer[1 as libc::c_int as usize].offset(ci as isize);
        *fresh5 = xbuf;
        ci += 1;
        compptr = compptr.offset(1)
    }
}

unsafe extern "C" fn make_funny_pointers(mut cinfo: crate::jpeglib_h::j_decompress_ptr)
/* Create the funny pointer lists discussed in the comments above.
 * The actual workspace is already allocated (in main_ptr->buffer),
 * and the space for the pointer lists is allocated too.
 * This routine just fills in the curiously ordered lists.
 * This will be repeated at the beginning of each pass.
 */
{
    let mut main_ptr: crate::src::jdmainct::my_main_ptr =
        (*cinfo).main as crate::src::jdmainct::my_main_ptr; /* height of a row group of component */
    let mut ci: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut rgroup: libc::c_int = 0;
    let mut M: libc::c_int = (*cinfo).min_DCT_scaled_size;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    let mut buf: crate::jpeglib_h::JSAMPARRAY = 0 as *mut crate::jpeglib_h::JSAMPROW;
    let mut xbuf0: crate::jpeglib_h::JSAMPARRAY = 0 as *mut crate::jpeglib_h::JSAMPROW;
    let mut xbuf1: crate::jpeglib_h::JSAMPARRAY = 0 as *mut crate::jpeglib_h::JSAMPROW;
    ci = 0 as libc::c_int;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        rgroup =
            (*compptr).v_samp_factor * (*compptr).DCT_scaled_size / (*cinfo).min_DCT_scaled_size;
        xbuf0 = *(*main_ptr).xbuffer[0 as libc::c_int as usize].offset(ci as isize);
        xbuf1 = *(*main_ptr).xbuffer[1 as libc::c_int as usize].offset(ci as isize);
        /* First copy the workspace pointers as-is */
        buf = (*main_ptr).buffer[ci as usize];
        i = 0 as libc::c_int;
        while i < rgroup * (M + 2 as libc::c_int) {
            let ref mut fresh6 = *xbuf1.offset(i as isize);
            *fresh6 = *buf.offset(i as isize);
            let ref mut fresh7 = *xbuf0.offset(i as isize);
            *fresh7 = *fresh6;
            i += 1
        }
        /* In the second list, put the last four row groups in swapped order */
        i = 0 as libc::c_int;
        while i < rgroup * 2 as libc::c_int {
            let ref mut fresh8 = *xbuf1.offset((rgroup * (M - 2 as libc::c_int) + i) as isize);
            *fresh8 = *buf.offset((rgroup * M + i) as isize);
            let ref mut fresh9 = *xbuf1.offset((rgroup * M + i) as isize);
            *fresh9 = *buf.offset((rgroup * (M - 2 as libc::c_int) + i) as isize);
            i += 1
        }
        /* The wraparound pointers at top and bottom will be filled later
         * (see set_wraparound_pointers, below).  Initially we want the "above"
         * pointers to duplicate the first actual data line.  This only needs
         * to happen in xbuffer[0].
         */
        i = 0 as libc::c_int;
        while i < rgroup {
            let ref mut fresh10 = *xbuf0.offset((i - rgroup) as isize);
            *fresh10 = *xbuf0.offset(0 as libc::c_int as isize);
            i += 1
        }
        ci += 1;
        compptr = compptr.offset(1)
    }
}

unsafe extern "C" fn set_bottom_pointers(mut cinfo: crate::jpeglib_h::j_decompress_ptr)
/* Change the pointer lists to duplicate the last sample row at the bottom
 * of the image.  whichptr indicates which xbuffer holds the final iMCU row.
 * Also sets rowgroups_avail to indicate number of nondummy row groups in row.
 */
{
    let mut main_ptr: crate::src::jdmainct::my_main_ptr =
        (*cinfo).main as crate::src::jdmainct::my_main_ptr;
    let mut ci: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut rgroup: libc::c_int = 0;
    let mut iMCUheight: libc::c_int = 0;
    let mut rows_left: libc::c_int = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    let mut xbuf: crate::jpeglib_h::JSAMPARRAY = 0 as *mut crate::jpeglib_h::JSAMPROW;
    ci = 0 as libc::c_int;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Count sample rows in one iMCU row and in one row group */
        iMCUheight = (*compptr).v_samp_factor * (*compptr).DCT_scaled_size;
        rgroup = iMCUheight / (*cinfo).min_DCT_scaled_size;
        /* Count nondummy sample rows remaining for this component */
        rows_left = (*compptr)
            .downsampled_height
            .wrapping_rem(iMCUheight as crate::jmorecfg_h::JDIMENSION)
            as libc::c_int;
        if rows_left == 0 as libc::c_int {
            rows_left = iMCUheight
        }
        /* Count nondummy row groups.  Should get same answer for each component,
         * so we need only do it once.
         */
        if ci == 0 as libc::c_int {
            (*main_ptr).rowgroups_avail = ((rows_left - 1 as libc::c_int) / rgroup
                + 1 as libc::c_int)
                as crate::jmorecfg_h::JDIMENSION
        }
        /* Duplicate the last real sample row rgroup*2 times; this pads out the
         * last partial rowgroup and ensures at least one full rowgroup of context.
         */
        xbuf = *(*main_ptr).xbuffer[(*main_ptr).whichptr as usize].offset(ci as isize);
        i = 0 as libc::c_int;
        while i < rgroup * 2 as libc::c_int {
            let ref mut fresh11 = *xbuf.offset((rows_left + i) as isize);
            *fresh11 = *xbuf.offset((rows_left - 1 as libc::c_int) as isize);
            i += 1
        }
        ci += 1;
        compptr = compptr.offset(1)
    }
}
/*
 * Initialize for a processing pass.
 */

unsafe extern "C" fn start_pass_main(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut pass_mode: crate::jpegint_h::J_BUF_MODE,
) {
    let mut main_ptr: crate::src::jdmainct::my_main_ptr =
        (*cinfo).main as crate::src::jdmainct::my_main_ptr; /* Create the xbuffer[] lists */
    match pass_mode as libc::c_uint {
        0 => {
            if (*(*cinfo).upsample).need_context_rows != 0 {
                (*main_ptr).pub_0.process_data = Some(
                    process_data_context_main
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: *mut crate::jmorecfg_h::JDIMENSION,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* Read first iMCU row into xbuffer[0] */
                make_funny_pointers(cinfo);
                (*main_ptr).whichptr = 0 as libc::c_int;
                (*main_ptr).context_state = 0 as libc::c_int;
                (*main_ptr).iMCU_row_ctr = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION
            } else {
                /* Simple case with no context needed */
                (*main_ptr).pub_0.process_data = Some(
                    process_data_simple_main
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: *mut crate::jmorecfg_h::JDIMENSION,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                )
            } /* Mark buffer empty */
            (*main_ptr).buffer_full = crate::jmorecfg_h::FALSE;
            (*main_ptr).rowgroup_ctr = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION
        }
        2 => {
            /* For last pass of 2-pass quantization, just crank the postprocessor */
            (*main_ptr).pub_0.process_data = Some(
                process_data_crank_post
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: *mut crate::jmorecfg_h::JDIMENSION,
                        _: crate::jmorecfg_h::JDIMENSION,
                    ) -> (),
            )
        }
        _ => {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_BUFFER_MODE as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
    };
}
/*
 * jdmainct.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2010, 2016, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains the main buffer controller for decompression.
 * The main buffer lies between the JPEG decompressor proper and the
 * post-processor; it holds downsampled data in the JPEG colorspace.
 *
 * Note that this code is bypassed in raw-data mode, since the application
 * supplies the equivalent of the main buffer in that case.
 */
/*
 * In the current system design, the main buffer need never be a full-image
 * buffer; any full-height buffers will be found inside the coefficient or
 * postprocessing controllers.  Nonetheless, the main controller is not
 * trivial.  Its responsibility is to provide context rows for upsampling/
 * rescaling, and doing this in an efficient fashion is a bit tricky.
 *
 * Postprocessor input data is counted in "row groups".  A row group
 * is defined to be (v_samp_factor * DCT_scaled_size / min_DCT_scaled_size)
 * sample rows of each component.  (We require DCT_scaled_size values to be
 * chosen such that these numbers are integers.  In practice DCT_scaled_size
 * values will likely be powers of two, so we actually have the stronger
 * condition that DCT_scaled_size / min_DCT_scaled_size is an integer.)
 * Upsampling will typically produce max_v_samp_factor pixel rows from each
 * row group (times any additional scale factor that the upsampler is
 * applying).
 *
 * The coefficient controller will deliver data to us one iMCU row at a time;
 * each iMCU row contains v_samp_factor * DCT_scaled_size sample rows, or
 * exactly min_DCT_scaled_size row groups.  (This amount of data corresponds
 * to one row of MCUs when the image is fully interleaved.)  Note that the
 * number of sample rows varies across components, but the number of row
 * groups does not.  Some garbage sample rows may be included in the last iMCU
 * row at the bottom of the image.
 *
 * Depending on the vertical scaling algorithm used, the upsampler may need
 * access to the sample row(s) above and below its current input row group.
 * The upsampler is required to set need_context_rows TRUE at global selection
 * time if so.  When need_context_rows is FALSE, this controller can simply
 * obtain one iMCU row at a time from the coefficient controller and dole it
 * out as row groups to the postprocessor.
 *
 * When need_context_rows is TRUE, this controller guarantees that the buffer
 * passed to postprocessing contains at least one row group's worth of samples
 * above and below the row group(s) being processed.  Note that the context
 * rows "above" the first passed row group appear at negative row offsets in
 * the passed buffer.  At the top and bottom of the image, the required
 * context rows are manufactured by duplicating the first or last real sample
 * row; this avoids having special cases in the upsampling inner loops.
 *
 * The amount of context is fixed at one row group just because that's a
 * convenient number for this controller to work with.  The existing
 * upsamplers really only need one sample row of context.  An upsampler
 * supporting arbitrary output rescaling might wish for more than one row
 * group of context when shrinking the image; tough, we don't handle that.
 * (This is justified by the assumption that downsizing will be handled mostly
 * by adjusting the DCT_scaled_size values, so that the actual scale factor at
 * the upsample step needn't be much less than one.)
 *
 * To provide the desired context, we have to retain the last two row groups
 * of one iMCU row while reading in the next iMCU row.  (The last row group
 * can't be processed until we have another row group for its below-context,
 * and so we have to save the next-to-last group too for its above-context.)
 * We could do this most simply by copying data around in our buffer, but
 * that'd be very slow.  We can avoid copying any data by creating a rather
 * strange pointer structure.  Here's how it works.  We allocate a workspace
 * consisting of M+2 row groups (where M = min_DCT_scaled_size is the number
 * of row groups per iMCU row).  We create two sets of redundant pointers to
 * the workspace.  Labeling the physical row groups 0 to M+1, the synthesized
 * pointer lists look like this:
 *                   M+1                          M-1
 * master pointer --> 0         master pointer --> 0
 *                    1                            1
 *                   ...                          ...
 *                   M-3                          M-3
 *                   M-2                           M
 *                   M-1                          M+1
 *                    M                           M-2
 *                   M+1                          M-1
 *                    0                            0
 * We read alternate iMCU rows using each master pointer; thus the last two
 * row groups of the previous iMCU row remain un-overwritten in the workspace.
 * The pointer lists are set up so that the required context rows appear to
 * be adjacent to the proper places when we pass the pointer lists to the
 * upsampler.
 *
 * The above pictures describe the normal state of the pointer lists.
 * At top and bottom of the image, we diddle the pointer lists to duplicate
 * the first or last sample row as necessary (this is cheaper than copying
 * sample rows around).
 *
 * This scheme breaks down if M < 2, ie, min_DCT_scaled_size is 1.  In that
 * situation each iMCU row provides only one row group so the buffering logic
 * must be different (eg, we must read two iMCU rows before we can emit the
 * first row group).  For now, we simply do not support providing context
 * rows when min_DCT_scaled_size is 1.  That combination seems unlikely to
 * be worth providing --- if someone wants a 1/8th-size preview, they probably
 * want it quick and dirty, so a context-free upsampler is sufficient.
 */
/* Forward declarations */
/*
 * Process some data.
 * This handles the simple case where no context is required.
 */

unsafe extern "C" fn process_data_simple_main(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut out_row_ctr: *mut crate::jmorecfg_h::JDIMENSION,
    mut out_rows_avail: crate::jmorecfg_h::JDIMENSION,
) {
    let mut main_ptr: crate::src::jdmainct::my_main_ptr =
        (*cinfo).main as crate::src::jdmainct::my_main_ptr;
    let mut rowgroups_avail: crate::jmorecfg_h::JDIMENSION = 0;
    /* Read input data if we haven't filled the main buffer yet */
    if (*main_ptr).buffer_full == 0 {
        if Some(
            (*(*cinfo).coef)
                .decompress_data
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo, (*main_ptr).buffer.as_mut_ptr())
            == 0
        {
            return;
        }
        (*main_ptr).buffer_full = crate::jmorecfg_h::TRUE /* suspension forced, can do nothing more */
        /* OK, we have an iMCU row to work with */
    }
    /* There are always min_DCT_scaled_size row groups in an iMCU row. */
    rowgroups_avail = (*cinfo).min_DCT_scaled_size as crate::jmorecfg_h::JDIMENSION;
    /* Note: at the bottom of the image, we may pass extra garbage row groups
     * to the postprocessor.  The postprocessor has to check for bottom
     * of image anyway (at row resolution), so no point in us doing it too.
     */
    /* Feed the postprocessor */
    Some(
        (*(*cinfo).post)
            .post_process_data
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo,
        (*main_ptr).buffer.as_mut_ptr(),
        &mut (*main_ptr).rowgroup_ctr,
        rowgroups_avail,
        output_buf,
        out_row_ctr,
        out_rows_avail,
    );
    /* Has postprocessor consumed all the data yet? If so, mark buffer empty */
    if (*main_ptr).rowgroup_ctr >= rowgroups_avail {
        (*main_ptr).buffer_full = crate::jmorecfg_h::FALSE;
        (*main_ptr).rowgroup_ctr = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION
    };
}
/*
 * Process some data.
 * This handles the case where context rows must be provided.
 */

unsafe extern "C" fn process_data_context_main(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut out_row_ctr: *mut crate::jmorecfg_h::JDIMENSION,
    mut out_rows_avail: crate::jmorecfg_h::JDIMENSION,
) {
    let mut main_ptr: crate::src::jdmainct::my_main_ptr =
        (*cinfo).main as crate::src::jdmainct::my_main_ptr;
    /* Read input data if we haven't filled the main buffer yet */
    if (*main_ptr).buffer_full == 0 {
        if Some(
            (*(*cinfo).coef)
                .decompress_data
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo,
            (*main_ptr).xbuffer[(*main_ptr).whichptr as usize],
        ) == 0
        {
            return;
        } /* suspension forced, can do nothing more */
        /* count rows received */
        (*main_ptr).buffer_full = crate::jmorecfg_h::TRUE; /* OK, we have an iMCU row to work with */
        (*main_ptr).iMCU_row_ctr = (*main_ptr).iMCU_row_ctr.wrapping_add(1)
    }
    let mut current_block_26: u64;
    /* Postprocessor typically will not swallow all the input data it is handed
     * in one call (due to filling the output buffer first).  Must be prepared
     * to exit and restart.  This switch lets us keep track of how far we got.
     * Note that each case falls through to the next on successful completion.
     */
    match (*main_ptr).context_state {
        2 => {
            /* Call postprocessor using previously set pointers for postponed row */
            Some(
                (*(*cinfo).post)
                    .post_process_data
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                (*main_ptr).xbuffer[(*main_ptr).whichptr as usize],
                &mut (*main_ptr).rowgroup_ctr,
                (*main_ptr).rowgroups_avail,
                output_buf,
                out_row_ctr,
                out_rows_avail,
            ); /* Need to suspend */
            if (*main_ptr).rowgroup_ctr < (*main_ptr).rowgroups_avail {
                return;
            } /* Postprocessor exactly filled output buf */
            (*main_ptr).context_state = 0 as libc::c_int;
            if *out_row_ctr >= out_rows_avail {
                return;
            }
            current_block_26 = 10531413684724535507;
        }
        0 => {
            current_block_26 = 10531413684724535507;
        }
        1 => {
            current_block_26 = 18099180955076792539;
        }
        _ => {
            current_block_26 = 16203760046146113240;
        }
    }
    match current_block_26 {
        10531413684724535507 =>
        /*FALLTHROUGH*/
        /* Prepare to process first M-1 row groups of this iMCU row */
        {
            (*main_ptr).rowgroup_ctr = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            (*main_ptr).rowgroups_avail =
                ((*cinfo).min_DCT_scaled_size - 1 as libc::c_int) as crate::jmorecfg_h::JDIMENSION;
            /* Check for bottom of image: if so, tweak pointers to "duplicate"
             * the last sample row, and adjust rowgroups_avail to ignore padding rows.
             */
            if (*main_ptr).iMCU_row_ctr == (*cinfo).total_iMCU_rows {
                set_bottom_pointers(cinfo);
            }
            (*main_ptr).context_state = 1 as libc::c_int;
            current_block_26 = 18099180955076792539;
        }
        _ => {}
    }
    match current_block_26 {
        18099180955076792539 =>
        /*FALLTHROUGH*/
        /* Call postprocessor using previously set pointers */
        {
            Some(
                (*(*cinfo).post)
                    .post_process_data
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                (*main_ptr).xbuffer[(*main_ptr).whichptr as usize],
                &mut (*main_ptr).rowgroup_ctr,
                (*main_ptr).rowgroups_avail,
                output_buf,
                out_row_ctr,
                out_rows_avail,
            ); /* Need to suspend */
            if (*main_ptr).rowgroup_ctr < (*main_ptr).rowgroups_avail {
                return;
            }
            /* After the first iMCU, change wraparound pointers to normal state */
            if (*main_ptr).iMCU_row_ctr == 1 as libc::c_int as libc::c_uint {
                set_wraparound_pointers(cinfo);
            }
            /* Prepare to load new iMCU row using other xbuffer list */
            (*main_ptr).whichptr ^= 1 as libc::c_int; /* 0=>1 or 1=>0 */
            (*main_ptr).buffer_full = crate::jmorecfg_h::FALSE;
            /* Still need to process last row group of this iMCU row, */
            /* which is saved at index M+1 of the other xbuffer */
            (*main_ptr).rowgroup_ctr =
                ((*cinfo).min_DCT_scaled_size + 1 as libc::c_int) as crate::jmorecfg_h::JDIMENSION;
            (*main_ptr).rowgroups_avail =
                ((*cinfo).min_DCT_scaled_size + 2 as libc::c_int) as crate::jmorecfg_h::JDIMENSION;
            (*main_ptr).context_state = 2 as libc::c_int
        }
        _ => {}
    };
}
/*
 * Process some data.
 * Final pass of two-pass quantization: just call the postprocessor.
 * Source data will be the postprocessor controller's internal buffer.
 */

unsafe extern "C" fn process_data_crank_post(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut out_row_ctr: *mut crate::jmorecfg_h::JDIMENSION,
    mut out_rows_avail: crate::jmorecfg_h::JDIMENSION,
) {
    Some(
        (*(*cinfo).post)
            .post_process_data
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo,
        crate::stddef_h::NULL as *mut libc::c_void as crate::jpeglib_h::JSAMPIMAGE,
        crate::stddef_h::NULL as *mut libc::c_void as *mut crate::jmorecfg_h::JDIMENSION,
        0 as libc::c_int as crate::jmorecfg_h::JDIMENSION,
        output_buf,
        out_row_ctr,
        out_rows_avail,
    );
}
/* QUANT_2PASS_SUPPORTED */
/*
 * Initialize main buffer controller.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_d_main_controller(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut need_full_buffer: crate::jmorecfg_h::boolean,
) {
    let mut main_ptr: crate::src::jdmainct::my_main_ptr =
        0 as *mut crate::src::jdmainct::my_main_controller;
    let mut ci: libc::c_int = 0;
    let mut rgroup: libc::c_int = 0;
    let mut ngroups: libc::c_int = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    main_ptr = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<crate::src::jdmainct::my_main_controller>() as libc::c_ulong,
    ) as crate::src::jdmainct::my_main_ptr;
    (*cinfo).main = main_ptr as *mut crate::jpegint_h::jpeg_d_main_controller;
    (*main_ptr).pub_0.start_pass = Some(
        start_pass_main
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::jpegint_h::J_BUF_MODE,
            ) -> (),
    );
    if need_full_buffer != 0 {
        /* shouldn't happen */
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_BUFFER_MODE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Allocate the workspace.
     * ngroups is the number of row groups we need.
     */
    if (*(*cinfo).upsample).need_context_rows != 0 {
        if (*cinfo).min_DCT_scaled_size < 2 as libc::c_int {
            /* unsupported, see comments above */
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_NOTIMPL as libc::c_int; /* Alloc space for xbuffer[] lists */
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            ); /* height of a row group of component */
        }
        alloc_funny_pointers(cinfo);
        ngroups = (*cinfo).min_DCT_scaled_size + 2 as libc::c_int
    } else {
        ngroups = (*cinfo).min_DCT_scaled_size
    }
    ci = 0 as libc::c_int;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        rgroup =
            (*compptr).v_samp_factor * (*compptr).DCT_scaled_size / (*cinfo).min_DCT_scaled_size;
        (*main_ptr).buffer[ci as usize] = Some(
            (*(*cinfo).mem)
                .alloc_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            (*compptr)
                .width_in_blocks
                .wrapping_mul((*compptr).DCT_scaled_size as libc::c_uint),
            (rgroup * ngroups) as crate::jmorecfg_h::JDIMENSION,
        );
        ci += 1;
        compptr = compptr.offset(1)
    }
}
