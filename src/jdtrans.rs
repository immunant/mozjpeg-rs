pub use crate::jerror::{
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
pub use crate::jmorecfg_h::{boolean, JCOEF, JDIMENSION, JOCTET, JSAMPLE, TRUE, UINT16, UINT8};
pub use crate::jpegint_h::{
    inverse_DCT_method_ptr, jinit_d_coef_controller, jinit_huff_decoder, jinit_phuff_decoder,
    jpeg_color_deconverter, jpeg_color_quantizer, jpeg_d_coef_controller, jpeg_d_main_controller,
    jpeg_d_post_controller, jpeg_decomp_master, jpeg_entropy_decoder, jpeg_input_controller,
    jpeg_inverse_dct, jpeg_marker_reader, jpeg_upsampler, DSTATE_BUFIMAGE, DSTATE_RDCOEFS,
    DSTATE_READY, DSTATE_STOPPING, JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT,
    JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_decompress_ptr, jpeg_common_struct, jpeg_component_info,
    jpeg_decompress_struct, jpeg_error_mgr, jpeg_marker_parser_method, jpeg_marker_struct,
    jpeg_memory_mgr, jpeg_progress_mgr, jpeg_saved_marker_ptr, jpeg_source_mgr,
    jvirt_barray_control, jvirt_barray_ptr, jvirt_sarray_control, jvirt_sarray_ptr,
    C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY, JBLOCKROW, JCOEFPTR, JCS_CMYK, JCS_EXT_ABGR,
    JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX,
    JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK,
    JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JDITHER_FS, JDITHER_NONE, JDITHER_ORDERED, JHUFF_TBL,
    JPEG_REACHED_EOI, JPEG_REACHED_SOS, JPEG_ROW_COMPLETED, JPEG_SUSPENDED, JQUANT_TBL, JSAMPARRAY,
    JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD, J_DITHER_MODE,
};
pub use crate::stddef_h::{size_t, NULL};
use libc::{self, c_int, c_long};
/* Read or write raw DCT coefficients --- useful for lossless transcoding. */
/*
 * Read the coefficient arrays from a JPEG file.
 * jpeg_read_header must be completed before calling this.
 *
 * The entire image is read into a set of virtual coefficient-block arrays,
 * one per component.  The return value is a pointer to the array of
 * virtual-array descriptors.  These can be manipulated directly via the
 * JPEG memory manager, or handed off to jpeg_write_coefficients().
 * To release the memory occupied by the virtual arrays, call
 * jpeg_finish_decompress() when done with the data.
 *
 * An alternative usage is to simply obtain access to the coefficient arrays
 * during a buffered-image-mode decompression operation.  This is allowed
 * after any jpeg_finish_output() call.  The arrays can be accessed until
 * jpeg_finish_decompress() is called.  (Note that any call to the library
 * may reposition the arrays, so don't rely on access_virt_barray() results
 * to stay valid across library calls.)
 *
 * Returns NULL if suspended.  This case need be checked only if
 * a suspending data source is used.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_read_coefficients(
    mut cinfo: j_decompress_ptr,
) -> *mut jvirt_barray_ptr {
    if (*cinfo).global_state == DSTATE_READY {
        transdecode_master_selection(cinfo);
        (*cinfo).global_state = DSTATE_RDCOEFS
    }
    if (*cinfo).global_state == DSTATE_RDCOEFS {
        loop {
            let mut retcode: c_int = 0;
            if !(*cinfo).progress.is_null() {
                (*(*cinfo).progress)
                    .progress_monitor
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            retcode = (*(*cinfo).inputctl)
                .consume_input
                .expect("non-null function pointer")(cinfo);
            if retcode == JPEG_SUSPENDED {
                return NULL as *mut jvirt_barray_ptr;
            }
            if retcode == JPEG_REACHED_EOI {
                break;
            }
            if !(*cinfo).progress.is_null()
                && (retcode == JPEG_ROW_COMPLETED || retcode == JPEG_REACHED_SOS)
            {
                (*(*cinfo).progress).pass_counter += 1;
                if (*(*cinfo).progress).pass_counter >= (*(*cinfo).progress).pass_limit {
                    (*(*cinfo).progress).pass_limit += (*cinfo).total_iMCU_rows as c_long
                }
            }
        }
        (*cinfo).global_state = DSTATE_STOPPING
    }
    if ((*cinfo).global_state == DSTATE_STOPPING || (*cinfo).global_state == DSTATE_BUFIMAGE)
        && 0 != (*cinfo).buffered_image
    {
        return (*(*cinfo).coef).coef_arrays;
    }
    (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
    (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
    (*(*cinfo).err)
        .error_exit
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    return NULL as *mut jvirt_barray_ptr;
}
/*
 * jdtrans.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1995-1997, Thomas G. Lane.
 * It was modified by The libjpeg-turbo Project to include only code relevant
 * to libjpeg-turbo.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains library routines for transcoding decompression,
 * that is, reading raw DCT coefficient arrays from an input JPEG file.
 * The routines in jdapimin.c will also be needed by a transcoder.
 */
/* Forward declarations */
/*
 * Master selection of decompression modules for transcoding.
 * This substitutes for jdmaster.c's initialization of the full decompressor.
 */
unsafe extern "C" fn transdecode_master_selection(mut cinfo: j_decompress_ptr) {
    (*cinfo).buffered_image = TRUE;
    if 0 != (*cinfo).arith_code {
        (*(*cinfo).err).msg_code = JERR_ARITH_NOTIMPL as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    } else if 0 != (*cinfo).progressive_mode {
        jinit_phuff_decoder(cinfo);
    } else {
        jinit_huff_decoder(cinfo);
    }
    jinit_d_coef_controller(cinfo, TRUE);
    (*(*cinfo).mem)
        .realize_virt_arrays
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    (*(*cinfo).inputctl)
        .start_input_pass
        .expect("non-null function pointer")(cinfo);
    if !(*cinfo).progress.is_null() {
        let mut nscans: c_int = 0;
        if 0 != (*cinfo).progressive_mode {
            nscans = 2i32 + 3i32 * (*cinfo).num_components
        } else if 0 != (*(*cinfo).inputctl).has_multiple_scans {
            nscans = (*cinfo).num_components
        } else {
            nscans = 1i32
        }
        (*(*cinfo).progress).pass_counter = 0i64;
        (*(*cinfo).progress).pass_limit = (*cinfo).total_iMCU_rows as c_long * nscans as c_long;
        (*(*cinfo).progress).completed_passes = 0i32;
        (*(*cinfo).progress).total_passes = 1i32
    };
}
