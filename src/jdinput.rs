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
pub use crate::jconfig_h::BITS_IN_JSAMPLE;
pub use crate::jmorecfg_h::{
    boolean, FALSE, JCOEF, JDIMENSION, JOCTET, JPEG_MAX_DIMENSION, JSAMPLE, MAX_COMPONENTS, TRUE,
    UINT16, UINT8,
};
pub use crate::jpegint_h::{
    inverse_DCT_method_ptr, jdiv_round_up, JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT,
    JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_decompress_ptr, jpeg_color_deconverter, jpeg_color_quantizer,
    jpeg_common_struct, jpeg_component_info, jpeg_d_coef_controller, jpeg_d_main_controller,
    jpeg_d_post_controller, jpeg_decomp_master, jpeg_decompress_struct, jpeg_entropy_decoder,
    jpeg_error_mgr, jpeg_input_controller, jpeg_inverse_dct, jpeg_marker_parser_method,
    jpeg_marker_reader, jpeg_marker_struct, jpeg_memory_mgr, jpeg_progress_mgr,
    jpeg_saved_marker_ptr, jpeg_source_mgr, jpeg_upsampler, jvirt_barray_control, jvirt_barray_ptr,
    jvirt_sarray_control, jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr, DCTSIZE,
    D_MAX_BLOCKS_IN_MCU, JBLOCK, JBLOCKARRAY, JBLOCKROW, JCOEFPTR, JCS_CMYK, JCS_EXT_ABGR,
    JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX,
    JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK,
    JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JDITHER_FS, JDITHER_NONE, JDITHER_ORDERED, JHUFF_TBL,
    JPOOL_IMAGE, JPOOL_PERMANENT, JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE,
    J_DCT_METHOD, J_DITHER_MODE, MAX_COMPS_IN_SCAN, MAX_SAMP_FACTOR, NUM_QUANT_TBLS,
};
pub use crate::stddef_h::{size_t, NULL};
use crate::stdlib::memcpy;
use libc::{self, c_int, c_long, c_uint, c_ulong, c_void};

pub type my_inputctl_ptr = *mut my_input_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_input_controller {
    pub pub_0: jpeg_input_controller,
    pub inheaders: boolean,
}
/*
 * Routines to calculate various quantities related to the size of the image.
 */

unsafe extern "C" fn initial_setup(mut cinfo: j_decompress_ptr)
/* Called once, when first SOS marker is reached */
{
    /* Make sure image isn't bigger than I can handle */
    if (*cinfo).image_height as c_long > JPEG_MAX_DIMENSION
        || (*cinfo).image_width as c_long > JPEG_MAX_DIMENSION
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_IMAGE_TOO_BIG as c_int;
        (*(*cinfo).err).msg_parm.i[0] = 65500i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* For now, precision must match compiled-in value... */
    if (*cinfo).data_precision != BITS_IN_JSAMPLE {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_PRECISION as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).data_precision;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Check that number of components won't exceed internal array sizes */
    if (*cinfo).num_components > MAX_COMPONENTS {
        (*(*cinfo).err).msg_code = super::jerror::JERR_COMPONENT_COUNT as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).num_components;
        (*(*cinfo).err).msg_parm.i[1] = 10i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Compute maximum sampling factors; check factor validity */
    (*cinfo).max_h_samp_factor = 1i32;
    (*cinfo).max_v_samp_factor = 1i32;

    let mut ci: c_int = 0i32;
    let mut compptr: *mut jpeg_component_info = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        if (*compptr).h_samp_factor <= 0i32
            || (*compptr).h_samp_factor > MAX_SAMP_FACTOR
            || (*compptr).v_samp_factor <= 0i32
            || (*compptr).v_samp_factor > MAX_SAMP_FACTOR
        {
            (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_SAMPLING as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
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
        compptr = compptr.offset(1)
    }
    /* We initialize DCT_scaled_size and min_DCT_scaled_size to DCTSIZE.
     * In the full decompressor, this will be overridden by jdmaster.c;
     * but in the transcoder, jdmaster.c is not used, so we must do it here.
     */
    (*cinfo).min_DCT_scaled_size = DCTSIZE;
    /* Compute dimensions of components */
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        (*compptr).DCT_scaled_size = DCTSIZE;
        /* Size in DCT blocks */
        (*compptr).width_in_blocks = jdiv_round_up(
            (*cinfo).image_width as c_long * (*compptr).h_samp_factor as c_long,
            ((*cinfo).max_h_samp_factor * DCTSIZE) as c_long,
        ) as JDIMENSION;
        (*compptr).height_in_blocks = jdiv_round_up(
            (*cinfo).image_height as c_long * (*compptr).v_samp_factor as c_long,
            ((*cinfo).max_v_samp_factor * DCTSIZE) as c_long,
        ) as JDIMENSION;
        /* Set the first and last MCU columns to decompress from multi-scan images.
         * By default, decompress all of the MCU columns.
         */
        (*(*cinfo).master).first_MCU_col[ci as usize] = 0u32;
        (*(*cinfo).master).last_MCU_col[ci as usize] = (*compptr).width_in_blocks - 1u32;
        /* downsampled_width and downsampled_height will also be overridden by
         * jdmaster.c if we are doing full decompression.  The transcoder library
         * doesn't use these values, but the calling application might.
         */
        /* Size in samples */
        (*compptr).downsampled_width = jdiv_round_up(
            (*cinfo).image_width as c_long * (*compptr).h_samp_factor as c_long,
            (*cinfo).max_h_samp_factor as c_long,
        ) as JDIMENSION;
        (*compptr).downsampled_height = jdiv_round_up(
            (*cinfo).image_height as c_long * (*compptr).v_samp_factor as c_long,
            (*cinfo).max_v_samp_factor as c_long,
        ) as JDIMENSION;
        /* Mark component needed, until color conversion says otherwise */
        (*compptr).component_needed = TRUE;
        /* Mark no quantization table yet saved for component */
        (*compptr).quant_table = NULL as *mut JQUANT_TBL;
        ci += 1;
        compptr = compptr.offset(1)
    }
    /* Compute number of fully interleaved MCU rows. */
    (*cinfo).total_iMCU_rows = jdiv_round_up(
        (*cinfo).image_height as c_long,
        ((*cinfo).max_v_samp_factor * DCTSIZE) as c_long,
    ) as JDIMENSION;
    /* Decide whether file contains multiple scans */
    if (*cinfo).comps_in_scan < (*cinfo).num_components || (*cinfo).progressive_mode != 0 {
        (*(*cinfo).inputctl).has_multiple_scans = TRUE
    } else {
        (*(*cinfo).inputctl).has_multiple_scans = FALSE
    };
}

unsafe extern "C" fn per_scan_setup(mut cinfo: j_decompress_ptr)
/* Do computations that are needed before processing a JPEG scan */
/* cinfo->comps_in_scan and cinfo->cur_comp_info[] were set from SOS marker */
{
    let mut tmp: c_int = 0;
    let mut compptr: *mut jpeg_component_info = ::std::ptr::null_mut::<jpeg_component_info>();
    if (*cinfo).comps_in_scan == 1i32 {
        /* Noninterleaved (single-component) scan */
        compptr = (*cinfo).cur_comp_info[0];
        /* Overall image size in MCUs */
        (*cinfo).MCUs_per_row = (*compptr).width_in_blocks;
        (*cinfo).MCU_rows_in_scan = (*compptr).height_in_blocks;
        /* For noninterleaved scan, always one block per MCU */
        (*compptr).MCU_width = 1i32;
        (*compptr).MCU_height = 1i32;
        (*compptr).MCU_blocks = 1i32;
        (*compptr).MCU_sample_width = (*compptr).DCT_scaled_size;
        (*compptr).last_col_width = 1i32;
        /* For noninterleaved scans, it is convenient to define last_row_height
         * as the number of block rows present in the last iMCU row.
         */
        tmp = ((*compptr).height_in_blocks % (*compptr).v_samp_factor as c_uint) as c_int;
        if tmp == 0i32 {
            tmp = (*compptr).v_samp_factor
        }
        (*compptr).last_row_height = tmp;
        /* Prepare array describing MCU composition */
        (*cinfo).blocks_in_MCU = 1i32;
        (*cinfo).MCU_membership[0] = 0i32
    } else {
        if (*cinfo).comps_in_scan <= 0i32 || (*cinfo).comps_in_scan > MAX_COMPS_IN_SCAN {
            (*(*cinfo).err).msg_code = super::jerror::JERR_COMPONENT_COUNT as c_int;
            (*(*cinfo).err).msg_parm.i[0] = (*cinfo).comps_in_scan;
            (*(*cinfo).err).msg_parm.i[1] = 4i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        /* Overall image size in MCUs */
        (*cinfo).MCUs_per_row = jdiv_round_up(
            (*cinfo).image_width as c_long,
            ((*cinfo).max_h_samp_factor * DCTSIZE) as c_long,
        ) as JDIMENSION;
        (*cinfo).MCU_rows_in_scan = jdiv_round_up(
            (*cinfo).image_height as c_long,
            ((*cinfo).max_v_samp_factor * DCTSIZE) as c_long,
        ) as JDIMENSION;
        (*cinfo).blocks_in_MCU = 0i32;
        let mut ci: c_int = 0i32;
        while ci < (*cinfo).comps_in_scan {
            compptr = (*cinfo).cur_comp_info[ci as usize];
            /* Sampling factors give # of blocks of component in each MCU */
            (*compptr).MCU_width = (*compptr).h_samp_factor;
            (*compptr).MCU_height = (*compptr).v_samp_factor;
            (*compptr).MCU_blocks = (*compptr).MCU_width * (*compptr).MCU_height;
            (*compptr).MCU_sample_width = (*compptr).MCU_width * (*compptr).DCT_scaled_size;
            /* Figure number of non-dummy blocks in last MCU column & row */
            tmp = ((*compptr).width_in_blocks % (*compptr).MCU_width as c_uint) as c_int;
            if tmp == 0i32 {
                tmp = (*compptr).MCU_width
            }
            (*compptr).last_col_width = tmp;
            tmp = ((*compptr).height_in_blocks % (*compptr).MCU_height as c_uint) as c_int;
            if tmp == 0i32 {
                tmp = (*compptr).MCU_height
            }
            (*compptr).last_row_height = tmp;
            let mut mcublks: c_int = (*compptr).MCU_blocks;
            if (*cinfo).blocks_in_MCU + mcublks > D_MAX_BLOCKS_IN_MCU {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_MCU_SIZE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            loop {
                let fresh0 = mcublks;
                mcublks -= 1;
                if !(fresh0 > 0i32) {
                    break;
                }
                let fresh1 = (*cinfo).blocks_in_MCU;
                (*cinfo).blocks_in_MCU = (*cinfo).blocks_in_MCU + 1;
                (*cinfo).MCU_membership[fresh1 as usize] = ci
            }
            ci += 1
        }
    };
}
/*
 * Save away a copy of the Q-table referenced by each component present
 * in the current scan, unless already saved during a prior scan.
 *
 * In a multiple-scan JPEG file, the encoder could assign different components
 * the same Q-table slot number, but change table definitions between scans
 * so that each component uses a different Q-table.  (The IJG encoder is not
 * currently capable of doing this, but other encoders might.)  Since we want
 * to be able to dequantize all the components at the end of the file, this
 * means that we have to save away the table actually used for each component.
 * We do this by copying the table at the start of the first scan containing
 * the component.
 * Rec. ITU-T T.81 | ISO/IEC 10918-1 prohibits the encoder from changing the
 * contents of a Q-table slot between scans of a component using that slot.  If
 * the encoder does so anyway, this decoder will simply use the Q-table values
 * that were current at the start of the first scan for the component.
 *
 * The decompressor output side looks only at the saved quant tables,
 * not at the current Q-table slots.
 */

unsafe extern "C" fn latch_quant_tables(mut cinfo: j_decompress_ptr) {
    let mut ci: c_int = 0i32;
    while ci < (*cinfo).comps_in_scan {
        let mut compptr: *mut jpeg_component_info = (*cinfo).cur_comp_info[ci as usize];
        /* No work if we already saved Q-table for this component */
        if (*compptr).quant_table.is_null() {
            let mut qtblno: c_int = (*compptr).quant_tbl_no;
            if qtblno < 0i32
                || qtblno >= NUM_QUANT_TBLS
                || (*cinfo).quant_tbl_ptrs[qtblno as usize].is_null()
            {
                (*(*cinfo).err).msg_code = super::jerror::JERR_NO_QUANT_TABLE as c_int;
                (*(*cinfo).err).msg_parm.i[0] = qtblno;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            let mut qtbl: *mut JQUANT_TBL = Some(
                (*(*cinfo).mem)
                    .alloc_small
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                JPOOL_IMAGE,
                ::std::mem::size_of::<JQUANT_TBL>() as c_ulong,
            ) as *mut JQUANT_TBL;
            memcpy(
                qtbl as *mut c_void,
                (*cinfo).quant_tbl_ptrs[qtblno as usize] as *const c_void,
                ::std::mem::size_of::<JQUANT_TBL>() as c_ulong,
            );
            (*compptr).quant_table = qtbl
        }
        ci += 1
    }
}
/*
 * Initialize the input modules to read a scan of compressed data.
 * The first call to this is done by jdmaster.c after initializing
 * the entire decompressor (during jpeg_start_decompress).
 * Subsequent calls come from consume_markers, below.
 */

unsafe extern "C" fn start_input_pass(mut cinfo: j_decompress_ptr) {
    per_scan_setup(cinfo);
    latch_quant_tables(cinfo);
    Some(
        (*(*cinfo).entropy)
            .start_pass
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    Some(
        (*(*cinfo).coef)
            .start_input_pass
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    (*(*cinfo).inputctl).consume_input = (*(*cinfo).coef).consume_data;
}
/*
 * Finish up after inputting a compressed-data scan.
 * This is called by the coefficient controller after it's read all
 * the expected data of the scan.
 */

unsafe extern "C" fn finish_input_pass(mut cinfo: j_decompress_ptr) {
    (*(*cinfo).inputctl).consume_input =
        Some(consume_markers as unsafe extern "C" fn(_: j_decompress_ptr) -> c_int);
}
/* Forward declarations */
/*
 * Read JPEG markers before, between, or after compressed-data scans.
 * Change state as necessary when a new scan is reached.
 * Return value is JPEG_SUSPENDED, JPEG_REACHED_SOS, or JPEG_REACHED_EOI.
 *
 * The consume_input method pointer points either here or to the
 * coefficient controller's consume_data routine, depending on whether
 * we are reading a compressed data segment or inter-segment markers.
 */

unsafe extern "C" fn consume_markers(mut cinfo: j_decompress_ptr) -> c_int {
    let mut inputctl: my_inputctl_ptr = (*cinfo).inputctl as my_inputctl_ptr;

    if (*inputctl).pub_0.eoi_reached != 0 {
        /* After hitting EOI, read no further */
        return 2i32;
    }
    let mut val: c_int = Some(
        (*(*cinfo).marker)
            .read_markers
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    match val {
        1 => {
            /* Found SOS */
            if (*inputctl).inheaders != 0 {
                /* 1st SOS */
                initial_setup(cinfo);
                (*inputctl).inheaders = FALSE
            /* Note: start_input_pass must be called by jdmaster.c
             * before any more input can be consumed.  jdapimin.c is
             * responsible for enforcing this sequencing.
             */
            } else {
                if (*inputctl).pub_0.has_multiple_scans == 0 {
                    (*(*cinfo).err).msg_code = super::jerror::JERR_EOI_EXPECTED as c_int; /* Oops, I wasn't expecting this! */
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
                }
                start_input_pass(cinfo);
            }
        }
        2 => {
            /* Found EOI */
            (*inputctl).pub_0.eoi_reached = TRUE;
            if (*inputctl).inheaders != 0 {
                /* Tables-only datastream, apparently */
                if (*(*cinfo).marker).saw_SOF != 0 {
                    (*(*cinfo).err).msg_code = super::jerror::JERR_SOF_NO_SOS as c_int;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
                }
            } else if (*cinfo).output_scan_number > (*cinfo).input_scan_number {
                (*cinfo).output_scan_number = (*cinfo).input_scan_number
            }
        }
        0 | _ => {}
    }
    return val;
}
/* Prevent infinite loop in coef ctlr's decompress_data routine
 * if user set output_scan_number larger than number of scans.
 */
/*
 * Reset state to begin a fresh datastream.
 */

unsafe extern "C" fn reset_input_controller(mut cinfo: j_decompress_ptr) {
    let mut inputctl: my_inputctl_ptr = (*cinfo).inputctl as my_inputctl_ptr; /* "unknown" would be better */
    (*inputctl).pub_0.consume_input =
        Some(consume_markers as unsafe extern "C" fn(_: j_decompress_ptr) -> c_int);
    (*inputctl).pub_0.has_multiple_scans = FALSE;
    (*inputctl).pub_0.eoi_reached = FALSE;
    (*inputctl).inheaders = TRUE;
    /* Reset other modules */
    Some(
        (*(*cinfo).err)
            .reset_error_mgr
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as j_common_ptr);
    Some(
        (*(*cinfo).marker)
            .reset_marker_reader
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    /* Reset progression state -- would be cleaner if entropy decoder did this */
    (*cinfo).coef_bits = NULL as *mut [c_int; 64];
}
/*
 * Initialize the input controller module.
 * This is called only once, when the decompression object is created.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_input_controller(mut cinfo: j_decompress_ptr) {
    let mut inputctl: my_inputctl_ptr = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_PERMANENT,
        ::std::mem::size_of::<my_input_controller>() as c_ulong,
    ) as my_inputctl_ptr;
    (*cinfo).inputctl = inputctl as *mut jpeg_input_controller;
    /* Initialize method pointers */
    (*inputctl).pub_0.consume_input =
        Some(consume_markers as unsafe extern "C" fn(_: j_decompress_ptr) -> c_int);
    (*inputctl).pub_0.reset_input_controller =
        Some(reset_input_controller as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*inputctl).pub_0.start_input_pass =
        Some(start_input_pass as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*inputctl).pub_0.finish_input_pass =
        Some(finish_input_pass as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    /* Initialize state: can't use reset_input_controller since we don't
     * want to try to reset other modules yet.
     */
    (*inputctl).pub_0.has_multiple_scans = FALSE; /* "unknown" would be better */
    (*inputctl).pub_0.eoi_reached = FALSE;
    (*inputctl).inheaders = TRUE;
}
