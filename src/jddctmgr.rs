pub use super::jdmaster::{my_decomp_master, my_master_ptr};
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
pub use crate::jdct_h::{
    jpeg_idct_10x10, jpeg_idct_11x11, jpeg_idct_12x12, jpeg_idct_13x13, jpeg_idct_14x14,
    jpeg_idct_15x15, jpeg_idct_16x16, jpeg_idct_1x1, jpeg_idct_2x2, jpeg_idct_3x3, jpeg_idct_4x4,
    jpeg_idct_5x5, jpeg_idct_6x6, jpeg_idct_7x7, jpeg_idct_9x9, jpeg_idct_float, jpeg_idct_ifast,
    jpeg_idct_islow, FLOAT_MULT_TYPE, IFAST_MULT_TYPE, ISLOW_MULT_TYPE,
};
pub use crate::jmorecfg_h::{boolean, INT16, JCOEF, JDIMENSION, JOCTET, JSAMPLE, UINT16, UINT8};
pub use crate::jpegint_h::{
    inverse_DCT_method_ptr, JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS,
    JBUF_SAVE_SOURCE, JLONG, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_decompress_ptr, jpeg_color_deconverter, jpeg_color_quantizer,
    jpeg_common_struct, jpeg_component_info, jpeg_d_coef_controller, jpeg_d_main_controller,
    jpeg_d_post_controller, jpeg_decomp_master, jpeg_decompress_struct, jpeg_entropy_decoder,
    jpeg_error_mgr, jpeg_idct_method, jpeg_idct_method_selector, jpeg_input_controller,
    jpeg_inverse_dct, jpeg_marker_parser_method, jpeg_marker_reader, jpeg_marker_struct,
    jpeg_memory_mgr, jpeg_progress_mgr, jpeg_saved_marker_ptr, jpeg_source_mgr, jpeg_upsampler,
    jvirt_barray_control, jvirt_barray_ptr, jvirt_sarray_control, jvirt_sarray_ptr,
    C2RustUnnamed_2, JCS_YCbCr, DCTSIZE2, JBLOCK, JBLOCKARRAY, JBLOCKROW, JCOEFPTR, JCS_CMYK,
    JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA,
    JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN,
    JCS_YCCK, JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JDITHER_FS, JDITHER_NONE, JDITHER_ORDERED,
    JHUFF_TBL, JPOOL_IMAGE, JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE,
    J_DCT_METHOD, J_DITHER_MODE,
};
pub use crate::stddef_h::{size_t, NULL};
use crate::stdlib::memset;
use libc::{self, c_double, c_int, c_ulong, intptr_t};

pub type my_idct_ptr = *mut my_idct_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_idct_controller {
    pub pub_0: jpeg_inverse_dct,
    pub cur_method: [c_int; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union multiplier_table {
    pub islow_array: [ISLOW_MULT_TYPE; 64],
    pub ifast_array: [IFAST_MULT_TYPE; 64],
    pub float_array: [FLOAT_MULT_TYPE; 64],
}
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
/* Generic versions of jpeg_abort and jpeg_destroy that work on either
 * flavor of JPEG object.  These may be more convenient in some places.
 */
/* Default restart-marker-resync procedure for use by data source modules */
/* Accessor functions for extension parameters */
/* Read ICC profile.  See libjpeg.txt for usage information. */
/*
 * Permit users to replace the IDCT method dynamically.
 * The selector callback is called after the default idct implementation was choosen,
 * and is able to override it.
 */
/* The current scaled-IDCT routines require ISLOW-style multiplier tables,
 * so be sure to compile that code if either ISLOW or SCALING is requested.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_set_idct_method_selector(
    mut cinfo: j_decompress_ptr,
    mut selector: jpeg_idct_method_selector,
) {
    let mut master: super::jdmaster::my_master_ptr =
        (*cinfo).master as super::jdmaster::my_master_ptr;
    (*master).custom_idct_selector = selector;
}
/*
 * Prepare for an output pass.
 * Here we select the proper IDCT routine for each component and build
 * a matching multiplier table.
 */

unsafe extern "C" fn start_pass(mut cinfo: j_decompress_ptr) {
    let mut idct: my_idct_ptr = (*cinfo).idct as my_idct_ptr;

    let mut method_ptr: inverse_DCT_method_ptr =
        ::std::mem::transmute::<intptr_t, inverse_DCT_method_ptr>(NULL as intptr_t);

    let mut ci: c_int = 0i32;
    let mut compptr: *mut jpeg_component_info = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Select the proper IDCT routine for this component's scaling */
        let mut method: c_int = 0i32;
        match (*compptr).DCT_scaled_size {
            1 => {
                method_ptr = Some(
                    jpeg_idct_1x1
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctred uses islow-style table */
                method = JDCT_ISLOW as c_int
            }
            2 => {
                if super::simd::x86_64::jsimd::jsimd_can_idct_2x2() != 0 {
                    method_ptr = Some(
                        super::simd::x86_64::jsimd::jsimd_idct_2x2
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: *mut jpeg_component_info,
                                _: JCOEFPTR,
                                _: JSAMPARRAY,
                                _: JDIMENSION,
                            ) -> (),
                    )
                } else {
                    method_ptr = Some(
                        jpeg_idct_2x2
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: *mut jpeg_component_info,
                                _: JCOEFPTR,
                                _: JSAMPARRAY,
                                _: JDIMENSION,
                            ) -> (),
                    )
                } /* jidctred uses islow-style table */
                method = JDCT_ISLOW as c_int
            }
            3 => {
                method_ptr = Some(
                    jpeg_idct_3x3
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as c_int
            }
            4 => {
                if super::simd::x86_64::jsimd::jsimd_can_idct_4x4() != 0 {
                    method_ptr = Some(
                        super::simd::x86_64::jsimd::jsimd_idct_4x4
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: *mut jpeg_component_info,
                                _: JCOEFPTR,
                                _: JSAMPARRAY,
                                _: JDIMENSION,
                            ) -> (),
                    )
                } else {
                    method_ptr = Some(
                        jpeg_idct_4x4
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: *mut jpeg_component_info,
                                _: JCOEFPTR,
                                _: JSAMPARRAY,
                                _: JDIMENSION,
                            ) -> (),
                    )
                } /* jidctred uses islow-style table */
                method = JDCT_ISLOW as c_int
            }
            5 => {
                method_ptr = Some(
                    jpeg_idct_5x5
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as c_int
            }
            6 => {
                method_ptr = Some(
                    jpeg_idct_6x6
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as c_int
            }
            7 => {
                method_ptr = Some(
                    jpeg_idct_7x7
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as c_int
            }
            8 => {
                match (*cinfo).dct_method {
                    0 => {
                        if super::simd::x86_64::jsimd::jsimd_can_idct_islow() != 0 {
                            method_ptr = Some(
                                super::simd::x86_64::jsimd::jsimd_idct_islow
                                    as unsafe extern "C" fn(
                                        _: j_decompress_ptr,
                                        _: *mut jpeg_component_info,
                                        _: JCOEFPTR,
                                        _: JSAMPARRAY,
                                        _: JDIMENSION,
                                    )
                                        -> (),
                            )
                        } else {
                            method_ptr = Some(
                                jpeg_idct_islow
                                    as unsafe extern "C" fn(
                                        _: j_decompress_ptr,
                                        _: *mut jpeg_component_info,
                                        _: JCOEFPTR,
                                        _: JSAMPARRAY,
                                        _: JDIMENSION,
                                    )
                                        -> (),
                            )
                        } /* jidctint uses islow-style table */
                        method = JDCT_ISLOW as c_int
                    }
                    1 => {
                        if super::simd::x86_64::jsimd::jsimd_can_idct_ifast() != 0 {
                            method_ptr = Some(
                                super::simd::x86_64::jsimd::jsimd_idct_ifast
                                    as unsafe extern "C" fn(
                                        _: j_decompress_ptr,
                                        _: *mut jpeg_component_info,
                                        _: JCOEFPTR,
                                        _: JSAMPARRAY,
                                        _: JDIMENSION,
                                    )
                                        -> (),
                            )
                        } else {
                            method_ptr = Some(
                                jpeg_idct_ifast
                                    as unsafe extern "C" fn(
                                        _: j_decompress_ptr,
                                        _: *mut jpeg_component_info,
                                        _: JCOEFPTR,
                                        _: JSAMPARRAY,
                                        _: JDIMENSION,
                                    )
                                        -> (),
                            )
                        } /* jidctint uses islow-style table */
                        method = JDCT_IFAST as c_int
                    }
                    2 => {
                        if super::simd::x86_64::jsimd::jsimd_can_idct_float() != 0 {
                            method_ptr = Some(
                                super::simd::x86_64::jsimd::jsimd_idct_float
                                    as unsafe extern "C" fn(
                                        _: j_decompress_ptr,
                                        _: *mut jpeg_component_info,
                                        _: JCOEFPTR,
                                        _: JSAMPARRAY,
                                        _: JDIMENSION,
                                    )
                                        -> (),
                            )
                        } else {
                            method_ptr = Some(
                                jpeg_idct_float
                                    as unsafe extern "C" fn(
                                        _: j_decompress_ptr,
                                        _: *mut jpeg_component_info,
                                        _: JCOEFPTR,
                                        _: JSAMPARRAY,
                                        _: JDIMENSION,
                                    )
                                        -> (),
                            )
                        } /* jidctint uses islow-style table */
                        method = JDCT_FLOAT as c_int
                    }
                    _ => {
                        (*(*cinfo).err).msg_code = super::jerror::JERR_NOT_COMPILED as c_int; /* jidctint uses islow-style table */
                        Some(
                            (*(*cinfo).err)
                                .error_exit
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as j_common_ptr
                        ); /* jidctint uses islow-style table */
                    }
                }
            }
            9 => {
                method_ptr = Some(
                    jpeg_idct_9x9
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as c_int
            }
            10 => {
                method_ptr = Some(
                    jpeg_idct_10x10
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as c_int
            }
            11 => {
                method_ptr = Some(
                    jpeg_idct_11x11
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as c_int
            }
            12 => {
                method_ptr = Some(
                    jpeg_idct_12x12
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                );
                method = JDCT_ISLOW as c_int
            }
            13 => {
                method_ptr = Some(
                    jpeg_idct_13x13
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                );
                method = JDCT_ISLOW as c_int
            }
            14 => {
                method_ptr = Some(
                    jpeg_idct_14x14
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                );
                method = JDCT_ISLOW as c_int
            }
            15 => {
                method_ptr = Some(
                    jpeg_idct_15x15
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                );
                method = JDCT_ISLOW as c_int
            }
            16 => {
                method_ptr = Some(
                    jpeg_idct_16x16
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                );
                method = JDCT_ISLOW as c_int
            }
            _ => {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_DCTSIZE as c_int;
                (*(*cinfo).err).msg_parm.i[0] = (*compptr).DCT_scaled_size;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        let mut master: super::jdmaster::my_master_ptr =
            (*cinfo).master as super::jdmaster::my_master_ptr;
        if (*master).custom_idct_selector.is_some() {
            (*master)
                .custom_idct_selector
                .expect("non-null function pointer")(
                cinfo, compptr, &mut method_ptr, &mut method
            );
        }
        (*idct).pub_0.inverse_DCT[ci as usize] = method_ptr;
        /* Create multiplier table from quant table.
         * However, we can skip this if the component is uninteresting
         * or if we already built the table.  Also, if no quant table
         * has yet been saved for the component, we leave the
         * multiplier table all-zero; we'll be reading zeroes from the
         * coefficient controller's buffer anyway.
         */
        if !((*compptr).component_needed == 0 || (*idct).cur_method[ci as usize] == method) {
            let mut qtbl: *mut JQUANT_TBL = (*compptr).quant_table;
            if !qtbl.is_null() {
                let mut i: c_int = 0;
                (*idct).cur_method[ci as usize] = method;
                match method {
                    0 => {
                        /* For LL&M IDCT method, multipliers are equal to raw quantization
                         * coefficients, but are stored as ints to ensure access efficiency.
                         */
                        let mut ismtbl: *mut ISLOW_MULT_TYPE =
                            (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
                        i = 0i32;
                        while i < DCTSIZE2 {
                            *ismtbl.offset(i as isize) =
                                (*qtbl).quantval[i as usize] as ISLOW_MULT_TYPE;
                            i += 1
                        }
                    }
                    1 => {
                        /* For AA&N IDCT method, multipliers are equal to quantization
                         * coefficients scaled by scalefactor[row]*scalefactor[col], where
                         *   scalefactor[0] = 1
                         *   scalefactor[k] = cos(k*PI/16) * sqrt(2)    for k=1..7
                         * For integer operation, the multiplier table is to be scaled by
                         * IFAST_SCALE_BITS.
                         */
                        let mut ifmtbl: *mut IFAST_MULT_TYPE =
                            (*compptr).dct_table as *mut IFAST_MULT_TYPE;
                        static mut aanscales: [INT16; 64] = [
                            16384i16, 22725i16, 21407i16, 19266i16, 16384i16, 12873i16, 8867i16,
                            4520i16, 22725i16, 31521i16, 29692i16, 26722i16, 22725i16, 17855i16,
                            12299i16, 6270i16, 21407i16, 29692i16, 27969i16, 25172i16, 21407i16,
                            16819i16, 11585i16, 5906i16, 19266i16, 26722i16, 25172i16, 22654i16,
                            19266i16, 15137i16, 10426i16, 5315i16, 16384i16, 22725i16, 21407i16,
                            19266i16, 16384i16, 12873i16, 8867i16, 4520i16, 12873i16, 17855i16,
                            16819i16, 15137i16, 12873i16, 10114i16, 6967i16, 3552i16, 8867i16,
                            12299i16, 11585i16, 10426i16, 8867i16, 6967i16, 4799i16, 2446i16,
                            4520i16, 6270i16, 5906i16, 5315i16, 4520i16, 3552i16, 2446i16, 1247i16,
                        ];
                        i = 0i32;
                        while i < DCTSIZE2 {
                            *ifmtbl.offset(i as isize) = ((*qtbl).quantval[i as usize] as JLONG
                                * aanscales[i as usize] as JLONG
                                + ((1i64) << 14i32 - 2i32 - 1i32)
                                >> 14i32 - 2i32)
                                as IFAST_MULT_TYPE;
                            i += 1
                        }
                    }
                    2 => {
                        /* For float AA&N IDCT method, multipliers are equal to quantization
                         * coefficients scaled by scalefactor[row]*scalefactor[col], where
                         *   scalefactor[0] = 1
                         *   scalefactor[k] = cos(k*PI/16) * sqrt(2)    for k=1..7
                         */
                        let mut fmtbl: *mut FLOAT_MULT_TYPE =
                            (*compptr).dct_table as *mut FLOAT_MULT_TYPE;

                        static mut aanscalefactor: [c_double; 8] = [
                            1.0f64,
                            1.387039845f64,
                            1.306562965f64,
                            1.175875602f64,
                            1.0f64,
                            0.785694958f64,
                            0.541196100f64,
                            0.275899379f64,
                        ];
                        i = 0i32;
                        let mut row: c_int = 0i32;
                        while row < 8i32 {
                            let mut col: c_int = 0i32;
                            while col < 8i32 {
                                *fmtbl.offset(i as isize) = ((*qtbl).quantval[i as usize]
                                    as c_double
                                    * aanscalefactor[row as usize]
                                    * aanscalefactor[col as usize])
                                    as FLOAT_MULT_TYPE;
                                i += 1;
                                col += 1
                            }
                            row += 1
                        }
                    }
                    _ => {
                        (*(*cinfo).err).msg_code = super::jerror::JERR_NOT_COMPILED as c_int;
                        Some(
                            (*(*cinfo).err)
                                .error_exit
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as j_common_ptr
                        );
                    }
                }
            }
        }
        /* happens if no data yet for component */
        ci += 1;
        compptr = compptr.offset(1)
    }
}
/*
 * Initialize IDCT manager.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_inverse_dct(mut cinfo: j_decompress_ptr) {
    let mut idct: my_idct_ptr = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<my_idct_controller>() as c_ulong,
    ) as my_idct_ptr;
    (*cinfo).idct = idct as *mut jpeg_inverse_dct;
    (*idct).pub_0.start_pass = Some(start_pass as unsafe extern "C" fn(_: j_decompress_ptr) -> ());

    let mut ci: c_int = 0i32;
    let mut compptr: *mut jpeg_component_info = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Allocate and pre-zero a multiplier table for each component */
        (*compptr).dct_table = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            ::std::mem::size_of::<multiplier_table>() as c_ulong,
        );
        memset(
            (*compptr).dct_table,
            0i32,
            ::std::mem::size_of::<multiplier_table>() as c_ulong,
        );
        /* Mark multiplier table not yet set up for any method */
        (*idct).cur_method[ci as usize] = -1i32;
        ci += 1;
        compptr = compptr.offset(1)
    }
}
