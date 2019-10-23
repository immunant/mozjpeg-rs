































































































































































































































use super::simd::x86_64::jsimd::{jsimd_can_h2v1_fancy_upsample,
                                 jsimd_can_h2v1_upsample,
                                 jsimd_can_h2v2_fancy_upsample,
                                 jsimd_can_h2v2_upsample,
                                 jsimd_h2v1_fancy_upsample,
                                 jsimd_h2v1_upsample,
                                 jsimd_h2v2_fancy_upsample,
                                 jsimd_h2v2_upsample};use libc::{c_uint, c_ulong, c_int, c_long, self};pub use crate::jmorecfg_h::{JDIMENSION, UINT8, boolean, FALSE, JCOEF, JOCTET,
                            JSAMPLE, TRUE, UINT16};pub use crate::jpegint_h::{inverse_DCT_method_ptr, jcopy_sample_rows,
                           jround_up, JBUF_CRANK_DEST, JBUF_PASS_THRU,
                           JBUF_REQUANT, JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE,
                           J_BUF_MODE};pub use crate::stddef_h::{size_t, NULL};pub use crate::jpeglib_h::{j_decompress_ptr, jpeg_component_info,
                           jpeg_upsampler, C2RustUnnamed_2, JSAMPARRAY,
                           j_common_ptr, jpeg_color_deconverter,
                           jpeg_color_quantizer, jpeg_common_struct,
                           jpeg_d_coef_controller, jpeg_d_main_controller,
                           jpeg_d_post_controller, jpeg_decomp_master,
                           jpeg_decompress_struct, jpeg_entropy_decoder,
                           jpeg_error_mgr, jpeg_input_controller,
                           jpeg_inverse_dct, jpeg_marker_parser_method,
                           jpeg_marker_reader, jpeg_marker_struct,
                           jpeg_memory_mgr, jpeg_progress_mgr,
                           jpeg_saved_marker_ptr, jpeg_source_mgr,
                           jvirt_barray_control, jvirt_barray_ptr,
                           jvirt_sarray_control, jvirt_sarray_ptr, JCS_YCbCr,
                           JBLOCK, JBLOCKARRAY, JBLOCKROW, JCOEFPTR, JCS_CMYK,
                           JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR,
                           JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB,
                           JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR,
                           JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565,
                           JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT, JDCT_IFAST,
                           JDCT_ISLOW, JDITHER_FS, JDITHER_NONE,
                           JDITHER_ORDERED, JHUFF_TBL, JPOOL_IMAGE,
                           JQUANT_TBL, JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE,
                           J_DCT_METHOD, J_DITHER_MODE};pub use super::jerror::{C2RustUnnamed_3, JERR_ARITH_NOTIMPL,
                        JERR_BAD_ALIGN_TYPE, JERR_BAD_ALLOC_CHUNK,
                        JERR_BAD_BUFFER_MODE, JERR_BAD_COMPONENT_ID,
                        JERR_BAD_CROP_SPEC, JERR_BAD_DCTSIZE,
                        JERR_BAD_DCT_COEF, JERR_BAD_HUFF_TABLE,
                        JERR_BAD_IN_COLORSPACE, JERR_BAD_J_COLORSPACE,
                        JERR_BAD_LENGTH, JERR_BAD_LIB_VERSION,
                        JERR_BAD_MCU_SIZE, JERR_BAD_PARAM,
                        JERR_BAD_PARAM_VALUE, JERR_BAD_POOL_ID,
                        JERR_BAD_PRECISION, JERR_BAD_PROGRESSION,
                        JERR_BAD_PROG_SCRIPT, JERR_BAD_SAMPLING,
                        JERR_BAD_SCAN_SCRIPT, JERR_BAD_STATE,
                        JERR_BAD_STRUCT_SIZE, JERR_BAD_VIRTUAL_ACCESS,
                        JERR_BUFFER_SIZE, JERR_CANT_SUSPEND,
                        JERR_CCIR601_NOTIMPL, JERR_COMPONENT_COUNT,
                        JERR_CONVERSION_NOTIMPL, JERR_DAC_INDEX,
                        JERR_DAC_VALUE, JERR_DHT_INDEX, JERR_DQT_INDEX,
                        JERR_EMPTY_IMAGE, JERR_EMS_READ, JERR_EMS_WRITE,
                        JERR_EOI_EXPECTED, JERR_FILE_READ, JERR_FILE_WRITE,
                        JERR_FRACT_SAMPLE_NOTIMPL, JERR_HUFF_CLEN_OVERFLOW,
                        JERR_HUFF_MISSING_CODE, JERR_IMAGE_TOO_BIG,
                        JERR_INPUT_EMPTY, JERR_INPUT_EOF,
                        JERR_MISMATCHED_QUANT_TABLE, JERR_MISSING_DATA,
                        JERR_MODE_CHANGE, JERR_NOTIMPL, JERR_NOT_COMPILED,
                        JERR_NO_BACKING_STORE, JERR_NO_HUFF_TABLE,
                        JERR_NO_IMAGE, JERR_NO_QUANT_TABLE, JERR_NO_SOI,
                        JERR_OUT_OF_MEMORY, JERR_QUANT_COMPONENTS,
                        JERR_QUANT_FEW_COLORS, JERR_QUANT_MANY_COLORS,
                        JERR_SOF_DUPLICATE, JERR_SOF_NO_SOS,
                        JERR_SOF_UNSUPPORTED, JERR_SOI_DUPLICATE,
                        JERR_SOS_NO_SOF, JERR_TFILE_CREATE, JERR_TFILE_READ,
                        JERR_TFILE_SEEK, JERR_TFILE_WRITE,
                        JERR_TOO_LITTLE_DATA, JERR_UNKNOWN_MARKER,
                        JERR_UNSUPPORTED_SUSPEND, JERR_VIRTUAL_BUG,
                        JERR_WIDTH_OVERFLOW, JERR_XMS_READ, JERR_XMS_WRITE,
                        JMSG_COPYRIGHT, JMSG_LASTMSGCODE, JMSG_NOMESSAGE,
                        JMSG_VERSION, JTRC_16BIT_TABLES, JTRC_ADOBE,
                        JTRC_APP0, JTRC_APP14, JTRC_DAC, JTRC_DHT, JTRC_DQT,
                        JTRC_DRI, JTRC_EMS_CLOSE, JTRC_EMS_OPEN, JTRC_EOI,
                        JTRC_HUFFBITS, JTRC_JFIF, JTRC_JFIF_BADTHUMBNAILSIZE,
                        JTRC_JFIF_EXTENSION, JTRC_JFIF_THUMBNAIL,
                        JTRC_MISC_MARKER, JTRC_PARMLESS_MARKER,
                        JTRC_QUANTVALS, JTRC_QUANT_3_NCOLORS,
                        JTRC_QUANT_NCOLORS, JTRC_QUANT_SELECTED,
                        JTRC_RECOVERY_ACTION, JTRC_RST, JTRC_SMOOTH_NOTIMPL,
                        JTRC_SOF, JTRC_SOF_COMPONENT, JTRC_SOI, JTRC_SOS,
                        JTRC_SOS_COMPONENT, JTRC_SOS_PARAMS, JTRC_TFILE_CLOSE,
                        JTRC_TFILE_OPEN, JTRC_THUMB_JPEG, JTRC_THUMB_PALETTE,
                        JTRC_THUMB_RGB, JTRC_UNKNOWN_IDS, JTRC_XMS_CLOSE,
                        JTRC_XMS_OPEN, JWRN_ADOBE_XFORM, JWRN_BOGUS_ICC,
                        JWRN_BOGUS_PROGRESSION, JWRN_EXTRANEOUS_DATA,
                        JWRN_HIT_MARKER, JWRN_HUFF_BAD_CODE, JWRN_JFIF_MAJOR,
                        JWRN_JPEG_EOF, JWRN_MUST_RESYNC, JWRN_NOT_SEQUENTIAL,
                        JWRN_TOO_MUCH_DATA};
// =============== BEGIN jdsample_h ================
pub type my_upsample_ptr = *mut my_upsampler;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_upsampler {
    pub pub_0: jpeg_upsampler,
    pub color_buf: [JSAMPARRAY; 10],
    pub methods: [upsample1_ptr; 10],
    pub next_row_out: c_int,
    pub rows_to_go: JDIMENSION,
    pub rowgroup_height: [c_int; 10],
    pub h_expand: [UINT8; 10],
    pub v_expand: [UINT8; 10],
}
/*
 * jdsample.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 */
/* Pointer to routine to upsample a single component */

pub type upsample1_ptr = Option<
    unsafe extern "C" fn(
        _: j_decompress_ptr,
        _: *mut jpeg_component_info,
        _: JSAMPARRAY,
        _: *mut JSAMPARRAY,
    ) -> (),
>;
/*
 * jdsample.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
 * Copyright (C) 2010, 2015-2016, D. R. Commander.
 * Copyright (C) 2014, MIPS Technologies, Inc., California.
 * Copyright (C) 2015, Google, Inc.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains upsampling routines.
 *
 * Upsampling input data is counted in "row groups".  A row group
 * is defined to be (v_samp_factor * DCT_scaled_size / min_DCT_scaled_size)
 * sample rows of each component.  Upsampling will normally produce
 * max_v_samp_factor pixel rows from each row group (but this could vary
 * if the upsampler is applying a scale factor of its own).
 *
 * An excellent reference for image resampling is
 *   Digital Image Warping, George Wolberg, 1990.
 *   Pub. by IEEE Computer Society Press, Los Alamitos, CA. ISBN 0-8186-8944-7.
 */
/*
 * Initialize for an upsampling pass.
 */

unsafe extern "C" fn start_pass_upsample(mut cinfo: j_decompress_ptr) {
    let mut upsample: my_upsample_ptr =
        (*cinfo).upsample as my_upsample_ptr;
    /* Mark the conversion buffer empty */
    (*upsample).next_row_out = (*cinfo).max_v_samp_factor;
    /* Initialize total-height counter for detecting bottom of image */
    (*upsample).rows_to_go = (*cinfo).output_height;
}
/*
 * Control routine to do upsampling (and color conversion).
 *
 * In this version we upsample each component independently.
 * We upsample one row group into the conversion buffer, then apply
 * color conversion a row at a time.
 */

unsafe extern "C" fn sep_upsample(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: *mut JDIMENSION,
    mut in_row_groups_avail: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut out_row_ctr: *mut JDIMENSION,
    mut out_rows_avail: JDIMENSION,
) {
     let mut upsample: my_upsample_ptr =
        (*cinfo).upsample as my_upsample_ptr;
    
    
    
    /* Fill the conversion buffer, if it's empty */
    if (*upsample).next_row_out >= (*cinfo).max_v_samp_factor {
          
         let mut ci:   c_int =  0i32; let mut compptr:   *mut jpeg_component_info =
     (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
            /* Invoke per-component upsample method.  Notice we pass a POINTER
             * to color_buf[ci], so that fullsize_upsample can change it.
             */
            Some(
                (*(*upsample).methods.as_mut_ptr().offset(ci as isize))
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                compptr,
                (*input_buf.offset(ci as isize)).offset(
                    (*in_row_group_ctr * (*upsample).rowgroup_height[ci as usize] as c_uint)
                        as isize,
                ),
                (*upsample).color_buf.as_mut_ptr().offset(ci as isize),
            );
            ci += 1;
            compptr = compptr.offset(1)
        }
        (*upsample).next_row_out = 0i32
    }
     let mut num_rows:   JDIMENSION =
    
        ((*cinfo).max_v_samp_factor - (*upsample).next_row_out) as JDIMENSION;
    /* Not more than the distance to the end of the image.  Need this test
     * in case the image height is not a multiple of max_v_samp_factor:
     */
    if num_rows > (*upsample).rows_to_go {
        num_rows = (*upsample).rows_to_go
    }
    /* And not more than what the client can accept: */
    out_rows_avail -=  *out_row_ctr;
    if num_rows > out_rows_avail {
        num_rows = out_rows_avail
    }
    Some(
        (*(*cinfo).cconvert)
            .color_convert
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo,
        (*upsample).color_buf.as_mut_ptr(),
        (*upsample).next_row_out as JDIMENSION,
        output_buf.offset(*out_row_ctr as isize),
        num_rows as c_int,
    );
    /* Adjust counts */
    *out_row_ctr = *out_row_ctr + num_rows;
    (*upsample).rows_to_go = (*upsample).rows_to_go - num_rows;
    (*upsample).next_row_out = ((((*upsample).next_row_out as c_uint + num_rows))) as c_int;
    /* When the buffer is emptied, declare this input row group consumed */
    if (*upsample).next_row_out >= (*cinfo).max_v_samp_factor {
        *in_row_group_ctr = *in_row_group_ctr + 1
    };
}
/*
 * These are the routines invoked by sep_upsample to upsample pixel values
 * of a single component.  One row group is processed per call.
 */
/*
 * For full-size components, we just make color_buf[ci] point at the
 * input buffer, and thus avoid copying any data.  Note that this is
 * safe only because sep_upsample doesn't declare the input row group
 * "consumed" until we are done color converting and emitting it.
 */

unsafe extern "C" fn fullsize_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
    *output_data_ptr = input_data;
}
/*
 * This is a no-op version used for "uninteresting" components.
 * These components will not be referenced by color conversion.
 */

unsafe extern "C" fn noop_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
    *output_data_ptr = NULL as JSAMPARRAY;
    /* safety check */
}
/*
 * This version handles any integral sampling ratios.
 * This is not used for typical JPEG files, so it need not be fast.
 * Nor, for that matter, is it particularly accurate: the algorithm is
 * simple replication of the input pixel onto the corresponding output
 * pixels.  The hi-falutin sampling literature refers to this as a
 * "box filter".  A box filter tends to introduce visible artifacts,
 * so if you are actually going to use 3:1 or 4:1 sampling ratios
 * you would be well advised to improve this code.
 */

unsafe extern "C" fn int_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
        let mut upsample: my_upsample_ptr =
        (*cinfo).upsample as my_upsample_ptr;
    let mut output_data: JSAMPARRAY = *output_data_ptr;
    
    
    
    
    
    
    
    
    
    
    
    
     let mut h_expand:   c_int =
     (*upsample).h_expand[(*compptr).component_index as usize] as c_int; let mut v_expand:   c_int =
     (*upsample).v_expand[(*compptr).component_index as usize] as c_int; let mut outrow:   c_int =  0i32; let mut inrow:   c_int =  outrow;
    while outrow < (*cinfo).max_v_samp_factor {
           /* don't need GETJSAMPLE() here */
        
         let mut inptr:   JSAMPROW =
     *input_data.offset(inrow as isize); let mut outptr:   JSAMPROW =
     *output_data.offset(outrow as isize); let mut outend:   JSAMPROW =
     outptr.offset((*cinfo).output_width as isize);
        while outptr < outend {
              let fresh0 = inptr;
            inptr = inptr.offset(1);
            
             let mut invalue:   JSAMPLE =  *fresh0; let mut h:   c_int =  h_expand;
            while h > 0i32 {
                let fresh1 = outptr;
                outptr = outptr.offset(1);
                *fresh1 = invalue;
                h -= 1
            }
        }
        /* Generate any additional output rows by duplicating the first one */
        if v_expand > 1i32 {
            jcopy_sample_rows(
                output_data,
                outrow,
                output_data,
                outrow + 1i32,
                v_expand - 1i32,
                (*cinfo).output_width,
            );
        }
        inrow += 1;
        outrow += v_expand
    }
}
/*
 * Fast processing for the common case of 2:1 horizontal and 1:1 vertical.
 * It's still a box filter.
 */

unsafe extern "C" fn h2v1_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
     let mut output_data: JSAMPARRAY = *output_data_ptr; /* don't need GETJSAMPLE() here */
    
    
    
    
    
     let mut inrow:   c_int =  0i32;
    while inrow < (*cinfo).max_v_samp_factor {
           
        
         let mut inptr:   JSAMPROW =
     *input_data.offset(inrow as isize); let mut outptr:   JSAMPROW =
     *output_data.offset(inrow as isize); let mut outend:   JSAMPROW =
     outptr.offset((*cinfo).output_width as isize);
        while outptr < outend {
             let fresh2 = inptr;
            inptr = inptr.offset(1);
             let mut invalue:   JSAMPLE =  *fresh2;
            let fresh3 = outptr;
            outptr = outptr.offset(1);
            *fresh3 = invalue;
            let fresh4 = outptr;
            outptr = outptr.offset(1);
            *fresh4 = invalue
        }
        inrow += 1
    }
}
/*
 * Fast processing for the common case of 2:1 horizontal and 2:1 vertical.
 * It's still a box filter.
 */

unsafe extern "C" fn h2v2_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
      let mut output_data: JSAMPARRAY = *output_data_ptr; /* don't need GETJSAMPLE() here */
    
    
    
    
    
    
    
     let mut outrow:   c_int =  0i32; let mut inrow:   c_int =  outrow;
    while outrow < (*cinfo).max_v_samp_factor {
           
        
         let mut inptr:   JSAMPROW =
     *input_data.offset(inrow as isize); let mut outptr:   JSAMPROW =
     *output_data.offset(outrow as isize); let mut outend:   JSAMPROW =
     outptr.offset((*cinfo).output_width as isize);
        while outptr < outend {
             let fresh5 = inptr;
            inptr = inptr.offset(1);
             let mut invalue:   JSAMPLE =  *fresh5;
            let fresh6 = outptr;
            outptr = outptr.offset(1);
            *fresh6 = invalue;
            let fresh7 = outptr;
            outptr = outptr.offset(1);
            *fresh7 = invalue
        }
        jcopy_sample_rows(
            output_data,
            outrow,
            output_data,
            outrow + 1i32,
            1i32,
            (*cinfo).output_width,
        );
        inrow += 1;
        outrow += 2i32
    }
}
/*
 * Fancy processing for the common case of 2:1 horizontal and 1:1 vertical.
 *
 * The upsampling algorithm is linear interpolation between pixel centers,
 * also known as a "triangle filter".  This is a good compromise between
 * speed and visual quality.  The centers of the output pixels are 1/4 and 3/4
 * of the way between input pixel centers.
 *
 * A note about the "bias" calculations: when rounding fractional values to
 * integer, we do not want to always round 0.5 up to the next integer.
 * If we did that, we'd introduce a noticeable bias towards larger values.
 * Instead, this code is arranged so that 0.5 will be rounded up or down at
 * alternate pixel locations (a simple ordered dither pattern).
 */

unsafe extern "C" fn h2v1_fancy_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
     let mut output_data: JSAMPARRAY = *output_data_ptr;
    
    
    
    
    
     let mut inrow:   c_int =  0i32;
    while inrow < (*cinfo).max_v_samp_factor {
            
         let mut inptr:   JSAMPROW =
     *input_data.offset(inrow as isize); let mut outptr:   JSAMPROW =
     *output_data.offset(inrow as isize);
        /* Special case for first column */
        let fresh8 = inptr;
        inptr = inptr.offset(1);
         let mut invalue:   c_int =  *fresh8 as c_int;
        let fresh9 = outptr;
        outptr = outptr.offset(1);
        *fresh9 = invalue as JSAMPLE;
        let fresh10 = outptr;
        outptr = outptr.offset(1);
        *fresh10 =
            (invalue * 3i32 + *inptr as c_int + 2i32 >> 2i32) as JSAMPLE;
         let mut colctr:   JDIMENSION =
      (*compptr)
            .downsampled_width - 2u32;
        while colctr > 0u32 {
            /* General case: 3/4 * nearer pixel + 1/4 * further pixel */
            let fresh11 = inptr;
            inptr = inptr.offset(1);
            invalue = *fresh11 as c_int * 3i32;
            let fresh12 = outptr;
            outptr = outptr.offset(1);
            *fresh12 = (invalue + *inptr.offset(-2isize) as c_int + 1i32 >> 2i32)
                as JSAMPLE;
            let fresh13 = outptr;
            outptr = outptr.offset(1);
            *fresh13 =
                (invalue + *inptr as c_int + 2i32 >> 2i32) as JSAMPLE;
            colctr -=  1
        }
        /* Special case for last column */
        invalue = *inptr as c_int;
        let fresh14 = outptr;
        outptr = outptr.offset(1);
        *fresh14 = (invalue * 3i32 + *inptr.offset(-1isize) as c_int + 1i32 >> 2i32)
            as JSAMPLE;
        let fresh15 = outptr;
        outptr = outptr.offset(1);
        *fresh15 = invalue as JSAMPLE;
        inrow += 1
    }
}
/*
 * Fancy processing for 1:1 horizontal and 2:1 vertical (4:4:0 subsampling).
 *
 * This is a less common case, but it can be encountered when losslessly
 * rotating/transposing a JPEG file that uses 4:2:2 chroma subsampling.
 */

unsafe extern "C" fn h1v2_fancy_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
      let mut output_data: JSAMPARRAY = *output_data_ptr;
    
    
    
    
    
    
    
    
    
     let mut outrow:   c_int =  0i32; let mut inrow:   c_int =  outrow;
    while outrow < (*cinfo).max_v_samp_factor {
          let mut v:   c_int =  0i32;
        while v < 2i32 {
             let mut inptr1:  JSAMPROW =
     ::std::ptr::null_mut::< JSAMPLE>();   let mut inptr0:   JSAMPROW =
     *input_data.offset(inrow as isize);
            if v == 0i32 {
                /* next nearest is row above */
                inptr1 = *input_data.offset((inrow - 1i32) as isize)
            } else {
                /* next nearest is row below */
                inptr1 = *input_data.offset((inrow + 1i32) as isize)
            }
            let fresh16 = outrow;
            outrow +=  1;
            
             let mut outptr:   JSAMPROW =
     *output_data.offset(fresh16 as isize); let mut colctr:   JDIMENSION =  0u32;
            while colctr < (*compptr).downsampled_width {
                 let fresh17 = inptr0;
                inptr0 = inptr0.offset(1);
                let fresh18 = inptr1;
                inptr1 = inptr1.offset(1);
                 let mut thiscolsum:   c_int =
     *fresh17 as c_int * 3i32 + *fresh18 as c_int;
                let fresh19 = outptr;
                outptr = outptr.offset(1);
                *fresh19 = (thiscolsum + 1i32 >> 2i32) as JSAMPLE;
                colctr +=  1
            }
            v += 1
        }
        inrow += 1
    }
}
/*
 * Fancy processing for the common case of 2:1 horizontal and 2:1 vertical.
 * Again a triangle filter; see comments for h2v1 case, above.
 *
 * It is OK for us to reference the adjacent input rows because we demanded
 * context from the main buffer controller (see initialization code).
 */

unsafe extern "C" fn h2v2_fancy_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
      let mut output_data: JSAMPARRAY = *output_data_ptr;
    
    
    
    
    
    
    
    
    
    
    
     let mut outrow:   c_int =  0i32; let mut inrow:   c_int =  outrow;
    while outrow < (*cinfo).max_v_samp_factor {
          let mut v:   c_int =  0i32;
        while v < 2i32 {
             let mut inptr1:  JSAMPROW =
     ::std::ptr::null_mut::< JSAMPLE>();      let mut inptr0:   JSAMPROW =
     *input_data.offset(inrow as isize);
            if v == 0i32 {
                /* next nearest is row above */
                inptr1 = *input_data.offset((inrow - 1i32) as isize)
            } else {
                /* next nearest is row below */
                inptr1 = *input_data.offset((inrow + 1i32) as isize)
            }
            let fresh20 = outrow;
            outrow +=  1;
             let mut outptr:   JSAMPROW =
     *output_data.offset(fresh20 as isize);
            /* Special case for first column */
            let fresh21 = inptr0;
            inptr0 = inptr0.offset(1);
            let fresh22 = inptr1;
            inptr1 = inptr1.offset(1);
             let mut thiscolsum:   c_int =
     *fresh21 as c_int * 3i32 + *fresh22 as c_int;
            let fresh23 = inptr0;
            inptr0 = inptr0.offset(1);
            let fresh24 = inptr1;
            inptr1 = inptr1.offset(1);
             let mut nextcolsum:   c_int =
     *fresh23 as c_int * 3i32 + *fresh24 as c_int;
            let fresh25 = outptr;
            outptr = outptr.offset(1);
            *fresh25 = (thiscolsum * 4i32 + 8i32 >> 4i32) as JSAMPLE;
            let fresh26 = outptr;
            outptr = outptr.offset(1);
            *fresh26 =
                (thiscolsum * 3i32 + nextcolsum + 7i32 >> 4i32) as JSAMPLE;
             let mut lastcolsum:   c_int =  thiscolsum;
            thiscolsum = nextcolsum;
             let mut colctr:   JDIMENSION =
      (*compptr)
                .downsampled_width - 2u32;
            while colctr > 0u32 {
                /* General case: 3/4 * nearer pixel + 1/4 * further pixel in each */
                /* dimension, thus 9/16, 3/16, 3/16, 1/16 overall */
                let fresh27 = inptr0;
                inptr0 = inptr0.offset(1);
                let fresh28 = inptr1;
                inptr1 = inptr1.offset(1);
                nextcolsum = *fresh27 as c_int * 3i32 + *fresh28 as c_int;
                let fresh29 = outptr;
                outptr = outptr.offset(1);
                *fresh29 =
                    (thiscolsum * 3i32 + lastcolsum + 8i32 >> 4i32) as JSAMPLE;
                let fresh30 = outptr;
                outptr = outptr.offset(1);
                *fresh30 =
                    (thiscolsum * 3i32 + nextcolsum + 7i32 >> 4i32) as JSAMPLE;
                lastcolsum = thiscolsum;
                thiscolsum = nextcolsum;
                colctr -=  1
            }
            /* Special case for last column */
            let fresh31 = outptr;
            outptr = outptr.offset(1);
            *fresh31 =
                (thiscolsum * 3i32 + lastcolsum + 8i32 >> 4i32) as JSAMPLE;
            let fresh32 = outptr;
            outptr = outptr.offset(1);
            *fresh32 = (thiscolsum * 4i32 + 7i32 >> 4i32) as JSAMPLE;
            v += 1
        }
        inrow += 1
    }
}
/*
 * Module initialization routine for upsampling.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_upsampler(mut cinfo: j_decompress_ptr) {
    
    
    
    
    
    
    
    
     let mut upsample:  my_upsample_ptr =
    
        ::std::ptr::null_mut::< my_upsampler>();   
    if (*(*cinfo).master).jinit_upsampler_no_alloc == 0 {
        upsample = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            ::std::mem::size_of::<my_upsampler>() as c_ulong,
        ) as my_upsample_ptr;
        (*cinfo).upsample = upsample as *mut jpeg_upsampler;
        (*upsample).pub_0.start_pass = Some(
            start_pass_upsample
                as unsafe extern "C" fn(_: j_decompress_ptr) -> (),
        );
        (*upsample).pub_0.upsample = Some(
            sep_upsample
                as unsafe extern "C" fn(
                    _: j_decompress_ptr,
                    _: JSAMPIMAGE,
                    _: *mut JDIMENSION,
                    _: JDIMENSION,
                    _: JSAMPARRAY,
                    _: *mut JDIMENSION,
                    _: JDIMENSION,
                ) -> (),
        );
        (*upsample).pub_0.need_context_rows = FALSE
    /* until we find out differently */
    } else {
        upsample = (*cinfo).upsample as my_upsample_ptr
    }
    if (*cinfo).CCIR601_sampling != 0 {
        /* this isn't supported */
        (*(*cinfo).err).msg_code = super::jerror::JERR_CCIR601_NOTIMPL as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* jdmainct.c doesn't support context rows when min_DCT_scaled_size = 1,
     * so don't ask for it.
     */
    
    /* Verify we can handle the sampling factors, select per-component methods,
     * and create storage as needed.
     */
    
     let mut do_fancy:   boolean =
    
        ((*cinfo).do_fancy_upsampling != 0 && (*cinfo).min_DCT_scaled_size > 1i32) as c_int; let mut ci:   c_int =  0i32; let mut compptr:   *mut jpeg_component_info =
     (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Compute size of an "input group" after IDCT scaling.  This many samples
         * are to be converted to max_h_samp_factor * max_v_samp_factor pixels.
         */
              /* save for use later */
        
        
         let mut h_in_group:   c_int =
    
            (*compptr).h_samp_factor * (*compptr).DCT_scaled_size / (*cinfo).min_DCT_scaled_size; let mut v_in_group:   c_int =
    
            (*compptr).v_samp_factor * (*compptr).DCT_scaled_size / (*cinfo).min_DCT_scaled_size; let mut h_out_group:   c_int =  (*cinfo).max_h_samp_factor; let mut v_out_group:   c_int =  (*cinfo).max_v_samp_factor;
        (*upsample).rowgroup_height[ci as usize] = v_in_group;
         let mut need_buffer:   boolean =  TRUE;
        if (*compptr).component_needed == 0 {
            /* Don't bother to upsample an uninteresting component. */
            (*upsample).methods[ci as usize] = Some(
                noop_upsample
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: *mut jpeg_component_info,
                        _: JSAMPARRAY,
                        _: *mut JSAMPARRAY,
                    ) -> (),
            );
            need_buffer = FALSE
        } else if h_in_group == h_out_group && v_in_group == v_out_group {
            /* Fullsize components can be processed without any work. */
            (*upsample).methods[ci as usize] = Some(
                fullsize_upsample
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: *mut jpeg_component_info,
                        _: JSAMPARRAY,
                        _: *mut JSAMPARRAY,
                    ) -> (),
            );
            need_buffer = FALSE
        } else if h_in_group * 2i32 == h_out_group && v_in_group == v_out_group {
            /* Special cases for 2h1v upsampling */
            if do_fancy != 0 && (*compptr).downsampled_width > 2u32 {
                if super::simd::x86_64::jsimd::jsimd_can_h2v1_fancy_upsample() != 0 {
                    (*upsample).methods[ci as usize] = Some(
                        super::simd::x86_64::jsimd::jsimd_h2v1_fancy_upsample
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: *mut jpeg_component_info,
                                _: JSAMPARRAY,
                                _: *mut JSAMPARRAY,
                            ) -> (),
                    )
                } else {
                    (*upsample).methods[ci as usize] = Some(
                        h2v1_fancy_upsample
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: *mut jpeg_component_info,
                                _: JSAMPARRAY,
                                _: *mut JSAMPARRAY,
                            ) -> (),
                    )
                }
            } else if super::simd::x86_64::jsimd::jsimd_can_h2v1_upsample() != 0 {
                (*upsample).methods[ci as usize] = Some(
                    super::simd::x86_64::jsimd::jsimd_h2v1_upsample
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JSAMPARRAY,
                            _: *mut JSAMPARRAY,
                        ) -> (),
                )
            } else {
                (*upsample).methods[ci as usize] = Some(
                    h2v1_upsample
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JSAMPARRAY,
                            _: *mut JSAMPARRAY,
                        ) -> (),
                )
            }
        } else if h_in_group == h_out_group && v_in_group * 2i32 == v_out_group && do_fancy != 0 {
            /* Non-fancy upsampling is handled by the generic method */
            (*upsample).methods[ci as usize] = Some(
                h1v2_fancy_upsample
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: *mut jpeg_component_info,
                        _: JSAMPARRAY,
                        _: *mut JSAMPARRAY,
                    ) -> (),
            );
            (*upsample).pub_0.need_context_rows = TRUE
        } else if h_in_group * 2i32 == h_out_group && v_in_group * 2i32 == v_out_group {
            /* Special cases for 2h2v upsampling */
            if do_fancy != 0 && (*compptr).downsampled_width > 2u32 {
                if super::simd::x86_64::jsimd::jsimd_can_h2v2_fancy_upsample() != 0 {
                    (*upsample).methods[ci as usize] = Some(
                        super::simd::x86_64::jsimd::jsimd_h2v2_fancy_upsample
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: *mut jpeg_component_info,
                                _: JSAMPARRAY,
                                _: *mut JSAMPARRAY,
                            ) -> (),
                    )
                } else {
                    (*upsample).methods[ci as usize] = Some(
                        h2v2_fancy_upsample
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: *mut jpeg_component_info,
                                _: JSAMPARRAY,
                                _: *mut JSAMPARRAY,
                            ) -> (),
                    )
                }
                (*upsample).pub_0.need_context_rows = TRUE
            } else if super::simd::x86_64::jsimd::jsimd_can_h2v2_upsample() != 0 {
                (*upsample).methods[ci as usize] = Some(
                    super::simd::x86_64::jsimd::jsimd_h2v2_upsample
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JSAMPARRAY,
                            _: *mut JSAMPARRAY,
                        ) -> (),
                )
            } else {
                (*upsample).methods[ci as usize] = Some(
                    h2v2_upsample
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JSAMPARRAY,
                            _: *mut JSAMPARRAY,
                        ) -> (),
                )
            }
        } else if h_out_group % h_in_group == 0i32 && v_out_group % v_in_group == 0i32 {
            /* Generic integral-factors upsampling method */
            (*upsample).methods[ci as usize] = Some(
                int_upsample
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: *mut jpeg_component_info,
                        _: JSAMPARRAY,
                        _: *mut JSAMPARRAY,
                    ) -> (),
            );
            (*upsample).h_expand[ci as usize] =
                (h_out_group / h_in_group) as UINT8;
            (*upsample).v_expand[ci as usize] =
                (v_out_group / v_in_group) as UINT8
        } else {
            (*(*cinfo).err).msg_code = super::jerror::JERR_FRACT_SAMPLE_NOTIMPL as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        if need_buffer != 0 && (*(*cinfo).master).jinit_upsampler_no_alloc == 0 {
            (*upsample).color_buf[ci as usize] = Some(
                (*(*cinfo).mem)
                    .alloc_sarray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                JPOOL_IMAGE,
                jround_up(
                    (*cinfo).output_width as c_long,
                    (*cinfo).max_h_samp_factor as c_long,
                ) as JDIMENSION,
                (*cinfo).max_v_samp_factor as JDIMENSION,
            )
        }
        ci += 1;
        compptr = compptr.offset(1)
    }
}
