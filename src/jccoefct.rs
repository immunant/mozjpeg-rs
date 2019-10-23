


















































































































































































































use libc::{c_uint, c_ulong, c_void, c_long, c_int, self};pub use crate::jmorecfg_h::{boolean, FALSE, JCOEF, JDIMENSION, JOCTET,
                            JSAMPLE, TRUE, UINT16, UINT8};pub use crate::jpegint_h::{jround_up, jzero_far, JBUF_CRANK_DEST,
                           JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS,
                           JBUF_SAVE_SOURCE, J_BUF_MODE};pub use crate::stddef_h::{size_t, NULL};pub use crate::jpeglib_h::{j_common_ptr, j_compress_ptr,
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
                           C2RustUnnamed_2, JCS_YCbCr, C_MAX_BLOCKS_IN_MCU,
                           DCTSIZE, JBLOCK, JBLOCKARRAY, JBLOCKROW, JCS_CMYK,
                           JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR,
                           JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB,
                           JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR,
                           JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565,
                           JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT, JDCT_IFAST,
                           JDCT_ISLOW, JHUFF_TBL, JPOOL_IMAGE, JQUANT_TBL,
                           JSAMPARRAY, JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE,
                           J_DCT_METHOD};pub use super::jchuff::{c_derived_tbl, jpeg_make_c_derived_tbl,
                        quantize_trellis};pub use super::jerror::{C2RustUnnamed_3, JERR_ARITH_NOTIMPL,
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

pub type my_coef_ptr = *mut my_coef_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_coef_controller {
    pub pub_0: jpeg_c_coef_controller,
    pub iMCU_row_num: JDIMENSION,
    pub mcu_ctr: JDIMENSION,
    pub MCU_vert_offset: c_int,
    pub MCU_rows_per_iMCU_row: c_int,
    pub MCU_buffer: [JBLOCKROW; 10],
    pub whole_image: [jvirt_barray_ptr; 10],
    pub whole_image_uq: [jvirt_barray_ptr; 10],
}

unsafe extern "C" fn start_iMCU_row(mut cinfo: j_compress_ptr)
/* Reset within-iMCU-row counters for a new row */
{
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    /* In an interleaved scan, an MCU row is the same as an iMCU row.
     * In a noninterleaved scan, an iMCU row has v_samp_factor MCU rows.
     * But at the bottom of the image, process only what's left.
     */
    if (*cinfo).comps_in_scan > 1i32 {
        (*coef).MCU_rows_per_iMCU_row = 1i32
    } else if (*coef).iMCU_row_num <  (*cinfo).total_iMCU_rows - 1u32 {
        (*coef).MCU_rows_per_iMCU_row = (*(*cinfo).cur_comp_info[0]).v_samp_factor
    } else {
        (*coef).MCU_rows_per_iMCU_row = (*(*cinfo).cur_comp_info[0]).last_row_height
    }
    (*coef).mcu_ctr = 0u32;
    (*coef).MCU_vert_offset = 0i32;
}
/*
 * Initialize for a processing pass.
 */

unsafe extern "C" fn start_pass_coef(
    mut cinfo: j_compress_ptr,
    mut pass_mode: J_BUF_MODE,
) {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    (*coef).iMCU_row_num = 0u32;
    start_iMCU_row(cinfo);
    match  pass_mode {
        0 => {
            if !(*coef).whole_image[0].is_null() {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_BUFFER_MODE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            (*coef).pub_0.compress_data = Some(
                compress_data
                    as unsafe extern "C" fn(
                        _: j_compress_ptr,
                        _: JSAMPIMAGE,
                    ) -> boolean,
            )
        }
        3 => {
            if (*coef).whole_image[0].is_null() {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_BUFFER_MODE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            (*coef).pub_0.compress_data = Some(
                compress_first_pass
                    as unsafe extern "C" fn(
                        _: j_compress_ptr,
                        _: JSAMPIMAGE,
                    ) -> boolean,
            )
        }
        2 => {
            if (*coef).whole_image[0].is_null() {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_BUFFER_MODE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            (*coef).pub_0.compress_data = Some(
                compress_output
                    as unsafe extern "C" fn(
                        _: j_compress_ptr,
                        _: JSAMPIMAGE,
                    ) -> boolean,
            )
        }
        4 => {
            if (*coef).whole_image[0].is_null() {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_BUFFER_MODE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            (*coef).pub_0.compress_data = Some(
                compress_trellis_pass
                    as unsafe extern "C" fn(
                        _: j_compress_ptr,
                        _: JSAMPIMAGE,
                    ) -> boolean,
            )
        }
        _ => {
            (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_BUFFER_MODE as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
    };
}
/* Forward declarations */
/*
 * Process some data in the single-pass case.
 * We process the equivalent of one fully interleaved MCU row ("iMCU" row)
 * per call, ie, v_samp_factor block rows for each component in the image.
 * Returns TRUE if the iMCU row is completed, FALSE if suspended.
 *
 * NB: input_buf contains a plane for each component in image,
 * which we index according to the component's SOF position.
 */

unsafe extern "C" fn compress_data(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPIMAGE,
) -> boolean {
     let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr; /* index of current MCU within row */
    
    let mut last_MCU_col: JDIMENSION =
        
        (*cinfo).MCUs_per_row - 1u32;
    let mut last_iMCU_row: JDIMENSION =
        
        (*cinfo).total_iMCU_rows - 1u32;
    
    
    
    
    
    
    
    
    
     let mut yoffset:   c_int =  (*coef).MCU_vert_offset;
    while yoffset < (*coef).MCU_rows_per_iMCU_row {
          let mut MCU_col_num:   JDIMENSION =  (*coef).mcu_ctr;
        while MCU_col_num <= last_MCU_col {
            /* Determine where data comes from in input_buf and do the DCT thing.
             * Each call on forward_DCT processes a horizontal row of DCT blocks
             * as wide as an MCU; we rely on having allocated the MCU_buffer[] blocks
             * sequentially.  Dummy blocks at the right or bottom edge are filled in
             * specially.  The data in them does not matter for image reconstruction,
             * so we fill them with values that will encode to the smallest amount of
             * data, viz: all zeroes in the AC entries, DC entries equal to previous
             * block's DC value.  (Thanks to Thomas Kinsman for this idea.)
             */
               /* ypos == (yoffset+yindex) * DCTSIZE */
             let mut blkn:   c_int =  0i32; let mut ci:   c_int =  0i32;
            while ci < (*cinfo).comps_in_scan {
                     
                
                
                
                 let mut compptr:   *mut jpeg_component_info =
     (*cinfo).cur_comp_info[ci as usize]; let mut blockcnt:   c_int =
     if MCU_col_num < last_MCU_col {
                    (*compptr).MCU_width
                } else {
                    (*compptr).last_col_width
                }; let mut xpos:   JDIMENSION =
      MCU_col_num * (*compptr).MCU_sample_width as c_uint; let mut ypos:   JDIMENSION =
     (yoffset * DCTSIZE) as JDIMENSION; let mut yindex:   c_int =  0i32;
                while yindex < (*compptr).MCU_height {
                     let mut bi:  c_int =  0;if (*coef).iMCU_row_num < last_iMCU_row
                        || yoffset + yindex < (*compptr).last_row_height
                    {
                        Some(
                            (*(*cinfo).fdct)
                                .forward_DCT
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo,
                            compptr,
                            *input_buf.offset((*compptr).component_index as isize),
                            (*coef).MCU_buffer[blkn as usize],
                            ypos,
                            xpos,
                            blockcnt as JDIMENSION,
                            NULL as JBLOCKROW,
                        );
                        if blockcnt < (*compptr).MCU_width {
                            /* Create some dummy blocks at the right edge of the image. */
                            jzero_far(
                                (*coef).MCU_buffer[(blkn + blockcnt) as usize] as *mut c_void,
                                ((*compptr).MCU_width - blockcnt) as c_ulong *
    
                                    ::std::mem::size_of::<JBLOCK>()
                                        as c_ulong,
                            );
                            bi = blockcnt;
                            while bi < (*compptr).MCU_width {
                                (*(*coef).MCU_buffer[(blkn + bi) as usize].offset(0))[0] =
                                    (*(*coef).MCU_buffer[(blkn + bi - 1i32) as usize].offset(0))[0];
                                bi += 1
                            }
                        }
                    } else {
                        /* Create a row of dummy blocks at the bottom of the image. */
                        jzero_far(
                            (*coef).MCU_buffer[blkn as usize] as *mut c_void,
                            (*compptr).MCU_width as c_ulong *
    ::std::mem::size_of::<JBLOCK>()
                                    as c_ulong,
                        );
                        bi = 0i32;
                        while bi < (*compptr).MCU_width {
                            (*(*coef).MCU_buffer[(blkn + bi) as usize].offset(0))[0] =
                                (*(*coef).MCU_buffer[(blkn - 1i32) as usize].offset(0))[0];
                            bi += 1
                        }
                    }
                    blkn += (*compptr).MCU_width;
                    ypos +=  DCTSIZE as c_uint;
                    yindex += 1
                }
                ci += 1
            }
            /* Try to write the MCU.  In event of a suspension failure, we will
             * re-DCT the MCU on restart (a bit inefficient, could be fixed...)
             */
            if Some(
                (*(*cinfo).entropy)
                    .encode_mcu
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo, (*coef).MCU_buffer.as_mut_ptr()
            ) == 0
            {
                /* Suspension forced; update state counters and exit */
                (*coef).MCU_vert_offset = yoffset;
                (*coef).mcu_ctr = MCU_col_num;
                return FALSE;
            }
            MCU_col_num +=  1
        }
        /* Completed an MCU row, but perhaps not an iMCU row */
        (*coef).mcu_ctr = 0u32;
        yoffset += 1
    }
    /* Completed the iMCU row, advance counters for next one */
    (*coef).iMCU_row_num =  (*coef).iMCU_row_num + 1;
    start_iMCU_row(cinfo);
    return TRUE;
}
/*
 * Process some data in the first pass of a multi-pass case.
 * We process the equivalent of one fully interleaved MCU row ("iMCU" row)
 * per call, ie, v_samp_factor block rows for each component in the image.
 * This amount of data is read from the source buffer, DCT'd and quantized,
 * and saved into the virtual arrays.  We also generate suitable dummy blocks
 * as needed at the right and lower edges.  (The dummy blocks are constructed
 * in the virtual arrays, which have been padded appropriately.)  This makes
 * it possible for subsequent passes not to worry about real vs. dummy blocks.
 *
 * We must also emit the data to the entropy encoder.  This is conveniently
 * done by calling compress_output() after we've loaded the current strip
 * of the virtual arrays.
 *
 * NB: input_buf contains a plane for each component in image.  All
 * components are DCT'd and loaded into the virtual arrays in this pass.
 * However, it may be that only a subset of the components are emitted to
 * the entropy encoder during this first pass; be careful about looking
 * at the scan-dependent variables (MCU dimensions, etc).
 */

unsafe extern "C" fn compress_first_pass(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPIMAGE,
) -> boolean {
      let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    let mut last_iMCU_row: JDIMENSION =
        
        (*cinfo).total_iMCU_rows - 1u32;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
     let mut ci:   c_int =  0i32; let mut compptr:   *mut jpeg_component_info =
     (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
         let mut bi:  c_int =  0;   let mut block_rows:  c_int =  0;  let mut lastDC:  JCOEF =  0;  let mut thisblockrow:  JBLOCKROW =
     ::std::ptr::null_mut::< JBLOCK>(); 
         let mut buffer:   JBLOCKARRAY =
     Some(
            (*(*cinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*coef).whole_image[ci as usize],
            
            (*coef)
                .iMCU_row_num * (*compptr).v_samp_factor as c_uint,
            (*compptr).v_samp_factor as JDIMENSION,
            TRUE,
        ); let mut buffer_dst:   JBLOCKARRAY =
     Some(
            (*(*cinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*coef).whole_image_uq[ci as usize],
            
            (*coef)
                .iMCU_row_num * (*compptr).v_samp_factor as c_uint,
            (*compptr).v_samp_factor as JDIMENSION,
            TRUE,
        );
        /* Count non-dummy DCT block rows in this iMCU row. */
        if (*coef).iMCU_row_num < last_iMCU_row {
            block_rows = (*compptr).v_samp_factor
        } else {
            /* NB: can't use last_row_height here, since may not be set! */
            block_rows = ( (*compptr)
                .height_in_blocks % (*compptr).v_samp_factor as c_uint)
                as c_int;
            if block_rows == 0i32 {
                block_rows = (*compptr).v_samp_factor
            }
        }
        
        
         let mut blocks_across:   JDIMENSION =
     (*compptr).width_in_blocks; let mut h_samp_factor:   c_int =  (*compptr).h_samp_factor; let mut ndummy:   c_int =
     ( blocks_across % h_samp_factor as c_uint) as c_int;
        if ndummy > 0i32 {
            ndummy = h_samp_factor - ndummy
        }
        /* Perform DCT for all non-dummy blocks in this iMCU row.  Each call
         * on forward_DCT processes a complete horizontal row of DCT blocks.
         */
         let mut block_row:   c_int =  0i32;
        while block_row < block_rows {
            thisblockrow = *buffer.offset(block_row as isize);
            Some(
                (*(*cinfo).fdct)
                    .forward_DCT
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                compptr,
                *input_buf.offset(ci as isize),
                thisblockrow,
                (block_row * DCTSIZE) as JDIMENSION,
                0u32,
                blocks_across,
                *buffer_dst.offset(block_row as isize),
            );
            if ndummy > 0i32 {
                /* Create dummy blocks at the right edge of the image. */
                thisblockrow = thisblockrow.offset(blocks_across as isize); /* => first dummy block */
                jzero_far(
                    thisblockrow as *mut c_void,
                    ndummy as c_ulong *
    
                            ::std::mem::size_of::<JBLOCK>() as c_ulong,
                );
                lastDC = (*thisblockrow.offset(-1isize))[0];
                bi = 0i32;
                while bi < ndummy {
                    (*thisblockrow.offset(bi as isize))[0] = lastDC;
                    bi += 1
                }
            }
            block_row += 1
        }
        /* If at end of image, create dummy block rows as needed.
         * The tricky part here is that within each MCU, we want the DC values
         * of the dummy blocks to match the last real block's DC value.
         * This squeezes a few more bytes out of the resulting file...
         */
        if (*coef).iMCU_row_num == last_iMCU_row {
             blocks_across +=  ndummy as c_uint; /* include lower right corner */
             let mut MCUs_across:   JDIMENSION =
      blocks_across / h_samp_factor as c_uint; /* advance to next MCU in row */
            block_row = block_rows;
            while block_row < (*compptr).v_samp_factor {
                  thisblockrow = *buffer.offset(block_row as isize);
                 let mut lastblockrow:   JBLOCKROW =
     *buffer.offset((block_row - 1i32) as isize);
                jzero_far(
                    thisblockrow as *mut c_void,
                    blocks_across as c_ulong *
    
                            ::std::mem::size_of::<JBLOCK>() as c_ulong,
                );
                 let mut MCUindex:   JDIMENSION =  0u32;
                while MCUindex < MCUs_across {
                    lastDC = (*lastblockrow.offset((h_samp_factor - 1i32) as isize))[0];
                    bi = 0i32;
                    while bi < h_samp_factor {
                        (*thisblockrow.offset(bi as isize))[0] = lastDC;
                        bi += 1
                    }
                    thisblockrow = thisblockrow.offset(h_samp_factor as isize);
                    lastblockrow = lastblockrow.offset(h_samp_factor as isize);
                    MCUindex +=  1
                }
                block_row += 1
            }
        }
        ci += 1;
        compptr = compptr.offset(1)
    }
    /* NB: compress_output will increment iMCU_row_num if successful.
     * A suspension return will result in redoing all the work above next time.
     */
    /* Emit data to the entropy encoder, sharing code with subsequent passes */
    return compress_output(cinfo, input_buf);
}

unsafe extern "C" fn compress_trellis_pass(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPIMAGE,
) -> boolean {
     let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    let mut last_iMCU_row: JDIMENSION =
        
        (*cinfo).total_iMCU_rows - 1u32;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
     let mut ci:   c_int =  0i32;
    while ci < (*cinfo).comps_in_scan {
          let mut bi:  c_int =  0;   let mut block_rows:  c_int =  0;     let mut thisblockrow:  JBLOCKROW =
     ::std::ptr::null_mut::< JBLOCK>(); let mut lastblockrow:  JBLOCKROW =
     ::std::ptr::null_mut::< JBLOCK>();  let mut dctbl_data:  super::jchuff::c_derived_tbl =
     super::jchuff::c_derived_tbl{ehufco:  [0; 256], ehufsi:  [0; 256],}; let mut actbl_data:  super::jchuff::c_derived_tbl =
     super::jchuff::c_derived_tbl{ehufco:  [0; 256], ehufsi:  [0; 256],};
        let mut dctbl: *mut super::jchuff::c_derived_tbl = &mut dctbl_data;
        
        let mut actbl: *mut super::jchuff::c_derived_tbl = &mut actbl_data;
         let mut compptr:   *mut jpeg_component_info =
     (*cinfo).cur_comp_info[ci as usize];
        super::jchuff::jpeg_make_c_derived_tbl(
            cinfo,
            TRUE,
            (*compptr).dc_tbl_no,
            &mut dctbl,
        );
        super::jchuff::jpeg_make_c_derived_tbl(
            cinfo,
            FALSE,
            (*compptr).ac_tbl_no,
            &mut actbl,
        );
        
         let mut buffer:   JBLOCKARRAY =
     Some(
            (*(*cinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*coef).whole_image[(*compptr).component_index as usize],
            
            (*coef)
                .iMCU_row_num * (*compptr).v_samp_factor as c_uint,
            (*compptr).v_samp_factor as JDIMENSION,
            TRUE,
        ); let mut buffer_dst:   JBLOCKARRAY =
     Some(
            (*(*cinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*coef).whole_image_uq[(*compptr).component_index as usize],
            
            (*coef)
                .iMCU_row_num * (*compptr).v_samp_factor as c_uint,
            (*compptr).v_samp_factor as JDIMENSION,
            TRUE,
        );
        /* Count non-dummy DCT block rows in this iMCU row. */
        if (*coef).iMCU_row_num < last_iMCU_row {
            block_rows = (*compptr).v_samp_factor
        } else {
            /* NB: can't use last_row_height here, since may not be set! */
            block_rows = ( (*compptr)
                .height_in_blocks % (*compptr).v_samp_factor as c_uint)
                as c_int;
            if block_rows == 0i32 {
                block_rows = (*compptr).v_samp_factor
            }
        }
        
        
         let mut blocks_across:   JDIMENSION =
     (*compptr).width_in_blocks; let mut h_samp_factor:   c_int =  (*compptr).h_samp_factor; let mut ndummy:   c_int =
     ( blocks_across % h_samp_factor as c_uint) as c_int;
        if ndummy > 0i32 {
            ndummy = h_samp_factor - ndummy
        }
        
        /* Perform DCT for all non-dummy blocks in this iMCU row.  Each call
         * on forward_DCT processes a complete horizontal row of DCT blocks.
         */
         let mut lastDC:   JCOEF =  0i16; let mut block_row:   c_int =  0i32;
        while block_row < block_rows {
            thisblockrow = *buffer.offset(block_row as isize);
            lastblockrow = if block_row > 0i32 {
                *buffer.offset((block_row - 1i32) as isize)
            } else {
                NULL as JBLOCKROW
            };
            super::jchuff::quantize_trellis(
                cinfo,
                dctbl,
                actbl,
                thisblockrow,
                *buffer_dst.offset(block_row as isize),
                blocks_across,
                (*cinfo).quant_tbl_ptrs[(*compptr).quant_tbl_no as usize],
                (*(*cinfo).master).norm_src[(*compptr).quant_tbl_no as usize].as_mut_ptr(),
                (*(*cinfo).master).norm_coef[(*compptr).quant_tbl_no as usize].as_mut_ptr(),
                &mut lastDC,
                lastblockrow,
                *buffer_dst.offset((block_row - 1i32) as isize),
            );
            if ndummy > 0i32 {
                /* Create dummy blocks at the right edge of the image. */
                thisblockrow = thisblockrow.offset(blocks_across as isize); /* => first dummy block */
                jzero_far(
                    thisblockrow as *mut c_void,
                    ndummy as c_ulong *
    
                            ::std::mem::size_of::<JBLOCK>() as c_ulong,
                );
                lastDC = (*thisblockrow.offset(-1isize))[0];
                bi = 0i32;
                while bi < ndummy {
                    (*thisblockrow.offset(bi as isize))[0] = lastDC;
                    bi += 1
                }
            }
            block_row += 1
        }
        /* If at end of image, create dummy block rows as needed.
         * The tricky part here is that within each MCU, we want the DC values
         * of the dummy blocks to match the last real block's DC value.
         * This squeezes a few more bytes out of the resulting file...
         */
        if (*coef).iMCU_row_num == last_iMCU_row {
             blocks_across +=  ndummy as c_uint; /* include lower right corner */
             let mut MCUs_across:   JDIMENSION =
      blocks_across / h_samp_factor as c_uint; /* advance to next MCU in row */
            block_row = block_rows;
            while block_row < (*compptr).v_samp_factor {
                 thisblockrow = *buffer.offset(block_row as isize);
                lastblockrow = *buffer.offset((block_row - 1i32) as isize);
                jzero_far(
                    thisblockrow as *mut c_void,
                    blocks_across as c_ulong *
    
                            ::std::mem::size_of::<JBLOCK>() as c_ulong,
                );
                 let mut MCUindex:   JDIMENSION =  0u32;
                while MCUindex < MCUs_across {
                    lastDC = (*lastblockrow.offset((h_samp_factor - 1i32) as isize))[0];
                    bi = 0i32;
                    while bi < h_samp_factor {
                        (*thisblockrow.offset(bi as isize))[0] = lastDC;
                        bi += 1
                    }
                    thisblockrow = thisblockrow.offset(h_samp_factor as isize);
                    lastblockrow = lastblockrow.offset(h_samp_factor as isize);
                    MCUindex +=  1
                }
                block_row += 1
            }
        }
        ci += 1
    }
    /* NB: compress_output will increment iMCU_row_num if successful.
     * A suspension return will result in redoing all the work above next time.
     */
    /* Emit data to the entropy encoder, sharing code with subsequent passes */
    return compress_output(cinfo, input_buf);
}
/*
 * Process some data in subsequent passes of a multi-pass case.
 * We process the equivalent of one fully interleaved MCU row ("iMCU" row)
 * per call, ie, v_samp_factor block rows for each component in the scan.
 * The data is obtained from the virtual arrays and fed to the entropy coder.
 * Returns TRUE if the iMCU row is completed, FALSE if suspended.
 *
 * NB: input_buf is ignored; it is likely to be a NULL pointer.
 */

unsafe extern "C" fn compress_output(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPIMAGE,
) -> boolean {
       let mut buffer:  [JBLOCKARRAY; 4] =
     [::std::ptr::null_mut::< JBLOCKROW>(); 4]; let mut compptr:  *mut jpeg_component_info =
    
        ::std::ptr::null_mut::< jpeg_component_info>();let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr; /* index of current MCU within row */
    
    
    
    
    
    
    
    
    
    
    /* Align the virtual buffers for the components used in this scan.
     * NB: during first pass, this is safe only because the buffers will
     * already be aligned properly, so jmemmgr.c won't need to do any I/O.
     */
     let mut ci:   c_int =  0i32;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        buffer[ci as usize] = Some(
            (*(*cinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*coef).whole_image[(*compptr).component_index as usize],
            
            (*coef)
                .iMCU_row_num * (*compptr).v_samp_factor as c_uint,
            (*compptr).v_samp_factor as JDIMENSION,
            FALSE,
        );
        ci += 1
    }
     let mut yoffset:   c_int =  (*coef).MCU_vert_offset;
    while yoffset < (*coef).MCU_rows_per_iMCU_row {
          let mut MCU_col_num:   JDIMENSION =  (*coef).mcu_ctr;
        while MCU_col_num < (*cinfo).MCUs_per_row {
             let mut blkn:   c_int =  0i32; /* index of current DCT block within MCU */
            ci = 0i32;
            while ci < (*cinfo).comps_in_scan {
                  compptr = (*cinfo).cur_comp_info[ci as usize];
                
                 let mut start_col:   JDIMENSION =
      MCU_col_num * (*compptr).MCU_width as c_uint; let mut yindex:   c_int =  0i32;
                while yindex < (*compptr).MCU_height {
                      
                     let mut buffer_ptr:   JBLOCKROW =
     (*buffer[ci as usize].offset((yindex + yoffset) as isize))
                        .offset(start_col as isize); let mut xindex:   c_int =  0i32;
                    while xindex < (*compptr).MCU_width {
                        let fresh0 = buffer_ptr;
                        buffer_ptr = buffer_ptr.offset(1);
                        let fresh1 = blkn;
                        blkn +=  1;
                        (*coef).MCU_buffer[fresh1 as usize] = fresh0;
                        xindex += 1
                    }
                    yindex += 1
                }
                ci += 1
            }
            /* Try to write the MCU. */
            if Some(
                (*(*cinfo).entropy)
                    .encode_mcu
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo, (*coef).MCU_buffer.as_mut_ptr()
            ) == 0
            {
                /* Suspension forced; update state counters and exit */
                (*coef).MCU_vert_offset = yoffset;
                (*coef).mcu_ctr = MCU_col_num;
                return FALSE;
            }
            MCU_col_num +=  1
        }
        /* Completed an MCU row, but perhaps not an iMCU row */
        (*coef).mcu_ctr = 0u32;
        yoffset += 1
    }
    /* Completed the iMCU row, advance counters for next one */
    (*coef).iMCU_row_num =  (*coef).iMCU_row_num + 1;
    start_iMCU_row(cinfo);
    return TRUE;
}
/* FULL_COEF_BUFFER_SUPPORTED */
/*
 * Initialize coefficient buffer controller.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_c_coef_controller(
    mut cinfo: j_compress_ptr,
    mut need_full_buffer: boolean,
) {
     
     let mut coef:   my_coef_ptr =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<my_coef_controller>() as c_ulong,
    ) as my_coef_ptr;
    (*cinfo).coef = coef as *mut jpeg_c_coef_controller;
    (*coef).pub_0.start_pass = Some(
        start_pass_coef
            as unsafe extern "C" fn(
                _: j_compress_ptr,
                _: J_BUF_MODE,
            ) -> (),
    );
    /* Create the coefficient buffer. */
    if need_full_buffer != 0 {
        
          
        
         let mut ci:   c_int =  0i32; let mut compptr:   *mut jpeg_component_info =
     (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
            (*coef).whole_image[ci as usize] = Some(
                (*(*cinfo).mem)
                    .request_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                JPOOL_IMAGE,
                FALSE,
                jround_up(
                    (*compptr).width_in_blocks as c_long,
                    (*compptr).h_samp_factor as c_long,
                ) as JDIMENSION,
                jround_up(
                    (*compptr).height_in_blocks as c_long,
                    (*compptr).v_samp_factor as c_long,
                ) as JDIMENSION,
                (*compptr).v_samp_factor as JDIMENSION,
            );
            (*coef).whole_image_uq[ci as usize] = Some(
                (*(*cinfo).mem)
                    .request_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                JPOOL_IMAGE,
                FALSE,
                jround_up(
                    (*compptr).width_in_blocks as c_long,
                    (*compptr).h_samp_factor as c_long,
                ) as JDIMENSION,
                jround_up(
                    (*compptr).height_in_blocks as c_long,
                    (*compptr).v_samp_factor as c_long,
                ) as JDIMENSION,
                (*compptr).v_samp_factor as JDIMENSION,
            );
            ci += 1;
            compptr = compptr.offset(1)
        }
    } else {
        
          
        
         let mut buffer:   JBLOCKROW =
     Some(
            (*(*cinfo).mem)
                .alloc_large
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            C_MAX_BLOCKS_IN_MCU as c_ulong *
    ::std::mem::size_of::<JBLOCK>() as c_ulong,
        ) as JBLOCKROW; let mut i:   c_int =  0i32;
        while i < C_MAX_BLOCKS_IN_MCU {
            (*coef).MCU_buffer[i as usize] = buffer.offset(i as isize);
            i += 1
        }
        (*coef).whole_image[0] = NULL as jvirt_barray_ptr
    };
}
