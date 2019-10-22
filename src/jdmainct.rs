pub use super::jerror::{
    C2RustUnnamed_3, JERR_ARITH_NOTIMPL, JERR_BAD_ALIGN_TYPE, JERR_BAD_ALLOC_CHUNK,
    JERR_BAD_BUFFER_MODE, JERR_BAD_COMPONENT_ID, JERR_BAD_CROP_SPEC, JERR_BAD_DCTSIZE,
    JERR_BAD_DCT_COEF, JERR_BAD_HUFF_TABLE, JERR_BAD_IN_COLORSPACE, JERR_BAD_J_COLORSPACE,
    JERR_BAD_LENGTH, JERR_BAD_LIB_VERSION, JERR_BAD_MCU_SIZE, JERR_BAD_PARAM, JERR_BAD_PARAM_VALUE,
    JERR_BAD_POOL_ID, JERR_BAD_PRECISION, JERR_BAD_PROGRESSION, JERR_BAD_PROG_SCRIPT,
    JERR_BAD_SAMPLING, JERR_BAD_SCAN_SCRIPT, JERR_BAD_STATE, JERR_BAD_STRUCT_SIZE,
    JERR_BAD_VIRTUAL_ACCESS, JERR_BUFFER_SIZE, JERR_CANT_SUSPEND, JERR_CCIR601_NOTIMPL,
    JERR_COMPONENT_COUNT, JERR_CONVERSION_NOTIMPL, JERR_DAC_INDEX, JERR_DAC_VALUE, JERR_DHT_INDEX,
    JERR_DQT_INDEX, JERR_EMPTY_IMAGE, JERR_EMS_READ, JERR_EMS_WRITE, JERR_EOI_EXPECTED,
    JERR_FILE_READ, JERR_FILE_WRITE, JERR_FRACT_SAMPLE_NOTIMPL, JERR_HUFF_CLEN_OVERFLOW,
    JERR_HUFF_MISSING_CODE, JERR_IMAGE_TOO_BIG, JERR_INPUT_EMPTY, JERR_INPUT_EOF,
    JERR_MISMATCHED_QUANT_TABLE, JERR_MISSING_DATA, JERR_MODE_CHANGE, JERR_NOTIMPL,
    JERR_NOT_COMPILED, JERR_NO_BACKING_STORE, JERR_NO_HUFF_TABLE, JERR_NO_IMAGE,
    JERR_NO_QUANT_TABLE, JERR_NO_SOI, JERR_OUT_OF_MEMORY, JERR_QUANT_COMPONENTS,
    JERR_QUANT_FEW_COLORS, JERR_QUANT_MANY_COLORS, JERR_SOF_DUPLICATE, JERR_SOF_NO_SOS,
    JERR_SOF_UNSUPPORTED, JERR_SOI_DUPLICATE, JERR_SOS_NO_SOF, JERR_TFILE_CREATE, JERR_TFILE_READ,
    JERR_TFILE_SEEK, JERR_TFILE_WRITE, JERR_TOO_LITTLE_DATA, JERR_UNKNOWN_MARKER,
    JERR_UNSUPPORTED_SUSPEND, JERR_VIRTUAL_BUG, JERR_WIDTH_OVERFLOW, JERR_XMS_READ, JERR_XMS_WRITE,
    JMSG_COPYRIGHT, JMSG_LASTMSGCODE, JMSG_NOMESSAGE, JMSG_VERSION, JTRC_16BIT_TABLES, JTRC_ADOBE,
    JTRC_APP0, JTRC_APP14, JTRC_DAC, JTRC_DHT, JTRC_DQT, JTRC_DRI, JTRC_EMS_CLOSE, JTRC_EMS_OPEN,
    JTRC_EOI, JTRC_HUFFBITS, JTRC_JFIF, JTRC_JFIF_BADTHUMBNAILSIZE, JTRC_JFIF_EXTENSION,
    JTRC_JFIF_THUMBNAIL, JTRC_MISC_MARKER, JTRC_PARMLESS_MARKER, JTRC_QUANTVALS,
    JTRC_QUANT_3_NCOLORS, JTRC_QUANT_NCOLORS, JTRC_QUANT_SELECTED, JTRC_RECOVERY_ACTION, JTRC_RST,
    JTRC_SMOOTH_NOTIMPL, JTRC_SOF, JTRC_SOF_COMPONENT, JTRC_SOI, JTRC_SOS, JTRC_SOS_COMPONENT,
    JTRC_SOS_PARAMS, JTRC_TFILE_CLOSE, JTRC_TFILE_OPEN, JTRC_THUMB_JPEG, JTRC_THUMB_PALETTE,
    JTRC_THUMB_RGB, JTRC_UNKNOWN_IDS, JTRC_XMS_CLOSE, JTRC_XMS_OPEN, JWRN_ADOBE_XFORM,
    JWRN_BOGUS_ICC, JWRN_BOGUS_PROGRESSION, JWRN_EXTRANEOUS_DATA, JWRN_HIT_MARKER,
    JWRN_HUFF_BAD_CODE, JWRN_JFIF_MAJOR, JWRN_JPEG_EOF, JWRN_MUST_RESYNC, JWRN_NOT_SEQUENTIAL,
    JWRN_TOO_MUCH_DATA,
};
pub use crate::jmorecfg_h::{
    boolean, FALSE, JCOEF, JDIMENSION, JOCTET, JSAMPLE, TRUE, UINT16, UINT8,
};
pub use crate::jpegint_h::{
    inverse_DCT_method_ptr, JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS,
    JBUF_SAVE_SOURCE, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_decompress_ptr, jpeg_color_deconverter, jpeg_color_quantizer,
    jpeg_common_struct, jpeg_component_info, jpeg_d_coef_controller, jpeg_d_main_controller,
    jpeg_d_post_controller, jpeg_decomp_master, jpeg_decompress_struct, jpeg_entropy_decoder,
    jpeg_error_mgr, jpeg_input_controller, jpeg_inverse_dct, jpeg_marker_parser_method,
    jpeg_marker_reader, jpeg_marker_struct, jpeg_memory_mgr, jpeg_progress_mgr,
    jpeg_saved_marker_ptr, jpeg_source_mgr, jpeg_upsampler, jvirt_barray_control, jvirt_barray_ptr,
    jvirt_sarray_control, jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY,
    JBLOCKROW, JCOEFPTR, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA,
    JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB,
    JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW,
    JDITHER_FS, JDITHER_NONE, JDITHER_ORDERED, JHUFF_TBL, JPOOL_IMAGE, JQUANT_TBL, JSAMPARRAY,
    JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD, J_DITHER_MODE,
};
pub use crate::stddef_h::{size_t, NULL};
use libc::{self, c_int, c_uint, c_ulong, c_void};
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
pub type my_main_ptr = *mut my_main_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_main_controller {
    pub pub_0: jpeg_d_main_controller,
    pub buffer: [JSAMPARRAY; 10],
    pub buffer_full: boolean,
    pub rowgroup_ctr: JDIMENSION,
    pub xbuffer: [JSAMPIMAGE; 2],
    pub whichptr: c_int,
    pub context_state: c_int,
    pub rowgroups_avail: JDIMENSION,
    pub iMCU_row_ctr: JDIMENSION,
}
/* context_state values: */

pub const CTX_PREPARE_FOR_IMCU: c_int = 0i32;
/* need to prepare for MCU row */
/* feeding iMCU to postprocessor */
/* feeding postponed row group */

pub unsafe extern "C" fn set_wraparound_pointers(mut cinfo: j_decompress_ptr)
/* Set up the "wraparound" pointers at top and bottom of the pointer lists.
 * This changes the pointer list state from top-of-image to the normal state.
 */
{
    let mut main_ptr: my_main_ptr = (*cinfo).main as my_main_ptr; /* height of a row group of component */
    let mut ci: c_int = 0;
    let mut i: c_int = 0;
    let mut rgroup: c_int = 0;
    let mut M: c_int = (*cinfo).min_DCT_scaled_size;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut xbuf0: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut xbuf1: JSAMPARRAY = 0 as *mut JSAMPROW;
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        rgroup =
            (*compptr).v_samp_factor * (*compptr).DCT_scaled_size / (*cinfo).min_DCT_scaled_size;
        xbuf0 = *(*main_ptr).xbuffer[0].offset(ci as isize);
        xbuf1 = *(*main_ptr).xbuffer[1].offset(ci as isize);
        i = 0i32;
        while i < rgroup {
            let ref mut fresh0 = *xbuf0.offset((i - rgroup) as isize);
            *fresh0 = *xbuf0.offset((rgroup * (M + 1i32) + i) as isize);
            let ref mut fresh1 = *xbuf1.offset((i - rgroup) as isize);
            *fresh1 = *xbuf1.offset((rgroup * (M + 1i32) + i) as isize);
            let ref mut fresh2 = *xbuf0.offset((rgroup * (M + 2i32) + i) as isize);
            *fresh2 = *xbuf0.offset(i as isize);
            let ref mut fresh3 = *xbuf1.offset((rgroup * (M + 2i32) + i) as isize);
            *fresh3 = *xbuf1.offset(i as isize);
            i += 1
        }
        ci += 1;
        compptr = compptr.offset(1)
    }
}

unsafe extern "C" fn alloc_funny_pointers(mut cinfo: j_decompress_ptr)
/* Allocate space for the funny pointer lists.
 * This is done only once, not once per pass.
 */
{
    let mut main_ptr: my_main_ptr = (*cinfo).main as my_main_ptr;
    let mut ci: c_int = 0;
    let mut rgroup: c_int = 0;
    let mut M: c_int = (*cinfo).min_DCT_scaled_size;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut xbuf: JSAMPARRAY = 0 as *mut JSAMPROW;
    /* Get top-level space for component array pointers.
     * We alloc both arrays with one call to save a few cycles.
     */
    (*main_ptr).xbuffer[0] = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (((*cinfo).num_components * 2i32) as c_ulong)
            .wrapping_mul(::std::mem::size_of::<JSAMPARRAY>() as c_ulong),
    ) as JSAMPIMAGE; /* height of a row group of component */
    (*main_ptr).xbuffer[1] = (*main_ptr).xbuffer[0].offset((*cinfo).num_components as isize);
    ci = 0i32;
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
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            ((2i32 * (rgroup * (M + 4i32))) as c_ulong)
                .wrapping_mul(::std::mem::size_of::<JSAMPROW>() as c_ulong),
        ) as JSAMPARRAY; /* want one row group at negative offsets */
        xbuf = xbuf.offset(rgroup as isize);
        let ref mut fresh4 = *(*main_ptr).xbuffer[0].offset(ci as isize);
        *fresh4 = xbuf;
        xbuf = xbuf.offset((rgroup * (M + 4i32)) as isize);
        let ref mut fresh5 = *(*main_ptr).xbuffer[1].offset(ci as isize);
        *fresh5 = xbuf;
        ci += 1;
        compptr = compptr.offset(1)
    }
}

unsafe extern "C" fn make_funny_pointers(mut cinfo: j_decompress_ptr)
/* Create the funny pointer lists discussed in the comments above.
 * The actual workspace is already allocated (in main_ptr->buffer),
 * and the space for the pointer lists is allocated too.
 * This routine just fills in the curiously ordered lists.
 * This will be repeated at the beginning of each pass.
 */
{
    let mut main_ptr: my_main_ptr = (*cinfo).main as my_main_ptr; /* height of a row group of component */
    let mut ci: c_int = 0;
    let mut i: c_int = 0;
    let mut rgroup: c_int = 0;
    let mut M: c_int = (*cinfo).min_DCT_scaled_size;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut buf: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut xbuf0: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut xbuf1: JSAMPARRAY = 0 as *mut JSAMPROW;
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        rgroup =
            (*compptr).v_samp_factor * (*compptr).DCT_scaled_size / (*cinfo).min_DCT_scaled_size;
        xbuf0 = *(*main_ptr).xbuffer[0].offset(ci as isize);
        xbuf1 = *(*main_ptr).xbuffer[1].offset(ci as isize);
        /* First copy the workspace pointers as-is */
        buf = (*main_ptr).buffer[ci as usize];
        i = 0i32;
        while i < rgroup * (M + 2i32) {
            let ref mut fresh6 = *xbuf1.offset(i as isize);
            *fresh6 = *buf.offset(i as isize);
            let ref mut fresh7 = *xbuf0.offset(i as isize);
            *fresh7 = *fresh6;
            i += 1
        }
        /* In the second list, put the last four row groups in swapped order */
        i = 0i32;
        while i < rgroup * 2i32 {
            let ref mut fresh8 = *xbuf1.offset((rgroup * (M - 2i32) + i) as isize);
            *fresh8 = *buf.offset((rgroup * M + i) as isize);
            let ref mut fresh9 = *xbuf1.offset((rgroup * M + i) as isize);
            *fresh9 = *buf.offset((rgroup * (M - 2i32) + i) as isize);
            i += 1
        }
        /* The wraparound pointers at top and bottom will be filled later
         * (see set_wraparound_pointers, below).  Initially we want the "above"
         * pointers to duplicate the first actual data line.  This only needs
         * to happen in xbuffer[0].
         */
        i = 0i32;
        while i < rgroup {
            let ref mut fresh10 = *xbuf0.offset((i - rgroup) as isize);
            *fresh10 = *xbuf0.offset(0);
            i += 1
        }
        ci += 1;
        compptr = compptr.offset(1)
    }
}

unsafe extern "C" fn set_bottom_pointers(mut cinfo: j_decompress_ptr)
/* Change the pointer lists to duplicate the last sample row at the bottom
 * of the image.  whichptr indicates which xbuffer holds the final iMCU row.
 * Also sets rowgroups_avail to indicate number of nondummy row groups in row.
 */
{
    let mut main_ptr: my_main_ptr = (*cinfo).main as my_main_ptr;
    let mut ci: c_int = 0;
    let mut i: c_int = 0;
    let mut rgroup: c_int = 0;
    let mut iMCUheight: c_int = 0;
    let mut rows_left: c_int = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut xbuf: JSAMPARRAY = 0 as *mut JSAMPROW;
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Count sample rows in one iMCU row and in one row group */
        iMCUheight = (*compptr).v_samp_factor * (*compptr).DCT_scaled_size;
        rgroup = iMCUheight / (*cinfo).min_DCT_scaled_size;
        /* Count nondummy sample rows remaining for this component */
        rows_left = (*compptr)
            .downsampled_height
            .wrapping_rem(iMCUheight as JDIMENSION) as c_int;
        if rows_left == 0i32 {
            rows_left = iMCUheight
        }
        /* Count nondummy row groups.  Should get same answer for each component,
         * so we need only do it once.
         */
        if ci == 0i32 {
            (*main_ptr).rowgroups_avail = ((rows_left - 1i32) / rgroup + 1i32) as JDIMENSION
        }
        /* Duplicate the last real sample row rgroup*2 times; this pads out the
         * last partial rowgroup and ensures at least one full rowgroup of context.
         */
        xbuf = *(*main_ptr).xbuffer[(*main_ptr).whichptr as usize].offset(ci as isize);
        i = 0i32;
        while i < rgroup * 2i32 {
            let ref mut fresh11 = *xbuf.offset((rows_left + i) as isize);
            *fresh11 = *xbuf.offset((rows_left - 1i32) as isize);
            i += 1
        }
        ci += 1;
        compptr = compptr.offset(1)
    }
}
/*
 * Initialize for a processing pass.
 */

unsafe extern "C" fn start_pass_main(mut cinfo: j_decompress_ptr, mut pass_mode: J_BUF_MODE) {
    let mut main_ptr: my_main_ptr = (*cinfo).main as my_main_ptr; /* Create the xbuffer[] lists */
    match pass_mode as c_uint {
        0 => {
            if (*(*cinfo).upsample).need_context_rows != 0 {
                (*main_ptr).pub_0.process_data = Some(
                    process_data_context_main
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPARRAY,
                            _: *mut JDIMENSION,
                            _: JDIMENSION,
                        ) -> (),
                ); /* Read first iMCU row into xbuffer[0] */
                make_funny_pointers(cinfo);
                (*main_ptr).whichptr = 0i32;
                (*main_ptr).context_state = 0i32;
                (*main_ptr).iMCU_row_ctr = 0i32 as JDIMENSION
            } else {
                /* Simple case with no context needed */
                (*main_ptr).pub_0.process_data = Some(
                    process_data_simple_main
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPARRAY,
                            _: *mut JDIMENSION,
                            _: JDIMENSION,
                        ) -> (),
                )
            } /* Mark buffer empty */
            (*main_ptr).buffer_full = FALSE;
            (*main_ptr).rowgroup_ctr = 0i32 as JDIMENSION
        }
        2 => {
            /* For last pass of 2-pass quantization, just crank the postprocessor */
            (*main_ptr).pub_0.process_data = Some(
                process_data_crank_post
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: JSAMPARRAY,
                        _: *mut JDIMENSION,
                        _: JDIMENSION,
                    ) -> (),
            )
        }
        _ => {
            (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_BUFFER_MODE as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
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
    mut cinfo: j_decompress_ptr,
    mut output_buf: JSAMPARRAY,
    mut out_row_ctr: *mut JDIMENSION,
    mut out_rows_avail: JDIMENSION,
) {
    let mut main_ptr: my_main_ptr = (*cinfo).main as my_main_ptr;
    let mut rowgroups_avail: JDIMENSION = 0;
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
        (*main_ptr).buffer_full = TRUE /* suspension forced, can do nothing more */
        /* OK, we have an iMCU row to work with */
    }
    /* There are always min_DCT_scaled_size row groups in an iMCU row. */
    rowgroups_avail = (*cinfo).min_DCT_scaled_size as JDIMENSION;
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
        (*main_ptr).buffer_full = FALSE;
        (*main_ptr).rowgroup_ctr = 0i32 as JDIMENSION
    };
}
/*
 * Process some data.
 * This handles the case where context rows must be provided.
 */

unsafe extern "C" fn process_data_context_main(
    mut cinfo: j_decompress_ptr,
    mut output_buf: JSAMPARRAY,
    mut out_row_ctr: *mut JDIMENSION,
    mut out_rows_avail: JDIMENSION,
) {
    let mut main_ptr: my_main_ptr = (*cinfo).main as my_main_ptr;
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
        (*main_ptr).buffer_full = TRUE; /* OK, we have an iMCU row to work with */
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
            (*main_ptr).context_state = 0i32;
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
            (*main_ptr).rowgroup_ctr = 0i32 as JDIMENSION;
            (*main_ptr).rowgroups_avail = ((*cinfo).min_DCT_scaled_size - 1i32) as JDIMENSION;
            /* Check for bottom of image: if so, tweak pointers to "duplicate"
             * the last sample row, and adjust rowgroups_avail to ignore padding rows.
             */
            if (*main_ptr).iMCU_row_ctr == (*cinfo).total_iMCU_rows {
                set_bottom_pointers(cinfo);
            }
            (*main_ptr).context_state = 1i32;
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
            if (*main_ptr).iMCU_row_ctr == 1i32 as c_uint {
                set_wraparound_pointers(cinfo);
            }
            /* Prepare to load new iMCU row using other xbuffer list */
            (*main_ptr).whichptr ^= 1i32; /* 0=>1 or 1=>0 */
            (*main_ptr).buffer_full = FALSE;
            /* Still need to process last row group of this iMCU row, */
            /* which is saved at index M+1 of the other xbuffer */
            (*main_ptr).rowgroup_ctr = ((*cinfo).min_DCT_scaled_size + 1i32) as JDIMENSION;
            (*main_ptr).rowgroups_avail = ((*cinfo).min_DCT_scaled_size + 2i32) as JDIMENSION;
            (*main_ptr).context_state = 2i32
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
    mut cinfo: j_decompress_ptr,
    mut output_buf: JSAMPARRAY,
    mut out_row_ctr: *mut JDIMENSION,
    mut out_rows_avail: JDIMENSION,
) {
    Some(
        (*(*cinfo).post)
            .post_process_data
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo,
        NULL as *mut c_void as JSAMPIMAGE,
        NULL as *mut c_void as *mut JDIMENSION,
        0i32 as JDIMENSION,
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
    mut cinfo: j_decompress_ptr,
    mut need_full_buffer: boolean,
) {
    let mut main_ptr: my_main_ptr = 0 as *mut my_main_controller;
    let mut ci: c_int = 0;
    let mut rgroup: c_int = 0;
    let mut ngroups: c_int = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    main_ptr = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<my_main_controller>() as c_ulong,
    ) as my_main_ptr;
    (*cinfo).main = main_ptr as *mut jpeg_d_main_controller;
    (*main_ptr).pub_0.start_pass =
        Some(start_pass_main as unsafe extern "C" fn(_: j_decompress_ptr, _: J_BUF_MODE) -> ());
    if need_full_buffer != 0 {
        /* shouldn't happen */
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_BUFFER_MODE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Allocate the workspace.
     * ngroups is the number of row groups we need.
     */
    if (*(*cinfo).upsample).need_context_rows != 0 {
        if (*cinfo).min_DCT_scaled_size < 2i32 {
            /* unsupported, see comments above */
            (*(*cinfo).err).msg_code = super::jerror::JERR_NOTIMPL as c_int; /* Alloc space for xbuffer[] lists */
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr); /* height of a row group of component */
        }
        alloc_funny_pointers(cinfo);
        ngroups = (*cinfo).min_DCT_scaled_size + 2i32
    } else {
        ngroups = (*cinfo).min_DCT_scaled_size
    }
    ci = 0i32;
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
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (*compptr)
                .width_in_blocks
                .wrapping_mul((*compptr).DCT_scaled_size as c_uint),
            (rgroup * ngroups) as JDIMENSION,
        );
        ci += 1;
        compptr = compptr.offset(1)
    }
}
