pub use crate::jerror::C2RustUnnamed_4;
pub use crate::jpeglib_h::C2RustUnnamed_3;
pub use crate::stdlib::C2RustUnnamed_0;
use libc::c_char;
use libc::c_int;
use libc::c_long;
use libc::c_uint;
use libc::c_ulong;
use libc::c_ushort;
/*
 * Transform parameters struct.
 * NB: application must not change any elements of this struct after
 * calling jtransform_request_workspace.
 */

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
/*
 * Codes for crop parameters, which can individually be unspecified,
 * positive or negative for xoffset or yoffset,
 * positive or forced for width or height.
 */
pub type JCROP_CODE = c_uint;
/*
 * transupp.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1997-2011, Thomas G. Lane, Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2017, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains declarations for image transformation routines and
 * other utility code used by the jpegtran sample application.  These are
 * NOT part of the core JPEG library.  But we keep these routines separate
 * from jpegtran.c to ease the task of maintaining jpegtran-like programs
 * that have other user interfaces.
 *
 * NOTE: all the routines declared here have very specific requirements
 * about when they are to be executed during the reading and writing of the
 * source and destination files.  See the comments in transupp.c, or see
 * jpegtran.c for an example of correct usage.
 */
/* If you happen not to want the image transform support, disable it here */
/* 0 disables transform code */
/*
 * Although rotating and flipping data expressed as DCT coefficients is not
 * hard, there is an asymmetry in the JPEG format specification for images
 * whose dimensions aren't multiples of the iMCU size.  The right and bottom
 * image edges are padded out to the next iMCU boundary with junk data; but
 * no padding is possible at the top and left edges.  If we were to flip
 * the whole image including the pad data, then pad garbage would become
 * visible at the top and/or left, and real pixels would disappear into the
 * pad margins --- perhaps permanently, since encoders & decoders may not
 * bother to preserve DCT blocks that appear to be completely outside the
 * nominal image area.  So, we have to exclude any partial iMCUs from the
 * basic transformation.
 *
 * Transpose is the only transformation that can handle partial iMCUs at the
 * right and bottom edges completely cleanly.  flip_h can flip partial iMCUs
 * at the bottom, but leaves any partial iMCUs at the right edge untouched.
 * Similarly flip_v leaves any partial iMCUs at the bottom edge untouched.
 * The other transforms are defined as combinations of these basic transforms
 * and process edge blocks in a way that preserves the equivalence.
 *
 * The "trim" option causes untransformable partial iMCUs to be dropped;
 * this is not strictly lossless, but it usually gives the best-looking
 * result for odd-size images.  Note that when this option is active,
 * the expected mathematical equivalences between the transforms may not hold.
 * (For example, -rot 270 -trim trims only the bottom edge, but -rot 90 -trim
 * followed by -rot 180 -trim trims both edges.)
 *
 * We also offer a lossless-crop option, which discards data outside a given
 * image region but losslessly preserves what is inside.  Like the rotate and
 * flip transforms, lossless crop is restricted by the JPEG format: the upper
 * left corner of the selected region must fall on an iMCU boundary.  If this
 * does not hold for the given crop parameters, we silently move the upper left
 * corner up and/or left to make it so, simultaneously increasing the region
 * dimensions to keep the lower right crop corner unchanged.  (Thus, the
 * output image covers at least the requested region, but may cover more.)
 * The adjustment of the region dimensions may be optionally disabled.
 *
 * We also provide a lossless-resize option, which is kind of a lossless-crop
 * operation in the DCT coefficient block domain - it discards higher-order
 * coefficients and losslessly preserves lower-order coefficients of a
 * sub-block.
 *
 * Rotate/flip transform, resize, and crop can be requested together in a
 * single invocation.  The crop is applied last --- that is, the crop region
 * is specified in terms of the destination image after transform/resize.
 *
 * We also offer a "force to grayscale" option, which simply discards the
 * chrominance channels of a YCbCr image.  This is lossless in the sense that
 * the luminance channel is preserved exactly.  It's not the same kind of
 * thing as the rotate/flip transformations, but it's convenient to handle it
 * as part of this package, mainly because the transformation routines have to
 * be aware of the option to know how many components to work on.
 */
/*
 * Codes for supported types of image transformations.
 */
pub type JXFORM_CODE = c_uint;
/* TRANSFORMS_SUPPORTED */
/*
 * Support for copying optional markers from source to destination file.
 */
pub type JCOPY_OPTION = c_uint;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::JDIMENSION;
use libc;

pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jpeg_c_coef_controller;
pub use crate::jpegint_h::jpeg_c_main_controller;
pub use crate::jpegint_h::jpeg_c_prep_controller;
pub use crate::jpegint_h::jpeg_color_converter;
pub use crate::jpegint_h::jpeg_color_deconverter;
pub use crate::jpegint_h::jpeg_color_quantizer;
pub use crate::jpegint_h::jpeg_comp_master;
pub use crate::jpegint_h::jpeg_d_coef_controller;
pub use crate::jpegint_h::jpeg_d_main_controller;
pub use crate::jpegint_h::jpeg_d_post_controller;
pub use crate::jpegint_h::jpeg_decomp_master;
pub use crate::jpegint_h::jpeg_downsampler;
pub use crate::jpegint_h::jpeg_entropy_decoder;
pub use crate::jpegint_h::jpeg_entropy_encoder;
pub use crate::jpegint_h::jpeg_forward_dct;
pub use crate::jpegint_h::jpeg_input_controller;
pub use crate::jpegint_h::jpeg_inverse_dct;
pub use crate::jpegint_h::jpeg_marker_reader;
pub use crate::jpegint_h::jpeg_marker_writer;
pub use crate::jpegint_h::jpeg_upsampler;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JCOEFPTR;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::stddef_h::size_t;
pub const JCROP_FORCE: JCROP_CODE = 3;
pub const JCROP_NEG: JCROP_CODE = 2;
pub const JCROP_POS: JCROP_CODE = 1;
pub const JCROP_UNSET: JCROP_CODE = 0;
/* 270-degree clockwise (or 90 ccw) */
pub const JXFORM_ROT_270: JXFORM_CODE = 7;
/* 180-degree rotation */
pub const JXFORM_ROT_180: JXFORM_CODE = 6;
/* 90-degree clockwise rotation */
pub const JXFORM_ROT_90: JXFORM_CODE = 5;
/* transpose across UR-to-LL axis */
pub const JXFORM_TRANSVERSE: JXFORM_CODE = 4;
/* transpose across UL-to-LR axis */
pub const JXFORM_TRANSPOSE: JXFORM_CODE = 3;
/* vertical flip */
pub const JXFORM_FLIP_V: JXFORM_CODE = 2;
/* horizontal flip */
pub const JXFORM_FLIP_H: JXFORM_CODE = 1;
/* no transformation */
pub const JXFORM_NONE: JXFORM_CODE = 0;
/* copy all optional markers except APP2 */
pub const JCOPYOPT_ALL_EXCEPT_ICC: JCOPY_OPTION = 3;
/* copy all optional markers */
pub const JCOPYOPT_ALL: JCOPY_OPTION = 2;
/* copy only comment (COM) markers */
pub const JCOPYOPT_COMMENTS: JCOPY_OPTION = 1;
/* copy no optional markers */
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
    if 0 != (*info).force_grayscale
        && (*srcinfo).jpeg_color_space as c_uint == JCS_YCbCr as c_int as c_uint
        && (*srcinfo).num_components == 3i32
    {
        (*info).num_components = 1i32
    } else {
        (*info).num_components = (*srcinfo).num_components
    }
    (*srcinfo).output_width = (*srcinfo).image_width;
    (*srcinfo).output_height = (*srcinfo).image_height;
    if 0 != (*info).perfect {
        if (*info).num_components == 1i32 {
            if 0 == jtransform_perfect_transform(
                (*srcinfo).output_width,
                (*srcinfo).output_height,
                (*srcinfo).min_DCT_scaled_size,
                (*srcinfo).min_DCT_scaled_size,
                (*info).transform,
            ) {
                return FALSE;
            }
        } else if 0
            == jtransform_perfect_transform(
                (*srcinfo).output_width,
                (*srcinfo).output_height,
                (*srcinfo).max_h_samp_factor * (*srcinfo).min_DCT_scaled_size,
                (*srcinfo).max_v_samp_factor * (*srcinfo).min_DCT_scaled_size,
                (*info).transform,
            )
        {
            return FALSE;
        }
    }
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
    if 0 != (*info).crop {
        if (*info).crop_xoffset_set as c_uint == JCROP_UNSET as c_int as c_uint {
            (*info).crop_xoffset = 0i32 as JDIMENSION
        }
        if (*info).crop_yoffset_set as c_uint == JCROP_UNSET as c_int as c_uint {
            (*info).crop_yoffset = 0i32 as JDIMENSION
        }
        if (*info).crop_xoffset >= (*info).output_width
            || (*info).crop_yoffset >= (*info).output_height
        {
            (*(*srcinfo).err).msg_code = JERR_BAD_CROP_SPEC as c_int;
            (*(*srcinfo).err)
                .error_exit
                .expect("non-null function pointer")(srcinfo as j_common_ptr);
        }
        if (*info).crop_width_set as c_uint == JCROP_UNSET as c_int as c_uint {
            (*info).crop_width = (*info).output_width.wrapping_sub((*info).crop_xoffset)
        }
        if (*info).crop_height_set as c_uint == JCROP_UNSET as c_int as c_uint {
            (*info).crop_height = (*info).output_height.wrapping_sub((*info).crop_yoffset)
        }
        if (*info).crop_width <= 0i32 as c_uint
            || (*info).crop_width > (*info).output_width
            || (*info).crop_height <= 0i32 as c_uint
            || (*info).crop_height > (*info).output_height
            || (*info).crop_xoffset > (*info).output_width.wrapping_sub((*info).crop_width)
            || (*info).crop_yoffset > (*info).output_height.wrapping_sub((*info).crop_height)
        {
            (*(*srcinfo).err).msg_code = JERR_BAD_CROP_SPEC as c_int;
            (*(*srcinfo).err)
                .error_exit
                .expect("non-null function pointer")(srcinfo as j_common_ptr);
        }
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
        (*info).x_crop_offset = xoffset.wrapping_div((*info).iMCU_sample_width as c_uint);
        (*info).y_crop_offset = yoffset.wrapping_div((*info).iMCU_sample_height as c_uint)
    } else {
        (*info).x_crop_offset = 0i32 as JDIMENSION;
        (*info).y_crop_offset = 0i32 as JDIMENSION
    }
    need_workspace = FALSE;
    transpose_it = FALSE;
    match (*info).transform as c_uint {
        0 => {
            if (*info).x_crop_offset != 0i32 as c_uint || (*info).y_crop_offset != 0i32 as c_uint {
                need_workspace = TRUE
            }
        }
        1 => {
            /* No workspace needed if neither cropping nor transforming */
            if 0 != (*info).trim {
                trim_right_edge(info, (*srcinfo).output_width);
            }
            if (*info).y_crop_offset != 0i32 as c_uint || 0 != (*info).slow_hflip {
                need_workspace = TRUE
            }
        }
        2 => {
            /* do_flip_h_no_crop doesn't need a workspace array */
            if 0 != (*info).trim {
                trim_bottom_edge(info, (*srcinfo).output_height);
            }
            need_workspace = TRUE
        }
        3 => {
            need_workspace = TRUE;
            transpose_it = TRUE
        }
        4 => {
            if 0 != (*info).trim {
                trim_right_edge(info, (*srcinfo).output_height);
                trim_bottom_edge(info, (*srcinfo).output_width);
            }
            need_workspace = TRUE;
            transpose_it = TRUE
        }
        5 => {
            if 0 != (*info).trim {
                trim_right_edge(info, (*srcinfo).output_height);
            }
            need_workspace = TRUE;
            transpose_it = TRUE
        }
        6 => {
            if 0 != (*info).trim {
                trim_right_edge(info, (*srcinfo).output_width);
                trim_bottom_edge(info, (*srcinfo).output_height);
            }
            need_workspace = TRUE
        }
        7 => {
            if 0 != (*info).trim {
                trim_bottom_edge(info, (*srcinfo).output_width);
            }
            need_workspace = TRUE;
            transpose_it = TRUE
        }
        _ => {}
    }
    if 0 != need_workspace {
        coef_arrays = (*(*srcinfo).mem)
            .alloc_small
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
                v_samp_factor = 1i32;
                h_samp_factor = v_samp_factor
            } else if 0 != transpose_it {
                h_samp_factor = (*compptr).v_samp_factor;
                v_samp_factor = (*compptr).h_samp_factor
            } else {
                h_samp_factor = (*compptr).h_samp_factor;
                v_samp_factor = (*compptr).v_samp_factor
            }
            width_in_blocks = width_in_iMCUs.wrapping_mul(h_samp_factor as c_uint);
            height_in_blocks = height_in_iMCUs.wrapping_mul(v_samp_factor as c_uint);
            let ref mut fresh28 = *coef_arrays.offset(ci as isize);
            *fresh28 = (*(*srcinfo).mem)
                .request_virt_barray
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
    if 0 != (*info).force_grayscale {
        if ((*dstinfo).jpeg_color_space as c_uint == JCS_YCbCr as c_int as c_uint
            && (*dstinfo).num_components == 3i32
            || (*dstinfo).jpeg_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint
                && (*dstinfo).num_components == 1i32)
            && (*(*srcinfo).comp_info.offset(0isize)).h_samp_factor == (*srcinfo).max_h_samp_factor
            && (*(*srcinfo).comp_info.offset(0isize)).v_samp_factor == (*srcinfo).max_v_samp_factor
        {
            let mut sv_quant_tbl_no: c_int = (*(*dstinfo).comp_info.offset(0isize)).quant_tbl_no;
            jpeg_set_colorspace(dstinfo, JCS_GRAYSCALE);
            (*(*dstinfo).comp_info.offset(0isize)).quant_tbl_no = sv_quant_tbl_no
        } else {
            (*(*dstinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as c_int;
            (*(*dstinfo).err)
                .error_exit
                .expect("non-null function pointer")(dstinfo as j_common_ptr);
        }
    } else if (*info).num_components == 1i32 {
        (*(*dstinfo).comp_info.offset(0isize)).h_samp_factor = 1i32;
        (*(*dstinfo).comp_info.offset(0isize)).v_samp_factor = 1i32
    }
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
    if !(*srcinfo).marker_list.is_null()
        && (*(*srcinfo).marker_list).marker as c_int == JPEG_APP0 + 1i32
        && (*(*srcinfo).marker_list).data_length >= 6i32 as c_uint
        && *(*(*srcinfo).marker_list).data.offset(0isize) as c_int == 0x45i32
        && *(*(*srcinfo).marker_list).data.offset(1isize) as c_int == 0x78i32
        && *(*(*srcinfo).marker_list).data.offset(2isize) as c_int == 0x69i32
        && *(*(*srcinfo).marker_list).data.offset(3isize) as c_int == 0x66i32
        && *(*(*srcinfo).marker_list).data.offset(4isize) as c_int == 0i32
        && *(*(*srcinfo).marker_list).data.offset(5isize) as c_int == 0i32
    {
        (*dstinfo).write_JFIF_header = FALSE;
        if (*dstinfo).image_width != (*srcinfo).image_width
            || (*dstinfo).image_height != (*srcinfo).image_height
        {
            adjust_exif_parameters(
                (*(*srcinfo).marker_list).data.offset(6isize),
                (*(*srcinfo).marker_list)
                    .data_length
                    .wrapping_sub(6i32 as c_uint),
                (*dstinfo).image_width,
                (*dstinfo).image_height,
            );
        }
    }
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
            if (*info).y_crop_offset != 0i32 as c_uint || 0 != (*info).slow_hflip {
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
    if option as c_uint != JCOPYOPT_NONE as c_int as c_uint {
        jpeg_save_markers(srcinfo, JPEG_COM, 0xffffi32 as c_uint);
    }
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
}
/* Copy markers saved in the given source object to the destination object */
/* SAVE_MARKERS_SUPPORTED */
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
    marker = (*srcinfo).marker_list;
    while !marker.is_null() {
        if !(0 != (*dstinfo).write_JFIF_header
            && (*marker).marker as c_int == JPEG_APP0
            && (*marker).data_length >= 5i32 as c_uint
            && *(*marker).data.offset(0isize) as c_int == 0x4ai32
            && *(*marker).data.offset(1isize) as c_int == 0x46i32
            && *(*marker).data.offset(2isize) as c_int == 0x49i32
            && *(*marker).data.offset(3isize) as c_int == 0x46i32
            && *(*marker).data.offset(4isize) as c_int == 0i32)
        {
            /* reject duplicate JFIF */
            if !(0 != (*dstinfo).write_Adobe_marker
                && (*marker).marker as c_int == JPEG_APP0 + 14i32
                && (*marker).data_length >= 5i32 as c_uint
                && *(*marker).data.offset(0isize) as c_int == 0x41i32
                && *(*marker).data.offset(1isize) as c_int == 0x64i32
                && *(*marker).data.offset(2isize) as c_int == 0x6fi32
                && *(*marker).data.offset(3isize) as c_int == 0x62i32
                && *(*marker).data.offset(4isize) as c_int == 0x65i32)
            {
                /* reject duplicate Adobe */
                jpeg_write_marker(
                    dstinfo,
                    (*marker).marker as c_int,
                    (*marker).data,
                    (*marker).data_length,
                );
            }
        }
        marker = (*marker).next
    }
}
/* recommended default */
pub const JCOPYOPT_DEFAULT: c_int = JCOPYOPT_COMMENTS as c_int;
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
    if 0 != *(*__ctype_b_loc()).offset(*spec as c_int as isize) as c_int
        & _ISdigit as c_int as c_ushort as c_int
    {
        if 0 == jt_read_integer(&mut spec, &mut (*info).crop_width) {
            return FALSE;
        }
        if *spec as c_int == 'f' as i32 || *spec as c_int == 'F' as i32 {
            spec = spec.offset(1isize);
            (*info).crop_width_set = JCROP_FORCE
        } else {
            (*info).crop_width_set = JCROP_POS
        }
    }
    if *spec as c_int == 'x' as i32 || *spec as c_int == 'X' as i32 {
        spec = spec.offset(1isize);
        if 0 == jt_read_integer(&mut spec, &mut (*info).crop_height) {
            return FALSE;
        }
        if *spec as c_int == 'f' as i32 || *spec as c_int == 'F' as i32 {
            spec = spec.offset(1isize);
            (*info).crop_height_set = JCROP_FORCE
        } else {
            (*info).crop_height_set = JCROP_POS
        }
    }
    if *spec as c_int == '+' as i32 || *spec as c_int == '-' as i32 {
        (*info).crop_xoffset_set = (if *spec as c_int == '-' as i32 {
            JCROP_NEG as c_int
        } else {
            JCROP_POS as c_int
        }) as JCROP_CODE;
        spec = spec.offset(1isize);
        if 0 == jt_read_integer(&mut spec, &mut (*info).crop_xoffset) {
            return FALSE;
        }
    }
    if *spec as c_int == '+' as i32 || *spec as c_int == '-' as i32 {
        (*info).crop_yoffset_set = (if *spec as c_int == '-' as i32 {
            JCROP_NEG as c_int
        } else {
            JCROP_POS as c_int
        }) as JCROP_CODE;
        spec = spec.offset(1isize);
        if 0 == jt_read_integer(&mut spec, &mut (*info).crop_yoffset) {
            return FALSE;
        }
    }
    if *spec as c_int != '\u{0}' as i32 {
        return FALSE;
    }
    (*info).crop = TRUE;
    return TRUE;
}
pub use crate::jerror::JERR_ARITH_NOTIMPL;
pub use crate::jerror::JERR_BAD_ALIGN_TYPE;
pub use crate::jerror::JERR_BAD_ALLOC_CHUNK;
pub use crate::jerror::JERR_BAD_BUFFER_MODE;
pub use crate::jerror::JERR_BAD_COMPONENT_ID;
pub use crate::jerror::JERR_BAD_CROP_SPEC;
pub use crate::jerror::JERR_BAD_DCTSIZE;
pub use crate::jerror::JERR_BAD_DCT_COEF;
pub use crate::jerror::JERR_BAD_HUFF_TABLE;
pub use crate::jerror::JERR_BAD_IN_COLORSPACE;
pub use crate::jerror::JERR_BAD_J_COLORSPACE;
pub use crate::jerror::JERR_BAD_LENGTH;
pub use crate::jerror::JERR_BAD_LIB_VERSION;
pub use crate::jerror::JERR_BAD_MCU_SIZE;
pub use crate::jerror::JERR_BAD_PARAM;
pub use crate::jerror::JERR_BAD_PARAM_VALUE;
pub use crate::jerror::JERR_BAD_POOL_ID;
pub use crate::jerror::JERR_BAD_PRECISION;
pub use crate::jerror::JERR_BAD_PROGRESSION;
pub use crate::jerror::JERR_BAD_PROG_SCRIPT;
pub use crate::jerror::JERR_BAD_SAMPLING;
pub use crate::jerror::JERR_BAD_SCAN_SCRIPT;
pub use crate::jerror::JERR_BAD_STATE;
pub use crate::jerror::JERR_BAD_STRUCT_SIZE;
pub use crate::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use crate::jerror::JERR_BUFFER_SIZE;
pub use crate::jerror::JERR_CANT_SUSPEND;
pub use crate::jerror::JERR_CCIR601_NOTIMPL;
pub use crate::jerror::JERR_COMPONENT_COUNT;
pub use crate::jerror::JERR_CONVERSION_NOTIMPL;
pub use crate::jerror::JERR_DAC_INDEX;
pub use crate::jerror::JERR_DAC_VALUE;
pub use crate::jerror::JERR_DHT_INDEX;
pub use crate::jerror::JERR_DQT_INDEX;
pub use crate::jerror::JERR_EMPTY_IMAGE;
pub use crate::jerror::JERR_EMS_READ;
pub use crate::jerror::JERR_EMS_WRITE;
pub use crate::jerror::JERR_EOI_EXPECTED;
pub use crate::jerror::JERR_FILE_READ;
pub use crate::jerror::JERR_FILE_WRITE;
pub use crate::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use crate::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use crate::jerror::JERR_HUFF_MISSING_CODE;
pub use crate::jerror::JERR_IMAGE_TOO_BIG;
pub use crate::jerror::JERR_INPUT_EMPTY;
pub use crate::jerror::JERR_INPUT_EOF;
pub use crate::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use crate::jerror::JERR_MISSING_DATA;
pub use crate::jerror::JERR_MODE_CHANGE;
pub use crate::jerror::JERR_NOTIMPL;
pub use crate::jerror::JERR_NOT_COMPILED;
pub use crate::jerror::JERR_NO_BACKING_STORE;
pub use crate::jerror::JERR_NO_HUFF_TABLE;
pub use crate::jerror::JERR_NO_IMAGE;
pub use crate::jerror::JERR_NO_QUANT_TABLE;
pub use crate::jerror::JERR_NO_SOI;
pub use crate::jerror::JERR_OUT_OF_MEMORY;
pub use crate::jerror::JERR_QUANT_COMPONENTS;
pub use crate::jerror::JERR_QUANT_FEW_COLORS;
pub use crate::jerror::JERR_QUANT_MANY_COLORS;
pub use crate::jerror::JERR_SOF_DUPLICATE;
pub use crate::jerror::JERR_SOF_NO_SOS;
pub use crate::jerror::JERR_SOF_UNSUPPORTED;
pub use crate::jerror::JERR_SOI_DUPLICATE;
pub use crate::jerror::JERR_SOS_NO_SOF;
pub use crate::jerror::JERR_TFILE_CREATE;
pub use crate::jerror::JERR_TFILE_READ;
pub use crate::jerror::JERR_TFILE_SEEK;
pub use crate::jerror::JERR_TFILE_WRITE;
pub use crate::jerror::JERR_TOO_LITTLE_DATA;
pub use crate::jerror::JERR_UNKNOWN_MARKER;
pub use crate::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use crate::jerror::JERR_VIRTUAL_BUG;
pub use crate::jerror::JERR_WIDTH_OVERFLOW;
pub use crate::jerror::JERR_XMS_READ;
pub use crate::jerror::JERR_XMS_WRITE;
pub use crate::jerror::JMSG_COPYRIGHT;
pub use crate::jerror::JMSG_LASTMSGCODE;
pub use crate::jerror::JMSG_NOMESSAGE;
pub use crate::jerror::JMSG_VERSION;
pub use crate::jerror::JTRC_16BIT_TABLES;
pub use crate::jerror::JTRC_ADOBE;
pub use crate::jerror::JTRC_APP0;
pub use crate::jerror::JTRC_APP14;
pub use crate::jerror::JTRC_DAC;
pub use crate::jerror::JTRC_DHT;
pub use crate::jerror::JTRC_DQT;
pub use crate::jerror::JTRC_DRI;
pub use crate::jerror::JTRC_EMS_CLOSE;
pub use crate::jerror::JTRC_EMS_OPEN;
pub use crate::jerror::JTRC_EOI;
pub use crate::jerror::JTRC_HUFFBITS;
pub use crate::jerror::JTRC_JFIF;
pub use crate::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use crate::jerror::JTRC_JFIF_EXTENSION;
pub use crate::jerror::JTRC_JFIF_THUMBNAIL;
pub use crate::jerror::JTRC_MISC_MARKER;
pub use crate::jerror::JTRC_PARMLESS_MARKER;
pub use crate::jerror::JTRC_QUANTVALS;
pub use crate::jerror::JTRC_QUANT_3_NCOLORS;
pub use crate::jerror::JTRC_QUANT_NCOLORS;
pub use crate::jerror::JTRC_QUANT_SELECTED;
pub use crate::jerror::JTRC_RECOVERY_ACTION;
pub use crate::jerror::JTRC_RST;
pub use crate::jerror::JTRC_SMOOTH_NOTIMPL;
pub use crate::jerror::JTRC_SOF;
pub use crate::jerror::JTRC_SOF_COMPONENT;
pub use crate::jerror::JTRC_SOI;
pub use crate::jerror::JTRC_SOS;
pub use crate::jerror::JTRC_SOS_COMPONENT;
pub use crate::jerror::JTRC_SOS_PARAMS;
pub use crate::jerror::JTRC_TFILE_CLOSE;
pub use crate::jerror::JTRC_TFILE_OPEN;
pub use crate::jerror::JTRC_THUMB_JPEG;
pub use crate::jerror::JTRC_THUMB_PALETTE;
pub use crate::jerror::JTRC_THUMB_RGB;
pub use crate::jerror::JTRC_UNKNOWN_IDS;
pub use crate::jerror::JTRC_XMS_CLOSE;
pub use crate::jerror::JTRC_XMS_OPEN;
pub use crate::jerror::JWRN_ADOBE_XFORM;
pub use crate::jerror::JWRN_BOGUS_ICC;
pub use crate::jerror::JWRN_BOGUS_PROGRESSION;
pub use crate::jerror::JWRN_EXTRANEOUS_DATA;
pub use crate::jerror::JWRN_HIT_MARKER;
pub use crate::jerror::JWRN_HUFF_BAD_CODE;
pub use crate::jerror::JWRN_JFIF_MAJOR;
pub use crate::jerror::JWRN_JPEG_EOF;
pub use crate::jerror::JWRN_MUST_RESYNC;
pub use crate::jerror::JWRN_NOT_SEQUENTIAL;
pub use crate::jerror::JWRN_TOO_MUCH_DATA;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jpegint_h::jcopy_block_row;
pub use crate::jpegint_h::jdiv_round_up;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpeglib_h::jpeg_save_markers;
pub use crate::jpeglib_h::jpeg_set_colorspace;
pub use crate::jpeglib_h::jpeg_write_marker;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::DCTSIZE;
pub use crate::jpeglib_h::DCTSIZE2;
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
pub use crate::jpeglib_h::JDITHER_FS;
pub use crate::jpeglib_h::JDITHER_NONE;
pub use crate::jpeglib_h::JDITHER_ORDERED;
pub use crate::jpeglib_h::JPEG_APP0;
pub use crate::jpeglib_h::JPEG_COM;
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::NUM_QUANT_TBLS;
pub use crate::stddef_h::NULL;
pub use crate::stdlib::_ISalnum;
pub use crate::stdlib::_ISalpha;
pub use crate::stdlib::_ISblank;
pub use crate::stdlib::_IScntrl;
pub use crate::stdlib::_ISdigit;
pub use crate::stdlib::_ISgraph;
pub use crate::stdlib::_ISlower;
pub use crate::stdlib::_ISprint;
pub use crate::stdlib::_ISpunct;
pub use crate::stdlib::_ISspace;
pub use crate::stdlib::_ISupper;
pub use crate::stdlib::_ISxdigit;
pub use crate::stdlib::__ctype_b_loc;
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
) {
    let mut dst_blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut y_crop_blocks: JDIMENSION = 0;
    let mut ci: c_int = 0;
    let mut offset_y: c_int = 0;
    let mut src_buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut dst_buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    ci = 0i32;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as c_uint);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as c_uint);
        dst_blk_y = 0i32 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = (*(*srcinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            src_buffer = (*(*srcinfo).mem)
                .access_virt_barray
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
) {
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
            buffer = (*(*srcinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *src_coef_arrays.offset(ci as isize),
                blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            offset_y = 0i32;
            while offset_y < (*compptr).v_samp_factor {
                blk_x = 0i32 as JDIMENSION;
                while blk_x.wrapping_mul(2i32 as c_uint) < comp_width {
                    ptr1 =
                        (*(*buffer.offset(offset_y as isize)).offset(blk_x as isize)).as_mut_ptr();
                    ptr2 =
                        (*(*buffer.offset(offset_y as isize))
                            .offset(comp_width.wrapping_sub(blk_x).wrapping_sub(1i32 as c_uint)
                                as isize))
                        .as_mut_ptr();
                    k = 0i32;
                    while k < DCTSIZE2 {
                        temp1 = *ptr1;
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
) {
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
            dst_buffer = (*(*srcinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            src_buffer = (*(*srcinfo).mem)
                .access_virt_barray
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
                        dst_ptr = (*dst_row_ptr.offset(dst_blk_x as isize)).as_mut_ptr();
                        src_ptr = (*src_row_ptr.offset(
                            comp_width
                                .wrapping_sub(x_crop_blocks)
                                .wrapping_sub(dst_blk_x)
                                .wrapping_sub(1i32 as c_uint) as isize,
                        ))
                        .as_mut_ptr();
                        k = 0i32;
                        while k < DCTSIZE2 {
                            let fresh5 = dst_ptr;
                            dst_ptr = dst_ptr.offset(1);
                            let fresh4 = src_ptr;
                            src_ptr = src_ptr.offset(1);
                            *fresh5 = *fresh4;
                            let fresh7 = dst_ptr;
                            dst_ptr = dst_ptr.offset(1);
                            let fresh6 = src_ptr;
                            src_ptr = src_ptr.offset(1);
                            *fresh7 = -(*fresh6 as c_int) as JCOEF;
                            k += 2i32
                        }
                    } else {
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
) {
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
            dst_buffer = (*(*srcinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            if y_crop_blocks.wrapping_add(dst_blk_y) < comp_height {
                src_buffer = (*(*srcinfo).mem)
                    .access_virt_barray
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
                src_buffer = (*(*srcinfo).mem)
                    .access_virt_barray
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
                            j = 0i32;
                            while j < DCTSIZE {
                                let fresh9 = dst_ptr;
                                dst_ptr = dst_ptr.offset(1);
                                let fresh8 = src_ptr;
                                src_ptr = src_ptr.offset(1);
                                *fresh9 = *fresh8;
                                j += 1
                            }
                            j = 0i32;
                            while j < DCTSIZE {
                                let fresh11 = dst_ptr;
                                dst_ptr = dst_ptr.offset(1);
                                let fresh10 = src_ptr;
                                src_ptr = src_ptr.offset(1);
                                *fresh11 = -(*fresh10 as c_int) as JCOEF;
                                j += 1
                            }
                            i += 2i32
                        }
                        dst_blk_x = dst_blk_x.wrapping_add(1)
                    }
                } else {
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
) {
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
    ci = 0i32;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as c_uint);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as c_uint);
        dst_blk_y = 0i32 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = (*(*srcinfo).mem)
                .access_virt_barray
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
                    src_buffer = (*(*srcinfo).mem)
                        .access_virt_barray
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
) {
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
            dst_buffer = (*(*srcinfo).mem)
                .access_virt_barray
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
                        src_buffer = (*(*srcinfo).mem)
                            .access_virt_barray
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
                        src_buffer = (*(*srcinfo).mem)
                            .access_virt_barray
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
) {
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
            dst_buffer = (*(*srcinfo).mem)
                .access_virt_barray
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
                    src_buffer = (*(*srcinfo).mem)
                        .access_virt_barray
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
) {
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
            dst_buffer = (*(*srcinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            if y_crop_blocks.wrapping_add(dst_blk_y) < comp_height {
                src_buffer = (*(*srcinfo).mem)
                    .access_virt_barray
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
                src_buffer = (*(*srcinfo).mem)
                    .access_virt_barray
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
                    src_row_ptr =
                        *src_buffer.offset(((*compptr).v_samp_factor - offset_y - 1i32) as isize);
                    dst_blk_x = 0i32 as JDIMENSION;
                    while dst_blk_x < (*compptr).width_in_blocks {
                        dst_ptr = (*dst_row_ptr.offset(dst_blk_x as isize)).as_mut_ptr();
                        if x_crop_blocks.wrapping_add(dst_blk_x) < comp_width {
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
                                j = 0i32;
                                while j < DCTSIZE {
                                    let fresh13 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    let fresh12 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    *fresh13 = *fresh12;
                                    let fresh15 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    let fresh14 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    *fresh15 = -(*fresh14 as c_int) as JCOEF;
                                    j += 2i32
                                }
                                j = 0i32;
                                while j < DCTSIZE {
                                    let fresh17 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    let fresh16 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    *fresh17 = -(*fresh16 as c_int) as JCOEF;
                                    let fresh19 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    let fresh18 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    *fresh19 = *fresh18;
                                    j += 2i32
                                }
                                i += 2i32
                            }
                        } else {
                            src_ptr = (*src_row_ptr
                                .offset(x_crop_blocks.wrapping_add(dst_blk_x) as isize))
                            .as_mut_ptr();
                            i = 0i32;
                            while i < DCTSIZE {
                                j = 0i32;
                                while j < DCTSIZE {
                                    let fresh21 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    let fresh20 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    *fresh21 = *fresh20;
                                    j += 1
                                }
                                j = 0i32;
                                while j < DCTSIZE {
                                    let fresh23 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    let fresh22 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    *fresh23 = -(*fresh22 as c_int) as JCOEF;
                                    j += 1
                                }
                                i += 2i32
                            }
                        }
                        dst_blk_x = dst_blk_x.wrapping_add(1)
                    }
                } else {
                    src_row_ptr = *src_buffer.offset(offset_y as isize);
                    dst_blk_x = 0i32 as JDIMENSION;
                    while dst_blk_x < (*compptr).width_in_blocks {
                        if x_crop_blocks.wrapping_add(dst_blk_x) < comp_width {
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
                                let fresh25 = dst_ptr;
                                dst_ptr = dst_ptr.offset(1);
                                let fresh24 = src_ptr;
                                src_ptr = src_ptr.offset(1);
                                *fresh25 = *fresh24;
                                let fresh27 = dst_ptr;
                                dst_ptr = dst_ptr.offset(1);
                                let fresh26 = src_ptr;
                                src_ptr = src_ptr.offset(1);
                                *fresh27 = -(*fresh26 as c_int) as JCOEF;
                                i += 2i32
                            }
                        } else {
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
) {
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
            dst_buffer = (*(*srcinfo).mem)
                .access_virt_barray
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
                        src_buffer = (*(*srcinfo).mem)
                            .access_virt_barray
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
                        src_buffer = (*(*srcinfo).mem)
                            .access_virt_barray
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
    let mut ptr: *const c_char = *strptr;
    let mut val: JDIMENSION = 0i32 as JDIMENSION;
    while 0
        != *(*__ctype_b_loc()).offset(*ptr as c_int as isize) as c_int
            & _ISdigit as c_int as c_ushort as c_int
    {
        val = val
            .wrapping_mul(10i32 as c_uint)
            .wrapping_add((*ptr as c_int - '0' as i32) as JDIMENSION);
        ptr = ptr.offset(1isize)
    }
    *result = val;
    if ptr == *strptr {
        return FALSE;
    }
    *strptr = ptr;
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
    jtemp = (*dstinfo).image_width;
    (*dstinfo).image_width = (*dstinfo).image_height;
    (*dstinfo).image_height = jtemp;
    ci = 0i32;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        itemp = (*compptr).h_samp_factor;
        (*compptr).h_samp_factor = (*compptr).v_samp_factor;
        (*compptr).v_samp_factor = itemp;
        ci += 1
    }
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
    /* Flag for byte order */
    let mut is_motorola: boolean = 0;
    let mut number_of_tags: c_uint = 0;
    let mut tagnum: c_uint = 0;
    let mut firstoffset: c_uint = 0;
    let mut offset: c_uint = 0;
    let mut new_value: JDIMENSION = 0;
    if length < 12i32 as c_uint {
        return;
    }
    if *data.offset(0isize) as c_int == 0x49i32 && *data.offset(1isize) as c_int == 0x49i32 {
        is_motorola = FALSE
    } else if *data.offset(0isize) as c_int == 0x4di32 && *data.offset(1isize) as c_int == 0x4di32 {
        is_motorola = TRUE
    } else {
        return;
    }
    if 0 != is_motorola {
        if *data.offset(2isize) as c_int != 0i32 {
            return;
        }
        if *data.offset(3isize) as c_int != 0x2ai32 {
            return;
        }
    } else {
        if *data.offset(3isize) as c_int != 0i32 {
            return;
        }
        if *data.offset(2isize) as c_int != 0x2ai32 {
            return;
        }
    }
    if 0 != is_motorola {
        if *data.offset(4isize) as c_int != 0i32 {
            return;
        }
        if *data.offset(5isize) as c_int != 0i32 {
            return;
        }
        firstoffset = *data.offset(6isize) as c_uint;
        firstoffset <<= 8i32;
        firstoffset = firstoffset.wrapping_add(*data.offset(7isize) as c_uint)
    } else {
        if *data.offset(7isize) as c_int != 0i32 {
            return;
        }
        if *data.offset(6isize) as c_int != 0i32 {
            return;
        }
        firstoffset = *data.offset(5isize) as c_uint;
        firstoffset <<= 8i32;
        firstoffset = firstoffset.wrapping_add(*data.offset(4isize) as c_uint)
    }
    if firstoffset > length.wrapping_sub(2i32 as c_uint) {
        return;
    }
    if 0 != is_motorola {
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
    loop {
        if firstoffset > length.wrapping_sub(12i32 as c_uint) {
            return;
        }
        if 0 != is_motorola {
            tagnum = *data.offset(firstoffset as isize) as c_uint;
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
            /* found ExifSubIFD offset Tag */
            break;
        } else {
            number_of_tags = number_of_tags.wrapping_sub(1);
            if number_of_tags == 0i32 as c_uint {
                return;
            }
            firstoffset = firstoffset.wrapping_add(12i32 as c_uint)
        }
    }
    if 0 != is_motorola {
        if *data.offset(firstoffset.wrapping_add(8i32 as c_uint) as isize) as c_int != 0i32 {
            return;
        }
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
    if 0 != is_motorola {
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
    loop {
        if offset > length.wrapping_sub(12i32 as c_uint) {
            return;
        }
        if 0 != is_motorola {
            tagnum = *data.offset(offset as isize) as c_uint;
            tagnum <<= 8i32;
            tagnum = tagnum
                .wrapping_add(*data.offset(offset.wrapping_add(1i32 as c_uint) as isize) as c_uint)
        } else {
            tagnum = *data.offset(offset.wrapping_add(1i32 as c_uint) as isize) as c_uint;
            tagnum <<= 8i32;
            tagnum = tagnum.wrapping_add(*data.offset(offset as isize) as c_uint)
        }
        if tagnum == 0xa002i32 as c_uint || tagnum == 0xa003i32 as c_uint {
            if tagnum == 0xa002i32 as c_uint {
                new_value = new_width
            } else {
                new_value = new_height
            }
            if 0 != is_motorola {
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
        if !(0 != number_of_tags) {
            break;
        }
    }
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
    /* initialize TRUE */
    let mut result: boolean = TRUE;
    match transform as c_uint {
        1 | 7 => {
            if 0 != image_width.wrapping_rem(MCU_width as JDIMENSION) {
                result = FALSE
            }
        }
        2 | 5 => {
            if 0 != image_height.wrapping_rem(MCU_height as JDIMENSION) {
                result = FALSE
            }
        }
        4 | 6 => {
            if 0 != image_width.wrapping_rem(MCU_width as JDIMENSION) {
                result = FALSE
            }
            if 0 != image_height.wrapping_rem(MCU_height as JDIMENSION) {
                result = FALSE
            }
        }
        _ => {}
    }
    return result;
}
