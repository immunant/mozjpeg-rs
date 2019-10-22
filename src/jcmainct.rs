use libc;

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
pub use crate::src::jerror::C2RustUnnamed_3;
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

pub type my_main_ptr = *mut my_main_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_main_controller {
    pub pub_0: crate::jpeglib_h::jpeg_c_main_controller,
    pub cur_iMCU_row: crate::jmorecfg_h::JDIMENSION,
    pub rowgroup_ctr: crate::jmorecfg_h::JDIMENSION,
    pub suspended: crate::jmorecfg_h::boolean,
    pub pass_mode: crate::jpegint_h::J_BUF_MODE,
    pub buffer: [crate::jpeglib_h::JSAMPARRAY; 10],
}
/*
 * Initialize for a processing pass.
 */

unsafe extern "C" fn start_pass_main(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut pass_mode: crate::jpegint_h::J_BUF_MODE,
) {
    let mut main_ptr: my_main_ptr = (*cinfo).main as my_main_ptr;
    /* Do nothing in raw-data mode. */
    if (*cinfo).raw_data_in != 0 {
        return;
    } /* initialize counters */
    if pass_mode as libc::c_uint != crate::jpegint_h::JBUF_PASS_THRU as libc::c_int as libc::c_uint
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_BUFFER_MODE as libc::c_int; /* save mode for use by process_data */
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    (*main_ptr).cur_iMCU_row = 0i32 as crate::jmorecfg_h::JDIMENSION;
    (*main_ptr).rowgroup_ctr = 0i32 as crate::jmorecfg_h::JDIMENSION;
    (*main_ptr).suspended = crate::jmorecfg_h::FALSE;
    (*main_ptr).pass_mode = pass_mode;
    (*main_ptr).pub_0.process_data = Some(
        process_data_simple_main
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_compress_ptr,
                _: crate::jpeglib_h::JSAMPARRAY,
                _: *mut crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::JDIMENSION,
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
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut in_row_ctr: *mut crate::jmorecfg_h::JDIMENSION,
    mut in_rows_avail: crate::jmorecfg_h::JDIMENSION,
) {
    let mut main_ptr: my_main_ptr = (*cinfo).main as my_main_ptr;
    while (*main_ptr).cur_iMCU_row < (*cinfo).total_iMCU_rows {
        /* Read input data if we haven't filled the main buffer yet */
        if (*main_ptr).rowgroup_ctr < crate::jpeglib_h::DCTSIZE as libc::c_uint {
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
                crate::jpeglib_h::DCTSIZE as crate::jmorecfg_h::JDIMENSION,
            );
        }
        /* If we don't have a full iMCU row buffered, return to application for
         * more data.  Note that preprocessor will always pad to fill the iMCU row
         * at the bottom of the image.
         */
        if (*main_ptr).rowgroup_ctr != crate::jpeglib_h::DCTSIZE as libc::c_uint {
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
                *in_row_ctr = (*in_row_ctr).wrapping_sub(1);
                (*main_ptr).suspended = crate::jmorecfg_h::TRUE
            }
            return;
        }
        /* We did finish the row.  Undo our little suspension hack if a previous
         * call suspended; then mark the main buffer empty.
         */
        if (*main_ptr).suspended != 0 {
            *in_row_ctr = (*in_row_ctr).wrapping_add(1);
            (*main_ptr).suspended = crate::jmorecfg_h::FALSE
        }
        (*main_ptr).rowgroup_ctr = 0i32 as crate::jmorecfg_h::JDIMENSION;
        (*main_ptr).cur_iMCU_row = (*main_ptr).cur_iMCU_row.wrapping_add(1)
    }
}
/*
 * Initialize main buffer controller.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_c_main_controller(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut need_full_buffer: crate::jmorecfg_h::boolean,
) {
    let mut main_ptr: my_main_ptr = ::std::ptr::null_mut::< my_main_controller>();
    let mut ci: libc::c_int = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_component_info>();
    main_ptr = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<my_main_controller>() as libc::c_ulong,
    ) as my_main_ptr;
    (*cinfo).main = main_ptr as *mut crate::jpeglib_h::jpeg_c_main_controller;
    (*main_ptr).pub_0.start_pass = Some(
        start_pass_main
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_compress_ptr,
                _: crate::jpegint_h::J_BUF_MODE,
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
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_BUFFER_MODE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    } else {
        /* Allocate a strip buffer for each component */
        ci = 0i32;
        compptr = (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
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
                    .wrapping_mul(crate::jpeglib_h::DCTSIZE as libc::c_uint),
                ((*compptr).v_samp_factor * crate::jpeglib_h::DCTSIZE)
                    as crate::jmorecfg_h::JDIMENSION,
            );
            ci += 1;
            compptr = compptr.offset(1)
        }
    };
}
