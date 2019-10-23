


















































































































































































































use libc::{c_uint, c_ulong, c_int, c_long, self};pub use crate::jmorecfg_h::{boolean, FALSE, JCOEF, JDIMENSION, JOCTET,
                            JSAMPLE, TRUE, UINT16, UINT8};pub use crate::stddef_h::size_t;pub use crate::jpegint_h::{jcopy_sample_rows, JBUF_CRANK_DEST, JBUF_PASS_THRU,
                           JBUF_REQUANT, JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE,
                           JLONG, J_BUF_MODE};pub use crate::jpeglib_h::{j_common_ptr, j_compress_ptr,
                           jpeg_c_coef_controller, jpeg_c_main_controller,
                           jpeg_c_prep_controller, jpeg_color_converter,
                           jpeg_common_struct, jpeg_comp_master,
                           jpeg_component_info, jpeg_compress_struct,
                           jpeg_destination_mgr, jpeg_downsampler,
                           jpeg_entropy_encoder, jpeg_error_mgr,
                           jpeg_forward_dct, jpeg_marker_writer,
                           jpeg_memory_mgr, jpeg_progress_mgr, jpeg_scan_info,
                           jvirt_barray_control, jvirt_barray_ptr,
                           jvirt_sarray_control, jvirt_sarray_ptr,
                           C2RustUnnamed_2, JCS_YCbCr, DCTSIZE, JBLOCK,
                           JBLOCKARRAY, JBLOCKROW, JCS_CMYK, JCS_EXT_ABGR,
                           JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA,
                           JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA,
                           JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB,
                           JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN,
                           JCS_YCCK, JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW,
                           JHUFF_TBL, JPOOL_IMAGE, JQUANT_TBL, JSAMPARRAY,
                           JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD};pub use super::jerror::{C2RustUnnamed_3, JERR_ARITH_NOTIMPL,
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
/*
 * jcsample.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
 * Copyright (C) 2014, MIPS Technologies, Inc., California.
 * Copyright (C) 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains downsampling routines.
 *
 * Downsampling input data is counted in "row groups".  A row group
 * is defined to be max_v_samp_factor pixel rows of each component,
 * from which the downsampler produces v_samp_factor sample rows.
 * A single row group is processed in each call to the downsampler module.
 *
 * The downsampler is responsible for edge-expansion of its output data
 * to fill an integral number of DCT blocks horizontally.  The source buffer
 * may be modified if it is helpful for this purpose (the source buffer is
 * allocated wide enough to correspond to the desired output width).
 * The caller (the prep controller) is responsible for vertical padding.
 *
 * The downsampler may request "context rows" by setting need_context_rows
 * during startup.  In this case, the input arrays will contain at least
 * one row group's worth of pixels above and below the passed-in data;
 * the caller will create dummy rows at image top and bottom by replicating
 * the first or last real pixel row.
 *
 * An excellent reference for image resampling is
 *   Digital Image Warping, George Wolberg, 1990.
 *   Pub. by IEEE Computer Society Press, Los Alamitos, CA. ISBN 0-8186-8944-7.
 *
 * The downsampling algorithm used here is a simple average of the source
 * pixels covered by the output pixel.  The hi-falutin sampling literature
 * refers to this as a "box filter".  In general the characteristics of a box
 * filter are not very good, but for the specific cases we normally use (1:1
 * and 2:1 ratios) the box is equivalent to a "triangle filter" which is not
 * nearly so bad.  If you intend to use other sampling ratios, you'd be well
 * advised to improve this code.
 *
 * A simple input-smoothing capability is provided.  This is mainly intended
 * for cleaning up color-dithered GIF input files (if you find it inadequate,
 * we suggest using an external filtering program such as pnmconvol).  When
 * enabled, each input pixel P is replaced by a weighted sum of itself and its
 * eight neighbors.  P's weight is 1-8*SF and each neighbor's weight is SF,
 * where SF = (smoothing_factor / 1024).
 * Currently, smoothing is only supported for 2h2v sampling factors.
 */
/* Pointer to routine to downsample a single component */

pub type downsample1_ptr = Option<
    unsafe extern "C" fn(
        _: j_compress_ptr,
        _: *mut jpeg_component_info,
        _: JSAMPARRAY,
        _: JSAMPARRAY,
    ) -> (),
>;

pub type my_downsample_ptr = *mut my_downsampler;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_downsampler {
    pub pub_0: jpeg_downsampler,
    pub methods: [downsample1_ptr; 10],
}
/*
 * Initialize for a downsampling pass.
 */

unsafe extern "C" fn start_pass_downsample(mut _cinfo: j_compress_ptr) {
    /* no work for now */
}
/*
 * Expand a component horizontally from width input_cols to width output_cols,
 * by duplicating the rightmost samples.
 */

unsafe extern "C" fn expand_right_edge(
    mut image_data: JSAMPARRAY,
    mut num_rows: c_int,
    mut input_cols: JDIMENSION,
    mut output_cols: JDIMENSION,
) {
     /* don't need GETJSAMPLE() here */
    
    
    
    let mut numcols: c_int = ( output_cols - input_cols) as c_int;
    if numcols > 0i32 {
          let mut row:   c_int =  0i32;
        while row < num_rows {
               
            
             let mut ptr:   JSAMPROW =
     (*image_data.offset(row as isize)).offset(input_cols as isize); let mut pixval:   JSAMPLE =  *ptr.offset(-1isize); let mut count:   c_int =  numcols;
            while count > 0i32 {
                let fresh0 = ptr;
                ptr = ptr.offset(1);
                *fresh0 = pixval;
                count -= 1
            }
            row += 1
        }
    };
}
/*
 * Do downsampling for a whole row group (all components).
 *
 * In this version we simply downsample each component independently.
 */

unsafe extern "C" fn sep_downsample(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_index: JDIMENSION,
    mut output_buf: JSAMPIMAGE,
    mut out_row_group_index: JDIMENSION,
) {
      let mut downsample: my_downsample_ptr = (*cinfo).downsample as my_downsample_ptr;
    
    
    
    
    
     let mut ci:   c_int =  0i32; let mut compptr:   *mut jpeg_component_info =
     (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
          
         let mut in_ptr:   JSAMPARRAY =
     (*input_buf.offset(ci as isize)).offset(in_row_index as isize); let mut out_ptr:   JSAMPARRAY =
     (*output_buf.offset(ci as isize)).offset(
            (
            out_row_group_index * (*compptr).v_samp_factor as c_uint) as isize,
        );
        Some(
            (*(*downsample).methods.as_mut_ptr().offset(ci as isize))
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo, compptr, in_ptr, out_ptr);
        ci += 1;
        compptr = compptr.offset(1)
    }
}
/*
 * Downsample pixel values of a single component.
 * One row group is processed per call.
 * This version handles arbitrary integral sampling ratios, without smoothing.
 * Note that this version is not actually used for customary sampling ratios.
 */

unsafe extern "C" fn int_downsample(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data: JSAMPARRAY,
) {
          
    let mut output_cols: JDIMENSION =  (*compptr)
        .width_in_blocks * DCTSIZE as c_uint;
    
    
    
    
    
    
     let mut h_expand:   c_int =
     (*cinfo).max_h_samp_factor / (*compptr).h_samp_factor; let mut v_expand:   c_int =
     (*cinfo).max_v_samp_factor / (*compptr).v_samp_factor; let mut numpix:   c_int =  h_expand * v_expand; let mut numpix2:   c_int =  numpix / 2i32;
    /* Expand input data enough to let all the output samples be generated
     * by the standard loop.  Special-casing padded output would be more
     * efficient.
     */
    expand_right_edge(
        input_data,
        (*cinfo).max_v_samp_factor,
        (*cinfo).image_width,
        
        output_cols * h_expand as c_uint,
    );
    
     let mut inrow:   c_int =  0i32; let mut outrow:   c_int =  0i32;
    while outrow < (*compptr).v_samp_factor {
           
        
         let mut outptr:   JSAMPROW =
     *output_data.offset(outrow as isize); let mut outcol:   JDIMENSION =  0u32; let mut outcol_h:   JDIMENSION =  0u32;
        while outcol < output_cols {
              
             let mut outvalue:   JLONG =  0i64; let mut v:   c_int =  0i32;
            while v < v_expand {
                  
                 let mut inptr:   JSAMPROW =
     (*input_data.offset((inrow + v) as isize)).offset(outcol_h as isize); let mut h:   c_int =  0i32;
                while h < h_expand {
                    let fresh1 = inptr;
                    inptr = inptr.offset(1);
                    outvalue += *fresh1 as c_int as JLONG;
                    h += 1
                }
                v += 1
            }
            let fresh2 = outptr;
            outptr = outptr.offset(1);
            *fresh2 = ((outvalue + numpix2 as c_long) / numpix as c_long)
                as JSAMPLE;
            outcol +=  1;
            outcol_h +=  h_expand as c_uint
        }
        inrow += v_expand;
        outrow += 1
    }
}
/*
 * Downsample pixel values of a single component.
 * This version handles the special case of a full-size component,
 * without smoothing.
 */

unsafe extern "C" fn fullsize_downsample(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data: JSAMPARRAY,
) {
    /* Copy the data */
    jcopy_sample_rows(
        input_data,
        0i32,
        output_data,
        0i32,
        (*cinfo).max_v_samp_factor,
        (*cinfo).image_width,
    );
    /* Edge-expand */
    expand_right_edge(
        output_data,
        (*cinfo).max_v_samp_factor,
        (*cinfo).image_width,
        
        (*compptr)
            .width_in_blocks * DCTSIZE as c_uint,
    );
}
/*
 * Downsample pixel values of a single component.
 * This version handles the common case of 2:1 horizontal and 1:1 vertical,
 * without smoothing.
 *
 * A note about the "bias" calculations: when rounding fractional values to
 * integer, we do not want to always round 0.5 up to the next integer.
 * If we did that, we'd introduce a noticeable bias towards larger values.
 * Instead, this code is arranged so that 0.5 will be rounded up or down at
 * alternate pixel locations (a simple ordered dither pattern).
 */

unsafe extern "C" fn h2v1_downsample(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data: JSAMPARRAY,
) {
    
     
    let mut output_cols: JDIMENSION =  (*compptr)
        .width_in_blocks * DCTSIZE as c_uint;
    
    
    
    /* Expand input data enough to let all the output samples be generated
     * by the standard loop.  Special-casing padded output would be more
     * efficient.
     */
    expand_right_edge(
        input_data,
        (*cinfo).max_v_samp_factor,
        (*cinfo).image_width,
        
        output_cols * 2u32,
    ); /* bias = 0,1,0,1,... for successive samples */
     let mut outrow:   c_int =  0i32; /* 0=>1, 1=>0 */
    while outrow < (*compptr).v_samp_factor {
            
        
        
         let mut outptr:   JSAMPROW =
     *output_data.offset(outrow as isize); let mut inptr:   JSAMPROW =
     *input_data.offset(outrow as isize); let mut bias:   c_int =  0i32; let mut outcol:   JDIMENSION =  0u32;
        while outcol < output_cols {
            let fresh3 = outptr;
            outptr = outptr.offset(1);
            *fresh3 = (*inptr as c_int + *inptr.offset(1) as c_int + bias >> 1i32)
                as JSAMPLE;
            bias ^= 1i32;
            inptr = inptr.offset(2);
            outcol +=  1
        }
        outrow += 1
    }
}
/*
 * Downsample pixel values of a single component.
 * This version handles the standard case of 2:1 horizontal and 2:1 vertical,
 * without smoothing.
 */

unsafe extern "C" fn h2v2_downsample(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data: JSAMPARRAY,
) {
    
    
      
    let mut output_cols: JDIMENSION =  (*compptr)
        .width_in_blocks * DCTSIZE as c_uint;
    
    
    
    
    /* Expand input data enough to let all the output samples be generated
     * by the standard loop.  Special-casing padded output would be more
     * efficient.
     */
    expand_right_edge(
        input_data,
        (*cinfo).max_v_samp_factor,
        (*cinfo).image_width,
        
        output_cols * 2u32,
    ); /* bias = 1,2,1,2,... for successive samples */
     /* 1=>2, 2=>1 */
     let mut inrow:   c_int =  0i32; let mut outrow:   c_int =  0i32;
    while outrow < (*compptr).v_samp_factor {
             
        
        
        
         let mut outptr:   JSAMPROW =
     *output_data.offset(outrow as isize); let mut inptr0:   JSAMPROW =
     *input_data.offset(inrow as isize); let mut inptr1:   JSAMPROW =
     *input_data.offset((inrow + 1i32) as isize); let mut bias:   c_int =  1i32; let mut outcol:   JDIMENSION =  0u32;
        while outcol < output_cols {
            let fresh4 = outptr;
            outptr = outptr.offset(1);
            *fresh4 = (*inptr0 as c_int
                + *inptr0.offset(1) as c_int
                + *inptr1 as c_int
                + *inptr1.offset(1) as c_int
                + bias
                >> 2i32) as JSAMPLE;
            bias ^= 3i32;
            inptr0 = inptr0.offset(2);
            inptr1 = inptr1.offset(2);
            outcol +=  1
        }
        inrow += 2i32;
        outrow += 1
    }
}
/*
 * Downsample pixel values of a single component.
 * This version handles the standard case of 2:1 horizontal and 2:1 vertical,
 * with smoothing.  One row of context is required.
 */

unsafe extern "C" fn h2v2_smooth_downsample(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data: JSAMPARRAY,
) {
    
    
        
    let mut output_cols: JDIMENSION =  (*compptr)
        .width_in_blocks * DCTSIZE as c_uint;
    
    
    
    
    
    
    
    
    
    /* Expand input data enough to let all the output samples be generated
     * by the standard loop.  Special-casing padded output would be more
     * efficient.
     */
    expand_right_edge(
        input_data.offset(-1),
        (*cinfo).max_v_samp_factor + 2i32,
        (*cinfo).image_width,
        
        output_cols * 2u32,
    );
    /* We don't bother to form the individual "smoothed" input pixel values;
     * we can directly compute the output which is the average of the four
     * smoothed values.  Each of the four member pixels contributes a fraction
     * (1-8*SF) to its own smoothed image and a fraction SF to each of the three
     * other smoothed pixels, therefore a total fraction (1-5*SF)/4 to the final
     * output.  The four corner-adjacent neighbor pixels contribute a fraction
     * SF to just one smoothed pixel, or SF/4 to the final output; while the
     * eight edge-adjacent neighbors contribute SF to each of two smoothed
     * pixels, or SF/2 overall.  In order to use integer arithmetic, these
     * factors are scaled by 2^16 = 65536.
     * Also recall that SF = smoothing_factor / 1024.
     */
     /* scaled (1-5*SF)/4 */
     /* scaled SF/4 */
    
     let mut memberscale:   JLONG =
     (16384i32 - (*cinfo).smoothing_factor * 80i32) as JLONG; let mut neighscale:   JLONG =
     ((*cinfo).smoothing_factor * 16i32) as JLONG; let mut inrow:   c_int =  0i32; let mut outrow:   c_int =  0i32;
    while outrow < (*compptr).v_samp_factor {
                
        
        
        
        
        
         let mut outptr:   JSAMPROW =
     *output_data.offset(outrow as isize); let mut inptr0:   JSAMPROW =
     *input_data.offset(inrow as isize); let mut inptr1:   JSAMPROW =
     *input_data.offset((inrow + 1i32) as isize); let mut above_ptr:   JSAMPROW =
     *input_data.offset((inrow - 1i32) as isize); let mut below_ptr:   JSAMPROW =
     *input_data.offset((inrow + 2i32) as isize); let mut membersum:   JLONG =
     (*inptr0 as c_int
            + *inptr0.offset(1) as c_int
            + *inptr1 as c_int
            + *inptr1.offset(1) as c_int) as JLONG; let mut neighsum:   JLONG =
     (*above_ptr as c_int
            + *above_ptr.offset(1) as c_int
            + *below_ptr as c_int
            + *below_ptr.offset(1) as c_int
            + *inptr0 as c_int
            + *inptr0.offset(2) as c_int
            + *inptr1 as c_int
            + *inptr1.offset(2) as c_int) as JLONG;
        neighsum += neighsum;
        neighsum += (*above_ptr as c_int
            + *above_ptr.offset(2) as c_int
            + *below_ptr as c_int
            + *below_ptr.offset(2) as c_int) as c_long;
        membersum = membersum * memberscale + neighsum * neighscale;
        let fresh5 = outptr;
        outptr = outptr.offset(1);
        *fresh5 = (membersum + 32768i64 >> 16i32) as JSAMPLE;
        inptr0 = inptr0.offset(2);
        inptr1 = inptr1.offset(2);
        above_ptr = above_ptr.offset(2);
        below_ptr = below_ptr.offset(2);
         let mut colctr:   JDIMENSION =   output_cols - 2u32;
        while colctr > 0u32 {
            /* sum of pixels directly mapped to this output element */
            membersum = (*inptr0 as c_int
                + *inptr0.offset(1) as c_int
                + *inptr1 as c_int
                + *inptr1.offset(1) as c_int)
                as JLONG;
            /* sum of edge-neighbor pixels */
            neighsum = (*above_ptr as c_int
                + *above_ptr.offset(1) as c_int
                + *below_ptr as c_int
                + *below_ptr.offset(1) as c_int
                + *inptr0.offset(-1isize) as c_int
                + *inptr0.offset(2) as c_int
                + *inptr1.offset(-1isize) as c_int
                + *inptr1.offset(2) as c_int)
                as JLONG;
            /* The edge-neighbors count twice as much as corner-neighbors */
            neighsum += neighsum;
            /* Add in the corner-neighbors */
            neighsum += (*above_ptr.offset(-1isize) as c_int
                + *above_ptr.offset(2) as c_int
                + *below_ptr.offset(-1isize) as c_int
                + *below_ptr.offset(2) as c_int) as c_long;
            /* form final output scaled up by 2^16 */
            membersum = membersum * memberscale + neighsum * neighscale;
            /* round, descale and output it */
            let fresh6 = outptr;
            outptr = outptr.offset(1);
            *fresh6 = (membersum + 32768i64 >> 16i32) as JSAMPLE;
            inptr0 = inptr0.offset(2);
            inptr1 = inptr1.offset(2);
            above_ptr = above_ptr.offset(2);
            below_ptr = below_ptr.offset(2);
            colctr -=  1
        }
        /* Special case for last column */
        membersum = (*inptr0 as c_int
            + *inptr0.offset(1) as c_int
            + *inptr1 as c_int
            + *inptr1.offset(1) as c_int) as JLONG;
        neighsum = (*above_ptr as c_int
            + *above_ptr.offset(1) as c_int
            + *below_ptr as c_int
            + *below_ptr.offset(1) as c_int
            + *inptr0.offset(-1isize) as c_int
            + *inptr0.offset(1) as c_int
            + *inptr1.offset(-1isize) as c_int
            + *inptr1.offset(1) as c_int) as JLONG;
        neighsum += neighsum;
        neighsum += (*above_ptr.offset(-1isize) as c_int
            + *above_ptr.offset(1) as c_int
            + *below_ptr.offset(-1isize) as c_int
            + *below_ptr.offset(1) as c_int) as c_long;
        membersum = membersum * memberscale + neighsum * neighscale;
        *outptr = (membersum + 32768i64 >> 16i32) as JSAMPLE;
        inrow += 2i32;
        outrow += 1
    }
}
/*
 * Downsample pixel values of a single component.
 * This version handles the special case of a full-size component,
 * with smoothing.  One row of context is required.
 */

unsafe extern "C" fn fullsize_smooth_downsample(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data: JSAMPARRAY,
) {
    
       
    let mut output_cols: JDIMENSION =  (*compptr)
        .width_in_blocks * DCTSIZE as c_uint;
    
    
    
    
    
    
    
    
    
    
    
    /* Expand input data enough to let all the output samples be generated
     * by the standard loop.  Special-casing padded output would be more
     * efficient.
     */
    expand_right_edge(
        input_data.offset(-1),
        (*cinfo).max_v_samp_factor + 2i32,
        (*cinfo).image_width,
        output_cols,
    );
    /* Each of the eight neighbor pixels contributes a fraction SF to the
     * smoothed pixel, while the main pixel contributes (1-8*SF).  In order
     * to use integer arithmetic, these factors are multiplied by 2^16 = 65536.
     * Also recall that SF = smoothing_factor / 1024.
     */
     /* scaled 1-8*SF */
     /* scaled SF */
     let mut memberscale:   JLONG =
     65536i64 - (*cinfo).smoothing_factor as c_long * 512i64; let mut neighscale:   JLONG =
     ((*cinfo).smoothing_factor * 64i32) as JLONG; let mut outrow:   c_int =  0i32;
    while outrow < (*compptr).v_samp_factor {
                  
        
        
         let mut outptr:   JSAMPROW =
     *output_data.offset(outrow as isize); let mut inptr:   JSAMPROW =
     *input_data.offset(outrow as isize); let mut above_ptr:   JSAMPROW =
     *input_data.offset((outrow - 1i32) as isize); let mut below_ptr:   JSAMPROW =
     *input_data.offset((outrow + 1i32) as isize);
        /* Special case for first column */
        let fresh7 = above_ptr;
        above_ptr = above_ptr.offset(1);
        let fresh8 = below_ptr;
        below_ptr = below_ptr.offset(1);
         let mut colsum:   c_int =
     *fresh7 as c_int + *fresh8 as c_int + *inptr as c_int;
        let fresh9 = inptr;
        inptr = inptr.offset(1);
        
        
         let mut membersum:   JLONG =
     *fresh9 as c_int as JLONG; let mut nextcolsum:   c_int =
     *above_ptr as c_int + *below_ptr as c_int + *inptr as c_int; let mut neighsum:   JLONG =
     colsum as c_long
            + (colsum as c_long - membersum)
            + nextcolsum as c_long;
        membersum = membersum * memberscale + neighsum * neighscale;
        let fresh10 = outptr;
        outptr = outptr.offset(1);
        *fresh10 = (membersum + 32768i64 >> 16i32) as JSAMPLE;
         let mut lastcolsum:   c_int =  colsum;
        colsum = nextcolsum;
         let mut colctr:   JDIMENSION =   output_cols - 2u32;
        while colctr > 0u32 {
            let fresh11 = inptr;
            inptr = inptr.offset(1);
            membersum = *fresh11 as c_int as JLONG;
            above_ptr = above_ptr.offset(1);
            below_ptr = below_ptr.offset(1);
            nextcolsum =
                *above_ptr as c_int + *below_ptr as c_int + *inptr as c_int;
            neighsum = lastcolsum as c_long
                + (colsum as c_long - membersum)
                + nextcolsum as c_long;
            membersum = membersum * memberscale + neighsum * neighscale;
            let fresh12 = outptr;
            outptr = outptr.offset(1);
            *fresh12 =
                (membersum + 32768i64 >> 16i32) as JSAMPLE;
            lastcolsum = colsum;
            colsum = nextcolsum;
            colctr -=  1
        }
        /* Special case for last column */
        membersum = *inptr as c_int as JLONG;
        neighsum = lastcolsum as c_long
            + (colsum as c_long - membersum)
            + colsum as c_long;
        membersum = membersum * memberscale + neighsum * neighscale;
        *outptr = (membersum + 32768i64 >> 16i32) as JSAMPLE;
        outrow += 1
    }
}
/* It is useful to allow each component to have a separate IDCT method. */
/* Upsampling (note that upsampler must also call color converter) */
/* TRUE if need rows above & below */
/* Colorspace conversion */
/* Color quantization or color precision reduction */
/* Miscellaneous useful macros */
/* We assume that right shift corresponds to signed division by 2 with
 * rounding towards minus infinity.  This is correct for typical "arithmetic
 * shift" instructions that shift in copies of the sign bit.  But some
 * C compilers implement >> with an unsigned shift.  For these machines you
 * must define RIGHT_SHIFT_IS_UNSIGNED.
 * RIGHT_SHIFT provides a proper signed right shift of a JLONG quantity.
 * It is only applied with constant shift counts.  SHIFT_TEMPS must be
 * included in the variables of any routine using RIGHT_SHIFT.
 */
/* Compression module initialization routines */
/* INPUT_SMOOTHING_SUPPORTED */
/*
 * Module initialization routine for downsampling.
 * Note that we must select a routine for each component.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_downsampler(mut cinfo: j_compress_ptr) {
    
    
       
    let mut smoothok: boolean = TRUE;
     let mut downsample:   my_downsample_ptr =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<my_downsampler>() as c_ulong,
    ) as my_downsample_ptr;
    (*cinfo).downsample = downsample as *mut jpeg_downsampler;
    (*downsample).pub_0.start_pass = Some(
        start_pass_downsample as unsafe extern "C" fn(_: j_compress_ptr) -> (),
    );
    (*downsample).pub_0.downsample = Some(
        sep_downsample
            as unsafe extern "C" fn(
                _: j_compress_ptr,
                _: JSAMPIMAGE,
                _: JDIMENSION,
                _: JSAMPIMAGE,
                _: JDIMENSION,
            ) -> (),
    );
    (*downsample).pub_0.need_context_rows = FALSE;
    if (*cinfo).CCIR601_sampling != 0 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_CCIR601_NOTIMPL as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    
     let mut ci:   c_int =  0i32; let mut compptr:   *mut jpeg_component_info =
     (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        if (*compptr).h_samp_factor == (*cinfo).max_h_samp_factor
            && (*compptr).v_samp_factor == (*cinfo).max_v_samp_factor
        {
            if (*cinfo).smoothing_factor != 0 {
                (*downsample).methods[ci as usize] = Some(
                    fullsize_smooth_downsample
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: *mut jpeg_component_info,
                            _: JSAMPARRAY,
                            _: JSAMPARRAY,
                        ) -> (),
                );
                (*downsample).pub_0.need_context_rows = TRUE
            } else {
                (*downsample).methods[ci as usize] = Some(
                    fullsize_downsample
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: *mut jpeg_component_info,
                            _: JSAMPARRAY,
                            _: JSAMPARRAY,
                        ) -> (),
                )
            }
        } else if (*compptr).h_samp_factor * 2i32 == (*cinfo).max_h_samp_factor
            && (*compptr).v_samp_factor == (*cinfo).max_v_samp_factor
        {
            smoothok = FALSE;
            if super::simd::x86_64::jsimd::jsimd_can_h2v1_downsample() != 0 {
                (*downsample).methods[ci as usize] = Some(
                    super::simd::x86_64::jsimd::jsimd_h2v1_downsample
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: *mut jpeg_component_info,
                            _: JSAMPARRAY,
                            _: JSAMPARRAY,
                        ) -> (),
                )
            } else {
                (*downsample).methods[ci as usize] = Some(
                    h2v1_downsample
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: *mut jpeg_component_info,
                            _: JSAMPARRAY,
                            _: JSAMPARRAY,
                        ) -> (),
                )
            }
        } else if (*compptr).h_samp_factor * 2i32 == (*cinfo).max_h_samp_factor
            && (*compptr).v_samp_factor * 2i32 == (*cinfo).max_v_samp_factor
        {
            if (*cinfo).smoothing_factor != 0 {
                (*downsample).methods[ci as usize] = Some(
                    h2v2_smooth_downsample
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: *mut jpeg_component_info,
                            _: JSAMPARRAY,
                            _: JSAMPARRAY,
                        ) -> (),
                );
                (*downsample).pub_0.need_context_rows = TRUE
            } else if super::simd::x86_64::jsimd::jsimd_can_h2v2_downsample() != 0 {
                (*downsample).methods[ci as usize] = Some(
                    super::simd::x86_64::jsimd::jsimd_h2v2_downsample
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: *mut jpeg_component_info,
                            _: JSAMPARRAY,
                            _: JSAMPARRAY,
                        ) -> (),
                )
            } else {
                (*downsample).methods[ci as usize] = Some(
                    h2v2_downsample
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: *mut jpeg_component_info,
                            _: JSAMPARRAY,
                            _: JSAMPARRAY,
                        ) -> (),
                )
            }
        } else if (*cinfo).max_h_samp_factor % (*compptr).h_samp_factor == 0i32
            && (*cinfo).max_v_samp_factor % (*compptr).v_samp_factor == 0i32
        {
            smoothok = FALSE;
            (*downsample).methods[ci as usize] = Some(
                int_downsample
                    as unsafe extern "C" fn(
                        _: j_compress_ptr,
                        _: *mut jpeg_component_info,
                        _: JSAMPARRAY,
                        _: JSAMPARRAY,
                    ) -> (),
            )
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
        ci += 1;
        compptr = compptr.offset(1)
    }
    if (*cinfo).smoothing_factor != 0 && smoothok == 0 {
        (*(*cinfo).err).msg_code = super::jerror::JTRC_SMOOTH_NOTIMPL as c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, 0i32);
    };
}
