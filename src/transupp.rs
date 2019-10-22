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
    inverse_DCT_method_ptr, jcopy_block_row, jdiv_round_up, JBUF_CRANK_DEST, JBUF_PASS_THRU,
    JBUF_REQUANT, JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_compress_ptr, j_decompress_ptr, jpeg_c_coef_controller, jpeg_c_main_controller,
    jpeg_c_prep_controller, jpeg_color_converter, jpeg_color_deconverter, jpeg_color_quantizer,
    jpeg_common_struct, jpeg_comp_master, jpeg_component_info, jpeg_compress_struct,
    jpeg_d_coef_controller, jpeg_d_main_controller, jpeg_d_post_controller, jpeg_decomp_master,
    jpeg_decompress_struct, jpeg_destination_mgr, jpeg_downsampler, jpeg_entropy_decoder,
    jpeg_entropy_encoder, jpeg_error_mgr, jpeg_forward_dct, jpeg_input_controller,
    jpeg_inverse_dct, jpeg_marker_parser_method, jpeg_marker_reader, jpeg_marker_struct,
    jpeg_marker_writer, jpeg_memory_mgr, jpeg_progress_mgr, jpeg_save_markers,
    jpeg_saved_marker_ptr, jpeg_scan_info, jpeg_set_colorspace, jpeg_source_mgr, jpeg_upsampler,
    jpeg_write_marker, jvirt_barray_control, jvirt_barray_ptr, jvirt_sarray_control,
    jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr, DCTSIZE, DCTSIZE2, JBLOCK, JBLOCKARRAY,
    JBLOCKROW, JCOEFPTR, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA,
    JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB,
    JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW,
    JDITHER_FS, JDITHER_NONE, JDITHER_ORDERED, JHUFF_TBL, JPEG_APP0, JPEG_COM, JPOOL_IMAGE,
    JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD, J_DITHER_MODE,
    NUM_QUANT_TBLS,
};
pub use crate::stddef_h::{size_t, NULL};
pub use crate::stdlib::{
    C2RustUnnamed_0, _ISalnum, _ISalpha, _ISblank, _IScntrl, _ISdigit, _ISgraph, _ISlower,
    _ISprint, _ISpunct, _ISspace, _ISupper, _ISxdigit, __ctype_b_loc,
};
use libc::{self, c_char, c_int, c_long, c_uint, c_ulong, c_ushort};
// =============== BEGIN transupp_h ================
pub type JXFORM_CODE = c_uint;

pub type JCROP_CODE = c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_transform_info {
    pub transform: JXFORM_CODE,
    pub perfect: boolean,
    pub trim: boolean,
    pub force_grayscale: boolean,
    pub crop: boolean,
    pub slow_hflip: boolean,
    pub crop_width: JDIMENSION,
    pub crop_width_set: JCROP_CODE,
    pub crop_height: JDIMENSION,
    pub crop_height_set: JCROP_CODE,
    pub crop_xoffset: JDIMENSION,
    pub crop_xoffset_set: JCROP_CODE,
    pub crop_yoffset: JDIMENSION,
    pub crop_yoffset_set: JCROP_CODE,
    pub num_components: c_int,
    pub workspace_coef_arrays: *mut jvirt_barray_ptr,
    pub output_width: JDIMENSION,
    pub output_height: JDIMENSION,
    pub x_crop_offset: JDIMENSION,
    pub y_crop_offset: JDIMENSION,
    pub iMCU_sample_width: c_int,
    pub iMCU_sample_height: c_int,
}

pub type JCOPY_OPTION = c_uint;

pub const JXFORM_ROT_270: JXFORM_CODE = 7;

pub const JXFORM_ROT_180: JXFORM_CODE = 6;

pub const JXFORM_ROT_90: JXFORM_CODE = 5;

pub const JXFORM_TRANSVERSE: JXFORM_CODE = 4;

pub const JXFORM_TRANSPOSE: JXFORM_CODE = 3;

pub const JXFORM_FLIP_V: JXFORM_CODE = 2;

pub const JXFORM_FLIP_H: JXFORM_CODE = 1;

pub const JXFORM_NONE: JXFORM_CODE = 0;

pub const JCROP_FORCE: JCROP_CODE = 3;

pub const JCROP_NEG: JCROP_CODE = 2;

pub const JCROP_POS: JCROP_CODE = 1;

pub const JCROP_UNSET: JCROP_CODE = 0;

pub const JCOPYOPT_ALL_EXCEPT_ICC: JCOPY_OPTION = 3;

pub const JCOPYOPT_ALL: JCOPY_OPTION = 2;

pub const JCOPYOPT_COMMENTS: JCOPY_OPTION = 1;

pub const JCOPYOPT_NONE: JCOPY_OPTION = 0;
/* jtransform_execute_transform used to be called
 * jtransform_execute_transformation, but some compilers complain about
 * routine names that long.  This macro is here to avoid breaking any
 * old source code that uses the original name...
 */

pub const jtransform_execute_transformation: unsafe extern "C" fn(
    _: j_decompress_ptr,
    _: j_compress_ptr,
    _: *mut jvirt_barray_ptr,
    _: *mut jpeg_transform_info,
) -> () = jtransform_execute_transform;

pub const JCOPYOPT_DEFAULT: c_int = JCOPYOPT_COMMENTS as c_int;
/*
 * transupp.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1997-2011, Thomas G. Lane, Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2010, 2017, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains image transformation routines and other utility code
 * used by the jpegtran sample application.  These are NOT part of the core
 * JPEG library.  But we keep these routines separate from jpegtran.c to
 * ease the task of maintaining jpegtran-like programs that have other user
 * interfaces.
 */
/* Although this file really shouldn't have access to the library internals,
 * it's helpful to let it call jround_up() and jcopy_block_row().
 */
/* to declare isdigit() */

pub const dstinfo_min_DCT_h_scaled_size: c_int = DCTSIZE;

pub const dstinfo_min_DCT_v_scaled_size: c_int = DCTSIZE;
/*
 * Lossless image transformation routines.  These routines work on DCT
 * coefficient arrays and thus do not require any lossy decompression
 * or recompression of the image.
 * Thanks to Guido Vollbeding for the initial design and code of this feature,
 * and to Ben Jackson for introducing the cropping feature.
 *
 * Horizontal flipping is done in-place, using a single top-to-bottom
 * pass through the virtual source array.  It will thus be much the
 * fastest option for images larger than main memory.
 *
 * The other routines require a set of destination virtual arrays, so they
 * need twice as much memory as jpegtran normally does.  The destination
 * arrays are always written in normal scan order (top to bottom) because
 * the virtual array manager expects this.  The source arrays will be scanned
 * in the corresponding order, which means multiple passes through the source
 * arrays for most of the transforms.  That could result in much thrashing
 * if the image is larger than main memory.
 *
 * If cropping or trimming is involved, the destination arrays may be smaller
 * than the source arrays.  Note it is not possible to do horizontal flip
 * in-place when a nonzero Y crop offset is specified, since we'd have to move
 * data from one block row to another but the virtual array manager doesn't
 * guarantee we can touch more than one row at a time.  So in that case,
 * we have to use a separate destination array.
 *
 * Some notes about the operating environment of the individual transform
 * routines:
 * 1. Both the source and destination virtual arrays are allocated from the
 *    source JPEG object, and therefore should be manipulated by calling the
 *    source's memory manager.
 * 2. The destination's component count should be used.  It may be smaller
 *    than the source's when forcing to grayscale.
 * 3. Likewise the destination's sampling factors should be used.  When
 *    forcing to grayscale the destination's sampling factors will be all 1,
 *    and we may as well take that as the effective iMCU size.
 * 4. When "trim" is in effect, the destination's dimensions will be the
 *    trimmed values but the source's will be untrimmed.
 * 5. When "crop" is in effect, the destination's dimensions will be the
 *    cropped values but the source's will be uncropped.  Each transform
 *    routine is responsible for picking up source data starting at the
 *    correct X and Y offset for the crop region.  (The X and Y offsets
 *    passed to the transform routines are measured in iMCU blocks of the
 *    destination.)
 * 6. All the routines assume that the source and destination buffers are
 *    padded out to a full iMCU boundary.  This is true, although for the
 *    source buffer it is an undocumented property of jdcoefct.c.
 */

unsafe extern "C" fn do_crop(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dst_coef_arrays: *mut jvirt_barray_ptr,
)
/* Crop.  This is only used when no rotate/flip is requested with the crop. */
{
    let mut dst_blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut y_crop_blocks: JDIMENSION = 0;
    let mut ci: c_int = 0;
    let mut offset_y: c_int = 0;
    let mut src_buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut dst_buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    /* We simply have to copy the right amount of data (the destination's
     * image size) starting at the given X and Y offsets in the source.
     */
    ci = 0i32;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as c_uint);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as c_uint);
        dst_blk_y = 0i32 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            src_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *src_coef_arrays.offset(ci as isize),
                dst_blk_y.wrapping_add(y_crop_blocks),
                (*compptr).v_samp_factor as JDIMENSION,
                FALSE,
            );
            offset_y = 0i32;
            while offset_y < (*compptr).v_samp_factor {
                jcopy_block_row(
                    (*src_buffer.offset(offset_y as isize)).offset(x_crop_blocks as isize),
                    *dst_buffer.offset(offset_y as isize),
                    (*compptr).width_in_blocks,
                );
                offset_y += 1
            }
            dst_blk_y = (dst_blk_y as c_uint).wrapping_add((*compptr).v_samp_factor as c_uint)
                as JDIMENSION as JDIMENSION
        }
        ci += 1
    }
}

unsafe extern "C" fn do_flip_h_no_crop(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
)
/* Horizontal flip; done in-place, so no separate dest array is required.
 * NB: this only works when y_crop_offset is zero.
 */
{
    let mut MCU_cols: JDIMENSION = 0;
    let mut comp_width: JDIMENSION = 0;
    let mut blk_x: JDIMENSION = 0;
    let mut blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut ci: c_int = 0;
    let mut k: c_int = 0;
    let mut offset_y: c_int = 0;
    let mut buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut ptr1: JCOEFPTR = 0 as *mut JCOEF;
    let mut ptr2: JCOEFPTR = 0 as *mut JCOEF;
    let mut temp1: JCOEF = 0;
    let mut temp2: JCOEF = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    /* Horizontal mirroring of DCT blocks is accomplished by swapping
     * pairs of blocks in-place.  Within a DCT block, we perform horizontal
     * mirroring by changing the signs of odd-numbered columns.
     * Partial iMCUs at the right edge are left untouched.
     */
    MCU_cols = (*srcinfo)
        .output_width
        .wrapping_div(((*dstinfo).max_h_samp_factor * dstinfo_min_DCT_h_scaled_size) as c_uint);
    ci = 0i32;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        comp_width = MCU_cols.wrapping_mul((*compptr).h_samp_factor as c_uint);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as c_uint);
        blk_y = 0i32 as JDIMENSION;
        while blk_y < (*compptr).height_in_blocks {
            buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *src_coef_arrays.offset(ci as isize),
                blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            offset_y = 0i32;
            while offset_y < (*compptr).v_samp_factor {
                /* Do the mirroring */
                blk_x = 0i32 as JDIMENSION;
                while blk_x.wrapping_mul(2i32 as c_uint) < comp_width {
                    ptr1 =
                        (*(*buffer.offset(offset_y as isize)).offset(blk_x as isize)).as_mut_ptr();
                    ptr2 =
                        (*(*buffer.offset(offset_y as isize))
                            .offset(comp_width.wrapping_sub(blk_x).wrapping_sub(1i32 as c_uint)
                                as isize))
                        .as_mut_ptr();
                    /* this unrolled loop doesn't need to know which row it's on... */
                    k = 0i32; /* swap even column */
                    while k < DCTSIZE2 {
                        temp1 = *ptr1; /* swap odd column with sign change */
                        temp2 = *ptr2;
                        let fresh0 = ptr1;
                        ptr1 = ptr1.offset(1);
                        *fresh0 = temp2;
                        let fresh1 = ptr2;
                        ptr2 = ptr2.offset(1);
                        *fresh1 = temp1;
                        temp1 = *ptr1;
                        temp2 = *ptr2;
                        let fresh2 = ptr1;
                        ptr1 = ptr1.offset(1);
                        *fresh2 = -(temp2 as c_int) as JCOEF;
                        let fresh3 = ptr2;
                        ptr2 = ptr2.offset(1);
                        *fresh3 = -(temp1 as c_int) as JCOEF;
                        k += 2i32
                    }
                    blk_x = blk_x.wrapping_add(1)
                }
                if x_crop_blocks > 0i32 as c_uint {
                    /* Now left-justify the portion of the data to be kept.
                     * We can't use a single jcopy_block_row() call because that routine
                     * depends on memcpy(), whose behavior is unspecified for overlapping
                     * source and destination areas.  Sigh.
                     */
                    blk_x = 0i32 as JDIMENSION;
                    while blk_x < (*compptr).width_in_blocks {
                        jcopy_block_row(
                            (*buffer.offset(offset_y as isize))
                                .offset(blk_x as isize)
                                .offset(x_crop_blocks as isize),
                            (*buffer.offset(offset_y as isize)).offset(blk_x as isize),
                            1i32 as JDIMENSION,
                        );
                        blk_x = blk_x.wrapping_add(1)
                    }
                }
                offset_y += 1
            }
            blk_y = (blk_y as c_uint).wrapping_add((*compptr).v_samp_factor as c_uint) as JDIMENSION
                as JDIMENSION
        }
        ci += 1
    }
}

unsafe extern "C" fn do_flip_h(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dst_coef_arrays: *mut jvirt_barray_ptr,
)
/* Horizontal flip in general cropping case */
{
    let mut MCU_cols: JDIMENSION = 0;
    let mut comp_width: JDIMENSION = 0;
    let mut dst_blk_x: JDIMENSION = 0;
    let mut dst_blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut y_crop_blocks: JDIMENSION = 0;
    let mut ci: c_int = 0;
    let mut k: c_int = 0;
    let mut offset_y: c_int = 0;
    let mut src_buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut dst_buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut src_row_ptr: JBLOCKROW = 0 as *mut JBLOCK;
    let mut dst_row_ptr: JBLOCKROW = 0 as *mut JBLOCK;
    let mut src_ptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut dst_ptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    /* Here we must output into a separate array because we can't touch
     * different rows of a single virtual array simultaneously.  Otherwise,
     * this is essentially the same as the routine above.
     */
    MCU_cols = (*srcinfo)
        .output_width
        .wrapping_div(((*dstinfo).max_h_samp_factor * dstinfo_min_DCT_h_scaled_size) as c_uint);
    ci = 0i32;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        comp_width = MCU_cols.wrapping_mul((*compptr).h_samp_factor as c_uint);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as c_uint);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as c_uint);
        dst_blk_y = 0i32 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            src_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *src_coef_arrays.offset(ci as isize),
                dst_blk_y.wrapping_add(y_crop_blocks),
                (*compptr).v_samp_factor as JDIMENSION,
                FALSE,
            );
            offset_y = 0i32;
            while offset_y < (*compptr).v_samp_factor {
                dst_row_ptr = *dst_buffer.offset(offset_y as isize);
                src_row_ptr = *src_buffer.offset(offset_y as isize);
                dst_blk_x = 0i32 as JDIMENSION;
                while dst_blk_x < (*compptr).width_in_blocks {
                    if x_crop_blocks.wrapping_add(dst_blk_x) < comp_width {
                        /* Do the mirrorable blocks */
                        dst_ptr = (*dst_row_ptr.offset(dst_blk_x as isize)).as_mut_ptr();
                        src_ptr = (*src_row_ptr.offset(
                            comp_width
                                .wrapping_sub(x_crop_blocks)
                                .wrapping_sub(dst_blk_x)
                                .wrapping_sub(1i32 as c_uint) as isize,
                        ))
                        .as_mut_ptr();
                        /* this unrolled loop doesn't need to know which row it's on... */
                        k = 0i32; /* copy even column */
                        while k < DCTSIZE2 {
                            let fresh4 = src_ptr;
                            src_ptr = src_ptr.offset(1);
                            let fresh5 = dst_ptr;
                            dst_ptr = dst_ptr.offset(1);
                            *fresh5 = *fresh4;
                            let fresh6 = src_ptr;
                            src_ptr = src_ptr.offset(1);
                            let fresh7 = dst_ptr;
                            dst_ptr = dst_ptr.offset(1);
                            *fresh7 = -(*fresh6 as c_int) as JCOEF;
                            k += 2i32
                            /* copy odd column with sign change */
                        }
                    } else {
                        /* Copy last partial block(s) verbatim */
                        jcopy_block_row(
                            src_row_ptr
                                .offset(dst_blk_x as isize)
                                .offset(x_crop_blocks as isize),
                            dst_row_ptr.offset(dst_blk_x as isize),
                            1i32 as JDIMENSION,
                        );
                    }
                    dst_blk_x = dst_blk_x.wrapping_add(1)
                }
                offset_y += 1
            }
            dst_blk_y = (dst_blk_y as c_uint).wrapping_add((*compptr).v_samp_factor as c_uint)
                as JDIMENSION as JDIMENSION
        }
        ci += 1
    }
}

unsafe extern "C" fn do_flip_v(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dst_coef_arrays: *mut jvirt_barray_ptr,
)
/* Vertical flip */
{
    let mut MCU_rows: JDIMENSION = 0;
    let mut comp_height: JDIMENSION = 0;
    let mut dst_blk_x: JDIMENSION = 0;
    let mut dst_blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut y_crop_blocks: JDIMENSION = 0;
    let mut ci: c_int = 0;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut offset_y: c_int = 0;
    let mut src_buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut dst_buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut src_row_ptr: JBLOCKROW = 0 as *mut JBLOCK;
    let mut dst_row_ptr: JBLOCKROW = 0 as *mut JBLOCK;
    let mut src_ptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut dst_ptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    /* We output into a separate array because we can't touch different
     * rows of the source virtual array simultaneously.  Otherwise, this
     * is a pretty straightforward analog of horizontal flip.
     * Within a DCT block, vertical mirroring is done by changing the signs
     * of odd-numbered rows.
     * Partial iMCUs at the bottom edge are copied verbatim.
     */
    MCU_rows = (*srcinfo)
        .output_height
        .wrapping_div(((*dstinfo).max_v_samp_factor * dstinfo_min_DCT_v_scaled_size) as c_uint);
    ci = 0i32;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        comp_height = MCU_rows.wrapping_mul((*compptr).v_samp_factor as c_uint);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as c_uint);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as c_uint);
        dst_blk_y = 0i32 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            if y_crop_blocks.wrapping_add(dst_blk_y) < comp_height {
                /* Row is within the mirrorable area. */
                src_buffer = Some(
                    (*(*srcinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    srcinfo as j_common_ptr,
                    *src_coef_arrays.offset(ci as isize),
                    comp_height
                        .wrapping_sub(y_crop_blocks)
                        .wrapping_sub(dst_blk_y)
                        .wrapping_sub((*compptr).v_samp_factor as JDIMENSION),
                    (*compptr).v_samp_factor as JDIMENSION,
                    FALSE,
                )
            } else {
                /* Bottom-edge blocks will be copied verbatim. */
                src_buffer = Some(
                    (*(*srcinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    srcinfo as j_common_ptr,
                    *src_coef_arrays.offset(ci as isize),
                    dst_blk_y.wrapping_add(y_crop_blocks),
                    (*compptr).v_samp_factor as JDIMENSION,
                    FALSE,
                )
            }
            offset_y = 0i32;
            while offset_y < (*compptr).v_samp_factor {
                if y_crop_blocks.wrapping_add(dst_blk_y) < comp_height {
                    /* Row is within the mirrorable area. */
                    dst_row_ptr = *dst_buffer.offset(offset_y as isize);
                    src_row_ptr =
                        *src_buffer.offset(((*compptr).v_samp_factor - offset_y - 1i32) as isize);
                    src_row_ptr = src_row_ptr.offset(x_crop_blocks as isize);
                    dst_blk_x = 0i32 as JDIMENSION;
                    while dst_blk_x < (*compptr).width_in_blocks {
                        dst_ptr = (*dst_row_ptr.offset(dst_blk_x as isize)).as_mut_ptr();
                        src_ptr = (*src_row_ptr.offset(dst_blk_x as isize)).as_mut_ptr();
                        i = 0i32;
                        while i < DCTSIZE {
                            /* copy even row */
                            j = 0i32;
                            while j < DCTSIZE {
                                let fresh8 = src_ptr;
                                src_ptr = src_ptr.offset(1);
                                let fresh9 = dst_ptr;
                                dst_ptr = dst_ptr.offset(1);
                                *fresh9 = *fresh8;
                                j += 1
                            }
                            /* copy odd row with sign change */
                            j = 0i32;
                            while j < DCTSIZE {
                                let fresh10 = src_ptr;
                                src_ptr = src_ptr.offset(1);
                                let fresh11 = dst_ptr;
                                dst_ptr = dst_ptr.offset(1);
                                *fresh11 = -(*fresh10 as c_int) as JCOEF;
                                j += 1
                            }
                            i += 2i32
                        }
                        dst_blk_x = dst_blk_x.wrapping_add(1)
                    }
                } else {
                    /* Just copy row verbatim. */
                    jcopy_block_row(
                        (*src_buffer.offset(offset_y as isize)).offset(x_crop_blocks as isize),
                        *dst_buffer.offset(offset_y as isize),
                        (*compptr).width_in_blocks,
                    );
                }
                offset_y += 1
            }
            dst_blk_y = (dst_blk_y as c_uint).wrapping_add((*compptr).v_samp_factor as c_uint)
                as JDIMENSION as JDIMENSION
        }
        ci += 1
    }
}

unsafe extern "C" fn do_transpose(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dst_coef_arrays: *mut jvirt_barray_ptr,
)
/* Transpose source into destination */
{
    let mut dst_blk_x: JDIMENSION = 0;
    let mut dst_blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut y_crop_blocks: JDIMENSION = 0;
    let mut ci: c_int = 0;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut offset_x: c_int = 0;
    let mut offset_y: c_int = 0;
    let mut src_buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut dst_buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut src_ptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut dst_ptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    /* Transposing pixels within a block just requires transposing the
     * DCT coefficients.
     * Partial iMCUs at the edges require no special treatment; we simply
     * process all the available DCT blocks for every component.
     */
    ci = 0i32;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as c_uint);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as c_uint);
        dst_blk_y = 0i32 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            offset_y = 0i32;
            while offset_y < (*compptr).v_samp_factor {
                dst_blk_x = 0i32 as JDIMENSION;
                while dst_blk_x < (*compptr).width_in_blocks {
                    src_buffer = Some(
                        (*(*srcinfo).mem)
                            .access_virt_barray
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        srcinfo as j_common_ptr,
                        *src_coef_arrays.offset(ci as isize),
                        dst_blk_x.wrapping_add(x_crop_blocks),
                        (*compptr).h_samp_factor as JDIMENSION,
                        FALSE,
                    );
                    offset_x = 0i32;
                    while offset_x < (*compptr).h_samp_factor {
                        dst_ptr = (*(*dst_buffer.offset(offset_y as isize))
                            .offset(dst_blk_x.wrapping_add(offset_x as c_uint) as isize))
                        .as_mut_ptr();
                        src_ptr = (*(*src_buffer.offset(offset_x as isize)).offset(
                            dst_blk_y
                                .wrapping_add(offset_y as c_uint)
                                .wrapping_add(y_crop_blocks) as isize,
                        ))
                        .as_mut_ptr();
                        i = 0i32;
                        while i < DCTSIZE {
                            j = 0i32;
                            while j < DCTSIZE {
                                *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                    *src_ptr.offset((i * DCTSIZE + j) as isize);
                                j += 1
                            }
                            i += 1
                        }
                        offset_x += 1
                    }
                    dst_blk_x = (dst_blk_x as c_uint)
                        .wrapping_add((*compptr).h_samp_factor as c_uint)
                        as JDIMENSION as JDIMENSION
                }
                offset_y += 1
            }
            dst_blk_y = (dst_blk_y as c_uint).wrapping_add((*compptr).v_samp_factor as c_uint)
                as JDIMENSION as JDIMENSION
        }
        ci += 1
    }
}

unsafe extern "C" fn do_rot_90(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dst_coef_arrays: *mut jvirt_barray_ptr,
)
/* 90 degree rotation is equivalent to
 *   1. Transposing the image;
 *   2. Horizontal mirroring.
 * These two steps are merged into a single processing routine.
 */
{
    let mut MCU_cols: JDIMENSION = 0;
    let mut comp_width: JDIMENSION = 0;
    let mut dst_blk_x: JDIMENSION = 0;
    let mut dst_blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut y_crop_blocks: JDIMENSION = 0;
    let mut ci: c_int = 0;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut offset_x: c_int = 0;
    let mut offset_y: c_int = 0;
    let mut src_buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut dst_buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut src_ptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut dst_ptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    /* Because of the horizontal mirror step, we can't process partial iMCUs
     * at the (output) right edge properly.  They just get transposed and
     * not mirrored.
     */
    MCU_cols = (*srcinfo)
        .output_height
        .wrapping_div(((*dstinfo).max_h_samp_factor * dstinfo_min_DCT_h_scaled_size) as c_uint);
    ci = 0i32;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        comp_width = MCU_cols.wrapping_mul((*compptr).h_samp_factor as c_uint);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as c_uint);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as c_uint);
        dst_blk_y = 0i32 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            offset_y = 0i32;
            while offset_y < (*compptr).v_samp_factor {
                dst_blk_x = 0i32 as JDIMENSION;
                while dst_blk_x < (*compptr).width_in_blocks {
                    if x_crop_blocks.wrapping_add(dst_blk_x) < comp_width {
                        /* Block is within the mirrorable area. */
                        src_buffer = Some(
                            (*(*srcinfo).mem)
                                .access_virt_barray
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            srcinfo as j_common_ptr,
                            *src_coef_arrays.offset(ci as isize),
                            comp_width
                                .wrapping_sub(x_crop_blocks)
                                .wrapping_sub(dst_blk_x)
                                .wrapping_sub((*compptr).h_samp_factor as JDIMENSION),
                            (*compptr).h_samp_factor as JDIMENSION,
                            FALSE,
                        )
                    } else {
                        /* Edge blocks are transposed but not mirrored. */
                        src_buffer = Some(
                            (*(*srcinfo).mem)
                                .access_virt_barray
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            srcinfo as j_common_ptr,
                            *src_coef_arrays.offset(ci as isize),
                            dst_blk_x.wrapping_add(x_crop_blocks),
                            (*compptr).h_samp_factor as JDIMENSION,
                            FALSE,
                        )
                    }
                    offset_x = 0i32;
                    while offset_x < (*compptr).h_samp_factor {
                        dst_ptr = (*(*dst_buffer.offset(offset_y as isize))
                            .offset(dst_blk_x.wrapping_add(offset_x as c_uint) as isize))
                        .as_mut_ptr();
                        if x_crop_blocks.wrapping_add(dst_blk_x) < comp_width {
                            /* Block is within the mirrorable area. */
                            src_ptr = (*(*src_buffer
                                .offset(((*compptr).h_samp_factor - offset_x - 1i32) as isize))
                            .offset(
                                dst_blk_y
                                    .wrapping_add(offset_y as c_uint)
                                    .wrapping_add(y_crop_blocks)
                                    as isize,
                            ))
                            .as_mut_ptr();
                            i = 0i32;
                            while i < DCTSIZE {
                                j = 0i32;
                                while j < DCTSIZE {
                                    *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                        *src_ptr.offset((i * DCTSIZE + j) as isize);
                                    j += 1
                                }
                                i += 1;
                                j = 0i32;
                                while j < DCTSIZE {
                                    *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                        -(*src_ptr.offset((i * DCTSIZE + j) as isize) as c_int)
                                            as JCOEF;
                                    j += 1
                                }
                                i += 1
                            }
                        } else {
                            /* Edge blocks are transposed but not mirrored. */
                            src_ptr = (*(*src_buffer.offset(offset_x as isize)).offset(
                                dst_blk_y
                                    .wrapping_add(offset_y as c_uint)
                                    .wrapping_add(y_crop_blocks)
                                    as isize,
                            ))
                            .as_mut_ptr();
                            i = 0i32;
                            while i < DCTSIZE {
                                j = 0i32;
                                while j < DCTSIZE {
                                    *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                        *src_ptr.offset((i * DCTSIZE + j) as isize);
                                    j += 1
                                }
                                i += 1
                            }
                        }
                        offset_x += 1
                    }
                    dst_blk_x = (dst_blk_x as c_uint)
                        .wrapping_add((*compptr).h_samp_factor as c_uint)
                        as JDIMENSION as JDIMENSION
                }
                offset_y += 1
            }
            dst_blk_y = (dst_blk_y as c_uint).wrapping_add((*compptr).v_samp_factor as c_uint)
                as JDIMENSION as JDIMENSION
        }
        ci += 1
    }
}

unsafe extern "C" fn do_rot_270(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dst_coef_arrays: *mut jvirt_barray_ptr,
)
/* 270 degree rotation is equivalent to
 *   1. Horizontal mirroring;
 *   2. Transposing the image.
 * These two steps are merged into a single processing routine.
 */
{
    let mut MCU_rows: JDIMENSION = 0;
    let mut comp_height: JDIMENSION = 0;
    let mut dst_blk_x: JDIMENSION = 0;
    let mut dst_blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut y_crop_blocks: JDIMENSION = 0;
    let mut ci: c_int = 0;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut offset_x: c_int = 0;
    let mut offset_y: c_int = 0;
    let mut src_buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut dst_buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut src_ptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut dst_ptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    /* Because of the horizontal mirror step, we can't process partial iMCUs
     * at the (output) bottom edge properly.  They just get transposed and
     * not mirrored.
     */
    MCU_rows = (*srcinfo)
        .output_width
        .wrapping_div(((*dstinfo).max_v_samp_factor * dstinfo_min_DCT_v_scaled_size) as c_uint);
    ci = 0i32;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        comp_height = MCU_rows.wrapping_mul((*compptr).v_samp_factor as c_uint);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as c_uint);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as c_uint);
        dst_blk_y = 0i32 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            offset_y = 0i32;
            while offset_y < (*compptr).v_samp_factor {
                dst_blk_x = 0i32 as JDIMENSION;
                while dst_blk_x < (*compptr).width_in_blocks {
                    src_buffer = Some(
                        (*(*srcinfo).mem)
                            .access_virt_barray
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        srcinfo as j_common_ptr,
                        *src_coef_arrays.offset(ci as isize),
                        dst_blk_x.wrapping_add(x_crop_blocks),
                        (*compptr).h_samp_factor as JDIMENSION,
                        FALSE,
                    );
                    offset_x = 0i32;
                    while offset_x < (*compptr).h_samp_factor {
                        dst_ptr = (*(*dst_buffer.offset(offset_y as isize))
                            .offset(dst_blk_x.wrapping_add(offset_x as c_uint) as isize))
                        .as_mut_ptr();
                        if y_crop_blocks.wrapping_add(dst_blk_y) < comp_height {
                            /* Block is within the mirrorable area. */
                            src_ptr = (*(*src_buffer.offset(offset_x as isize)).offset(
                                comp_height
                                    .wrapping_sub(y_crop_blocks)
                                    .wrapping_sub(dst_blk_y)
                                    .wrapping_sub(offset_y as c_uint)
                                    .wrapping_sub(1i32 as c_uint)
                                    as isize,
                            ))
                            .as_mut_ptr();
                            i = 0i32;
                            while i < DCTSIZE {
                                j = 0i32;
                                while j < DCTSIZE {
                                    *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                        *src_ptr.offset((i * DCTSIZE + j) as isize);
                                    j += 1;
                                    *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                        -(*src_ptr.offset((i * DCTSIZE + j) as isize) as c_int)
                                            as JCOEF;
                                    j += 1
                                }
                                i += 1
                            }
                        } else {
                            /* Edge blocks are transposed but not mirrored. */
                            src_ptr = (*(*src_buffer.offset(offset_x as isize)).offset(
                                dst_blk_y
                                    .wrapping_add(offset_y as c_uint)
                                    .wrapping_add(y_crop_blocks)
                                    as isize,
                            ))
                            .as_mut_ptr();
                            i = 0i32;
                            while i < DCTSIZE {
                                j = 0i32;
                                while j < DCTSIZE {
                                    *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                        *src_ptr.offset((i * DCTSIZE + j) as isize);
                                    j += 1
                                }
                                i += 1
                            }
                        }
                        offset_x += 1
                    }
                    dst_blk_x = (dst_blk_x as c_uint)
                        .wrapping_add((*compptr).h_samp_factor as c_uint)
                        as JDIMENSION as JDIMENSION
                }
                offset_y += 1
            }
            dst_blk_y = (dst_blk_y as c_uint).wrapping_add((*compptr).v_samp_factor as c_uint)
                as JDIMENSION as JDIMENSION
        }
        ci += 1
    }
}

unsafe extern "C" fn do_rot_180(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dst_coef_arrays: *mut jvirt_barray_ptr,
)
/* 180 degree rotation is equivalent to
 *   1. Vertical mirroring;
 *   2. Horizontal mirroring.
 * These two steps are merged into a single processing routine.
 */
{
    let mut MCU_cols: JDIMENSION = 0;
    let mut MCU_rows: JDIMENSION = 0;
    let mut comp_width: JDIMENSION = 0;
    let mut comp_height: JDIMENSION = 0;
    let mut dst_blk_x: JDIMENSION = 0;
    let mut dst_blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut y_crop_blocks: JDIMENSION = 0;
    let mut ci: c_int = 0;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut offset_y: c_int = 0;
    let mut src_buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut dst_buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut src_row_ptr: JBLOCKROW = 0 as *mut JBLOCK;
    let mut dst_row_ptr: JBLOCKROW = 0 as *mut JBLOCK;
    let mut src_ptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut dst_ptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    MCU_cols = (*srcinfo)
        .output_width
        .wrapping_div(((*dstinfo).max_h_samp_factor * dstinfo_min_DCT_h_scaled_size) as c_uint);
    MCU_rows = (*srcinfo)
        .output_height
        .wrapping_div(((*dstinfo).max_v_samp_factor * dstinfo_min_DCT_v_scaled_size) as c_uint);
    ci = 0i32;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        comp_width = MCU_cols.wrapping_mul((*compptr).h_samp_factor as c_uint);
        comp_height = MCU_rows.wrapping_mul((*compptr).v_samp_factor as c_uint);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as c_uint);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as c_uint);
        dst_blk_y = 0i32 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            if y_crop_blocks.wrapping_add(dst_blk_y) < comp_height {
                /* Row is within the vertically mirrorable area. */
                src_buffer = Some(
                    (*(*srcinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    srcinfo as j_common_ptr,
                    *src_coef_arrays.offset(ci as isize),
                    comp_height
                        .wrapping_sub(y_crop_blocks)
                        .wrapping_sub(dst_blk_y)
                        .wrapping_sub((*compptr).v_samp_factor as JDIMENSION),
                    (*compptr).v_samp_factor as JDIMENSION,
                    FALSE,
                )
            } else {
                /* Bottom-edge rows are only mirrored horizontally. */
                src_buffer = Some(
                    (*(*srcinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    srcinfo as j_common_ptr,
                    *src_coef_arrays.offset(ci as isize),
                    dst_blk_y.wrapping_add(y_crop_blocks),
                    (*compptr).v_samp_factor as JDIMENSION,
                    FALSE,
                )
            }
            offset_y = 0i32;
            while offset_y < (*compptr).v_samp_factor {
                dst_row_ptr = *dst_buffer.offset(offset_y as isize);
                if y_crop_blocks.wrapping_add(dst_blk_y) < comp_height {
                    /* Row is within the mirrorable area. */
                    src_row_ptr =
                        *src_buffer.offset(((*compptr).v_samp_factor - offset_y - 1i32) as isize);
                    dst_blk_x = 0i32 as JDIMENSION;
                    while dst_blk_x < (*compptr).width_in_blocks {
                        dst_ptr = (*dst_row_ptr.offset(dst_blk_x as isize)).as_mut_ptr();
                        if x_crop_blocks.wrapping_add(dst_blk_x) < comp_width {
                            /* Process the blocks that can be mirrored both ways. */
                            src_ptr = (*src_row_ptr.offset(
                                comp_width
                                    .wrapping_sub(x_crop_blocks)
                                    .wrapping_sub(dst_blk_x)
                                    .wrapping_sub(1i32 as c_uint)
                                    as isize,
                            ))
                            .as_mut_ptr();
                            i = 0i32;
                            while i < DCTSIZE {
                                /* For even row, negate every odd column. */
                                j = 0i32;
                                while j < DCTSIZE {
                                    let fresh12 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    let fresh13 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    *fresh13 = *fresh12;
                                    let fresh14 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    let fresh15 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    *fresh15 = -(*fresh14 as c_int) as JCOEF;
                                    j += 2i32
                                }
                                /* For odd row, negate every even column. */
                                j = 0i32;
                                while j < DCTSIZE {
                                    let fresh16 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    let fresh17 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    *fresh17 = -(*fresh16 as c_int) as JCOEF;
                                    let fresh18 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    let fresh19 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    *fresh19 = *fresh18;
                                    j += 2i32
                                }
                                i += 2i32
                            }
                        } else {
                            /* Any remaining right-edge blocks are only mirrored vertically. */
                            src_ptr = (*src_row_ptr
                                .offset(x_crop_blocks.wrapping_add(dst_blk_x) as isize))
                            .as_mut_ptr();
                            i = 0i32;
                            while i < DCTSIZE {
                                j = 0i32;
                                while j < DCTSIZE {
                                    let fresh20 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    let fresh21 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    *fresh21 = *fresh20;
                                    j += 1
                                }
                                j = 0i32;
                                while j < DCTSIZE {
                                    let fresh22 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    let fresh23 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    *fresh23 = -(*fresh22 as c_int) as JCOEF;
                                    j += 1
                                }
                                i += 2i32
                            }
                        }
                        dst_blk_x = dst_blk_x.wrapping_add(1)
                    }
                } else {
                    /* Remaining rows are just mirrored horizontally. */
                    src_row_ptr = *src_buffer.offset(offset_y as isize);
                    dst_blk_x = 0i32 as JDIMENSION;
                    while dst_blk_x < (*compptr).width_in_blocks {
                        if x_crop_blocks.wrapping_add(dst_blk_x) < comp_width {
                            /* Process the blocks that can be mirrored. */
                            dst_ptr = (*dst_row_ptr.offset(dst_blk_x as isize)).as_mut_ptr();
                            src_ptr = (*src_row_ptr.offset(
                                comp_width
                                    .wrapping_sub(x_crop_blocks)
                                    .wrapping_sub(dst_blk_x)
                                    .wrapping_sub(1i32 as c_uint)
                                    as isize,
                            ))
                            .as_mut_ptr();
                            i = 0i32;
                            while i < DCTSIZE2 {
                                let fresh24 = src_ptr;
                                src_ptr = src_ptr.offset(1);
                                let fresh25 = dst_ptr;
                                dst_ptr = dst_ptr.offset(1);
                                *fresh25 = *fresh24;
                                let fresh26 = src_ptr;
                                src_ptr = src_ptr.offset(1);
                                let fresh27 = dst_ptr;
                                dst_ptr = dst_ptr.offset(1);
                                *fresh27 = -(*fresh26 as c_int) as JCOEF;
                                i += 2i32
                            }
                        } else {
                            /* Any remaining right-edge blocks are only copied. */
                            jcopy_block_row(
                                src_row_ptr
                                    .offset(dst_blk_x as isize)
                                    .offset(x_crop_blocks as isize),
                                dst_row_ptr.offset(dst_blk_x as isize),
                                1i32 as JDIMENSION,
                            );
                        }
                        dst_blk_x = dst_blk_x.wrapping_add(1)
                    }
                }
                offset_y += 1
            }
            dst_blk_y = (dst_blk_y as c_uint).wrapping_add((*compptr).v_samp_factor as c_uint)
                as JDIMENSION as JDIMENSION
        }
        ci += 1
    }
}

unsafe extern "C" fn do_transverse(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dst_coef_arrays: *mut jvirt_barray_ptr,
)
/* Transverse transpose is equivalent to
 *   1. 180 degree rotation;
 *   2. Transposition;
 * or
 *   1. Horizontal mirroring;
 *   2. Transposition;
 *   3. Horizontal mirroring.
 * These steps are merged into a single processing routine.
 */
{
    let mut MCU_cols: JDIMENSION = 0;
    let mut MCU_rows: JDIMENSION = 0;
    let mut comp_width: JDIMENSION = 0;
    let mut comp_height: JDIMENSION = 0;
    let mut dst_blk_x: JDIMENSION = 0;
    let mut dst_blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut y_crop_blocks: JDIMENSION = 0;
    let mut ci: c_int = 0;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut offset_x: c_int = 0;
    let mut offset_y: c_int = 0;
    let mut src_buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut dst_buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut src_ptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut dst_ptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    MCU_cols = (*srcinfo)
        .output_height
        .wrapping_div(((*dstinfo).max_h_samp_factor * dstinfo_min_DCT_h_scaled_size) as c_uint);
    MCU_rows = (*srcinfo)
        .output_width
        .wrapping_div(((*dstinfo).max_v_samp_factor * dstinfo_min_DCT_v_scaled_size) as c_uint);
    ci = 0i32;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        comp_width = MCU_cols.wrapping_mul((*compptr).h_samp_factor as c_uint);
        comp_height = MCU_rows.wrapping_mul((*compptr).v_samp_factor as c_uint);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as c_uint);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as c_uint);
        dst_blk_y = 0i32 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            offset_y = 0i32;
            while offset_y < (*compptr).v_samp_factor {
                dst_blk_x = 0i32 as JDIMENSION;
                while dst_blk_x < (*compptr).width_in_blocks {
                    if x_crop_blocks.wrapping_add(dst_blk_x) < comp_width {
                        /* Block is within the mirrorable area. */
                        src_buffer = Some(
                            (*(*srcinfo).mem)
                                .access_virt_barray
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            srcinfo as j_common_ptr,
                            *src_coef_arrays.offset(ci as isize),
                            comp_width
                                .wrapping_sub(x_crop_blocks)
                                .wrapping_sub(dst_blk_x)
                                .wrapping_sub((*compptr).h_samp_factor as JDIMENSION),
                            (*compptr).h_samp_factor as JDIMENSION,
                            FALSE,
                        )
                    } else {
                        src_buffer = Some(
                            (*(*srcinfo).mem)
                                .access_virt_barray
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            srcinfo as j_common_ptr,
                            *src_coef_arrays.offset(ci as isize),
                            dst_blk_x.wrapping_add(x_crop_blocks),
                            (*compptr).h_samp_factor as JDIMENSION,
                            FALSE,
                        )
                    }
                    offset_x = 0i32;
                    while offset_x < (*compptr).h_samp_factor {
                        dst_ptr = (*(*dst_buffer.offset(offset_y as isize))
                            .offset(dst_blk_x.wrapping_add(offset_x as c_uint) as isize))
                        .as_mut_ptr();
                        if y_crop_blocks.wrapping_add(dst_blk_y) < comp_height {
                            if x_crop_blocks.wrapping_add(dst_blk_x) < comp_width {
                                /* Block is within the mirrorable area. */
                                src_ptr = (*(*src_buffer.offset(
                                    ((*compptr).h_samp_factor - offset_x - 1i32) as isize,
                                ))
                                .offset(
                                    comp_height
                                        .wrapping_sub(y_crop_blocks)
                                        .wrapping_sub(dst_blk_y)
                                        .wrapping_sub(offset_y as c_uint)
                                        .wrapping_sub(1i32 as c_uint)
                                        as isize,
                                ))
                                .as_mut_ptr();
                                i = 0i32;
                                while i < DCTSIZE {
                                    j = 0i32;
                                    while j < DCTSIZE {
                                        *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                            *src_ptr.offset((i * DCTSIZE + j) as isize);
                                        j += 1;
                                        *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                            -(*src_ptr.offset((i * DCTSIZE + j) as isize) as c_int)
                                                as JCOEF;
                                        j += 1
                                    }
                                    i += 1;
                                    j = 0i32;
                                    while j < DCTSIZE {
                                        *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                            -(*src_ptr.offset((i * DCTSIZE + j) as isize) as c_int)
                                                as JCOEF;
                                        j += 1;
                                        *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                            *src_ptr.offset((i * DCTSIZE + j) as isize);
                                        j += 1
                                    }
                                    i += 1
                                }
                            } else {
                                /* Right-edge blocks are mirrored in y only */
                                src_ptr = (*(*src_buffer.offset(offset_x as isize)).offset(
                                    comp_height
                                        .wrapping_sub(y_crop_blocks)
                                        .wrapping_sub(dst_blk_y)
                                        .wrapping_sub(offset_y as c_uint)
                                        .wrapping_sub(1i32 as c_uint)
                                        as isize,
                                ))
                                .as_mut_ptr();
                                i = 0i32;
                                while i < DCTSIZE {
                                    j = 0i32;
                                    while j < DCTSIZE {
                                        *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                            *src_ptr.offset((i * DCTSIZE + j) as isize);
                                        j += 1;
                                        *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                            -(*src_ptr.offset((i * DCTSIZE + j) as isize) as c_int)
                                                as JCOEF;
                                        j += 1
                                    }
                                    i += 1
                                }
                            }
                        } else if x_crop_blocks.wrapping_add(dst_blk_x) < comp_width {
                            /* Bottom-edge blocks are mirrored in x only */
                            src_ptr = (*(*src_buffer
                                .offset(((*compptr).h_samp_factor - offset_x - 1i32) as isize))
                            .offset(
                                dst_blk_y
                                    .wrapping_add(offset_y as c_uint)
                                    .wrapping_add(y_crop_blocks)
                                    as isize,
                            ))
                            .as_mut_ptr();
                            i = 0i32;
                            while i < DCTSIZE {
                                j = 0i32;
                                while j < DCTSIZE {
                                    *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                        *src_ptr.offset((i * DCTSIZE + j) as isize);
                                    j += 1
                                }
                                i += 1;
                                j = 0i32;
                                while j < DCTSIZE {
                                    *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                        -(*src_ptr.offset((i * DCTSIZE + j) as isize) as c_int)
                                            as JCOEF;
                                    j += 1
                                }
                                i += 1
                            }
                        } else {
                            /* At lower right corner, just transpose, no mirroring */
                            src_ptr = (*(*src_buffer.offset(offset_x as isize)).offset(
                                dst_blk_y
                                    .wrapping_add(offset_y as c_uint)
                                    .wrapping_add(y_crop_blocks)
                                    as isize,
                            ))
                            .as_mut_ptr();
                            i = 0i32;
                            while i < DCTSIZE {
                                j = 0i32;
                                while j < DCTSIZE {
                                    *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                        *src_ptr.offset((i * DCTSIZE + j) as isize);
                                    j += 1
                                }
                                i += 1
                            }
                        }
                        offset_x += 1
                    }
                    dst_blk_x = (dst_blk_x as c_uint)
                        .wrapping_add((*compptr).h_samp_factor as c_uint)
                        as JDIMENSION as JDIMENSION
                }
                offset_y += 1
            }
            dst_blk_y = (dst_blk_y as c_uint).wrapping_add((*compptr).v_samp_factor as c_uint)
                as JDIMENSION as JDIMENSION
        }
        ci += 1
    }
}
/* Parse an unsigned integer: subroutine for jtransform_parse_crop_spec.
 * Returns TRUE if valid integer found, FALSE if not.
 * *strptr is advanced over the digit string, and *result is set to its value.
 */

unsafe extern "C" fn jt_read_integer(
    mut strptr: *mut *const c_char,
    mut result: *mut JDIMENSION,
) -> boolean {
    let mut ptr: *const c_char = *strptr; /* oops, no digits */
    let mut val: JDIMENSION = 0i32 as JDIMENSION;
    while *(*__ctype_b_loc()).offset(*ptr as c_int as isize) as c_int
        & _ISdigit as c_int as c_ushort as c_int
        != 0
    {
        val = val
            .wrapping_mul(10i32 as c_uint)
            .wrapping_add((*ptr as c_int - '0' as i32) as JDIMENSION);
        ptr = ptr.offset(1)
    }
    *result = val;
    if ptr == *strptr {
        return FALSE;
    }
    *strptr = ptr;
    return TRUE;
}
/* Parse a crop specification (written in X11 geometry style) */
/* Parse a crop specification (written in X11 geometry style).
 * The routine returns TRUE if the spec string is valid, FALSE if not.
 *
 * The crop spec string should have the format
 *      <width>[f]x<height>[f]{+-}<xoffset>{+-}<yoffset>
 * where width, height, xoffset, and yoffset are unsigned integers.
 * Each of the elements can be omitted to indicate a default value.
 * (A weakness of this style is that it is not possible to omit xoffset
 * while specifying yoffset, since they look alike.)
 *
 * This code is loosely based on XParseGeometry from the X11 distribution.
 */
#[no_mangle]

pub unsafe extern "C" fn jtransform_parse_crop_spec(
    mut info: *mut jpeg_transform_info,
    mut spec: *const c_char,
) -> boolean {
    (*info).crop = FALSE;
    (*info).crop_width_set = JCROP_UNSET;
    (*info).crop_height_set = JCROP_UNSET;
    (*info).crop_xoffset_set = JCROP_UNSET;
    (*info).crop_yoffset_set = JCROP_UNSET;
    if *(*__ctype_b_loc()).offset(*spec as c_int as isize) as c_int
        & _ISdigit as c_int as c_ushort as c_int
        != 0
    {
        /* fetch width */
        if jt_read_integer(&mut spec, &mut (*info).crop_width) == 0 {
            return FALSE;
        }
        if *spec as c_int == 'f' as i32 || *spec as c_int == 'F' as i32 {
            spec = spec.offset(1);
            (*info).crop_width_set = JCROP_FORCE
        } else {
            (*info).crop_width_set = JCROP_POS
        }
    }
    if *spec as c_int == 'x' as i32 || *spec as c_int == 'X' as i32 {
        /* fetch height */
        spec = spec.offset(1);
        if jt_read_integer(&mut spec, &mut (*info).crop_height) == 0 {
            return FALSE;
        }
        if *spec as c_int == 'f' as i32 || *spec as c_int == 'F' as i32 {
            spec = spec.offset(1);
            (*info).crop_height_set = JCROP_FORCE
        } else {
            (*info).crop_height_set = JCROP_POS
        }
    }
    if *spec as c_int == '+' as i32 || *spec as c_int == '-' as i32 {
        /* fetch xoffset */
        (*info).crop_xoffset_set = if *spec as c_int == '-' as i32 {
            JCROP_NEG as c_int
        } else {
            JCROP_POS as c_int
        } as JCROP_CODE;
        spec = spec.offset(1);
        if jt_read_integer(&mut spec, &mut (*info).crop_xoffset) == 0 {
            return FALSE;
        }
    }
    if *spec as c_int == '+' as i32 || *spec as c_int == '-' as i32 {
        /* fetch yoffset */
        (*info).crop_yoffset_set = if *spec as c_int == '-' as i32 {
            JCROP_NEG as c_int
        } else {
            JCROP_POS as c_int
        } as JCROP_CODE;
        spec = spec.offset(1);
        if jt_read_integer(&mut spec, &mut (*info).crop_yoffset) == 0 {
            return FALSE;
        }
    }
    /* We had better have gotten to the end of the string. */
    if *spec as c_int != '\u{0}' as i32 {
        return FALSE;
    }
    (*info).crop = TRUE;
    return TRUE;
}
/* Trim off any partial iMCUs on the indicated destination edge */

unsafe extern "C" fn trim_right_edge(
    mut info: *mut jpeg_transform_info,
    mut full_width: JDIMENSION,
) {
    let mut MCU_cols: JDIMENSION = 0;
    MCU_cols = (*info)
        .output_width
        .wrapping_div((*info).iMCU_sample_width as c_uint);
    if MCU_cols > 0i32 as c_uint
        && (*info).x_crop_offset.wrapping_add(MCU_cols)
            == full_width.wrapping_div((*info).iMCU_sample_width as c_uint)
    {
        (*info).output_width = MCU_cols.wrapping_mul((*info).iMCU_sample_width as c_uint)
    };
}

unsafe extern "C" fn trim_bottom_edge(
    mut info: *mut jpeg_transform_info,
    mut full_height: JDIMENSION,
) {
    let mut MCU_rows: JDIMENSION = 0;
    MCU_rows = (*info)
        .output_height
        .wrapping_div((*info).iMCU_sample_height as c_uint);
    if MCU_rows > 0i32 as c_uint
        && (*info).y_crop_offset.wrapping_add(MCU_rows)
            == full_height.wrapping_div((*info).iMCU_sample_height as c_uint)
    {
        (*info).output_height = MCU_rows.wrapping_mul((*info).iMCU_sample_height as c_uint)
    };
}
/* Request any required workspace */
/* Request any required workspace.
 *
 * This routine figures out the size that the output image will be
 * (which implies that all the transform parameters must be set before
 * it is called).
 *
 * We allocate the workspace virtual arrays from the source decompression
 * object, so that all the arrays (both the original data and the workspace)
 * will be taken into account while making memory management decisions.
 * Hence, this routine must be called after jpeg_read_header (which reads
 * the image dimensions) and before jpeg_read_coefficients (which realizes
 * the source's virtual arrays).
 *
 * This function returns FALSE right away if -perfect is given
 * and transformation is not perfect.  Otherwise returns TRUE.
 */
#[no_mangle]

pub unsafe extern "C" fn jtransform_request_workspace(
    mut srcinfo: j_decompress_ptr,
    mut info: *mut jpeg_transform_info,
) -> boolean {
    let mut coef_arrays: *mut jvirt_barray_ptr = 0 as *mut jvirt_barray_ptr;
    let mut need_workspace: boolean = 0;
    let mut transpose_it: boolean = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut xoffset: JDIMENSION = 0;
    let mut yoffset: JDIMENSION = 0;
    let mut width_in_iMCUs: JDIMENSION = 0;
    let mut height_in_iMCUs: JDIMENSION = 0;
    let mut width_in_blocks: JDIMENSION = 0;
    let mut height_in_blocks: JDIMENSION = 0;
    let mut ci: c_int = 0;
    let mut h_samp_factor: c_int = 0;
    let mut v_samp_factor: c_int = 0;
    /* Determine number of components in output image */
    if (*info).force_grayscale != 0
        && (*srcinfo).jpeg_color_space as c_uint == JCS_YCbCr as c_int as c_uint
        && (*srcinfo).num_components == 3i32
    {
        /* We'll only process the first component */
        (*info).num_components = 1i32
    } else {
        /* Process all the components */
        (*info).num_components = (*srcinfo).num_components
    }
    /* Compute output image dimensions and related values. */
    (*srcinfo).output_width = (*srcinfo).image_width;
    (*srcinfo).output_height = (*srcinfo).image_height;
    /* Return right away if -perfect is given and transformation is not perfect.
     */
    if (*info).perfect != 0 {
        if (*info).num_components == 1i32 {
            if jtransform_perfect_transform(
                (*srcinfo).output_width,
                (*srcinfo).output_height,
                (*srcinfo).min_DCT_scaled_size,
                (*srcinfo).min_DCT_scaled_size,
                (*info).transform,
            ) == 0
            {
                return FALSE;
            }
        } else if jtransform_perfect_transform(
            (*srcinfo).output_width,
            (*srcinfo).output_height,
            (*srcinfo).max_h_samp_factor * (*srcinfo).min_DCT_scaled_size,
            (*srcinfo).max_v_samp_factor * (*srcinfo).min_DCT_scaled_size,
            (*info).transform,
        ) == 0
        {
            return FALSE;
        }
    }
    /* If there is only one output component, force the iMCU size to be 1;
     * else use the source iMCU size.  (This allows us to do the right thing
     * when reducing color to grayscale, and also provides a handy way of
     * cleaning up "funny" grayscale images whose sampling factors are not 1x1.)
     */
    match (*info).transform as c_uint {
        3 | 4 | 5 | 7 => {
            (*info).output_width = (*srcinfo).output_height;
            (*info).output_height = (*srcinfo).output_width;
            if (*info).num_components == 1i32 {
                (*info).iMCU_sample_width = (*srcinfo).min_DCT_scaled_size;
                (*info).iMCU_sample_height = (*srcinfo).min_DCT_scaled_size
            } else {
                (*info).iMCU_sample_width =
                    (*srcinfo).max_v_samp_factor * (*srcinfo).min_DCT_scaled_size;
                (*info).iMCU_sample_height =
                    (*srcinfo).max_h_samp_factor * (*srcinfo).min_DCT_scaled_size
            }
        }
        _ => {
            (*info).output_width = (*srcinfo).output_width;
            (*info).output_height = (*srcinfo).output_height;
            if (*info).num_components == 1i32 {
                (*info).iMCU_sample_width = (*srcinfo).min_DCT_scaled_size;
                (*info).iMCU_sample_height = (*srcinfo).min_DCT_scaled_size
            } else {
                (*info).iMCU_sample_width =
                    (*srcinfo).max_h_samp_factor * (*srcinfo).min_DCT_scaled_size;
                (*info).iMCU_sample_height =
                    (*srcinfo).max_v_samp_factor * (*srcinfo).min_DCT_scaled_size
            }
        }
    }
    /* If cropping has been requested, compute the crop area's position and
     * dimensions, ensuring that its upper left corner falls at an iMCU boundary.
     */
    if (*info).crop != 0 {
        /* Insert default values for unset crop parameters */
        if (*info).crop_xoffset_set as c_uint == JCROP_UNSET as c_int as c_uint {
            (*info).crop_xoffset = 0i32 as JDIMENSION
        } /* default to +0 */
        if (*info).crop_yoffset_set as c_uint == JCROP_UNSET as c_int as c_uint {
            (*info).crop_yoffset = 0i32 as JDIMENSION
        } /* default to +0 */
        if (*info).crop_xoffset >= (*info).output_width
            || (*info).crop_yoffset >= (*info).output_height
        {
            (*(*srcinfo).err).msg_code = super::jerror::JERR_BAD_CROP_SPEC as c_int;
            Some(
                (*(*srcinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(srcinfo as j_common_ptr);
        }
        if (*info).crop_width_set as c_uint == JCROP_UNSET as c_int as c_uint {
            (*info).crop_width = (*info).output_width.wrapping_sub((*info).crop_xoffset)
        }
        if (*info).crop_height_set as c_uint == JCROP_UNSET as c_int as c_uint {
            (*info).crop_height = (*info).output_height.wrapping_sub((*info).crop_yoffset)
        }
        /* Ensure parameters are valid */
        if (*info).crop_width <= 0i32 as c_uint
            || (*info).crop_width > (*info).output_width
            || (*info).crop_height <= 0i32 as c_uint
            || (*info).crop_height > (*info).output_height
            || (*info).crop_xoffset > (*info).output_width.wrapping_sub((*info).crop_width)
            || (*info).crop_yoffset > (*info).output_height.wrapping_sub((*info).crop_height)
        {
            (*(*srcinfo).err).msg_code = super::jerror::JERR_BAD_CROP_SPEC as c_int;
            Some(
                (*(*srcinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(srcinfo as j_common_ptr);
        }
        /* Convert negative crop offsets into regular offsets */
        if (*info).crop_xoffset_set as c_uint == JCROP_NEG as c_int as c_uint {
            xoffset = (*info)
                .output_width
                .wrapping_sub((*info).crop_width)
                .wrapping_sub((*info).crop_xoffset)
        } else {
            xoffset = (*info).crop_xoffset
        }
        if (*info).crop_yoffset_set as c_uint == JCROP_NEG as c_int as c_uint {
            yoffset = (*info)
                .output_height
                .wrapping_sub((*info).crop_height)
                .wrapping_sub((*info).crop_yoffset)
        } else {
            yoffset = (*info).crop_yoffset
        }
        /* Now adjust so that upper left corner falls at an iMCU boundary */
        if (*info).crop_width_set as c_uint == JCROP_FORCE as c_int as c_uint {
            (*info).output_width = (*info).crop_width
        } else {
            (*info).output_width = (*info)
                .crop_width
                .wrapping_add(xoffset.wrapping_rem((*info).iMCU_sample_width as c_uint))
        }
        if (*info).crop_height_set as c_uint == JCROP_FORCE as c_int as c_uint {
            (*info).output_height = (*info).crop_height
        } else {
            (*info).output_height = (*info)
                .crop_height
                .wrapping_add(yoffset.wrapping_rem((*info).iMCU_sample_height as c_uint))
        }
        /* Save x/y offsets measured in iMCUs */
        (*info).x_crop_offset = xoffset.wrapping_div((*info).iMCU_sample_width as c_uint);
        (*info).y_crop_offset = yoffset.wrapping_div((*info).iMCU_sample_height as c_uint)
    } else {
        (*info).x_crop_offset = 0i32 as JDIMENSION;
        (*info).y_crop_offset = 0i32 as JDIMENSION
    }
    /* Figure out whether we need workspace arrays,
     * and if so whether they are transposed relative to the source.
     */
    need_workspace = FALSE;
    transpose_it = FALSE;
    match (*info).transform as c_uint {
        0 => {
            if (*info).x_crop_offset != 0i32 as c_uint || (*info).y_crop_offset != 0i32 as c_uint {
                need_workspace = TRUE
            }
        }
        1 => {
            if (*info).trim != 0 {
                trim_right_edge(info, (*srcinfo).output_width);
            }
            if (*info).y_crop_offset != 0i32 as c_uint || (*info).slow_hflip != 0 {
                need_workspace = TRUE
            }
        }
        2 => {
            if (*info).trim != 0 {
                trim_bottom_edge(info, (*srcinfo).output_height);
            }
            /* Need workspace arrays having same dimensions as source image. */
            need_workspace = TRUE
        }
        3 => {
            /* transpose does NOT have to trim anything */
            /* Need workspace arrays having transposed dimensions. */
            need_workspace = TRUE;
            transpose_it = TRUE
        }
        4 => {
            if (*info).trim != 0 {
                trim_right_edge(info, (*srcinfo).output_height);
                trim_bottom_edge(info, (*srcinfo).output_width);
            }
            /* Need workspace arrays having transposed dimensions. */
            need_workspace = TRUE;
            transpose_it = TRUE
        }
        5 => {
            if (*info).trim != 0 {
                trim_right_edge(info, (*srcinfo).output_height);
            }
            /* Need workspace arrays having transposed dimensions. */
            need_workspace = TRUE;
            transpose_it = TRUE
        }
        6 => {
            if (*info).trim != 0 {
                trim_right_edge(info, (*srcinfo).output_width);
                trim_bottom_edge(info, (*srcinfo).output_height);
            }
            /* Need workspace arrays having same dimensions as source image. */
            need_workspace = TRUE
        }
        7 => {
            if (*info).trim != 0 {
                trim_bottom_edge(info, (*srcinfo).output_width);
            }
            /* Need workspace arrays having transposed dimensions. */
            need_workspace = TRUE;
            transpose_it = TRUE
        }
        _ => {}
    }
    /* Allocate workspace if needed.
     * Note that we allocate arrays padded out to the next iMCU boundary,
     * so that transform routines need not worry about missing edge blocks.
     */
    if need_workspace != 0 {
        coef_arrays = Some(
            (*(*srcinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            srcinfo as j_common_ptr,
            JPOOL_IMAGE,
            (::std::mem::size_of::<jvirt_barray_ptr>() as c_ulong)
                .wrapping_mul((*info).num_components as c_ulong),
        ) as *mut jvirt_barray_ptr;
        width_in_iMCUs = jdiv_round_up(
            (*info).output_width as c_long,
            (*info).iMCU_sample_width as c_long,
        ) as JDIMENSION;
        height_in_iMCUs = jdiv_round_up(
            (*info).output_height as c_long,
            (*info).iMCU_sample_height as c_long,
        ) as JDIMENSION;
        ci = 0i32;
        while ci < (*info).num_components {
            compptr = (*srcinfo).comp_info.offset(ci as isize);
            if (*info).num_components == 1i32 {
                /* we're going to force samp factors to 1x1 in this case */
                v_samp_factor = 1i32;
                h_samp_factor = v_samp_factor
            } else if transpose_it != 0 {
                h_samp_factor = (*compptr).v_samp_factor;
                v_samp_factor = (*compptr).h_samp_factor
            } else {
                h_samp_factor = (*compptr).h_samp_factor;
                v_samp_factor = (*compptr).v_samp_factor
            }
            width_in_blocks = width_in_iMCUs.wrapping_mul(h_samp_factor as c_uint);
            height_in_blocks = height_in_iMCUs.wrapping_mul(v_samp_factor as c_uint);
            let ref mut fresh28 = *coef_arrays.offset(ci as isize);
            *fresh28 = Some(
                (*(*srcinfo).mem)
                    .request_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                JPOOL_IMAGE,
                FALSE,
                width_in_blocks,
                height_in_blocks,
                v_samp_factor as JDIMENSION,
            );
            ci += 1
        }
        (*info).workspace_coef_arrays = coef_arrays
    } else {
        (*info).workspace_coef_arrays = NULL as *mut jvirt_barray_ptr
    }
    return TRUE;
}
/* Transpose destination image parameters */

unsafe extern "C" fn transpose_critical_parameters(mut dstinfo: j_compress_ptr) {
    let mut tblno: c_int = 0;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut ci: c_int = 0;
    let mut itemp: c_int = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut qtblptr: *mut JQUANT_TBL = 0 as *mut JQUANT_TBL;
    let mut jtemp: JDIMENSION = 0;
    let mut qtemp: UINT16 = 0;
    /* Transpose image dimensions */
    jtemp = (*dstinfo).image_width;
    (*dstinfo).image_width = (*dstinfo).image_height;
    (*dstinfo).image_height = jtemp;
    /* Transpose sampling factors */
    ci = 0i32;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        itemp = (*compptr).h_samp_factor;
        (*compptr).h_samp_factor = (*compptr).v_samp_factor;
        (*compptr).v_samp_factor = itemp;
        ci += 1
    }
    /* Transpose quantization tables */
    tblno = 0i32;
    while tblno < NUM_QUANT_TBLS {
        qtblptr = (*dstinfo).quant_tbl_ptrs[tblno as usize];
        if !qtblptr.is_null() {
            i = 0i32;
            while i < DCTSIZE {
                j = 0i32;
                while j < i {
                    qtemp = (*qtblptr).quantval[(i * DCTSIZE + j) as usize];
                    (*qtblptr).quantval[(i * DCTSIZE + j) as usize] =
                        (*qtblptr).quantval[(j * DCTSIZE + i) as usize];
                    (*qtblptr).quantval[(j * DCTSIZE + i) as usize] = qtemp;
                    j += 1
                }
                i += 1
            }
        }
        tblno += 1
    }
}
/* Adjust Exif image parameters.
 *
 * We try to adjust the Tags ExifImageWidth and ExifImageHeight if possible.
 */

unsafe extern "C" fn adjust_exif_parameters(
    mut data: *mut JOCTET,
    mut length: c_uint,
    mut new_width: JDIMENSION,
    mut new_height: JDIMENSION,
) {
    let mut is_motorola: boolean = 0; /* Flag for byte order */
    let mut number_of_tags: c_uint = 0; /* Length of an IFD entry */
    let mut tagnum: c_uint = 0;
    let mut firstoffset: c_uint = 0;
    let mut offset: c_uint = 0;
    let mut new_value: JDIMENSION = 0;
    if length < 12i32 as c_uint {
        return;
    }
    /* Discover byte order */
    if *data.offset(0) as c_int == 0x49i32 && *data.offset(1) as c_int == 0x49i32 {
        is_motorola = FALSE
    } else if *data.offset(0) as c_int == 0x4di32 && *data.offset(1) as c_int == 0x4di32 {
        is_motorola = TRUE
    } else {
        return;
    }
    /* Check Tag Mark */
    if is_motorola != 0 {
        if *data.offset(2) as c_int != 0i32 {
            return;
        }
        if *data.offset(3) as c_int != 0x2ai32 {
            return;
        }
    } else {
        if *data.offset(3) as c_int != 0i32 {
            return;
        }
        if *data.offset(2) as c_int != 0x2ai32 {
            return;
        }
    }
    /* Get first IFD offset (offset to IFD0) */
    if is_motorola != 0 {
        if *data.offset(4) as c_int != 0i32 {
            return;
        } /* check end of data segment */
        if *data.offset(5) as c_int != 0i32 {
            return;
        }
        firstoffset = *data.offset(6) as c_uint;
        firstoffset <<= 8i32;
        firstoffset = firstoffset.wrapping_add(*data.offset(7) as c_uint)
    } else {
        if *data.offset(7) as c_int != 0i32 {
            return;
        }
        if *data.offset(6) as c_int != 0i32 {
            return;
        }
        firstoffset = *data.offset(5) as c_uint;
        firstoffset <<= 8i32;
        firstoffset = firstoffset.wrapping_add(*data.offset(4) as c_uint)
    }
    if firstoffset > length.wrapping_sub(2i32 as c_uint) {
        return;
    }
    /* Get the number of directory entries contained in this IFD */
    if is_motorola != 0 {
        number_of_tags = *data.offset(firstoffset as isize) as c_uint;
        number_of_tags <<= 8i32;
        number_of_tags = number_of_tags
            .wrapping_add(*data.offset(firstoffset.wrapping_add(1i32 as c_uint) as isize) as c_uint)
    } else {
        number_of_tags = *data.offset(firstoffset.wrapping_add(1i32 as c_uint) as isize) as c_uint;
        number_of_tags <<= 8i32;
        number_of_tags = number_of_tags.wrapping_add(*data.offset(firstoffset as isize) as c_uint)
    }
    if number_of_tags == 0i32 as c_uint {
        return;
    }
    firstoffset = firstoffset.wrapping_add(2i32 as c_uint);
    loop
    /* Search for ExifSubIFD offset Tag in IFD0 */
    {
        if firstoffset > length.wrapping_sub(12i32 as c_uint) {
            return;
        } /* check end of data segment */
        /* Get Tag number */
        if is_motorola != 0 {
            tagnum = *data.offset(firstoffset as isize) as c_uint; /* found ExifSubIFD offset Tag */
            tagnum <<= 8i32;
            tagnum = tagnum.wrapping_add(
                *data.offset(firstoffset.wrapping_add(1i32 as c_uint) as isize) as c_uint,
            )
        } else {
            tagnum = *data.offset(firstoffset.wrapping_add(1i32 as c_uint) as isize) as c_uint;
            tagnum <<= 8i32;
            tagnum = tagnum.wrapping_add(*data.offset(firstoffset as isize) as c_uint)
        }
        if tagnum == 0x8769i32 as c_uint {
            break;
        }
        number_of_tags = number_of_tags.wrapping_sub(1);
        if number_of_tags == 0i32 as c_uint {
            return;
        }
        firstoffset = firstoffset.wrapping_add(12i32 as c_uint)
    }
    /* Get the ExifSubIFD offset */
    if is_motorola != 0 {
        if *data.offset(firstoffset.wrapping_add(8i32 as c_uint) as isize) as c_int != 0i32 {
            return;
        } /* check end of data segment */
        if *data.offset(firstoffset.wrapping_add(9i32 as c_uint) as isize) as c_int != 0i32 {
            return;
        }
        offset = *data.offset(firstoffset.wrapping_add(10i32 as c_uint) as isize) as c_uint;
        offset <<= 8i32;
        offset = offset.wrapping_add(
            *data.offset(firstoffset.wrapping_add(11i32 as c_uint) as isize) as c_uint,
        )
    } else {
        if *data.offset(firstoffset.wrapping_add(11i32 as c_uint) as isize) as c_int != 0i32 {
            return;
        }
        if *data.offset(firstoffset.wrapping_add(10i32 as c_uint) as isize) as c_int != 0i32 {
            return;
        }
        offset = *data.offset(firstoffset.wrapping_add(9i32 as c_uint) as isize) as c_uint;
        offset <<= 8i32;
        offset = offset
            .wrapping_add(*data.offset(firstoffset.wrapping_add(8i32 as c_uint) as isize) as c_uint)
    }
    if offset > length.wrapping_sub(2i32 as c_uint) {
        return;
    }
    /* Get the number of directory entries contained in this SubIFD */
    if is_motorola != 0 {
        number_of_tags = *data.offset(offset as isize) as c_uint;
        number_of_tags <<= 8i32;
        number_of_tags = number_of_tags
            .wrapping_add(*data.offset(offset.wrapping_add(1i32 as c_uint) as isize) as c_uint)
    } else {
        number_of_tags = *data.offset(offset.wrapping_add(1i32 as c_uint) as isize) as c_uint;
        number_of_tags <<= 8i32;
        number_of_tags = number_of_tags.wrapping_add(*data.offset(offset as isize) as c_uint)
    }
    if number_of_tags < 2i32 as c_uint {
        return;
    }
    offset = offset.wrapping_add(2i32 as c_uint);
    loop
    /* Search for ExifImageWidth and ExifImageHeight Tags in this SubIFD */
    {
        if offset > length.wrapping_sub(12i32 as c_uint) {
            return;
        } /* check end of data segment */
        /* Get Tag number */
        if is_motorola != 0 {
            tagnum = *data.offset(offset as isize) as c_uint; /* ExifImageHeight Tag */
            tagnum <<= 8i32; /* ExifImageWidth Tag */
            tagnum = tagnum
                .wrapping_add(*data.offset(offset.wrapping_add(1i32 as c_uint) as isize) as c_uint)
        } else {
            tagnum = *data.offset(offset.wrapping_add(1i32 as c_uint) as isize) as c_uint; /* Format = unsigned long (4 octets) */
            tagnum <<= 8i32; /* Number Of Components = 1 */
            tagnum = tagnum.wrapping_add(*data.offset(offset as isize) as c_uint)
        } /* Format = unsigned long (4 octets) */
        if tagnum == 0xa002i32 as c_uint || tagnum == 0xa003i32 as c_uint {
            if tagnum == 0xa002i32 as c_uint {
                new_value = new_width
            } else {
                new_value = new_height
            } /* Number Of Components = 1 */
            if is_motorola != 0 {
                *data.offset(offset.wrapping_add(2i32 as c_uint) as isize) = 0i32 as JOCTET;
                *data.offset(offset.wrapping_add(3i32 as c_uint) as isize) = 4i32 as JOCTET;
                *data.offset(offset.wrapping_add(4i32 as c_uint) as isize) = 0i32 as JOCTET;
                *data.offset(offset.wrapping_add(5i32 as c_uint) as isize) = 0i32 as JOCTET;
                *data.offset(offset.wrapping_add(6i32 as c_uint) as isize) = 0i32 as JOCTET;
                *data.offset(offset.wrapping_add(7i32 as c_uint) as isize) = 1i32 as JOCTET;
                *data.offset(offset.wrapping_add(8i32 as c_uint) as isize) = 0i32 as JOCTET;
                *data.offset(offset.wrapping_add(9i32 as c_uint) as isize) = 0i32 as JOCTET;
                *data.offset(offset.wrapping_add(10i32 as c_uint) as isize) =
                    (new_value >> 8i32 & 0xffi32 as c_uint) as JOCTET;
                *data.offset(offset.wrapping_add(11i32 as c_uint) as isize) =
                    (new_value & 0xffi32 as c_uint) as JOCTET
            } else {
                *data.offset(offset.wrapping_add(2i32 as c_uint) as isize) = 4i32 as JOCTET;
                *data.offset(offset.wrapping_add(3i32 as c_uint) as isize) = 0i32 as JOCTET;
                *data.offset(offset.wrapping_add(4i32 as c_uint) as isize) = 1i32 as JOCTET;
                *data.offset(offset.wrapping_add(5i32 as c_uint) as isize) = 0i32 as JOCTET;
                *data.offset(offset.wrapping_add(6i32 as c_uint) as isize) = 0i32 as JOCTET;
                *data.offset(offset.wrapping_add(7i32 as c_uint) as isize) = 0i32 as JOCTET;
                *data.offset(offset.wrapping_add(8i32 as c_uint) as isize) =
                    (new_value & 0xffi32 as c_uint) as JOCTET;
                *data.offset(offset.wrapping_add(9i32 as c_uint) as isize) =
                    (new_value >> 8i32 & 0xffi32 as c_uint) as JOCTET;
                *data.offset(offset.wrapping_add(10i32 as c_uint) as isize) = 0i32 as JOCTET;
                *data.offset(offset.wrapping_add(11i32 as c_uint) as isize) = 0i32 as JOCTET
            }
        }
        offset = offset.wrapping_add(12i32 as c_uint);
        number_of_tags = number_of_tags.wrapping_sub(1);
        if !(number_of_tags != 0) {
            break;
        }
    }
}
/* Adjust output image parameters */
/* Adjust output image parameters as needed.
 *
 * This must be called after jpeg_copy_critical_parameters()
 * and before jpeg_write_coefficients().
 *
 * The return value is the set of virtual coefficient arrays to be written
 * (either the ones allocated by jtransform_request_workspace, or the
 * original source data arrays).  The caller will need to pass this value
 * to jpeg_write_coefficients().
 */
#[no_mangle]

pub unsafe extern "C" fn jtransform_adjust_parameters(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut info: *mut jpeg_transform_info,
) -> *mut jvirt_barray_ptr {
    /* If force-to-grayscale is requested, adjust destination parameters */
    if (*info).force_grayscale != 0 {
        /* First, ensure we have YCbCr or grayscale data, and that the source's
         * Y channel is full resolution.  (No reasonable person would make Y
         * be less than full resolution, so actually coping with that case
         * isn't worth extra code space.  But we check it to avoid crashing.)
         */
        if ((*dstinfo).jpeg_color_space as c_uint == JCS_YCbCr as c_int as c_uint
            && (*dstinfo).num_components == 3i32
            || (*dstinfo).jpeg_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint
                && (*dstinfo).num_components == 1i32)
            && (*(*srcinfo).comp_info.offset(0)).h_samp_factor == (*srcinfo).max_h_samp_factor
            && (*(*srcinfo).comp_info.offset(0)).v_samp_factor == (*srcinfo).max_v_samp_factor
        {
            /* We use jpeg_set_colorspace to make sure subsidiary settings get fixed
             * properly.  Among other things, it sets the target h_samp_factor &
             * v_samp_factor to 1, which typically won't match the source.
             * We have to preserve the source's quantization table number, however.
             */
            let mut sv_quant_tbl_no: c_int = (*(*dstinfo).comp_info.offset(0)).quant_tbl_no;
            jpeg_set_colorspace(dstinfo, JCS_GRAYSCALE);
            (*(*dstinfo).comp_info.offset(0)).quant_tbl_no = sv_quant_tbl_no
        } else {
            /* Sorry, can't do it */
            (*(*dstinfo).err).msg_code = super::jerror::JERR_CONVERSION_NOTIMPL as c_int;
            Some(
                (*(*dstinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(dstinfo as j_common_ptr);
        }
    } else if (*info).num_components == 1i32 {
        /* For a single-component source, we force the destination sampling factors
         * to 1x1, with or without force_grayscale.  This is useful because some
         * decoders choke on grayscale images with other sampling factors.
         */
        (*(*dstinfo).comp_info.offset(0)).h_samp_factor = 1i32;
        (*(*dstinfo).comp_info.offset(0)).v_samp_factor = 1i32
    }
    /* Correct the destination's image dimensions as necessary
     * for rotate/flip, resize, and crop operations.
     */
    /* Transpose destination image parameters */
    match (*info).transform as c_uint {
        3 | 4 | 5 | 7 => {
            (*dstinfo).image_width = (*info).output_height;
            (*dstinfo).image_height = (*info).output_width;
            transpose_critical_parameters(dstinfo);
        }
        _ => {
            (*dstinfo).image_width = (*info).output_width;
            (*dstinfo).image_height = (*info).output_height
        }
    }
    /* Adjust Exif properties */
    if !(*srcinfo).marker_list.is_null()
        && (*(*srcinfo).marker_list).marker as c_int == JPEG_APP0 + 1i32
        && (*(*srcinfo).marker_list).data_length >= 6i32 as c_uint
        && *(*(*srcinfo).marker_list).data.offset(0) as c_int == 0x45i32
        && *(*(*srcinfo).marker_list).data.offset(1) as c_int == 0x78i32
        && *(*(*srcinfo).marker_list).data.offset(2) as c_int == 0x69i32
        && *(*(*srcinfo).marker_list).data.offset(3) as c_int == 0x66i32
        && *(*(*srcinfo).marker_list).data.offset(4) as c_int == 0i32
        && *(*(*srcinfo).marker_list).data.offset(5) as c_int == 0i32
    {
        /* Suppress output of JFIF marker */
        (*dstinfo).write_JFIF_header = FALSE;
        /* Adjust Exif image parameters */
        if (*dstinfo).image_width != (*srcinfo).image_width
            || (*dstinfo).image_height != (*srcinfo).image_height
        {
            /* Align data segment to start of TIFF structure for parsing */
            adjust_exif_parameters(
                (*(*srcinfo).marker_list).data.offset(6),
                (*(*srcinfo).marker_list)
                    .data_length
                    .wrapping_sub(6i32 as c_uint),
                (*dstinfo).image_width,
                (*dstinfo).image_height,
            );
        }
    }
    /* Return the appropriate output data set */
    if !(*info).workspace_coef_arrays.is_null() {
        return (*info).workspace_coef_arrays;
    }
    return src_coef_arrays;
}
/* Execute the actual transformation, if any */
/* Execute the actual transformation, if any.
 *
 * This must be called *after* jpeg_write_coefficients, because it depends
 * on jpeg_write_coefficients to have computed subsidiary values such as
 * the per-component width and height fields in the destination object.
 *
 * Note that some transformations will modify the source data arrays!
 */
#[no_mangle]

pub unsafe extern "C" fn jtransform_execute_transform(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut info: *mut jpeg_transform_info,
) {
    let mut dst_coef_arrays: *mut jvirt_barray_ptr = (*info).workspace_coef_arrays;
    /* Note: conditions tested here should match those in switch statement
     * in jtransform_request_workspace()
     */
    match (*info).transform as c_uint {
        0 => {
            if (*info).x_crop_offset != 0i32 as c_uint || (*info).y_crop_offset != 0i32 as c_uint {
                do_crop(
                    srcinfo,
                    dstinfo,
                    (*info).x_crop_offset,
                    (*info).y_crop_offset,
                    src_coef_arrays,
                    dst_coef_arrays,
                );
            }
        }
        1 => {
            if (*info).y_crop_offset != 0i32 as c_uint || (*info).slow_hflip != 0 {
                do_flip_h(
                    srcinfo,
                    dstinfo,
                    (*info).x_crop_offset,
                    (*info).y_crop_offset,
                    src_coef_arrays,
                    dst_coef_arrays,
                );
            } else {
                do_flip_h_no_crop(srcinfo, dstinfo, (*info).x_crop_offset, src_coef_arrays);
            }
        }
        2 => {
            do_flip_v(
                srcinfo,
                dstinfo,
                (*info).x_crop_offset,
                (*info).y_crop_offset,
                src_coef_arrays,
                dst_coef_arrays,
            );
        }
        3 => {
            do_transpose(
                srcinfo,
                dstinfo,
                (*info).x_crop_offset,
                (*info).y_crop_offset,
                src_coef_arrays,
                dst_coef_arrays,
            );
        }
        4 => {
            do_transverse(
                srcinfo,
                dstinfo,
                (*info).x_crop_offset,
                (*info).y_crop_offset,
                src_coef_arrays,
                dst_coef_arrays,
            );
        }
        5 => {
            do_rot_90(
                srcinfo,
                dstinfo,
                (*info).x_crop_offset,
                (*info).y_crop_offset,
                src_coef_arrays,
                dst_coef_arrays,
            );
        }
        6 => {
            do_rot_180(
                srcinfo,
                dstinfo,
                (*info).x_crop_offset,
                (*info).y_crop_offset,
                src_coef_arrays,
                dst_coef_arrays,
            );
        }
        7 => {
            do_rot_270(
                srcinfo,
                dstinfo,
                (*info).x_crop_offset,
                (*info).y_crop_offset,
                src_coef_arrays,
                dst_coef_arrays,
            );
        }
        _ => {}
    };
}
/* Determine whether lossless transformation is perfectly
 * possible for a specified image and transformation.
 */
/* jtransform_perfect_transform
 *
 * Determine whether lossless transformation is perfectly
 * possible for a specified image and transformation.
 *
 * Inputs:
 *   image_width, image_height: source image dimensions.
 *   MCU_width, MCU_height: pixel dimensions of MCU.
 *   transform: transformation identifier.
 * Parameter sources from initialized jpeg_struct
 * (after reading source header):
 *   image_width = cinfo.image_width
 *   image_height = cinfo.image_height
 *   MCU_width = cinfo.max_h_samp_factor * cinfo.block_size
 *   MCU_height = cinfo.max_v_samp_factor * cinfo.block_size
 * Result:
 *   TRUE = perfect transformation possible
 *   FALSE = perfect transformation not possible
 *           (may use custom action then)
 */
#[no_mangle]

pub unsafe extern "C" fn jtransform_perfect_transform(
    mut image_width: JDIMENSION,
    mut image_height: JDIMENSION,
    mut MCU_width: c_int,
    mut MCU_height: c_int,
    mut transform: JXFORM_CODE,
) -> boolean {
    let mut result: boolean = TRUE; /* initialize TRUE */
    match transform as c_uint {
        1 | 7 => {
            if image_width.wrapping_rem(MCU_width as JDIMENSION) != 0 {
                result = FALSE
            }
        }
        2 | 5 => {
            if image_height.wrapping_rem(MCU_height as JDIMENSION) != 0 {
                result = FALSE
            }
        }
        4 | 6 => {
            if image_width.wrapping_rem(MCU_width as JDIMENSION) != 0 {
                result = FALSE
            }
            if image_height.wrapping_rem(MCU_height as JDIMENSION) != 0 {
                result = FALSE
            }
        }
        _ => {}
    }
    return result;
}
/* recommended default */
/* Setup decompression object to save desired markers in memory */
/* TRANSFORMS_SUPPORTED */
/* Setup decompression object to save desired markers in memory.
 * This must be called before jpeg_read_header() to have the desired effect.
 */
#[no_mangle]

pub unsafe extern "C" fn jcopy_markers_setup(
    mut srcinfo: j_decompress_ptr,
    mut option: JCOPY_OPTION,
) {
    let mut m: c_int = 0;
    /* Save comments except under NONE option */
    if option as c_uint != JCOPYOPT_NONE as c_int as c_uint {
        jpeg_save_markers(srcinfo, JPEG_COM, 0xffffi32 as c_uint);
    }
    /* Save all types of APPn markers iff ALL option */
    if option as c_uint == JCOPYOPT_ALL as c_int as c_uint
        || option as c_uint == JCOPYOPT_ALL_EXCEPT_ICC as c_int as c_uint
    {
        m = 0i32;
        while m < 16i32 {
            if !(option as c_uint == JCOPYOPT_ALL_EXCEPT_ICC as c_int as c_uint && m == 2i32) {
                jpeg_save_markers(srcinfo, JPEG_APP0 + m, 0xffffi32 as c_uint);
            }
            m += 1
        }
    };
    /* SAVE_MARKERS_SUPPORTED */
}
/* Copy markers saved in the given source object to the destination object */
/* Copy markers saved in the given source object to the destination object.
 * This should be called just after jpeg_start_compress() or
 * jpeg_write_coefficients().
 * Note that those routines will have written the SOI, and also the
 * JFIF APP0 or Adobe APP14 markers if selected.
 */
#[no_mangle]

pub unsafe extern "C" fn jcopy_markers_execute(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut option: JCOPY_OPTION,
) {
    let mut marker: jpeg_saved_marker_ptr = 0 as *mut jpeg_marker_struct;
    /* In the current implementation, we don't actually need to examine the
     * option flag here; we just copy everything that got saved.
     * But to avoid confusion, we do not output JFIF and Adobe APP14 markers
     * if the encoder library already wrote one.
     */
    marker = (*srcinfo).marker_list; /* reject duplicate JFIF */
    while !marker.is_null() {
        if !((*dstinfo).write_JFIF_header != 0
            && (*marker).marker as c_int == JPEG_APP0
            && (*marker).data_length >= 5i32 as c_uint
            && *(*marker).data.offset(0) as c_int == 0x4ai32
            && *(*marker).data.offset(1) as c_int == 0x46i32
            && *(*marker).data.offset(2) as c_int == 0x49i32
            && *(*marker).data.offset(3) as c_int == 0x46i32
            && *(*marker).data.offset(4) as c_int == 0i32)
        {
            if !((*dstinfo).write_Adobe_marker != 0
                && (*marker).marker as c_int == JPEG_APP0 + 14i32
                && (*marker).data_length >= 5i32 as c_uint
                && *(*marker).data.offset(0) as c_int == 0x41i32
                && *(*marker).data.offset(1) as c_int == 0x64i32
                && *(*marker).data.offset(2) as c_int == 0x6fi32
                && *(*marker).data.offset(3) as c_int == 0x62i32
                && *(*marker).data.offset(4) as c_int == 0x65i32)
            {
                jpeg_write_marker(
                    dstinfo,
                    (*marker).marker as c_int,
                    (*marker).data,
                    (*marker).data_length,
                ); /* reject duplicate Adobe */
            }
        }
        marker = (*marker).next
    }
}
