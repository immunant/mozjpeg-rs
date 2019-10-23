pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::src::jerror::C2RustUnnamed_3;
pub use crate::stdlib::C2RustUnnamed_0;
use libc;

pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jcopy_block_row;
pub use crate::jpegint_h::jdiv_round_up;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::jpeg_c_coef_controller;
pub use crate::jpeglib_h::jpeg_c_main_controller;
pub use crate::jpeglib_h::jpeg_c_prep_controller;
pub use crate::jpeglib_h::jpeg_color_converter;
pub use crate::jpeglib_h::jpeg_color_deconverter;
pub use crate::jpeglib_h::jpeg_color_quantizer;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_comp_master;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_d_coef_controller;
pub use crate::jpeglib_h::jpeg_d_main_controller;
pub use crate::jpeglib_h::jpeg_d_post_controller;
pub use crate::jpeglib_h::jpeg_decomp_master;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_downsampler;
pub use crate::jpeglib_h::jpeg_entropy_decoder;
pub use crate::jpeglib_h::jpeg_entropy_encoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_forward_dct;
pub use crate::jpeglib_h::jpeg_input_controller;
pub use crate::jpeglib_h::jpeg_inverse_dct;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_reader;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_marker_writer;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_save_markers;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jpeg_set_colorspace;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jpeg_upsampler;
pub use crate::jpeglib_h::jpeg_write_marker;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::DCTSIZE;
pub use crate::jpeglib_h::DCTSIZE2;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JCOEFPTR;
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
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JPEG_APP0;
pub use crate::jpeglib_h::JPEG_COM;
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::jpeglib_h::NUM_QUANT_TBLS;
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
pub use crate::stddef_h::size_t;
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
// =============== BEGIN transupp_h ================
pub type JXFORM_CODE = libc::c_uint;

pub type JCROP_CODE = libc::c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_transform_info {
    pub transform: crate::src::transupp::JXFORM_CODE,
    pub perfect: crate::jmorecfg_h::boolean,
    pub trim: crate::jmorecfg_h::boolean,
    pub force_grayscale: crate::jmorecfg_h::boolean,
    pub crop: crate::jmorecfg_h::boolean,
    pub slow_hflip: crate::jmorecfg_h::boolean,
    pub crop_width: crate::jmorecfg_h::JDIMENSION,
    pub crop_width_set: crate::src::transupp::JCROP_CODE,
    pub crop_height: crate::jmorecfg_h::JDIMENSION,
    pub crop_height_set: crate::src::transupp::JCROP_CODE,
    pub crop_xoffset: crate::jmorecfg_h::JDIMENSION,
    pub crop_xoffset_set: crate::src::transupp::JCROP_CODE,
    pub crop_yoffset: crate::jmorecfg_h::JDIMENSION,
    pub crop_yoffset_set: crate::src::transupp::JCROP_CODE,
    pub num_components: libc::c_int,
    pub workspace_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
    pub output_width: crate::jmorecfg_h::JDIMENSION,
    pub output_height: crate::jmorecfg_h::JDIMENSION,
    pub x_crop_offset: crate::jmorecfg_h::JDIMENSION,
    pub y_crop_offset: crate::jmorecfg_h::JDIMENSION,
    pub iMCU_sample_width: libc::c_int,
    pub iMCU_sample_height: libc::c_int,
}

pub type JCOPY_OPTION = libc::c_uint;

pub const JXFORM_ROT_270: crate::src::transupp::JXFORM_CODE = 7;

pub const JXFORM_ROT_180: crate::src::transupp::JXFORM_CODE = 6;

pub const JXFORM_ROT_90: crate::src::transupp::JXFORM_CODE = 5;

pub const JXFORM_TRANSVERSE: crate::src::transupp::JXFORM_CODE = 4;

pub const JXFORM_TRANSPOSE: crate::src::transupp::JXFORM_CODE = 3;

pub const JXFORM_FLIP_V: crate::src::transupp::JXFORM_CODE = 2;

pub const JXFORM_FLIP_H: crate::src::transupp::JXFORM_CODE = 1;

pub const JXFORM_NONE: crate::src::transupp::JXFORM_CODE = 0;

pub const JCROP_FORCE: crate::src::transupp::JCROP_CODE = 3;

pub const JCROP_NEG: crate::src::transupp::JCROP_CODE = 2;

pub const JCROP_POS: crate::src::transupp::JCROP_CODE = 1;

pub const JCROP_UNSET: crate::src::transupp::JCROP_CODE = 0;

pub const JCOPYOPT_ALL_EXCEPT_ICC: crate::src::transupp::JCOPY_OPTION = 3;

pub const JCOPYOPT_ALL: crate::src::transupp::JCOPY_OPTION = 2;

pub const JCOPYOPT_COMMENTS: crate::src::transupp::JCOPY_OPTION = 1;

pub const JCOPYOPT_NONE: crate::src::transupp::JCOPY_OPTION = 0;
/* jtransform_execute_transform used to be called
 * jtransform_execute_transformation, but some compilers complain about
 * routine names that long.  This macro is here to avoid breaking any
 * old source code that uses the original name...
 */

pub const jtransform_execute_transformation: unsafe extern "C" fn(
    _: crate::jpeglib_h::j_decompress_ptr,
    _: crate::jpeglib_h::j_compress_ptr,
    _: *mut crate::jpeglib_h::jvirt_barray_ptr,
    _: *mut crate::src::transupp::jpeg_transform_info,
) -> () = crate::src::transupp::jtransform_execute_transform;

pub const JCOPYOPT_DEFAULT: libc::c_int = crate::src::transupp::JCOPYOPT_COMMENTS as libc::c_int;
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

pub const dstinfo_min_DCT_h_scaled_size: libc::c_int = crate::jpeglib_h::DCTSIZE;

pub const dstinfo_min_DCT_v_scaled_size: libc::c_int = crate::jpeglib_h::DCTSIZE;
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
    mut srcinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dstinfo: crate::jpeglib_h::j_compress_ptr,
    mut x_crop_offset: crate::jmorecfg_h::JDIMENSION,
    mut y_crop_offset: crate::jmorecfg_h::JDIMENSION,
    mut src_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
    mut dst_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
)
/* Crop.  This is only used when no rotate/flip is requested with the crop. */
{
    
    
    
    
    
    
    
     
    /* We simply have to copy the right amount of data (the destination's
     * image size) starting at the given X and Y offsets in the source.
     */
     let mut ci:   libc::c_int =  0i32;
    while ci < (*dstinfo).num_components {
            
        
        
         let mut compptr:   *mut crate::jpeglib_h::jpeg_component_info =
     (*dstinfo).comp_info.offset(ci as isize); let mut x_crop_blocks:   crate::jmorecfg_h::JDIMENSION =
      x_crop_offset * (*compptr).h_samp_factor as libc::c_uint; let mut y_crop_blocks:   crate::jmorecfg_h::JDIMENSION =
      y_crop_offset * (*compptr).v_samp_factor as libc::c_uint; let mut dst_blk_y:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while dst_blk_y < (*compptr).height_in_blocks {
               
            
             let mut dst_buffer:   crate::jpeglib_h::JBLOCKARRAY =
     Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as crate::jpeglib_h::j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
                crate::jmorecfg_h::TRUE,
            ); let mut src_buffer:   crate::jpeglib_h::JBLOCKARRAY =
     Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as crate::jpeglib_h::j_common_ptr,
                *src_coef_arrays.offset(ci as isize),
                
                dst_blk_y + y_crop_blocks,
                (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
                crate::jmorecfg_h::FALSE,
            ); let mut offset_y:   libc::c_int =  0i32;
            while offset_y < (*compptr).v_samp_factor {
                crate::jpegint_h::jcopy_block_row(
                    (*src_buffer.offset(offset_y as isize)).offset(x_crop_blocks as isize),
                    *dst_buffer.offset(offset_y as isize),
                    (*compptr).width_in_blocks,
                );
                offset_y += 1
            }
            dst_blk_y = dst_blk_y + (*compptr).v_samp_factor as libc::c_uint
        }
        ci += 1
    }
}

unsafe extern "C" fn do_flip_h_no_crop(
    mut srcinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dstinfo: crate::jpeglib_h::j_compress_ptr,
    mut x_crop_offset: crate::jmorecfg_h::JDIMENSION,
    mut src_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
)
/* Horizontal flip; done in-place, so no separate dest array is required.
 * NB: this only works when y_crop_offset is zero.
 */
{
    
    
    
    
    
    
    
    
    
    
    
    
    
      
    /* Horizontal mirroring of DCT blocks is accomplished by swapping
     * pairs of blocks in-place.  Within a DCT block, we perform horizontal
     * mirroring by changing the signs of odd-numbered columns.
     * Partial iMCUs at the right edge are left untouched.
     */
    
     let mut MCU_cols:   crate::jmorecfg_h::JDIMENSION =
      (*srcinfo).output_width /
    
        ((*dstinfo).max_h_samp_factor * dstinfo_min_DCT_h_scaled_size) as libc::c_uint; let mut ci:   libc::c_int =  0i32;
    while ci < (*dstinfo).num_components {
            
        
        
         let mut compptr:   *mut crate::jpeglib_h::jpeg_component_info =
     (*dstinfo).comp_info.offset(ci as isize); let mut comp_width:   crate::jmorecfg_h::JDIMENSION =
      MCU_cols * (*compptr).h_samp_factor as libc::c_uint; let mut x_crop_blocks:   crate::jmorecfg_h::JDIMENSION =
      x_crop_offset * (*compptr).h_samp_factor as libc::c_uint; let mut blk_y:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while blk_y < (*compptr).height_in_blocks {
              
             let mut buffer:   crate::jpeglib_h::JBLOCKARRAY =
     Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as crate::jpeglib_h::j_common_ptr,
                *src_coef_arrays.offset(ci as isize),
                blk_y,
                (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
                crate::jmorecfg_h::TRUE,
            ); let mut offset_y:   libc::c_int =  0i32;
            while offset_y < (*compptr).v_samp_factor {
                 let mut blk_x:   crate::jmorecfg_h::JDIMENSION =  0u32;
                while  blk_x * 2u32 < comp_width {
                       
                    
                     let mut ptr1:   crate::jpeglib_h::JCOEFPTR =
    
                        (*(*buffer.offset(offset_y as isize)).offset(blk_x as isize)).as_mut_ptr(); let mut ptr2:   crate::jpeglib_h::JCOEFPTR =
     (*(*buffer.offset(offset_y as isize)).offset(
                        (
                        comp_width - blk_x - 1u32) as isize,
                    ))
                    .as_mut_ptr(); let mut k:   libc::c_int =  0i32; /* swap even column */
                    while k < crate::jpeglib_h::DCTSIZE2 {
                           /* swap odd column with sign change */
                         let mut temp1:   crate::jmorecfg_h::JCOEF =  *ptr1; let mut temp2:   crate::jmorecfg_h::JCOEF =  *ptr2;
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
                        *fresh2 = -(temp2 as libc::c_int) as crate::jmorecfg_h::JCOEF;
                        let fresh3 = ptr2;
                        ptr2 = ptr2.offset(1);
                        *fresh3 = -(temp1 as libc::c_int) as crate::jmorecfg_h::JCOEF;
                        k += 2i32
                    }
                    blk_x =  blk_x + 1
                }
                if x_crop_blocks > 0u32 {
                    /* Now left-justify the portion of the data to be kept.
                     * We can't use a single jcopy_block_row() call because that routine
                     * depends on memcpy(), whose behavior is unspecified for overlapping
                     * source and destination areas.  Sigh.
                     */
                    blk_x = 0u32;
                    while blk_x < (*compptr).width_in_blocks {
                        crate::jpegint_h::jcopy_block_row(
                            (*buffer.offset(offset_y as isize))
                                .offset(blk_x as isize)
                                .offset(x_crop_blocks as isize),
                            (*buffer.offset(offset_y as isize)).offset(blk_x as isize),
                            1u32,
                        );
                        blk_x =  blk_x + 1
                    }
                }
                offset_y += 1
            }
            blk_y = blk_y + (*compptr).v_samp_factor as libc::c_uint
        }
        ci += 1
    }
}

unsafe extern "C" fn do_flip_h(
    mut srcinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dstinfo: crate::jpeglib_h::j_compress_ptr,
    mut x_crop_offset: crate::jmorecfg_h::JDIMENSION,
    mut y_crop_offset: crate::jmorecfg_h::JDIMENSION,
    mut src_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
    mut dst_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
)
/* Horizontal flip in general cropping case */
{
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
      
    /* Here we must output into a separate array because we can't touch
     * different rows of a single virtual array simultaneously.  Otherwise,
     * this is essentially the same as the routine above.
     */
    
     let mut MCU_cols:   crate::jmorecfg_h::JDIMENSION =
      (*srcinfo).output_width /
    
        ((*dstinfo).max_h_samp_factor * dstinfo_min_DCT_h_scaled_size) as libc::c_uint; let mut ci:   libc::c_int =  0i32;
    while ci < (*dstinfo).num_components {
             
        
        
        
         let mut compptr:   *mut crate::jpeglib_h::jpeg_component_info =
     (*dstinfo).comp_info.offset(ci as isize); let mut comp_width:   crate::jmorecfg_h::JDIMENSION =
      MCU_cols * (*compptr).h_samp_factor as libc::c_uint; let mut x_crop_blocks:   crate::jmorecfg_h::JDIMENSION =
      x_crop_offset * (*compptr).h_samp_factor as libc::c_uint; let mut y_crop_blocks:   crate::jmorecfg_h::JDIMENSION =
      y_crop_offset * (*compptr).v_samp_factor as libc::c_uint; let mut dst_blk_y:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while dst_blk_y < (*compptr).height_in_blocks {
               
            
             let mut dst_buffer:   crate::jpeglib_h::JBLOCKARRAY =
     Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as crate::jpeglib_h::j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
                crate::jmorecfg_h::TRUE,
            ); let mut src_buffer:   crate::jpeglib_h::JBLOCKARRAY =
     Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as crate::jpeglib_h::j_common_ptr,
                *src_coef_arrays.offset(ci as isize),
                
                dst_blk_y + y_crop_blocks,
                (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
                crate::jmorecfg_h::FALSE,
            ); let mut offset_y:   libc::c_int =  0i32;
            while offset_y < (*compptr).v_samp_factor {
                   
                
                 let mut dst_row_ptr:   crate::jpeglib_h::JBLOCKROW =
     *dst_buffer.offset(offset_y as isize); let mut src_row_ptr:   crate::jpeglib_h::JBLOCKROW =
     *src_buffer.offset(offset_y as isize); let mut dst_blk_x:   crate::jmorecfg_h::JDIMENSION =  0u32;
                while dst_blk_x < (*compptr).width_in_blocks {
                    if  x_crop_blocks + dst_blk_x < comp_width {
                          
                        
                         let mut dst_ptr:   crate::jpeglib_h::JCOEFPTR =
     (*dst_row_ptr.offset(dst_blk_x as isize)).as_mut_ptr(); let mut src_ptr:   crate::jpeglib_h::JCOEFPTR =
     (*src_row_ptr.offset(
                            (
                            comp_width - x_crop_blocks - dst_blk_x - 1u32)
                                as isize,
                        ))
                        .as_mut_ptr(); let mut k:   libc::c_int =  0i32; /* copy even column */
                        while k < crate::jpeglib_h::DCTSIZE2 {
                            let fresh4 = src_ptr;
                            src_ptr = src_ptr.offset(1);
                            let fresh5 = dst_ptr;
                            dst_ptr = dst_ptr.offset(1);
                            *fresh5 = *fresh4;
                            let fresh6 = src_ptr;
                            src_ptr = src_ptr.offset(1);
                            let fresh7 = dst_ptr;
                            dst_ptr = dst_ptr.offset(1);
                            *fresh7 = -(*fresh6 as libc::c_int) as crate::jmorecfg_h::JCOEF;
                            k += 2i32
                            /* copy odd column with sign change */
                        }
                    } else {
                        /* Copy last partial block(s) verbatim */
                        crate::jpegint_h::jcopy_block_row(
                            src_row_ptr
                                .offset(dst_blk_x as isize)
                                .offset(x_crop_blocks as isize),
                            dst_row_ptr.offset(dst_blk_x as isize),
                            1u32,
                        );
                    }
                    dst_blk_x =  dst_blk_x + 1
                }
                offset_y += 1
            }
            dst_blk_y = dst_blk_y + (*compptr).v_samp_factor as libc::c_uint
        }
        ci += 1
    }
}

unsafe extern "C" fn do_flip_v(
    mut srcinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dstinfo: crate::jpeglib_h::j_compress_ptr,
    mut x_crop_offset: crate::jmorecfg_h::JDIMENSION,
    mut y_crop_offset: crate::jmorecfg_h::JDIMENSION,
    mut src_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
    mut dst_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
)
/* Vertical flip */
{
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
      
    /* We output into a separate array because we can't touch different
     * rows of the source virtual array simultaneously.  Otherwise, this
     * is a pretty straightforward analog of horizontal flip.
     * Within a DCT block, vertical mirroring is done by changing the signs
     * of odd-numbered rows.
     * Partial iMCUs at the bottom edge are copied verbatim.
     */
    
     let mut MCU_rows:   crate::jmorecfg_h::JDIMENSION =
      (*srcinfo).output_height /
    
        ((*dstinfo).max_v_samp_factor * dstinfo_min_DCT_v_scaled_size) as libc::c_uint; let mut ci:   libc::c_int =  0i32;
    while ci < (*dstinfo).num_components {
             
        
        
        
         let mut compptr:   *mut crate::jpeglib_h::jpeg_component_info =
     (*dstinfo).comp_info.offset(ci as isize); let mut comp_height:   crate::jmorecfg_h::JDIMENSION =
      MCU_rows * (*compptr).v_samp_factor as libc::c_uint; let mut x_crop_blocks:   crate::jmorecfg_h::JDIMENSION =
      x_crop_offset * (*compptr).h_samp_factor as libc::c_uint; let mut y_crop_blocks:   crate::jmorecfg_h::JDIMENSION =
      y_crop_offset * (*compptr).v_samp_factor as libc::c_uint; let mut dst_blk_y:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while dst_blk_y < (*compptr).height_in_blocks {
              let mut src_buffer:  crate::jpeglib_h::JBLOCKARRAY =
     ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCKROW>();  let mut dst_buffer:   crate::jpeglib_h::JBLOCKARRAY =
     Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as crate::jpeglib_h::j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
                crate::jmorecfg_h::TRUE,
            );
            if  y_crop_blocks + dst_blk_y < comp_height {
                /* Row is within the mirrorable area. */
                src_buffer = Some(
                    (*(*srcinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    srcinfo as crate::jpeglib_h::j_common_ptr,
                    *src_coef_arrays.offset(ci as isize),
                    
                    comp_height - y_crop_blocks - dst_blk_y -
    (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
                    (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
                    crate::jmorecfg_h::FALSE,
                )
            } else {
                /* Bottom-edge blocks will be copied verbatim. */
                src_buffer = Some(
                    (*(*srcinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    srcinfo as crate::jpeglib_h::j_common_ptr,
                    *src_coef_arrays.offset(ci as isize),
                    
                    dst_blk_y + y_crop_blocks,
                    (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
                    crate::jmorecfg_h::FALSE,
                )
            }
             let mut offset_y:   libc::c_int =  0i32;
            while offset_y < (*compptr).v_samp_factor {
                if  y_crop_blocks + dst_blk_y < comp_height {
                      
                     let mut dst_row_ptr:   crate::jpeglib_h::JBLOCKROW =
     *dst_buffer.offset(offset_y as isize); let mut src_row_ptr:   crate::jpeglib_h::JBLOCKROW =
    
                        *src_buffer.offset(((*compptr).v_samp_factor - offset_y - 1i32) as isize);
                    src_row_ptr = src_row_ptr.offset(x_crop_blocks as isize);
                     let mut dst_blk_x:   crate::jmorecfg_h::JDIMENSION =  0u32;
                    while dst_blk_x < (*compptr).width_in_blocks {
                           
                        
                         let mut dst_ptr:   crate::jpeglib_h::JCOEFPTR =
     (*dst_row_ptr.offset(dst_blk_x as isize)).as_mut_ptr(); let mut src_ptr:   crate::jpeglib_h::JCOEFPTR =
     (*src_row_ptr.offset(dst_blk_x as isize)).as_mut_ptr(); let mut i:   libc::c_int =  0i32;
                        while i < crate::jpeglib_h::DCTSIZE {
                             let mut j:   libc::c_int =  0i32;
                            while j < crate::jpeglib_h::DCTSIZE {
                                let fresh8 = src_ptr;
                                src_ptr = src_ptr.offset(1);
                                let fresh9 = dst_ptr;
                                dst_ptr = dst_ptr.offset(1);
                                *fresh9 = *fresh8;
                                j += 1
                            }
                            /* copy odd row with sign change */
                            j = 0i32;
                            while j < crate::jpeglib_h::DCTSIZE {
                                let fresh10 = src_ptr;
                                src_ptr = src_ptr.offset(1);
                                let fresh11 = dst_ptr;
                                dst_ptr = dst_ptr.offset(1);
                                *fresh11 = -(*fresh10 as libc::c_int) as crate::jmorecfg_h::JCOEF;
                                j += 1
                            }
                            i += 2i32
                        }
                        dst_blk_x =  dst_blk_x + 1
                    }
                } else {
                    /* Just copy row verbatim. */
                    crate::jpegint_h::jcopy_block_row(
                        (*src_buffer.offset(offset_y as isize)).offset(x_crop_blocks as isize),
                        *dst_buffer.offset(offset_y as isize),
                        (*compptr).width_in_blocks,
                    );
                }
                offset_y += 1
            }
            dst_blk_y = dst_blk_y + (*compptr).v_samp_factor as libc::c_uint
        }
        ci += 1
    }
}

unsafe extern "C" fn do_transpose(
    mut srcinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dstinfo: crate::jpeglib_h::j_compress_ptr,
    mut x_crop_offset: crate::jmorecfg_h::JDIMENSION,
    mut y_crop_offset: crate::jmorecfg_h::JDIMENSION,
    mut src_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
    mut dst_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
)
/* Transpose source into destination */
{
    
    
    
    
    
    
    
    
    
    
    
    
    
     
    /* Transposing pixels within a block just requires transposing the
     * DCT coefficients.
     * Partial iMCUs at the edges require no special treatment; we simply
     * process all the available DCT blocks for every component.
     */
     let mut ci:   libc::c_int =  0i32;
    while ci < (*dstinfo).num_components {
            
        
        
         let mut compptr:   *mut crate::jpeglib_h::jpeg_component_info =
     (*dstinfo).comp_info.offset(ci as isize); let mut x_crop_blocks:   crate::jmorecfg_h::JDIMENSION =
      x_crop_offset * (*compptr).h_samp_factor as libc::c_uint; let mut y_crop_blocks:   crate::jmorecfg_h::JDIMENSION =
      y_crop_offset * (*compptr).v_samp_factor as libc::c_uint; let mut dst_blk_y:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while dst_blk_y < (*compptr).height_in_blocks {
              
             let mut dst_buffer:   crate::jpeglib_h::JBLOCKARRAY =
     Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as crate::jpeglib_h::j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
                crate::jmorecfg_h::TRUE,
            ); let mut offset_y:   libc::c_int =  0i32;
            while offset_y < (*compptr).v_samp_factor {
                  let mut dst_blk_x:   crate::jmorecfg_h::JDIMENSION =  0u32;
                while dst_blk_x < (*compptr).width_in_blocks {
                      
                     let mut src_buffer:   crate::jpeglib_h::JBLOCKARRAY =
     Some(
                        (*(*srcinfo).mem)
                            .access_virt_barray
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        srcinfo as crate::jpeglib_h::j_common_ptr,
                        *src_coef_arrays.offset(ci as isize),
                        
                        dst_blk_x + x_crop_blocks,
                        (*compptr).h_samp_factor as crate::jmorecfg_h::JDIMENSION,
                        crate::jmorecfg_h::FALSE,
                    ); let mut offset_x:   libc::c_int =  0i32;
                    while offset_x < (*compptr).h_samp_factor {
                           
                        
                         let mut dst_ptr:   crate::jpeglib_h::JCOEFPTR =
     (*(*dst_buffer.offset(offset_y as isize))
                            .offset((dst_blk_x + offset_x as libc::c_uint) as isize))
                        .as_mut_ptr(); let mut src_ptr:   crate::jpeglib_h::JCOEFPTR =
     (*(*src_buffer.offset(offset_x as isize)).offset(
                            (
                            dst_blk_y + offset_y as libc::c_uint + y_crop_blocks) as isize,
                        ))
                        .as_mut_ptr(); let mut i:   libc::c_int =  0i32;
                        while i < crate::jpeglib_h::DCTSIZE {
                              let mut j:   libc::c_int =  0i32;
                            while j < crate::jpeglib_h::DCTSIZE {
                                *dst_ptr.offset((j * crate::jpeglib_h::DCTSIZE + i) as isize) =
                                    *src_ptr.offset((i * crate::jpeglib_h::DCTSIZE + j) as isize);
                                j += 1
                            }
                            i += 1
                        }
                        offset_x += 1
                    }
                    dst_blk_x = dst_blk_x + (*compptr).h_samp_factor as libc::c_uint
                }
                offset_y += 1
            }
            dst_blk_y = dst_blk_y + (*compptr).v_samp_factor as libc::c_uint
        }
        ci += 1
    }
}

unsafe extern "C" fn do_rot_90(
    mut srcinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dstinfo: crate::jpeglib_h::j_compress_ptr,
    mut x_crop_offset: crate::jmorecfg_h::JDIMENSION,
    mut y_crop_offset: crate::jmorecfg_h::JDIMENSION,
    mut src_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
    mut dst_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
)
/* 90 degree rotation is equivalent to
 *   1. Transposing the image;
 *   2. Horizontal mirroring.
 * These two steps are merged into a single processing routine.
 */
{
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
      
    /* Because of the horizontal mirror step, we can't process partial iMCUs
     * at the (output) right edge properly.  They just get transposed and
     * not mirrored.
     */
    
     let mut MCU_cols:   crate::jmorecfg_h::JDIMENSION =
      (*srcinfo).output_height /
    
        ((*dstinfo).max_h_samp_factor * dstinfo_min_DCT_h_scaled_size) as libc::c_uint; let mut ci:   libc::c_int =  0i32;
    while ci < (*dstinfo).num_components {
             
        
        
        
         let mut compptr:   *mut crate::jpeglib_h::jpeg_component_info =
     (*dstinfo).comp_info.offset(ci as isize); let mut comp_width:   crate::jmorecfg_h::JDIMENSION =
      MCU_cols * (*compptr).h_samp_factor as libc::c_uint; let mut x_crop_blocks:   crate::jmorecfg_h::JDIMENSION =
      x_crop_offset * (*compptr).h_samp_factor as libc::c_uint; let mut y_crop_blocks:   crate::jmorecfg_h::JDIMENSION =
      y_crop_offset * (*compptr).v_samp_factor as libc::c_uint; let mut dst_blk_y:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while dst_blk_y < (*compptr).height_in_blocks {
              
             let mut dst_buffer:   crate::jpeglib_h::JBLOCKARRAY =
     Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as crate::jpeglib_h::j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
                crate::jmorecfg_h::TRUE,
            ); let mut offset_y:   libc::c_int =  0i32;
            while offset_y < (*compptr).v_samp_factor {
                  let mut dst_blk_x:   crate::jmorecfg_h::JDIMENSION =  0u32;
                while dst_blk_x < (*compptr).width_in_blocks {
                      let mut src_buffer:  crate::jpeglib_h::JBLOCKARRAY =
     ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCKROW>();if  x_crop_blocks + dst_blk_x < comp_width {
                        /* Block is within the mirrorable area. */
                        src_buffer = Some(
                            (*(*srcinfo).mem)
                                .access_virt_barray
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            srcinfo as crate::jpeglib_h::j_common_ptr,
                            *src_coef_arrays.offset(ci as isize),
                            
                            comp_width - x_crop_blocks - dst_blk_x -
    
                                    (*compptr).h_samp_factor as crate::jmorecfg_h::JDIMENSION,
                            (*compptr).h_samp_factor as crate::jmorecfg_h::JDIMENSION,
                            crate::jmorecfg_h::FALSE,
                        )
                    } else {
                        /* Edge blocks are transposed but not mirrored. */
                        src_buffer = Some(
                            (*(*srcinfo).mem)
                                .access_virt_barray
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            srcinfo as crate::jpeglib_h::j_common_ptr,
                            *src_coef_arrays.offset(ci as isize),
                            
                            dst_blk_x + x_crop_blocks,
                            (*compptr).h_samp_factor as crate::jmorecfg_h::JDIMENSION,
                            crate::jmorecfg_h::FALSE,
                        )
                    }
                     let mut offset_x:   libc::c_int =  0i32;
                    while offset_x < (*compptr).h_samp_factor {
                         let mut i:  libc::c_int =  0; let mut j:  libc::c_int =  0; let mut src_ptr:  crate::jpeglib_h::JCOEFPTR =
     ::std::ptr::null_mut::< crate::jmorecfg_h::JCOEF>();  let mut dst_ptr:   crate::jpeglib_h::JCOEFPTR =
     (*(*dst_buffer.offset(offset_y as isize))
                            .offset((dst_blk_x + offset_x as libc::c_uint) as isize))
                        .as_mut_ptr();
                        if  x_crop_blocks + dst_blk_x < comp_width {
                            /* Block is within the mirrorable area. */
                            src_ptr = (*(*src_buffer
                                .offset(((*compptr).h_samp_factor - offset_x - 1i32) as isize))
                            .offset(
                                (
                                dst_blk_y + offset_y as libc::c_uint + y_crop_blocks)
                                    as isize,
                            ))
                            .as_mut_ptr();
                            i = 0i32;
                            while i < crate::jpeglib_h::DCTSIZE {
                                j = 0i32;
                                while j < crate::jpeglib_h::DCTSIZE {
                                    *dst_ptr.offset((j * crate::jpeglib_h::DCTSIZE + i) as isize) =
                                        *src_ptr
                                            .offset((i * crate::jpeglib_h::DCTSIZE + j) as isize);
                                    j += 1
                                }
                                i += 1;
                                j = 0i32;
                                while j < crate::jpeglib_h::DCTSIZE {
                                    *dst_ptr.offset((j * crate::jpeglib_h::DCTSIZE + i) as isize) =
                                        -(*src_ptr
                                            .offset((i * crate::jpeglib_h::DCTSIZE + j) as isize)
                                            as libc::c_int)
                                            as crate::jmorecfg_h::JCOEF;
                                    j += 1
                                }
                                i += 1
                            }
                        } else {
                            /* Edge blocks are transposed but not mirrored. */
                            src_ptr = (*(*src_buffer.offset(offset_x as isize)).offset(
                                (
                                dst_blk_y + offset_y as libc::c_uint + y_crop_blocks)
                                    as isize,
                            ))
                            .as_mut_ptr();
                            i = 0i32;
                            while i < crate::jpeglib_h::DCTSIZE {
                                j = 0i32;
                                while j < crate::jpeglib_h::DCTSIZE {
                                    *dst_ptr.offset((j * crate::jpeglib_h::DCTSIZE + i) as isize) =
                                        *src_ptr
                                            .offset((i * crate::jpeglib_h::DCTSIZE + j) as isize);
                                    j += 1
                                }
                                i += 1
                            }
                        }
                        offset_x += 1
                    }
                    dst_blk_x = dst_blk_x + (*compptr).h_samp_factor as libc::c_uint
                }
                offset_y += 1
            }
            dst_blk_y = dst_blk_y + (*compptr).v_samp_factor as libc::c_uint
        }
        ci += 1
    }
}

unsafe extern "C" fn do_rot_270(
    mut srcinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dstinfo: crate::jpeglib_h::j_compress_ptr,
    mut x_crop_offset: crate::jmorecfg_h::JDIMENSION,
    mut y_crop_offset: crate::jmorecfg_h::JDIMENSION,
    mut src_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
    mut dst_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
)
/* 270 degree rotation is equivalent to
 *   1. Horizontal mirroring;
 *   2. Transposing the image.
 * These two steps are merged into a single processing routine.
 */
{
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
      
    /* Because of the horizontal mirror step, we can't process partial iMCUs
     * at the (output) bottom edge properly.  They just get transposed and
     * not mirrored.
     */
    
     let mut MCU_rows:   crate::jmorecfg_h::JDIMENSION =
      (*srcinfo).output_width /
    
        ((*dstinfo).max_v_samp_factor * dstinfo_min_DCT_v_scaled_size) as libc::c_uint; let mut ci:   libc::c_int =  0i32;
    while ci < (*dstinfo).num_components {
             
        
        
        
         let mut compptr:   *mut crate::jpeglib_h::jpeg_component_info =
     (*dstinfo).comp_info.offset(ci as isize); let mut comp_height:   crate::jmorecfg_h::JDIMENSION =
      MCU_rows * (*compptr).v_samp_factor as libc::c_uint; let mut x_crop_blocks:   crate::jmorecfg_h::JDIMENSION =
      x_crop_offset * (*compptr).h_samp_factor as libc::c_uint; let mut y_crop_blocks:   crate::jmorecfg_h::JDIMENSION =
      y_crop_offset * (*compptr).v_samp_factor as libc::c_uint; let mut dst_blk_y:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while dst_blk_y < (*compptr).height_in_blocks {
              
             let mut dst_buffer:   crate::jpeglib_h::JBLOCKARRAY =
     Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as crate::jpeglib_h::j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
                crate::jmorecfg_h::TRUE,
            ); let mut offset_y:   libc::c_int =  0i32;
            while offset_y < (*compptr).v_samp_factor {
                  let mut dst_blk_x:   crate::jmorecfg_h::JDIMENSION =  0u32;
                while dst_blk_x < (*compptr).width_in_blocks {
                      
                     let mut src_buffer:   crate::jpeglib_h::JBLOCKARRAY =
     Some(
                        (*(*srcinfo).mem)
                            .access_virt_barray
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        srcinfo as crate::jpeglib_h::j_common_ptr,
                        *src_coef_arrays.offset(ci as isize),
                        
                        dst_blk_x + x_crop_blocks,
                        (*compptr).h_samp_factor as crate::jmorecfg_h::JDIMENSION,
                        crate::jmorecfg_h::FALSE,
                    ); let mut offset_x:   libc::c_int =  0i32;
                    while offset_x < (*compptr).h_samp_factor {
                         let mut i:  libc::c_int =  0; let mut j:  libc::c_int =  0; let mut src_ptr:  crate::jpeglib_h::JCOEFPTR =
     ::std::ptr::null_mut::< crate::jmorecfg_h::JCOEF>();  let mut dst_ptr:   crate::jpeglib_h::JCOEFPTR =
     (*(*dst_buffer.offset(offset_y as isize))
                            .offset((dst_blk_x + offset_x as libc::c_uint) as isize))
                        .as_mut_ptr();
                        if  y_crop_blocks + dst_blk_y < comp_height {
                            /* Block is within the mirrorable area. */
                            src_ptr = (*(*src_buffer.offset(offset_x as isize)).offset(
                                (
                                comp_height - y_crop_blocks - dst_blk_y - offset_y as libc::c_uint -
    1u32)
                                    as isize,
                            ))
                            .as_mut_ptr();
                            i = 0i32;
                            while i < crate::jpeglib_h::DCTSIZE {
                                j = 0i32;
                                while j < crate::jpeglib_h::DCTSIZE {
                                    *dst_ptr.offset((j * crate::jpeglib_h::DCTSIZE + i) as isize) =
                                        *src_ptr
                                            .offset((i * crate::jpeglib_h::DCTSIZE + j) as isize);
                                    j += 1;
                                    *dst_ptr.offset((j * crate::jpeglib_h::DCTSIZE + i) as isize) =
                                        -(*src_ptr
                                            .offset((i * crate::jpeglib_h::DCTSIZE + j) as isize)
                                            as libc::c_int)
                                            as crate::jmorecfg_h::JCOEF;
                                    j += 1
                                }
                                i += 1
                            }
                        } else {
                            /* Edge blocks are transposed but not mirrored. */
                            src_ptr = (*(*src_buffer.offset(offset_x as isize)).offset(
                                (
                                dst_blk_y + offset_y as libc::c_uint + y_crop_blocks)
                                    as isize,
                            ))
                            .as_mut_ptr();
                            i = 0i32;
                            while i < crate::jpeglib_h::DCTSIZE {
                                j = 0i32;
                                while j < crate::jpeglib_h::DCTSIZE {
                                    *dst_ptr.offset((j * crate::jpeglib_h::DCTSIZE + i) as isize) =
                                        *src_ptr
                                            .offset((i * crate::jpeglib_h::DCTSIZE + j) as isize);
                                    j += 1
                                }
                                i += 1
                            }
                        }
                        offset_x += 1
                    }
                    dst_blk_x = dst_blk_x + (*compptr).h_samp_factor as libc::c_uint
                }
                offset_y += 1
            }
            dst_blk_y = dst_blk_y + (*compptr).v_samp_factor as libc::c_uint
        }
        ci += 1
    }
}

unsafe extern "C" fn do_rot_180(
    mut srcinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dstinfo: crate::jpeglib_h::j_compress_ptr,
    mut x_crop_offset: crate::jmorecfg_h::JDIMENSION,
    mut y_crop_offset: crate::jmorecfg_h::JDIMENSION,
    mut src_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
    mut dst_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
)
/* 180 degree rotation is equivalent to
 *   1. Vertical mirroring;
 *   2. Horizontal mirroring.
 * These two steps are merged into a single processing routine.
 */
{
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
       
    
    
     let mut MCU_cols:   crate::jmorecfg_h::JDIMENSION =
      (*srcinfo).output_width /
    
        ((*dstinfo).max_h_samp_factor * dstinfo_min_DCT_h_scaled_size) as libc::c_uint; let mut MCU_rows:   crate::jmorecfg_h::JDIMENSION =
      (*srcinfo).output_height /
    
        ((*dstinfo).max_v_samp_factor * dstinfo_min_DCT_v_scaled_size) as libc::c_uint; let mut ci:   libc::c_int =  0i32;
    while ci < (*dstinfo).num_components {
              
        
        
        
        
         let mut compptr:   *mut crate::jpeglib_h::jpeg_component_info =
     (*dstinfo).comp_info.offset(ci as isize); let mut comp_width:   crate::jmorecfg_h::JDIMENSION =
      MCU_cols * (*compptr).h_samp_factor as libc::c_uint; let mut comp_height:   crate::jmorecfg_h::JDIMENSION =
      MCU_rows * (*compptr).v_samp_factor as libc::c_uint; let mut x_crop_blocks:   crate::jmorecfg_h::JDIMENSION =
      x_crop_offset * (*compptr).h_samp_factor as libc::c_uint; let mut y_crop_blocks:   crate::jmorecfg_h::JDIMENSION =
      y_crop_offset * (*compptr).v_samp_factor as libc::c_uint; let mut dst_blk_y:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while dst_blk_y < (*compptr).height_in_blocks {
              let mut src_buffer:  crate::jpeglib_h::JBLOCKARRAY =
     ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCKROW>();  let mut dst_buffer:   crate::jpeglib_h::JBLOCKARRAY =
     Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as crate::jpeglib_h::j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
                crate::jmorecfg_h::TRUE,
            );
            if  y_crop_blocks + dst_blk_y < comp_height {
                /* Row is within the vertically mirrorable area. */
                src_buffer = Some(
                    (*(*srcinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    srcinfo as crate::jpeglib_h::j_common_ptr,
                    *src_coef_arrays.offset(ci as isize),
                    
                    comp_height - y_crop_blocks - dst_blk_y -
    (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
                    (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
                    crate::jmorecfg_h::FALSE,
                )
            } else {
                /* Bottom-edge rows are only mirrored horizontally. */
                src_buffer = Some(
                    (*(*srcinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    srcinfo as crate::jpeglib_h::j_common_ptr,
                    *src_coef_arrays.offset(ci as isize),
                    
                    dst_blk_y + y_crop_blocks,
                    (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
                    crate::jmorecfg_h::FALSE,
                )
            }
             let mut offset_y:   libc::c_int =  0i32;
            while offset_y < (*compptr).v_samp_factor {
                 let mut dst_blk_x:  crate::jmorecfg_h::JDIMENSION =  0; let mut i:  libc::c_int =  0; let mut src_row_ptr:  crate::jpeglib_h::JBLOCKROW =
     ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCK>();  let mut src_ptr:  crate::jpeglib_h::JCOEFPTR =
     ::std::ptr::null_mut::< crate::jmorecfg_h::JCOEF>(); let mut dst_ptr:  crate::jpeglib_h::JCOEFPTR =
     ::std::ptr::null_mut::< crate::jmorecfg_h::JCOEF>(); let mut dst_row_ptr:   crate::jpeglib_h::JBLOCKROW =
     *dst_buffer.offset(offset_y as isize);
                if  y_crop_blocks + dst_blk_y < comp_height {
                    /* Row is within the mirrorable area. */
                    src_row_ptr =
                        *src_buffer.offset(((*compptr).v_samp_factor - offset_y - 1i32) as isize);
                    dst_blk_x = 0u32;
                    while dst_blk_x < (*compptr).width_in_blocks {
                         let mut j:  libc::c_int =  0;dst_ptr = (*dst_row_ptr.offset(dst_blk_x as isize)).as_mut_ptr();
                        if  x_crop_blocks + dst_blk_x < comp_width {
                            /* Process the blocks that can be mirrored both ways. */
                            src_ptr = (*src_row_ptr.offset(
                                (
                                comp_width - x_crop_blocks - dst_blk_x - 1u32)
                                    as isize,
                            ))
                            .as_mut_ptr();
                            i = 0i32;
                            while i < crate::jpeglib_h::DCTSIZE {
                                /* For even row, negate every odd column. */
                                j = 0i32;
                                while j < crate::jpeglib_h::DCTSIZE {
                                    let fresh12 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    let fresh13 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    *fresh13 = *fresh12;
                                    let fresh14 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    let fresh15 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    *fresh15 =
                                        -(*fresh14 as libc::c_int) as crate::jmorecfg_h::JCOEF;
                                    j += 2i32
                                }
                                /* For odd row, negate every even column. */
                                j = 0i32;
                                while j < crate::jpeglib_h::DCTSIZE {
                                    let fresh16 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    let fresh17 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    *fresh17 =
                                        -(*fresh16 as libc::c_int) as crate::jmorecfg_h::JCOEF;
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
                                .offset((x_crop_blocks + dst_blk_x) as isize))
                            .as_mut_ptr();
                            i = 0i32;
                            while i < crate::jpeglib_h::DCTSIZE {
                                j = 0i32;
                                while j < crate::jpeglib_h::DCTSIZE {
                                    let fresh20 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    let fresh21 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    *fresh21 = *fresh20;
                                    j += 1
                                }
                                j = 0i32;
                                while j < crate::jpeglib_h::DCTSIZE {
                                    let fresh22 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    let fresh23 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    *fresh23 =
                                        -(*fresh22 as libc::c_int) as crate::jmorecfg_h::JCOEF;
                                    j += 1
                                }
                                i += 2i32
                            }
                        }
                        dst_blk_x =  dst_blk_x + 1
                    }
                } else {
                    /* Remaining rows are just mirrored horizontally. */
                    src_row_ptr = *src_buffer.offset(offset_y as isize);
                    dst_blk_x = 0u32;
                    while dst_blk_x < (*compptr).width_in_blocks {
                        if  x_crop_blocks + dst_blk_x < comp_width {
                            /* Process the blocks that can be mirrored. */
                            dst_ptr = (*dst_row_ptr.offset(dst_blk_x as isize)).as_mut_ptr();
                            src_ptr = (*src_row_ptr.offset(
                                (
                                comp_width - x_crop_blocks - dst_blk_x - 1u32)
                                    as isize,
                            ))
                            .as_mut_ptr();
                            i = 0i32;
                            while i < crate::jpeglib_h::DCTSIZE2 {
                                let fresh24 = src_ptr;
                                src_ptr = src_ptr.offset(1);
                                let fresh25 = dst_ptr;
                                dst_ptr = dst_ptr.offset(1);
                                *fresh25 = *fresh24;
                                let fresh26 = src_ptr;
                                src_ptr = src_ptr.offset(1);
                                let fresh27 = dst_ptr;
                                dst_ptr = dst_ptr.offset(1);
                                *fresh27 = -(*fresh26 as libc::c_int) as crate::jmorecfg_h::JCOEF;
                                i += 2i32
                            }
                        } else {
                            /* Any remaining right-edge blocks are only copied. */
                            crate::jpegint_h::jcopy_block_row(
                                src_row_ptr
                                    .offset(dst_blk_x as isize)
                                    .offset(x_crop_blocks as isize),
                                dst_row_ptr.offset(dst_blk_x as isize),
                                1u32,
                            );
                        }
                        dst_blk_x =  dst_blk_x + 1
                    }
                }
                offset_y += 1
            }
            dst_blk_y = dst_blk_y + (*compptr).v_samp_factor as libc::c_uint
        }
        ci += 1
    }
}

unsafe extern "C" fn do_transverse(
    mut srcinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dstinfo: crate::jpeglib_h::j_compress_ptr,
    mut x_crop_offset: crate::jmorecfg_h::JDIMENSION,
    mut y_crop_offset: crate::jmorecfg_h::JDIMENSION,
    mut src_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
    mut dst_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
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
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
       
    
    
     let mut MCU_cols:   crate::jmorecfg_h::JDIMENSION =
      (*srcinfo).output_height /
    
        ((*dstinfo).max_h_samp_factor * dstinfo_min_DCT_h_scaled_size) as libc::c_uint; let mut MCU_rows:   crate::jmorecfg_h::JDIMENSION =
      (*srcinfo).output_width /
    
        ((*dstinfo).max_v_samp_factor * dstinfo_min_DCT_v_scaled_size) as libc::c_uint; let mut ci:   libc::c_int =  0i32;
    while ci < (*dstinfo).num_components {
              
        
        
        
        
         let mut compptr:   *mut crate::jpeglib_h::jpeg_component_info =
     (*dstinfo).comp_info.offset(ci as isize); let mut comp_width:   crate::jmorecfg_h::JDIMENSION =
      MCU_cols * (*compptr).h_samp_factor as libc::c_uint; let mut comp_height:   crate::jmorecfg_h::JDIMENSION =
      MCU_rows * (*compptr).v_samp_factor as libc::c_uint; let mut x_crop_blocks:   crate::jmorecfg_h::JDIMENSION =
      x_crop_offset * (*compptr).h_samp_factor as libc::c_uint; let mut y_crop_blocks:   crate::jmorecfg_h::JDIMENSION =
      y_crop_offset * (*compptr).v_samp_factor as libc::c_uint; let mut dst_blk_y:   crate::jmorecfg_h::JDIMENSION =  0u32;
        while dst_blk_y < (*compptr).height_in_blocks {
              
             let mut dst_buffer:   crate::jpeglib_h::JBLOCKARRAY =
     Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as crate::jpeglib_h::j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
                crate::jmorecfg_h::TRUE,
            ); let mut offset_y:   libc::c_int =  0i32;
            while offset_y < (*compptr).v_samp_factor {
                  let mut dst_blk_x:   crate::jmorecfg_h::JDIMENSION =  0u32;
                while dst_blk_x < (*compptr).width_in_blocks {
                      let mut src_buffer:  crate::jpeglib_h::JBLOCKARRAY =
     ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCKROW>();if  x_crop_blocks + dst_blk_x < comp_width {
                        /* Block is within the mirrorable area. */
                        src_buffer = Some(
                            (*(*srcinfo).mem)
                                .access_virt_barray
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            srcinfo as crate::jpeglib_h::j_common_ptr,
                            *src_coef_arrays.offset(ci as isize),
                            
                            comp_width - x_crop_blocks - dst_blk_x -
    
                                    (*compptr).h_samp_factor as crate::jmorecfg_h::JDIMENSION,
                            (*compptr).h_samp_factor as crate::jmorecfg_h::JDIMENSION,
                            crate::jmorecfg_h::FALSE,
                        )
                    } else {
                        src_buffer = Some(
                            (*(*srcinfo).mem)
                                .access_virt_barray
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            srcinfo as crate::jpeglib_h::j_common_ptr,
                            *src_coef_arrays.offset(ci as isize),
                            
                            dst_blk_x + x_crop_blocks,
                            (*compptr).h_samp_factor as crate::jmorecfg_h::JDIMENSION,
                            crate::jmorecfg_h::FALSE,
                        )
                    }
                     let mut offset_x:   libc::c_int =  0i32;
                    while offset_x < (*compptr).h_samp_factor {
                         let mut i:  libc::c_int =  0; let mut j:  libc::c_int =  0; let mut src_ptr:  crate::jpeglib_h::JCOEFPTR =
     ::std::ptr::null_mut::< crate::jmorecfg_h::JCOEF>();  let mut dst_ptr:   crate::jpeglib_h::JCOEFPTR =
     (*(*dst_buffer.offset(offset_y as isize))
                            .offset((dst_blk_x + offset_x as libc::c_uint) as isize))
                        .as_mut_ptr();
                        if  y_crop_blocks + dst_blk_y < comp_height {
                            if  x_crop_blocks + dst_blk_x < comp_width {
                                /* Block is within the mirrorable area. */
                                src_ptr = (*(*src_buffer.offset(
                                    ((*compptr).h_samp_factor - offset_x - 1i32) as isize,
                                ))
                                .offset(
                                    (
                                    comp_height - y_crop_blocks - dst_blk_y - offset_y as libc::c_uint -
    1u32)
                                        as isize,
                                ))
                                .as_mut_ptr();
                                i = 0i32;
                                while i < crate::jpeglib_h::DCTSIZE {
                                    j = 0i32;
                                    while j < crate::jpeglib_h::DCTSIZE {
                                        *dst_ptr
                                            .offset((j * crate::jpeglib_h::DCTSIZE + i) as isize) =
                                            *src_ptr.offset(
                                                (i * crate::jpeglib_h::DCTSIZE + j) as isize,
                                            );
                                        j += 1;
                                        *dst_ptr
                                            .offset((j * crate::jpeglib_h::DCTSIZE + i) as isize) =
                                            -(*src_ptr.offset(
                                                (i * crate::jpeglib_h::DCTSIZE + j) as isize,
                                            )
                                                as libc::c_int)
                                                as crate::jmorecfg_h::JCOEF;
                                        j += 1
                                    }
                                    i += 1;
                                    j = 0i32;
                                    while j < crate::jpeglib_h::DCTSIZE {
                                        *dst_ptr
                                            .offset((j * crate::jpeglib_h::DCTSIZE + i) as isize) =
                                            -(*src_ptr.offset(
                                                (i * crate::jpeglib_h::DCTSIZE + j) as isize,
                                            )
                                                as libc::c_int)
                                                as crate::jmorecfg_h::JCOEF;
                                        j += 1;
                                        *dst_ptr
                                            .offset((j * crate::jpeglib_h::DCTSIZE + i) as isize) =
                                            *src_ptr.offset(
                                                (i * crate::jpeglib_h::DCTSIZE + j) as isize,
                                            );
                                        j += 1
                                    }
                                    i += 1
                                }
                            } else {
                                /* Right-edge blocks are mirrored in y only */
                                src_ptr = (*(*src_buffer.offset(offset_x as isize)).offset(
                                    (
                                    comp_height - y_crop_blocks - dst_blk_y - offset_y as libc::c_uint -
    1u32)
                                        as isize,
                                ))
                                .as_mut_ptr();
                                i = 0i32;
                                while i < crate::jpeglib_h::DCTSIZE {
                                    j = 0i32;
                                    while j < crate::jpeglib_h::DCTSIZE {
                                        *dst_ptr
                                            .offset((j * crate::jpeglib_h::DCTSIZE + i) as isize) =
                                            *src_ptr.offset(
                                                (i * crate::jpeglib_h::DCTSIZE + j) as isize,
                                            );
                                        j += 1;
                                        *dst_ptr
                                            .offset((j * crate::jpeglib_h::DCTSIZE + i) as isize) =
                                            -(*src_ptr.offset(
                                                (i * crate::jpeglib_h::DCTSIZE + j) as isize,
                                            )
                                                as libc::c_int)
                                                as crate::jmorecfg_h::JCOEF;
                                        j += 1
                                    }
                                    i += 1
                                }
                            }
                        } else if  x_crop_blocks + dst_blk_x < comp_width {
                            /* Bottom-edge blocks are mirrored in x only */
                            src_ptr = (*(*src_buffer
                                .offset(((*compptr).h_samp_factor - offset_x - 1i32) as isize))
                            .offset(
                                (
                                dst_blk_y + offset_y as libc::c_uint + y_crop_blocks)
                                    as isize,
                            ))
                            .as_mut_ptr();
                            i = 0i32;
                            while i < crate::jpeglib_h::DCTSIZE {
                                j = 0i32;
                                while j < crate::jpeglib_h::DCTSIZE {
                                    *dst_ptr.offset((j * crate::jpeglib_h::DCTSIZE + i) as isize) =
                                        *src_ptr
                                            .offset((i * crate::jpeglib_h::DCTSIZE + j) as isize);
                                    j += 1
                                }
                                i += 1;
                                j = 0i32;
                                while j < crate::jpeglib_h::DCTSIZE {
                                    *dst_ptr.offset((j * crate::jpeglib_h::DCTSIZE + i) as isize) =
                                        -(*src_ptr
                                            .offset((i * crate::jpeglib_h::DCTSIZE + j) as isize)
                                            as libc::c_int)
                                            as crate::jmorecfg_h::JCOEF;
                                    j += 1
                                }
                                i += 1
                            }
                        } else {
                            /* At lower right corner, just transpose, no mirroring */
                            src_ptr = (*(*src_buffer.offset(offset_x as isize)).offset(
                                (
                                dst_blk_y + offset_y as libc::c_uint + y_crop_blocks)
                                    as isize,
                            ))
                            .as_mut_ptr();
                            i = 0i32;
                            while i < crate::jpeglib_h::DCTSIZE {
                                j = 0i32;
                                while j < crate::jpeglib_h::DCTSIZE {
                                    *dst_ptr.offset((j * crate::jpeglib_h::DCTSIZE + i) as isize) =
                                        *src_ptr
                                            .offset((i * crate::jpeglib_h::DCTSIZE + j) as isize);
                                    j += 1
                                }
                                i += 1
                            }
                        }
                        offset_x += 1
                    }
                    dst_blk_x = dst_blk_x + (*compptr).h_samp_factor as libc::c_uint
                }
                offset_y += 1
            }
            dst_blk_y = dst_blk_y + (*compptr).v_samp_factor as libc::c_uint
        }
        ci += 1
    }
}
/* Parse an unsigned integer: subroutine for jtransform_parse_crop_spec.
 * Returns TRUE if valid integer found, FALSE if not.
 * *strptr is advanced over the digit string, and *result is set to its value.
 */

unsafe extern "C" fn jt_read_integer(
    mut strptr: *mut *const libc::c_char,
    mut result: *mut crate::jmorecfg_h::JDIMENSION,
) -> crate::jmorecfg_h::boolean {
     let mut val:  crate::jmorecfg_h::JDIMENSION =  0u32;let mut ptr: *const libc::c_char = *strptr; /* oops, no digits */
    
    while *(*crate::stdlib::__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as libc::c_int
        &  crate::stdlib::_ISdigit as libc::c_ushort as libc::c_int
        != 0
    {
        val =  val * 10u32 +
    (*ptr as libc::c_int - '0' as i32) as crate::jmorecfg_h::JDIMENSION;
        ptr = ptr.offset(1)
    }
    *result = val;
    if ptr == *strptr {
        return crate::jmorecfg_h::FALSE;
    }
    *strptr = ptr;
    return crate::jmorecfg_h::TRUE;
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
    mut info: *mut crate::src::transupp::jpeg_transform_info,
    mut spec: *const libc::c_char,
) -> crate::jmorecfg_h::boolean {
    (*info).crop = crate::jmorecfg_h::FALSE;
    (*info).crop_width_set = crate::src::transupp::JCROP_UNSET;
    (*info).crop_height_set = crate::src::transupp::JCROP_UNSET;
    (*info).crop_xoffset_set = crate::src::transupp::JCROP_UNSET;
    (*info).crop_yoffset_set = crate::src::transupp::JCROP_UNSET;
    if *(*crate::stdlib::__ctype_b_loc()).offset(*spec as libc::c_int as isize) as libc::c_int
        &  crate::stdlib::_ISdigit as libc::c_ushort as libc::c_int
        != 0
    {
        /* fetch width */
        if jt_read_integer(&mut spec, &mut (*info).crop_width) == 0 {
            return crate::jmorecfg_h::FALSE;
        }
        if *spec as libc::c_int == 'f' as i32 || *spec as libc::c_int == 'F' as i32 {
            spec = spec.offset(1);
            (*info).crop_width_set = crate::src::transupp::JCROP_FORCE
        } else {
            (*info).crop_width_set = crate::src::transupp::JCROP_POS
        }
    }
    if *spec as libc::c_int == 'x' as i32 || *spec as libc::c_int == 'X' as i32 {
        /* fetch height */
        spec = spec.offset(1);
        if jt_read_integer(&mut spec, &mut (*info).crop_height) == 0 {
            return crate::jmorecfg_h::FALSE;
        }
        if *spec as libc::c_int == 'f' as i32 || *spec as libc::c_int == 'F' as i32 {
            spec = spec.offset(1);
            (*info).crop_height_set = crate::src::transupp::JCROP_FORCE
        } else {
            (*info).crop_height_set = crate::src::transupp::JCROP_POS
        }
    }
    if *spec as libc::c_int == '+' as i32 || *spec as libc::c_int == '-' as i32 {
        /* fetch xoffset */
        (*info).crop_xoffset_set = if *spec as libc::c_int == '-' as i32 {
            crate::src::transupp::JCROP_NEG as libc::c_int
        } else {
            crate::src::transupp::JCROP_POS as libc::c_int
        } as crate::src::transupp::JCROP_CODE;
        spec = spec.offset(1);
        if jt_read_integer(&mut spec, &mut (*info).crop_xoffset) == 0 {
            return crate::jmorecfg_h::FALSE;
        }
    }
    if *spec as libc::c_int == '+' as i32 || *spec as libc::c_int == '-' as i32 {
        /* fetch yoffset */
        (*info).crop_yoffset_set = if *spec as libc::c_int == '-' as i32 {
            crate::src::transupp::JCROP_NEG as libc::c_int
        } else {
            crate::src::transupp::JCROP_POS as libc::c_int
        } as crate::src::transupp::JCROP_CODE;
        spec = spec.offset(1);
        if jt_read_integer(&mut spec, &mut (*info).crop_yoffset) == 0 {
            return crate::jmorecfg_h::FALSE;
        }
    }
    /* We had better have gotten to the end of the string. */
    if *spec as libc::c_int != '\u{0}' as i32 {
        return crate::jmorecfg_h::FALSE;
    }
    (*info).crop = crate::jmorecfg_h::TRUE;
    return crate::jmorecfg_h::TRUE;
}
/* Trim off any partial iMCUs on the indicated destination edge */

unsafe extern "C" fn trim_right_edge(
    mut info: *mut crate::src::transupp::jpeg_transform_info,
    mut full_width: crate::jmorecfg_h::JDIMENSION,
) {
     
     let mut MCU_cols:   crate::jmorecfg_h::JDIMENSION =
      (*info)
        .output_width / (*info).iMCU_sample_width as libc::c_uint;
    if MCU_cols > 0u32
        &&  (*info).x_crop_offset + MCU_cols
            ==  full_width / (*info).iMCU_sample_width as libc::c_uint
    {
        (*info).output_width =  MCU_cols * (*info).iMCU_sample_width as libc::c_uint
    };
}

unsafe extern "C" fn trim_bottom_edge(
    mut info: *mut crate::src::transupp::jpeg_transform_info,
    mut full_height: crate::jmorecfg_h::JDIMENSION,
) {
     
     let mut MCU_rows:   crate::jmorecfg_h::JDIMENSION =
      (*info)
        .output_height / (*info).iMCU_sample_height as libc::c_uint;
    if MCU_rows > 0u32
        &&  (*info).y_crop_offset + MCU_rows
            ==  full_height / (*info).iMCU_sample_height as libc::c_uint
    {
        (*info).output_height =  MCU_rows * (*info).iMCU_sample_height as libc::c_uint
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
    mut srcinfo: crate::jpeglib_h::j_decompress_ptr,
    mut info: *mut crate::src::transupp::jpeg_transform_info,
) -> crate::jmorecfg_h::boolean {
    
    
    
    
    
    
    
    
    
    
    
    
      
    /* Determine number of components in output image */
    if (*info).force_grayscale != 0
        &&  (*srcinfo).jpeg_color_space
            ==  crate::jpeglib_h::JCS_YCbCr
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
                return crate::jmorecfg_h::FALSE;
            }
        } else if jtransform_perfect_transform(
            (*srcinfo).output_width,
            (*srcinfo).output_height,
            (*srcinfo).max_h_samp_factor * (*srcinfo).min_DCT_scaled_size,
            (*srcinfo).max_v_samp_factor * (*srcinfo).min_DCT_scaled_size,
            (*info).transform,
        ) == 0
        {
            return crate::jmorecfg_h::FALSE;
        }
    }
    /* If there is only one output component, force the iMCU size to be 1;
     * else use the source iMCU size.  (This allows us to do the right thing
     * when reducing color to grayscale, and also provides a handy way of
     * cleaning up "funny" grayscale images whose sampling factors are not 1x1.)
     */
    match  (*info).transform {
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
         let mut xoffset:  crate::jmorecfg_h::JDIMENSION =  0; let mut yoffset:  crate::jmorecfg_h::JDIMENSION =  0;if  (*info).crop_xoffset_set
            ==  crate::src::transupp::JCROP_UNSET
        {
            (*info).crop_xoffset = 0u32
        } /* default to +0 */
        if  (*info).crop_yoffset_set
            ==  crate::src::transupp::JCROP_UNSET
        {
            (*info).crop_yoffset = 0u32
        } /* default to +0 */
        if (*info).crop_xoffset >= (*info).output_width
            || (*info).crop_yoffset >= (*info).output_height
        {
            (*(*srcinfo).err).msg_code = crate::src::jerror::JERR_BAD_CROP_SPEC as libc::c_int;
            Some(
                (*(*srcinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        if  (*info).crop_width_set
            ==  crate::src::transupp::JCROP_UNSET
        {
            (*info).crop_width =  (*info).output_width - (*info).crop_xoffset
        }
        if  (*info).crop_height_set
            ==  crate::src::transupp::JCROP_UNSET
        {
            (*info).crop_height =  (*info).output_height - (*info).crop_yoffset
        }
        /* Ensure parameters are valid */
        if (*info).crop_width <= 0u32
            || (*info).crop_width > (*info).output_width
            || (*info).crop_height <= 0u32
            || (*info).crop_height > (*info).output_height
            || (*info).crop_xoffset >  (*info).output_width - (*info).crop_width
            || (*info).crop_yoffset >  (*info).output_height - (*info).crop_height
        {
            (*(*srcinfo).err).msg_code = crate::src::jerror::JERR_BAD_CROP_SPEC as libc::c_int;
            Some(
                (*(*srcinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        /* Convert negative crop offsets into regular offsets */
        if  (*info).crop_xoffset_set
            ==  crate::src::transupp::JCROP_NEG
        {
            xoffset =  (*info)
                .output_width - (*info).crop_width - (*info).crop_xoffset
        } else {
            xoffset = (*info).crop_xoffset
        }
        if  (*info).crop_yoffset_set
            ==  crate::src::transupp::JCROP_NEG
        {
            yoffset =  (*info)
                .output_height - (*info).crop_height - (*info).crop_yoffset
        } else {
            yoffset = (*info).crop_yoffset
        }
        /* Now adjust so that upper left corner falls at an iMCU boundary */
        if  (*info).crop_width_set
            ==  crate::src::transupp::JCROP_FORCE
        {
            (*info).output_width = (*info).crop_width
        } else {
            (*info).output_width =  (*info)
                .crop_width + xoffset % (*info).iMCU_sample_width as libc::c_uint
        }
        if  (*info).crop_height_set
            ==  crate::src::transupp::JCROP_FORCE
        {
            (*info).output_height = (*info).crop_height
        } else {
            (*info).output_height =  (*info)
                .crop_height + yoffset % (*info).iMCU_sample_height as libc::c_uint
        }
        /* Save x/y offsets measured in iMCUs */
        (*info).x_crop_offset =  xoffset / (*info).iMCU_sample_width as libc::c_uint;
        (*info).y_crop_offset =  yoffset / (*info).iMCU_sample_height as libc::c_uint
    } else {
        (*info).x_crop_offset = 0u32;
        (*info).y_crop_offset = 0u32
    }
    /* Figure out whether we need workspace arrays,
     * and if so whether they are transposed relative to the source.
     */
    
     let mut need_workspace:   crate::jmorecfg_h::boolean =  crate::jmorecfg_h::FALSE; let mut transpose_it:   crate::jmorecfg_h::boolean =  crate::jmorecfg_h::FALSE;
    match  (*info).transform {
        0 => {
            if (*info).x_crop_offset != 0u32
                || (*info).y_crop_offset != 0u32
            {
                need_workspace = crate::jmorecfg_h::TRUE
            }
        }
        1 => {
            if (*info).trim != 0 {
                trim_right_edge(info, (*srcinfo).output_width);
            }
            if (*info).y_crop_offset != 0u32 || (*info).slow_hflip != 0 {
                need_workspace = crate::jmorecfg_h::TRUE
            }
        }
        2 => {
            if (*info).trim != 0 {
                trim_bottom_edge(info, (*srcinfo).output_height);
            }
            /* Need workspace arrays having same dimensions as source image. */
            need_workspace = crate::jmorecfg_h::TRUE
        }
        3 => {
            /* transpose does NOT have to trim anything */
            /* Need workspace arrays having transposed dimensions. */
            need_workspace = crate::jmorecfg_h::TRUE;
            transpose_it = crate::jmorecfg_h::TRUE
        }
        4 => {
            if (*info).trim != 0 {
                trim_right_edge(info, (*srcinfo).output_height);
                trim_bottom_edge(info, (*srcinfo).output_width);
            }
            /* Need workspace arrays having transposed dimensions. */
            need_workspace = crate::jmorecfg_h::TRUE;
            transpose_it = crate::jmorecfg_h::TRUE
        }
        5 => {
            if (*info).trim != 0 {
                trim_right_edge(info, (*srcinfo).output_height);
            }
            /* Need workspace arrays having transposed dimensions. */
            need_workspace = crate::jmorecfg_h::TRUE;
            transpose_it = crate::jmorecfg_h::TRUE
        }
        6 => {
            if (*info).trim != 0 {
                trim_right_edge(info, (*srcinfo).output_width);
                trim_bottom_edge(info, (*srcinfo).output_height);
            }
            /* Need workspace arrays having same dimensions as source image. */
            need_workspace = crate::jmorecfg_h::TRUE
        }
        7 => {
            if (*info).trim != 0 {
                trim_bottom_edge(info, (*srcinfo).output_width);
            }
            /* Need workspace arrays having transposed dimensions. */
            need_workspace = crate::jmorecfg_h::TRUE;
            transpose_it = crate::jmorecfg_h::TRUE
        }
        _ => {}
    }
    /* Allocate workspace if needed.
     * Note that we allocate arrays padded out to the next iMCU boundary,
     * so that transform routines need not worry about missing edge blocks.
     */
    if need_workspace != 0 {
            
        
        
         let mut coef_arrays:   *mut crate::jpeglib_h::jvirt_barray_ptr =
     Some(
            (*(*srcinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            srcinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            ::std::mem::size_of::<crate::jpeglib_h::jvirt_barray_ptr>() as libc::c_ulong *
    (*info).num_components as libc::c_ulong,
        ) as *mut crate::jpeglib_h::jvirt_barray_ptr; let mut width_in_iMCUs:   crate::jmorecfg_h::JDIMENSION =
     crate::jpegint_h::jdiv_round_up(
            (*info).output_width as libc::c_long,
            (*info).iMCU_sample_width as libc::c_long,
        ) as crate::jmorecfg_h::JDIMENSION; let mut height_in_iMCUs:   crate::jmorecfg_h::JDIMENSION =
     crate::jpegint_h::jdiv_round_up(
            (*info).output_height as libc::c_long,
            (*info).iMCU_sample_height as libc::c_long,
        ) as crate::jmorecfg_h::JDIMENSION; let mut ci:   libc::c_int =  0i32;
        while ci < (*info).num_components {
                let mut h_samp_factor:  libc::c_int =  0; let mut v_samp_factor:  libc::c_int =  0; let mut compptr:   *mut crate::jpeglib_h::jpeg_component_info =
     (*srcinfo).comp_info.offset(ci as isize);
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
            
             let mut width_in_blocks:   crate::jmorecfg_h::JDIMENSION =
      width_in_iMCUs * h_samp_factor as libc::c_uint; let mut height_in_blocks:   crate::jmorecfg_h::JDIMENSION =
      height_in_iMCUs * v_samp_factor as libc::c_uint;
            let ref mut fresh28 = *coef_arrays.offset(ci as isize);
            *fresh28 = Some(
                (*(*srcinfo).mem)
                    .request_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as crate::jpeglib_h::j_common_ptr,
                crate::jpeglib_h::JPOOL_IMAGE,
                crate::jmorecfg_h::FALSE,
                width_in_blocks,
                height_in_blocks,
                v_samp_factor as crate::jmorecfg_h::JDIMENSION,
            );
            ci += 1
        }
        (*info).workspace_coef_arrays = coef_arrays
    } else {
        (*info).workspace_coef_arrays =
            crate::stddef_h::NULL as *mut crate::jpeglib_h::jvirt_barray_ptr
    }
    return crate::jmorecfg_h::TRUE;
}
/* Transpose destination image parameters */

unsafe extern "C" fn transpose_critical_parameters(mut dstinfo: crate::jpeglib_h::j_compress_ptr) {
    
    
    
    
    
    
    
    
       
     let mut jtemp:   crate::jmorecfg_h::JDIMENSION =  (*dstinfo).image_width;
    (*dstinfo).image_width = (*dstinfo).image_height;
    (*dstinfo).image_height = jtemp;
     let mut ci:   libc::c_int =  0i32;
    while ci < (*dstinfo).num_components {
          
         let mut compptr:   *mut crate::jpeglib_h::jpeg_component_info =
     (*dstinfo).comp_info.offset(ci as isize); let mut itemp:   libc::c_int =  (*compptr).h_samp_factor;
        (*compptr).h_samp_factor = (*compptr).v_samp_factor;
        (*compptr).v_samp_factor = itemp;
        ci += 1
    }
     let mut tblno:   libc::c_int =  0i32;
    while tblno < crate::jpeglib_h::NUM_QUANT_TBLS {
          let mut qtblptr:   *mut crate::jpeglib_h::JQUANT_TBL =
     (*dstinfo).quant_tbl_ptrs[tblno as usize];
        if !qtblptr.is_null() {
              let mut i:   libc::c_int =  0i32;
            while i < crate::jpeglib_h::DCTSIZE {
                  let mut j:   libc::c_int =  0i32;
                while j < i {
                      let mut qtemp:   crate::jmorecfg_h::UINT16 =
     (*qtblptr).quantval[(i * crate::jpeglib_h::DCTSIZE + j) as usize];
                    (*qtblptr).quantval[(i * crate::jpeglib_h::DCTSIZE + j) as usize] =
                        (*qtblptr).quantval[(j * crate::jpeglib_h::DCTSIZE + i) as usize];
                    (*qtblptr).quantval[(j * crate::jpeglib_h::DCTSIZE + i) as usize] = qtemp;
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
    mut data: *mut crate::jmorecfg_h::JOCTET,
    mut length: libc::c_uint,
    mut new_width: crate::jmorecfg_h::JDIMENSION,
    mut new_height: crate::jmorecfg_h::JDIMENSION,
) {
     /* Flag for byte order */
     /* Length of an IFD entry */
    
    
    
     let mut is_motorola:  crate::jmorecfg_h::boolean =  0; let mut number_of_tags:  libc::c_uint =  0; let mut tagnum:  libc::c_uint =  0; let mut firstoffset:  libc::c_uint =  0; let mut offset:  libc::c_uint =  0;
    if length < 12u32 {
        return;
    }
    /* Discover byte order */
    if *data.offset(0) as libc::c_int == 0x49i32 && *data.offset(1) as libc::c_int == 0x49i32 {
        is_motorola = crate::jmorecfg_h::FALSE
    } else if *data.offset(0) as libc::c_int == 0x4di32 && *data.offset(1) as libc::c_int == 0x4di32
    {
        is_motorola = crate::jmorecfg_h::TRUE
    } else {
        return;
    }
    /* Check Tag Mark */
    if is_motorola != 0 {
        if *data.offset(2) as libc::c_int != 0i32 {
            return;
        }
        if *data.offset(3) as libc::c_int != 0x2ai32 {
            return;
        }
    } else {
        if *data.offset(3) as libc::c_int != 0i32 {
            return;
        }
        if *data.offset(2) as libc::c_int != 0x2ai32 {
            return;
        }
    }
    /* Get first IFD offset (offset to IFD0) */
    if is_motorola != 0 {
        if *data.offset(4) as libc::c_int != 0i32 {
            return;
        } /* check end of data segment */
        if *data.offset(5) as libc::c_int != 0i32 {
            return;
        }
        firstoffset = *data.offset(6) as libc::c_uint;
        firstoffset <<= 8i32;
        firstoffset =  firstoffset + *data.offset(7) as libc::c_uint
    } else {
        if *data.offset(7) as libc::c_int != 0i32 {
            return;
        }
        if *data.offset(6) as libc::c_int != 0i32 {
            return;
        }
        firstoffset = *data.offset(5) as libc::c_uint;
        firstoffset <<= 8i32;
        firstoffset =  firstoffset + *data.offset(4) as libc::c_uint
    }
    if firstoffset >  length - 2u32 {
        return;
    }
    /* Get the number of directory entries contained in this IFD */
    if is_motorola != 0 {
        number_of_tags = *data.offset(firstoffset as isize) as libc::c_uint;
        number_of_tags <<= 8i32;
        number_of_tags =  number_of_tags +
    
            *data.offset((firstoffset + 1u32) as isize) as libc::c_uint
    } else {
        number_of_tags =
            *data.offset((firstoffset + 1u32) as isize) as libc::c_uint;
        number_of_tags <<= 8i32;
        number_of_tags =
            
            number_of_tags + *data.offset(firstoffset as isize) as libc::c_uint
    }
    if number_of_tags == 0u32 {
        return;
    }
    firstoffset =  firstoffset + 2u32;
    loop
    /* Search for ExifSubIFD offset Tag in IFD0 */
    {
        if firstoffset >  length - 12u32 {
            return;
        } /* check end of data segment */
        /* Get Tag number */
        if is_motorola != 0 {
            tagnum = *data.offset(firstoffset as isize) as libc::c_uint; /* found ExifSubIFD offset Tag */
            tagnum <<= 8i32;
            tagnum =  tagnum +
    
                *data.offset((firstoffset + 1u32) as isize)
                    as libc::c_uint
        } else {
            tagnum = *data.offset((firstoffset + 1u32) as isize)
                as libc::c_uint;
            tagnum <<= 8i32;
            tagnum =  tagnum + *data.offset(firstoffset as isize) as libc::c_uint
        }
        if tagnum == 0x8769u32 {
            break;
        }
        number_of_tags =  number_of_tags - 1;
        if number_of_tags == 0u32 {
            return;
        }
        firstoffset =  firstoffset + 12u32
    }
    /* Get the ExifSubIFD offset */
    if is_motorola != 0 {
        if *data.offset((firstoffset + 8u32) as isize) as libc::c_int
            != 0i32
        {
            return;
        } /* check end of data segment */
        if *data.offset((firstoffset + 9u32) as isize) as libc::c_int
            != 0i32
        {
            return;
        }
        offset =
            *data.offset((firstoffset + 10u32) as isize) as libc::c_uint;
        offset <<= 8i32;
        offset =  offset +
    
            *data.offset((firstoffset + 11u32) as isize) as libc::c_uint
    } else {
        if *data.offset((firstoffset + 11u32) as isize) as libc::c_int
            != 0i32
        {
            return;
        }
        if *data.offset((firstoffset + 10u32) as isize) as libc::c_int
            != 0i32
        {
            return;
        }
        offset =
            *data.offset((firstoffset + 9u32) as isize) as libc::c_uint;
        offset <<= 8i32;
        offset =  offset +
    
            *data.offset((firstoffset + 8u32) as isize) as libc::c_uint
    }
    if offset >  length - 2u32 {
        return;
    }
    /* Get the number of directory entries contained in this SubIFD */
    if is_motorola != 0 {
        number_of_tags = *data.offset(offset as isize) as libc::c_uint;
        number_of_tags <<= 8i32;
        number_of_tags =  number_of_tags +
    
            *data.offset((offset + 1u32) as isize) as libc::c_uint
    } else {
        number_of_tags =
            *data.offset((offset + 1u32) as isize) as libc::c_uint;
        number_of_tags <<= 8i32;
        number_of_tags =  number_of_tags + *data.offset(offset as isize) as libc::c_uint
    }
    if number_of_tags < 2u32 {
        return;
    }
    offset =  offset + 2u32;
    loop
    /* Search for ExifImageWidth and ExifImageHeight Tags in this SubIFD */
    {
        if offset >  length - 12u32 {
            return;
        } /* check end of data segment */
        /* Get Tag number */
        if is_motorola != 0 {
            tagnum = *data.offset(offset as isize) as libc::c_uint; /* ExifImageHeight Tag */
            tagnum <<= 8i32; /* ExifImageWidth Tag */
            tagnum =  tagnum +
    
                *data.offset((offset + 1u32) as isize) as libc::c_uint
        } else {
            tagnum =
                *data.offset((offset + 1u32) as isize) as libc::c_uint; /* Format = unsigned long (4 octets) */
            tagnum <<= 8i32; /* Number Of Components = 1 */
            tagnum =  tagnum + *data.offset(offset as isize) as libc::c_uint
        } /* Format = unsigned long (4 octets) */
        if tagnum == 0xa002u32 || tagnum == 0xa003u32 {
             let mut new_value:  crate::jmorecfg_h::JDIMENSION =  0;if tagnum == 0xa002u32 {
                new_value = new_width
            } else {
                new_value = new_height
            } /* Number Of Components = 1 */
            if is_motorola != 0 {
                *data.offset((offset + 2u32) as isize) =
                    0u8;
                *data.offset((offset + 3u32) as isize) =
                    4u8;
                *data.offset((offset + 4u32) as isize) =
                    0u8;
                *data.offset((offset + 5u32) as isize) =
                    0u8;
                *data.offset((offset + 6u32) as isize) =
                    0u8;
                *data.offset((offset + 7u32) as isize) =
                    1u8;
                *data.offset((offset + 8u32) as isize) =
                    0u8;
                *data.offset((offset + 9u32) as isize) =
                    0u8;
                *data.offset((offset + 10u32) as isize) =
                    (new_value >> 8i32 & 0xffu32) as crate::jmorecfg_h::JOCTET;
                *data.offset((offset + 11u32) as isize) =
                    (new_value & 0xffu32) as crate::jmorecfg_h::JOCTET
            } else {
                *data.offset((offset + 2u32) as isize) =
                    4u8;
                *data.offset((offset + 3u32) as isize) =
                    0u8;
                *data.offset((offset + 4u32) as isize) =
                    1u8;
                *data.offset((offset + 5u32) as isize) =
                    0u8;
                *data.offset((offset + 6u32) as isize) =
                    0u8;
                *data.offset((offset + 7u32) as isize) =
                    0u8;
                *data.offset((offset + 8u32) as isize) =
                    (new_value & 0xffu32) as crate::jmorecfg_h::JOCTET;
                *data.offset((offset + 9u32) as isize) =
                    (new_value >> 8i32 & 0xffu32) as crate::jmorecfg_h::JOCTET;
                *data.offset((offset + 10u32) as isize) =
                    0u8;
                *data.offset((offset + 11u32) as isize) =
                    0u8
            }
        }
        offset =  offset + 12u32;
        number_of_tags =  number_of_tags - 1;
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
    mut srcinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dstinfo: crate::jpeglib_h::j_compress_ptr,
    mut src_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
    mut info: *mut crate::src::transupp::jpeg_transform_info,
) -> *mut crate::jpeglib_h::jvirt_barray_ptr {
    /* If force-to-grayscale is requested, adjust destination parameters */
    if (*info).force_grayscale != 0 {
        /* First, ensure we have YCbCr or grayscale data, and that the source's
         * Y channel is full resolution.  (No reasonable person would make Y
         * be less than full resolution, so actually coping with that case
         * isn't worth extra code space.  But we check it to avoid crashing.)
         */
        if ((*dstinfo).jpeg_color_space
            ==  crate::jpeglib_h::JCS_YCbCr
            && (*dstinfo).num_components == 3i32
            ||  (*dstinfo).jpeg_color_space
                ==  crate::jpeglib_h::JCS_GRAYSCALE
                && (*dstinfo).num_components == 1i32)
            && (*(*srcinfo).comp_info.offset(0)).h_samp_factor == (*srcinfo).max_h_samp_factor
            && (*(*srcinfo).comp_info.offset(0)).v_samp_factor == (*srcinfo).max_v_samp_factor
        {
            /* We use jpeg_set_colorspace to make sure subsidiary settings get fixed
             * properly.  Among other things, it sets the target h_samp_factor &
             * v_samp_factor to 1, which typically won't match the source.
             * We have to preserve the source's quantization table number, however.
             */
            let mut sv_quant_tbl_no: libc::c_int = (*(*dstinfo).comp_info.offset(0)).quant_tbl_no;
            crate::jpeglib_h::jpeg_set_colorspace(dstinfo, crate::jpeglib_h::JCS_GRAYSCALE);
            (*(*dstinfo).comp_info.offset(0)).quant_tbl_no = sv_quant_tbl_no
        } else {
            /* Sorry, can't do it */
            (*(*dstinfo).err).msg_code = crate::src::jerror::JERR_CONVERSION_NOTIMPL as libc::c_int;
            Some(
                (*(*dstinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                dstinfo as crate::jpeglib_h::j_common_ptr
            );
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
    match  (*info).transform {
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
        && (*(*srcinfo).marker_list).marker as libc::c_int == crate::jpeglib_h::JPEG_APP0 + 1i32
        && (*(*srcinfo).marker_list).data_length >= 6u32
        && *(*(*srcinfo).marker_list).data.offset(0) as libc::c_int == 0x45i32
        && *(*(*srcinfo).marker_list).data.offset(1) as libc::c_int == 0x78i32
        && *(*(*srcinfo).marker_list).data.offset(2) as libc::c_int == 0x69i32
        && *(*(*srcinfo).marker_list).data.offset(3) as libc::c_int == 0x66i32
        && *(*(*srcinfo).marker_list).data.offset(4) as libc::c_int == 0i32
        && *(*(*srcinfo).marker_list).data.offset(5) as libc::c_int == 0i32
    {
        /* Suppress output of JFIF marker */
        (*dstinfo).write_JFIF_header = crate::jmorecfg_h::FALSE;
        /* Adjust Exif image parameters */
        if (*dstinfo).image_width != (*srcinfo).image_width
            || (*dstinfo).image_height != (*srcinfo).image_height
        {
            /* Align data segment to start of TIFF structure for parsing */
            adjust_exif_parameters(
                (*(*srcinfo).marker_list).data.offset(6),
                
                (*(*srcinfo).marker_list)
                    .data_length - 6u32,
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
    mut srcinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dstinfo: crate::jpeglib_h::j_compress_ptr,
    mut src_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
    mut info: *mut crate::src::transupp::jpeg_transform_info,
) {
    let mut dst_coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr =
        (*info).workspace_coef_arrays;
    /* Note: conditions tested here should match those in switch statement
     * in jtransform_request_workspace()
     */
    match  (*info).transform {
        0 => {
            if (*info).x_crop_offset != 0u32
                || (*info).y_crop_offset != 0u32
            {
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
            if (*info).y_crop_offset != 0u32 || (*info).slow_hflip != 0 {
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
    mut image_width: crate::jmorecfg_h::JDIMENSION,
    mut image_height: crate::jmorecfg_h::JDIMENSION,
    mut MCU_width: libc::c_int,
    mut MCU_height: libc::c_int,
    mut transform: crate::src::transupp::JXFORM_CODE,
) -> crate::jmorecfg_h::boolean {
    let mut result: crate::jmorecfg_h::boolean = crate::jmorecfg_h::TRUE; /* initialize TRUE */
    match  transform {
        1 | 7 => {
            if  image_width % MCU_width as crate::jmorecfg_h::JDIMENSION != 0 {
                result = crate::jmorecfg_h::FALSE
            }
        }
        2 | 5 => {
            if  image_height % MCU_height as crate::jmorecfg_h::JDIMENSION != 0 {
                result = crate::jmorecfg_h::FALSE
            }
        }
        4 | 6 => {
            if  image_width % MCU_width as crate::jmorecfg_h::JDIMENSION != 0 {
                result = crate::jmorecfg_h::FALSE
            }
            if  image_height % MCU_height as crate::jmorecfg_h::JDIMENSION != 0 {
                result = crate::jmorecfg_h::FALSE
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
    mut srcinfo: crate::jpeglib_h::j_decompress_ptr,
    mut option: crate::src::transupp::JCOPY_OPTION,
) {
    
    /* Save comments except under NONE option */
    if  option !=  crate::src::transupp::JCOPYOPT_NONE
    {
        crate::jpeglib_h::jpeg_save_markers(
            srcinfo,
            crate::jpeglib_h::JPEG_COM,
            0xffffu32,
        );
    }
    /* Save all types of APPn markers iff ALL option */
    if  option ==  crate::src::transupp::JCOPYOPT_ALL
        ||  option
            ==  crate::src::transupp::JCOPYOPT_ALL_EXCEPT_ICC
    {
          let mut m:   libc::c_int =  0i32;
        while m < 16i32 {
            if !(option
                ==  crate::src::transupp::JCOPYOPT_ALL_EXCEPT_ICC
                && m == 2i32)
            {
                crate::jpeglib_h::jpeg_save_markers(
                    srcinfo,
                    crate::jpeglib_h::JPEG_APP0 + m,
                    0xffffu32,
                );
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
    mut srcinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dstinfo: crate::jpeglib_h::j_compress_ptr,
    mut option: crate::src::transupp::JCOPY_OPTION,
) {
     
    /* In the current implementation, we don't actually need to examine the
     * option flag here; we just copy everything that got saved.
     * But to avoid confusion, we do not output JFIF and Adobe APP14 markers
     * if the encoder library already wrote one.
     */
     let mut marker:   crate::jpeglib_h::jpeg_saved_marker_ptr =
     (*srcinfo).marker_list; /* reject duplicate JFIF */
    while !marker.is_null() {
        if !((*dstinfo).write_JFIF_header != 0
            && (*marker).marker as libc::c_int == crate::jpeglib_h::JPEG_APP0
            && (*marker).data_length >= 5u32
            && *(*marker).data.offset(0) as libc::c_int == 0x4ai32
            && *(*marker).data.offset(1) as libc::c_int == 0x46i32
            && *(*marker).data.offset(2) as libc::c_int == 0x49i32
            && *(*marker).data.offset(3) as libc::c_int == 0x46i32
            && *(*marker).data.offset(4) as libc::c_int == 0i32)
        {
            if !((*dstinfo).write_Adobe_marker != 0
                && (*marker).marker as libc::c_int == crate::jpeglib_h::JPEG_APP0 + 14i32
                && (*marker).data_length >= 5u32
                && *(*marker).data.offset(0) as libc::c_int == 0x41i32
                && *(*marker).data.offset(1) as libc::c_int == 0x64i32
                && *(*marker).data.offset(2) as libc::c_int == 0x6fi32
                && *(*marker).data.offset(3) as libc::c_int == 0x62i32
                && *(*marker).data.offset(4) as libc::c_int == 0x65i32)
            {
                crate::jpeglib_h::jpeg_write_marker(
                    dstinfo,
                    (*marker).marker as libc::c_int,
                    (*marker).data,
                    (*marker).data_length,
                ); /* reject duplicate Adobe */
            }
        }
        marker = (*marker).next
    }
}
