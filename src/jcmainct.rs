use libc::c_uint;use libc::c_ulong;use libc::c_int;use libc;

/* JERROR_H */

/* Informational/debugging messages */

/* Nonfatal errors (we can keep going, but the data is probably corrupt) */

/* Fatal errors (print message and exit) */

/* Macros to simplify using the error and trace message stuff */

/* The first parameter is either type of cinfo pointer */

/* Zap JMESSAGE macro so that future re-inclusions do nothing by default */

/* JMAKE_ENUM_LIST */
pub use crate::stddef_h::size_t;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_c_coef_controller;
pub use crate::jpeglib_h::jpeg_c_main_controller;
pub use crate::jpeglib_h::jpeg_c_prep_controller;
pub use crate::jpeglib_h::jpeg_color_converter;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_comp_master;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
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
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::DCTSIZE;
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

pub type my_main_ptr = *mut my_main_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_main_controller {
    pub pub_0: jpeg_c_main_controller,
    pub cur_iMCU_row: JDIMENSION,
    pub rowgroup_ctr: JDIMENSION,
    pub suspended: boolean,
    pub pass_mode: J_BUF_MODE,
    pub buffer: [JSAMPARRAY; 10],
}
/*
 * Initialize for a processing pass.
 */

unsafe extern "C" fn start_pass_main(
    mut cinfo: j_compress_ptr,
    mut pass_mode: J_BUF_MODE,
) {
    let mut main_ptr: my_main_ptr = (*cinfo).main as my_main_ptr;
    /* Do nothing in raw-data mode. */
    if (*cinfo).raw_data_in != 0 {
        return;
    } /* initialize counters */
    if  pass_mode !=  JBUF_PASS_THRU
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_BUFFER_MODE as c_int; /* save mode for use by process_data */
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*main_ptr).cur_iMCU_row = 0u32;
    (*main_ptr).rowgroup_ctr = 0u32;
    (*main_ptr).suspended = FALSE;
    (*main_ptr).pass_mode = pass_mode;
    (*main_ptr).pub_0.process_data = Some(
        process_data_simple_main
            as unsafe extern "C" fn(
                _: j_compress_ptr,
                _: JSAMPARRAY,
                _: *mut JDIMENSION,
                _: JDIMENSION,
            ) -> (),
    );
}
/* Forward declarations */
/*
 * Process some data.
 * This routine handles the simple pass-through mode,
 * where we have only a strip buffer.
 */

unsafe extern "C" fn process_data_simple_main(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut in_row_ctr: *mut JDIMENSION,
    mut in_rows_avail: JDIMENSION,
) {
    let mut main_ptr: my_main_ptr = (*cinfo).main as my_main_ptr;
    while (*main_ptr).cur_iMCU_row < (*cinfo).total_iMCU_rows {
        /* Read input data if we haven't filled the main buffer yet */
        if (*main_ptr).rowgroup_ctr < DCTSIZE as c_uint {
            Some(
                (*(*cinfo).prep)
                    .pre_process_data
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                input_buf,
                in_row_ctr,
                in_rows_avail,
                (*main_ptr).buffer.as_mut_ptr(),
                &mut (*main_ptr).rowgroup_ctr,
                DCTSIZE as JDIMENSION,
            );
        }
        /* If we don't have a full iMCU row buffered, return to application for
         * more data.  Note that preprocessor will always pad to fill the iMCU row
         * at the bottom of the image.
         */
        if (*main_ptr).rowgroup_ctr != DCTSIZE as c_uint {
            return;
        }
        /* Send the completed row to the compressor */
        if Some(
            (*(*cinfo).coef)
                .compress_data
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo, (*main_ptr).buffer.as_mut_ptr())
            == 0
        {
            /* If compressor did not consume the whole row, then we must need to
             * suspend processing and return to the application.  In this situation
             * we pretend we didn't yet consume the last input row; otherwise, if
             * it happened to be the last row of the image, the application would
             * think we were done.
             */
            if (*main_ptr).suspended == 0 {
                *in_row_ctr = *in_row_ctr - 1;
                (*main_ptr).suspended = TRUE
            }
            return;
        }
        /* We did finish the row.  Undo our little suspension hack if a previous
         * call suspended; then mark the main buffer empty.
         */
        if (*main_ptr).suspended != 0 {
            *in_row_ctr = *in_row_ctr + 1;
            (*main_ptr).suspended = FALSE
        }
        (*main_ptr).rowgroup_ctr = 0u32;
        (*main_ptr).cur_iMCU_row =  (*main_ptr).cur_iMCU_row + 1
    }
}
/*
 * Initialize main buffer controller.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_c_main_controller(
    mut cinfo: j_compress_ptr,
    mut need_full_buffer: boolean,
) {
    
    
     
     let mut main_ptr:   my_main_ptr =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<my_main_controller>() as c_ulong,
    ) as my_main_ptr;
    (*cinfo).main = main_ptr as *mut jpeg_c_main_controller;
    (*main_ptr).pub_0.start_pass = Some(
        start_pass_main
            as unsafe extern "C" fn(
                _: j_compress_ptr,
                _: J_BUF_MODE,
            ) -> (),
    );
    /* We don't need to create a buffer in raw-data mode. */
    if (*cinfo).raw_data_in != 0 {
        return;
    }
    /* Create the buffer.  It holds downsampled data, so each component
     * may be of a different size.
     */
    if need_full_buffer != 0 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_BUFFER_MODE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    } else {
         
         let mut ci:   c_int =  0i32; let mut compptr:   *mut jpeg_component_info =
     (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
            (*main_ptr).buffer[ci as usize] = Some(
                (*(*cinfo).mem)
                    .alloc_sarray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                JPOOL_IMAGE,
                
                (*compptr)
                    .width_in_blocks * DCTSIZE as c_uint,
                ((*compptr).v_samp_factor * DCTSIZE)
                    as JDIMENSION,
            );
            ci += 1;
            compptr = compptr.offset(1)
        }
    };
}
