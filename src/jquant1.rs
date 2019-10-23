use libc::c_uint;use libc::c_ulong;use libc::c_int;use libc::c_long;use libc::c_void;use libc;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::rgb_blue;
pub use crate::jmorecfg_h::rgb_green;
pub use crate::jmorecfg_h::rgb_red;
pub use crate::jmorecfg_h::EXT_BGRX_BLUE;
pub use crate::jmorecfg_h::EXT_BGRX_GREEN;
pub use crate::jmorecfg_h::EXT_BGRX_RED;
pub use crate::jmorecfg_h::EXT_BGR_BLUE;
pub use crate::jmorecfg_h::EXT_BGR_GREEN;
pub use crate::jmorecfg_h::EXT_BGR_RED;
pub use crate::jmorecfg_h::EXT_RGBX_BLUE;
pub use crate::jmorecfg_h::EXT_RGBX_GREEN;
pub use crate::jmorecfg_h::EXT_RGBX_RED;
pub use crate::jmorecfg_h::EXT_RGB_BLUE;
pub use crate::jmorecfg_h::EXT_RGB_GREEN;
pub use crate::jmorecfg_h::EXT_RGB_RED;
pub use crate::jmorecfg_h::EXT_XBGR_BLUE;
pub use crate::jmorecfg_h::EXT_XBGR_GREEN;
pub use crate::jmorecfg_h::EXT_XBGR_RED;
pub use crate::jmorecfg_h::EXT_XRGB_BLUE;
pub use crate::jmorecfg_h::EXT_XRGB_GREEN;
pub use crate::jmorecfg_h::EXT_XRGB_RED;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::INT16;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::MAXJSAMPLE;
pub use crate::jmorecfg_h::RGB_BLUE;
pub use crate::jmorecfg_h::RGB_GREEN;
pub use crate::jmorecfg_h::RGB_RED;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jzero_far;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::JLONG;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_color_deconverter;
pub use crate::jpeglib_h::jpeg_color_quantizer;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_d_coef_controller;
pub use crate::jpeglib_h::jpeg_d_main_controller;
pub use crate::jpeglib_h::jpeg_d_post_controller;
pub use crate::jpeglib_h::jpeg_decomp_master;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_entropy_decoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_input_controller;
pub use crate::jpeglib_h::jpeg_inverse_dct;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_reader;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jpeg_upsampler;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JCS_YCbCr;
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
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
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
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
/* use 'int' for calculation temps */

pub type FSERRPTR = *mut FSERROR;
/* Declarations for Floyd-Steinberg dithering.
 *
 * Errors are accumulated into the array fserrors[], at a resolution of
 * 1/16th of a pixel count.  The error at a given pixel is propagated
 * to its not-yet-processed neighbors using the standard F-S fractions,
 *              ...     (here)  7/16
 *              3/16    5/16    1/16
 * We work left-to-right on even rows, right-to-left on odd rows.
 *
 * We can get away with a single array (holding one row's worth of errors)
 * by using it to store the current row's errors at pixel columns not yet
 * processed, but the next row's errors at columns already processed.  We
 * need only a few extra variables to hold the errors immediately around the
 * current column.  (If we are lucky, those variables are in registers, but
 * even if not, they're probably cheaper to access than array elements are.)
 *
 * The fserrors[] array is indexed [component#][position].
 * We provide (#columns + 2) entries per component; the extra entry at each
 * end saves us from special-casing the first and last pixels.
 */

pub type FSERROR = INT16;

pub type my_cquantize_ptr = *mut my_cquantizer;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_cquantizer {
    pub pub_0: jpeg_color_quantizer,
    pub sv_colormap: JSAMPARRAY,
    pub sv_actual: c_int,
    pub colorindex: JSAMPARRAY,
    pub is_padded: boolean,
    pub Ncolors: [c_int; 4],
    pub row_index: c_int,
    pub odither: [ODITHER_MATRIX_PTR; 4],
    pub fserrors: [FSERRPTR; 4],
    pub on_odd_row: boolean,
}

pub type ODITHER_MATRIX_PTR = *mut [c_int; 16];
/* 16 bits should be enough */

pub type LOCFSERROR = c_int;
/* mask for wrapping around
counters */

pub type ODITHER_MATRIX = [[c_int; 16]; 16];
/*
 * jquant1.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009, 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains 1-pass color quantization (color mapping) routines.
 * These routines provide mapping to a fixed color map using equally spaced
 * color values.  Optional Floyd-Steinberg or ordered dithering is available.
 */
/*
 * The main purpose of 1-pass quantization is to provide a fast, if not very
 * high quality, colormapped output capability.  A 2-pass quantizer usually
 * gives better visual quality; however, for quantized grayscale output this
 * quantizer is perfectly adequate.  Dithering is highly recommended with this
 * quantizer, though you can turn it off if you really want to.
 *
 * In 1-pass quantization the colormap must be chosen in advance of seeing the
 * image.  We use a map consisting of all combinations of Ncolors[i] color
 * values for the i'th component.  The Ncolors[] values are chosen so that
 * their product, the total number of colors, is no more than that requested.
 * (In most cases, the product will be somewhat less.)
 *
 * Since the colormap is orthogonal, the representative value for each color
 * component can be determined without considering the other components;
 * then these indexes can be combined into a colormap index by a standard
 * N-dimensional-array-subscript calculation.  Most of the arithmetic involved
 * can be precalculated and stored in the lookup table colorindex[].
 * colorindex[i][j] maps pixel value j in component i to the nearest
 * representative value (grid plane) for that component; this index is
 * multiplied by the array stride for component i, so that the
 * index of the colormap entry closest to a given pixel value is just
 *    sum( colorindex[component-number][pixel-component-value] )
 * Aside from being fast, this scheme allows for variable spacing between
 * representative values with no additional lookup cost.
 *
 * If gamma correction has been applied in color conversion, it might be wise
 * to adjust the color grid spacing so that the representative colors are
 * equidistant in linear space.  At this writing, gamma correction is not
 * implemented by jdcolor, so nothing is done here.
 */
/* Declarations for ordered dithering.
 *
 * We use a standard 16x16 ordered dither array.  The basic concept of ordered
 * dithering is described in many references, for instance Dale Schumacher's
 * chapter II.2 of Graphics Gems II (James Arvo, ed. Academic Press, 1991).
 * In place of Schumacher's comparisons against a "threshold" value, we add a
 * "dither" value to the input pixel and then round the result to the nearest
 * output value.  The dither value is equivalent to (0.5 - threshold) times
 * the distance between output values.  For ordered dithering, we assume that
 * the output colors are equally spaced; if not, results will probably be
 * worse, since the dither may be too much or too little at a given point.
 *
 * The normal calculation would be to form pixel value + dither, range-limit
 * this to 0..MAXJSAMPLE, and then index into the colorindex table as usual.
 * We can skip the separate range-limiting step by extending the colorindex
 * table in both directions.
 */

pub const ODITHER_SIZE: c_int = 16i32;
/* dimension of dither matrix */
/* NB: if ODITHER_SIZE is not a power of 2, ODITHER_MASK uses will break */

pub const ODITHER_CELLS: c_int = ODITHER_SIZE * ODITHER_SIZE;
/* # cells in matrix */

pub const ODITHER_MASK: c_int = ODITHER_SIZE - 1i32;

static mut base_dither_matrix: [[UINT8; 16]; 16] = [
    [
        0u8,
        192u8,
        48u8,
        240u8,
        12u8,
        204u8,
        60u8,
        252u8,
        3u8,
        195u8,
        51u8,
        243u8,
        15u8,
        207u8,
        63u8,
        255u8,
    ],
    [
        128u8,
        64u8,
        176u8,
        112u8,
        140u8,
        76u8,
        188u8,
        124u8,
        131u8,
        67u8,
        179u8,
        115u8,
        143u8,
        79u8,
        191u8,
        127u8,
    ],
    [
        32u8,
        224u8,
        16u8,
        208u8,
        44u8,
        236u8,
        28u8,
        220u8,
        35u8,
        227u8,
        19u8,
        211u8,
        47u8,
        239u8,
        31u8,
        223u8,
    ],
    [
        160u8,
        96u8,
        144u8,
        80u8,
        172u8,
        108u8,
        156u8,
        92u8,
        163u8,
        99u8,
        147u8,
        83u8,
        175u8,
        111u8,
        159u8,
        95u8,
    ],
    [
        8u8,
        200u8,
        56u8,
        248u8,
        4u8,
        196u8,
        52u8,
        244u8,
        11u8,
        203u8,
        59u8,
        251u8,
        7u8,
        199u8,
        55u8,
        247u8,
    ],
    [
        136u8,
        72u8,
        184u8,
        120u8,
        132u8,
        68u8,
        180u8,
        116u8,
        139u8,
        75u8,
        187u8,
        123u8,
        135u8,
        71u8,
        183u8,
        119u8,
    ],
    [
        40u8,
        232u8,
        24u8,
        216u8,
        36u8,
        228u8,
        20u8,
        212u8,
        43u8,
        235u8,
        27u8,
        219u8,
        39u8,
        231u8,
        23u8,
        215u8,
    ],
    [
        168u8,
        104u8,
        152u8,
        88u8,
        164u8,
        100u8,
        148u8,
        84u8,
        171u8,
        107u8,
        155u8,
        91u8,
        167u8,
        103u8,
        151u8,
        87u8,
    ],
    [
        2u8,
        194u8,
        50u8,
        242u8,
        14u8,
        206u8,
        62u8,
        254u8,
        1u8,
        193u8,
        49u8,
        241u8,
        13u8,
        205u8,
        61u8,
        253u8,
    ],
    [
        130u8,
        66u8,
        178u8,
        114u8,
        142u8,
        78u8,
        190u8,
        126u8,
        129u8,
        65u8,
        177u8,
        113u8,
        141u8,
        77u8,
        189u8,
        125u8,
    ],
    [
        34u8,
        226u8,
        18u8,
        210u8,
        46u8,
        238u8,
        30u8,
        222u8,
        33u8,
        225u8,
        17u8,
        209u8,
        45u8,
        237u8,
        29u8,
        221u8,
    ],
    [
        162u8,
        98u8,
        146u8,
        82u8,
        174u8,
        110u8,
        158u8,
        94u8,
        161u8,
        97u8,
        145u8,
        81u8,
        173u8,
        109u8,
        157u8,
        93u8,
    ],
    [
        10u8,
        202u8,
        58u8,
        250u8,
        6u8,
        198u8,
        54u8,
        246u8,
        9u8,
        201u8,
        57u8,
        249u8,
        5u8,
        197u8,
        53u8,
        245u8,
    ],
    [
        138u8,
        74u8,
        186u8,
        122u8,
        134u8,
        70u8,
        182u8,
        118u8,
        137u8,
        73u8,
        185u8,
        121u8,
        133u8,
        69u8,
        181u8,
        117u8,
    ],
    [
        42u8,
        234u8,
        26u8,
        218u8,
        38u8,
        230u8,
        22u8,
        214u8,
        41u8,
        233u8,
        25u8,
        217u8,
        37u8,
        229u8,
        21u8,
        213u8,
    ],
    [
        170u8,
        106u8,
        154u8,
        90u8,
        166u8,
        102u8,
        150u8,
        86u8,
        169u8,
        105u8,
        153u8,
        89u8,
        165u8,
        101u8,
        149u8,
        85u8,
    ],
];
/* pointer to error array */
/* Private subobject */

pub const MAX_Q_COMPS: c_int = 4i32;
/*
 * Policy-making subroutines for create_colormap and create_colorindex.
 * These routines determine the colormap to be used.  The rest of the module
 * only assumes that the colormap is orthogonal.
 *
 *  * select_ncolors decides how to divvy up the available colors
 *    among the components.
 *  * output_value defines the set of representative values for a component.
 *  * largest_input_value defines the mapping from input values to
 *    representative values for a component.
 * Note that the latter two routines may impose different policies for
 * different components, though this is not currently done.
 */

unsafe extern "C" fn select_ncolors(
    mut cinfo: j_decompress_ptr,
    mut Ncolors: *mut c_int,
) -> c_int
/* Determine allocation of desired colors to components, */
/* and fill in Ncolors[] array to indicate choice. */
/* Return value is total number of colors (product of Ncolors[] values). */ {
       let mut i:  c_int =  0; let mut temp:  c_long =  0;let mut nc: c_int = (*cinfo).out_color_components; /* number of color components */
    let mut max_colors: c_int = (*cinfo).desired_number_of_colors;
    
    
    
    
    
    
    let mut RGB_order: [c_int; 3] = [
        RGB_GREEN,
        RGB_RED,
        RGB_BLUE,
    ];
    RGB_order[0] = rgb_green[(*cinfo).out_color_space as usize];
    RGB_order[1] = rgb_red[(*cinfo).out_color_space as usize];
    RGB_order[2] = rgb_blue[(*cinfo).out_color_space as usize];
     let mut iroot:   c_int =  1i32; /* repeat till iroot exceeds root */
    loop {
        iroot += 1; /* set temp = iroot ** nc */
        temp = iroot as c_long; /* now iroot = floor(root) */
        i = 1i32;
        while i < nc {
            temp *= iroot as c_long;
            i += 1
        }
        if !(temp <= max_colors as c_long) {
            break;
        }
    }
    iroot -= 1;
    /* Must have at least 2 color values per component */
    if iroot < 2i32 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_QUANT_FEW_COLORS as c_int;
        (*(*cinfo).err).msg_parm.i[0] = temp as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
     let mut total_colors:   c_int =  1i32;
    i = 0i32;
    while i < nc {
        *Ncolors.offset(i as isize) = iroot;
        total_colors *= iroot;
        i += 1
    }
    loop
    /* We may be able to increment the count for one or more components without
     * exceeding max_colors, though we know not all can be incremented.
     * Sometimes, the first component can be incremented more than once!
     * (Example: for 16 colors, we start at 2*2*2, go to 3*2*2, then 4*2*2.)
     * In RGB colorspace, try to increment G first, then R, then B.
     */
    {
          let mut changed:   boolean =  FALSE;
        i = 0i32;
        while i < nc {
              let mut j:   c_int =
     if  (*cinfo).out_color_space
                ==  JCS_RGB
            {
                RGB_order[i as usize]
            } else {
                i
            };
            /* calculate new total_colors if Ncolors[j] is incremented */
            temp = (total_colors / *Ncolors.offset(j as isize)) as c_long; /* done in long arith to avoid oflo */
            temp *= (*Ncolors.offset(j as isize) + 1i32) as c_long; /* won't fit, done with this pass */
            if temp > max_colors as c_long {
                break; /* OK, apply the increment */
            }
            let ref mut fresh0 = *Ncolors.offset(j as isize);
            *fresh0 += 1;
            total_colors = temp as c_int;
            changed = TRUE;
            i += 1
        }
        if !(changed != 0) {
            break;
        }
    }
    return total_colors;
}

unsafe extern "C" fn output_value(
    mut cinfo: j_decompress_ptr,
    mut ci: c_int,
    mut j: c_int,
    mut maxj: c_int,
) -> c_int
/* Return j'th output value, where j will range from 0 to maxj */
/* The output values must fall in 0..MAXJSAMPLE in increasing order */ {
    /* We always provide values 0 and MAXJSAMPLE for each component;
     * any additional values are equally spaced between these limits.
     * (Forcing the upper and lower values to the limits ensures that
     * dithering can't produce a color outside the selected gamut.)
     */
    return ((j as JLONG * MAXJSAMPLE as c_long
        + (maxj / 2i32) as c_long)
        / maxj as c_long) as c_int;
}

unsafe extern "C" fn largest_input_value(
    mut cinfo: j_decompress_ptr,
    mut ci: c_int,
    mut j: c_int,
    mut maxj: c_int,
) -> c_int
/* Return largest input value that should map to j'th output value */
/* Must have largest(j=0) >= 0, and largest(j=maxj) >= MAXJSAMPLE */ {
    /* Breakpoints are halfway between values returned by output_value */
    return (((2i32 * j + 1i32) as JLONG
        * MAXJSAMPLE as c_long
        + maxj as c_long)
        / (2i32 * maxj) as c_long) as c_int;
}
/*
 * Create the colormap.
 */

unsafe extern "C" fn create_colormap(mut cinfo: j_decompress_ptr) {
        let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* Created colormap */
      let mut total_colors:   c_int =
     select_ncolors(cinfo, (*cquantize).Ncolors.as_mut_ptr());
    /* Report selected color counts */
    if (*cinfo).out_color_components == 3i32 {
        let mut _mp: *mut c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
        *_mp.offset(0) = total_colors;
        *_mp.offset(1) = (*cquantize).Ncolors[0];
        *_mp.offset(2) = (*cquantize).Ncolors[1];
        *_mp.offset(3) = (*cquantize).Ncolors[2];
        (*(*cinfo).err).msg_code = super::jerror::JTRC_QUANT_3_NCOLORS as c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, 1i32);
    } else {
        (*(*cinfo).err).msg_code = super::jerror::JTRC_QUANT_NCOLORS as c_int;
        (*(*cinfo).err).msg_parm.i[0] = total_colors;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, 1i32);
    }
    
    
     let mut colormap:   JSAMPARRAY =
     Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        total_colors as JDIMENSION,
        (*cinfo).out_color_components as JDIMENSION,
    ); let mut blkdist:   c_int =  total_colors; let mut i:   c_int =  0i32;
    while i < (*cinfo).out_color_components {
          
        
         let mut nci:   c_int =  (*cquantize).Ncolors[i as usize]; let mut blksize:   c_int =  blkdist / nci; let mut j:   c_int =  0i32;
        while j < nci {
             
             let mut val:   c_int =  output_value(cinfo, i, j, nci - 1i32); let mut ptr:   c_int =  j * blksize;
            while ptr < total_colors {
                 let mut k:   c_int =  0i32;
                while k < blksize {
                    *(*colormap.offset(i as isize)).offset((ptr + k) as isize) =
                        val as JSAMPLE;
                    k += 1
                }
                ptr += blkdist
            }
            j += 1
        }
        blkdist = blksize;
        i += 1
    }
    /* Save the colormap in private storage,
     * where it will survive color quantization mode changes.
     */
    (*cquantize).sv_colormap = colormap;
    (*cquantize).sv_actual = total_colors;
}
/*
 * Create the color index table.
 */

unsafe extern "C" fn create_colorindex(mut cinfo: j_decompress_ptr) {
       let mut pad:  c_int =  0;let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    
    
    
    
    
    
    
    
    /* For ordered dither, we pad the color index tables by MAXJSAMPLE in
     * each direction (input index values can be -MAXJSAMPLE .. 2*MAXJSAMPLE).
     * This is not necessary in the other dithering modes.  However, we
     * flag whether it was done in case user changes dithering mode.
     */
    if  (*cinfo).dither_mode
        ==  JDITHER_ORDERED
    {
        pad = MAXJSAMPLE * 2i32;
        (*cquantize).is_padded = TRUE
    } else {
        pad = 0i32;
        (*cquantize).is_padded = FALSE
    }
    (*cquantize).colorindex = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (MAXJSAMPLE + 1i32 + pad) as JDIMENSION,
        (*cinfo).out_color_components as JDIMENSION,
    );
    
     let mut blksize:   c_int =  (*cquantize).sv_actual; let mut i:   c_int =  0i32;
    while i < (*cinfo).out_color_components {
             let mut nci:   c_int =  (*cquantize).Ncolors[i as usize]; /* # of distinct values for this color */
        blksize = blksize / nci;
        /* adjust colorindex pointers to provide padding at negative indexes. */
        if pad != 0 {
            let ref mut fresh1 = *(*cquantize).colorindex.offset(i as isize);
            *fresh1 = (*fresh1).offset(MAXJSAMPLE as isize)
        }
        
        
        
         let mut indexptr:   JSAMPROW =
     *(*cquantize).colorindex.offset(i as isize); let mut val:   c_int =  0i32; let mut k:   c_int =  largest_input_value(cinfo, i, 0i32, nci - 1i32); let mut j:   c_int =  0i32;
        while j <= MAXJSAMPLE {
            while j > k {
                /* advance val if past boundary */
                val += 1;
                k = largest_input_value(cinfo, i, val, nci - 1i32)
            }
            /* premultiply so that no multiplication needed in main processing */
            *indexptr.offset(j as isize) = (val * blksize) as JSAMPLE;
            j += 1
        }
        /* Pad at both ends if necessary */
        if pad != 0 {
            j = 1i32;
            while j <= MAXJSAMPLE {
                *indexptr.offset(-j as isize) = *indexptr.offset(0);
                *indexptr.offset((MAXJSAMPLE + j) as isize) =
                    *indexptr.offset(MAXJSAMPLE as isize);
                j += 1
            }
        }
        i += 1
    }
}
/*
 * Create an ordered-dither array for a component having ncolors
 * distinct output values.
 */

unsafe extern "C" fn make_odither_array(
    mut cinfo: j_decompress_ptr,
    mut ncolors: c_int,
) -> ODITHER_MATRIX_PTR {
    
    
    
    
       
    
    /* The inter-value distance for this color is MAXJSAMPLE/(ncolors-1).
     * Hence the dither value for the matrix cell with fill order f
     * (f=0..N-1) should be (N-1-2*f)/(2*N) * MAXJSAMPLE/(ncolors-1).
     * On 16-bit-int machine, be careful to avoid overflow.
     */
    
     let mut odither:   ODITHER_MATRIX_PTR =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<ODITHER_MATRIX>() as c_ulong,
    ) as ODITHER_MATRIX_PTR; let mut den:   JLONG =
     (2i32 * ODITHER_CELLS) as c_long * (ncolors - 1i32) as JLONG; let mut j:   c_int =  0i32;
    while j < ODITHER_SIZE {
          let mut k:   c_int =  0i32;
        while k < ODITHER_SIZE {
              let mut num:   JLONG =
     (ODITHER_CELLS
                - 1i32
                - 2i32 * base_dither_matrix[j as usize][k as usize] as c_int)
                as JLONG
                * MAXJSAMPLE as c_long;
            /* Ensure round towards zero despite C's lack of consistency
             * about rounding negative values in integer division...
             */
            (*odither.offset(j as isize))[k as usize] = if num < 0i64 {
                -(-num / den)
            } else {
                (num) / den
            } as c_int;
            k += 1
        }
        j += 1
    }
    return odither;
}
/*
 * Create the ordered-dither tables.
 * Components having the same number of representative colors may
 * share a dither table.
 */

unsafe extern "C" fn create_odither_tables(mut cinfo: j_decompress_ptr) {
     let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* # of distinct values for this color */
      let mut i:   c_int =  0i32;
    while i < (*cinfo).out_color_components {
           
        
         let mut nci:   c_int =  (*cquantize).Ncolors[i as usize]; let mut odither:   ODITHER_MATRIX_PTR =
     NULL as ODITHER_MATRIX_PTR; let mut j:   c_int =  0i32;
        while j < i {
            if nci == (*cquantize).Ncolors[j as usize] {
                odither = (*cquantize).odither[j as usize];
                break;
            } else {
                j += 1
            }
        }
        if odither.is_null() {
            /* need a new table? */
            odither = make_odither_array(cinfo, nci)
        }
        (*cquantize).odither[i as usize] = odither;
        i += 1
    }
}
/*
 * Map some rows of pixels to the output colormapped representation.
 */

unsafe extern "C" fn color_quantize(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
)
/* General case, no dithering */
{
     let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut colorindex: JSAMPARRAY = (*cquantize).colorindex;
    
    
    
    
    
    
    let mut width: JDIMENSION = (*cinfo).output_width;
    let mut nc: c_int = (*cinfo).out_color_components;
     let mut row:   c_int =  0i32;
    while row < num_rows {
           
        
         let mut ptrin:   JSAMPROW =  *input_buf.offset(row as isize); let mut ptrout:   JSAMPROW =  *output_buf.offset(row as isize); let mut col:   JDIMENSION =  width;
        while col > 0u32 {
              
             let mut pixcode:   c_int =  0i32; let mut ci:   c_int =  0i32;
            while ci < nc {
                let fresh2 = ptrin;
                ptrin = ptrin.offset(1);
                pixcode += *(*colorindex.offset(ci as isize))
                    .offset(*fresh2 as c_int as isize)
                    as c_int;
                ci += 1
            }
            let fresh3 = ptrout;
            ptrout = ptrout.offset(1);
            *fresh3 = pixcode as JSAMPLE;
            col -=  1
        }
        row += 1
    }
}

unsafe extern "C" fn color_quantize3(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
)
/* Fast path for out_color_components==3, no dithering */
{
     let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    
    
    
    let mut colorindex0: JSAMPROW = *(*cquantize).colorindex.offset(0);
    let mut colorindex1: JSAMPROW = *(*cquantize).colorindex.offset(1);
    let mut colorindex2: JSAMPROW = *(*cquantize).colorindex.offset(2);
    
    
    let mut width: JDIMENSION = (*cinfo).output_width;
     let mut row:   c_int =  0i32;
    while row < num_rows {
           
        
         let mut ptrin:   JSAMPROW =  *input_buf.offset(row as isize); let mut ptrout:   JSAMPROW =  *output_buf.offset(row as isize); let mut col:   JDIMENSION =  width;
        while col > 0u32 {
             let fresh4 = ptrin;
            ptrin = ptrin.offset(1);
             let mut pixcode:   c_int =
     *colorindex0.offset(*fresh4 as c_int as isize) as c_int;
            let fresh5 = ptrin;
            ptrin = ptrin.offset(1);
            pixcode += *colorindex1.offset(*fresh5 as c_int as isize) as c_int;
            let fresh6 = ptrin;
            ptrin = ptrin.offset(1);
            pixcode += *colorindex2.offset(*fresh6 as c_int as isize) as c_int;
            let fresh7 = ptrout;
            ptrout = ptrout.offset(1);
            *fresh7 = pixcode as JSAMPLE;
            col -=  1
        }
        row += 1
    }
}

unsafe extern "C" fn quantize_ord_dither(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
)
/* General case, with ordered dithering */
{
     let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* points to active row of dither matrix */
     /* current indexes into dither matrix */
    
    
    
    
    
    let mut nc: c_int = (*cinfo).out_color_components;
    
    
    
    let mut width: JDIMENSION = (*cinfo).output_width;
     let mut row:   c_int =  0i32;
    while row < num_rows {
         jzero_far(
            *output_buf.offset(row as isize) as *mut c_void,
            width as c_ulong *
    ::std::mem::size_of::<JSAMPLE>() as c_ulong,
        );
        
         let mut row_index:   c_int =  (*cquantize).row_index; let mut ci:   c_int =  0i32;
        while ci < nc {
                  
            
            
            
            
             let mut input_ptr:   JSAMPROW =
     (*input_buf.offset(row as isize)).offset(ci as isize); let mut output_ptr:   JSAMPROW =
     *output_buf.offset(row as isize); let mut colorindex_ci:   JSAMPROW =
     *(*cquantize).colorindex.offset(ci as isize); let mut dither:   *mut c_int =
     (*(*cquantize).odither[ci as usize].offset(row_index as isize)).as_mut_ptr(); let mut col_index:   c_int =  0i32; let mut col:   JDIMENSION =  width;
            while col > 0u32 {
                /* Form pixel value + dither, range-limit to 0..MAXJSAMPLE,
                 * select output value, accumulate into output code for this pixel.
                 * Range-limiting need not be done explicitly, as we have extended
                 * the colorindex table to produce the right answers for out-of-range
                 * inputs.  The maximum dither is +- MAXJSAMPLE; this sets the
                 * required amount of padding.
                 */
                *output_ptr = (*output_ptr as c_int
                    + *colorindex_ci.offset(
                        (*input_ptr as c_int + *dither.offset(col_index as isize)) as isize,
                    ) as c_int) as JSAMPLE;
                input_ptr = input_ptr.offset(nc as isize);
                output_ptr = output_ptr.offset(1);
                col_index = col_index + 1i32 & ODITHER_MASK;
                col -=  1
            }
            ci += 1
        }
        /* Advance row index for next row */
        row_index = row_index + 1i32 & ODITHER_MASK;
        (*cquantize).row_index = row_index;
        row += 1
    }
}

unsafe extern "C" fn quantize3_ord_dither(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
)
/* Fast path for out_color_components==3, with ordered dithering */
{
     let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* points to active row of dither matrix */
     /* current indexes into dither matrix */
    
    
    let mut colorindex0: JSAMPROW = *(*cquantize).colorindex.offset(0);
    let mut colorindex1: JSAMPROW = *(*cquantize).colorindex.offset(1);
    let mut colorindex2: JSAMPROW = *(*cquantize).colorindex.offset(2);
    
    
    
    
    
    
    
    let mut width: JDIMENSION = (*cinfo).output_width;
     let mut row:   c_int =  0i32;
    while row < num_rows {
                
        
        
        
        
        
        
         let mut row_index:   c_int =  (*cquantize).row_index; let mut input_ptr:   JSAMPROW =
     *input_buf.offset(row as isize); let mut output_ptr:   JSAMPROW =
     *output_buf.offset(row as isize); let mut dither0:   *mut c_int =
     (*(*cquantize).odither[0].offset(row_index as isize)).as_mut_ptr(); let mut dither1:   *mut c_int =
     (*(*cquantize).odither[1].offset(row_index as isize)).as_mut_ptr(); let mut dither2:   *mut c_int =
     (*(*cquantize).odither[2].offset(row_index as isize)).as_mut_ptr(); let mut col_index:   c_int =  0i32; let mut col:   JDIMENSION =  width;
        while col > 0u32 {
             let fresh8 = input_ptr;
            input_ptr = input_ptr.offset(1);
             let mut pixcode:   c_int =
     *colorindex0
                .offset((*fresh8 as c_int + *dither0.offset(col_index as isize)) as isize)
                as c_int;
            let fresh9 = input_ptr;
            input_ptr = input_ptr.offset(1);
            pixcode += *colorindex1
                .offset((*fresh9 as c_int + *dither1.offset(col_index as isize)) as isize)
                as c_int;
            let fresh10 = input_ptr;
            input_ptr = input_ptr.offset(1);
            pixcode += *colorindex2
                .offset((*fresh10 as c_int + *dither2.offset(col_index as isize)) as isize)
                as c_int;
            let fresh11 = output_ptr;
            output_ptr = output_ptr.offset(1);
            *fresh11 = pixcode as JSAMPLE;
            col_index = col_index + 1i32 & ODITHER_MASK;
            col -=  1
        }
        row_index = row_index + 1i32 & ODITHER_MASK;
        (*cquantize).row_index = row_index;
        row += 1
    }
}

unsafe extern "C" fn quantize_fs_dither(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
)
/* General case, with Floyd-Steinberg dithering */
{
     let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* current error or pixel value */
     /* error for pixel below cur */
     /* error for below/prev col */
     /* error for below/next col */
     /* => fserrors[] at column before current */
     /* 1 for left-to-right, -1 for right-to-left */
     /* dir * nc */
    
    
    
    
    
    let mut nc: c_int = (*cinfo).out_color_components;
    
    
    
    
    
    let mut width: JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
     let mut row:   c_int =  0i32;
    while row < num_rows {
        jzero_far(
            *output_buf.offset(row as isize) as *mut c_void,
            width as c_ulong *
    ::std::mem::size_of::<JSAMPLE>() as c_ulong,
        );
         let mut ci:   c_int =  0i32;
        while ci < nc {
                let mut errorptr:  FSERRPTR =  ::std::ptr::null_mut::< FSERROR>();     let mut dir:  c_int =  0; let mut dirnc:  c_int =  0; 
             let mut input_ptr:   JSAMPROW =
     (*input_buf.offset(row as isize)).offset(ci as isize); let mut output_ptr:   JSAMPROW =
     *output_buf.offset(row as isize);
            if (*cquantize).on_odd_row != 0 {
                /* unload prev err into array */
                /* work right to left in this row */
                input_ptr = input_ptr.offset(
                    ((
                    width - 1u32) * nc as c_uint) as isize,
                );
                output_ptr = output_ptr.offset((width - 1u32) as isize);
                dir = -1i32;
                dirnc = -nc;
                errorptr = (*cquantize).fserrors[ci as usize]
                    .offset((width + 1u32) as isize) /* so point to rightmost pixel */
            /* => entry after last column */
            } else {
                /* work left to right in this row */
                dir = 1i32;
                dirnc = nc;
                errorptr = (*cquantize).fserrors[ci as usize]
                /* => entry before first column */
            }
            
            
            
            
            
             let mut colorindex_ci:   JSAMPROW =
     *(*cquantize).colorindex.offset(ci as isize); let mut colormap_ci:   JSAMPROW =
     *(*cquantize).sv_colormap.offset(ci as isize); let mut cur:   LOCFSERROR =  0i32; let mut bpreverr:   LOCFSERROR =  0i32; let mut belowerr:   LOCFSERROR =  bpreverr; let mut col:   JDIMENSION =  width;
            while col > 0u32 {
                /* Preset error values: no error propagated to first pixel from left */
                /* and no error propagated to row below yet */
                /* cur holds the error propagated from the previous pixel on the
                 * current line.  Add the error propagated from the previous line
                 * to form the complete error correction term for this pixel, and
                 * round the error term (which is expressed * 16) to an integer.
                 * RIGHT_SHIFT rounds towards minus infinity, so adding 8 is correct
                 * for either sign of the error value.
                 * Note: errorptr points to *previous* column's array entry.
                 */
                   cur = cur + *errorptr.offset(dir as isize) as c_int + 8i32 >> 4i32;
                /* advance errorptr to current column */
                cur += *input_ptr as c_int;
                cur = *range_limit.offset(cur as isize) as c_int;
                 let mut pixcode:   c_int =
     *colorindex_ci.offset(cur as isize) as c_int;
                *output_ptr = (*output_ptr as c_int
                    + pixcode as JSAMPLE as c_int)
                    as JSAMPLE;
                cur -= *colormap_ci.offset(pixcode as isize) as c_int;
                
                 let mut bnexterr:   LOCFSERROR =  cur; let mut delta:   LOCFSERROR =  cur * 2i32;
                cur += delta;
                *errorptr.offset(0) = (bpreverr + cur) as FSERROR;
                cur += delta;
                bpreverr = belowerr + cur;
                belowerr = bnexterr;
                cur += delta;
                input_ptr = input_ptr.offset(dirnc as isize);
                output_ptr = output_ptr.offset(dir as isize);
                errorptr = errorptr.offset(dir as isize);
                col -=  1
            }
            *errorptr.offset(0) = bpreverr as FSERROR;
            ci += 1
        }
        (*cquantize).on_odd_row = if (*cquantize).on_odd_row != 0 {
            FALSE
        } else {
            TRUE
        };
        row += 1
    }
}
/* Form pixel value + error, and range-limit to 0..MAXJSAMPLE.
 * The maximum error is +- MAXJSAMPLE; this sets the required size
 * of the range_limit array.
 */
/* Select output value, accumulate into output code for this pixel */
/* Compute actual representation error at this pixel */
/* Note: we can do this even though we don't have the final */
/* pixel code, because the colormap is orthogonal. */
/* Compute error fractions to be propagated to adjacent pixels.
 * Add these into the running sums, and simultaneously shift the
 * next-line error sums left by 1 column.
 */
/* form error * 3 */
/* form error * 5 */
/* form error * 7 */
/* At this point cur contains the 7/16 error value to be propagated
 * to the next pixel on the current line, and all the errors for the
 * next line have been shifted over. We are therefore ready to move on.
 */
/* advance input ptr to next column */
/* advance output ptr to next column */
/* Post-loop cleanup: we must unload the final error value into the
 * final fserrors[] entry.  Note we need not unload belowerr because
 * it is for the dummy column before or after the actual array.
 */
/*
 * Allocate workspace for Floyd-Steinberg errors.
 */

unsafe extern "C" fn alloc_fs_workspace(mut cinfo: j_decompress_ptr) {
      let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    
    
    
     let mut arraysize:   size_t =
     ((*cinfo).output_width + 2u32) as c_ulong *
    ::std::mem::size_of::<FSERROR>() as c_ulong; let mut i:   c_int =  0i32;
    while i < (*cinfo).out_color_components {
        (*cquantize).fserrors[i as usize] = Some(
            (*(*cinfo).mem)
                .alloc_large
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            arraysize,
        ) as FSERRPTR;
        i += 1
    }
}
/*
 * Initialize for one-pass color quantization.
 */

unsafe extern "C" fn start_pass_1_quant(
    mut cinfo: j_decompress_ptr,
    mut is_pre_scan: boolean,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    
    
    /* Install my colormap. */
    (*cinfo).colormap = (*cquantize).sv_colormap;
    (*cinfo).actual_number_of_colors = (*cquantize).sv_actual;
    /* Initialize for desired dithering mode. */
    match  (*cinfo).dither_mode {
        0 => {
            if (*cinfo).out_color_components == 3i32 {
                (*cquantize).pub_0.color_quantize = Some(
                    color_quantize3
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                )
            } else {
                (*cquantize).pub_0.color_quantize = Some(
                    color_quantize
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                )
            }
        }
        1 => {
            if (*cinfo).out_color_components == 3i32 {
                (*cquantize).pub_0.color_quantize = Some(
                    quantize3_ord_dither
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                )
            } else {
                (*cquantize).pub_0.color_quantize = Some(
                    quantize_ord_dither
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPARRAY,
                            _: c_int,
                        ) -> (),
                )
            } /* initialize state for ordered dither */
            (*cquantize).row_index = 0i32;
            /* If user changed to ordered dither from another mode,
             * we must recreate the color index table with padding.
             * This will cost extra space, but probably isn't very likely.
             */
            if (*cquantize).is_padded == 0 {
                create_colorindex(cinfo);
            }
            /* Create ordered-dither tables if we didn't already. */
            if (*cquantize).odither[0].is_null() {
                create_odither_tables(cinfo); /* initialize state for F-S dither */
            }
        }
        2 => {
              (*cquantize).pub_0.color_quantize = Some(
                quantize_fs_dither
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: JSAMPARRAY,
                        _: JSAMPARRAY,
                        _: c_int,
                    ) -> (),
            );
            (*cquantize).on_odd_row = FALSE;
            /* Allocate Floyd-Steinberg workspace if didn't already. */
            if (*cquantize).fserrors[0].is_null() {
                alloc_fs_workspace(cinfo);
            }
            
             let mut arraysize:   size_t =
     ((*cinfo).output_width + 2u32) as c_ulong *
    ::std::mem::size_of::<FSERROR>() as c_ulong; let mut i:   c_int =  0i32;
            while i < (*cinfo).out_color_components {
                jzero_far(
                    (*cquantize).fserrors[i as usize] as *mut c_void,
                    arraysize,
                );
                i += 1
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
    };
}
/*
 * Finish up at the end of the pass.
 */

unsafe extern "C" fn finish_pass_1_quant(mut cinfo: j_decompress_ptr) {
    /* no work in 1-pass case */
}
/*
 * Switch to a new external colormap between output passes.
 * Shouldn't get to this module!
 */

unsafe extern "C" fn new_color_map_1_quant(mut cinfo: j_decompress_ptr) {
    (*(*cinfo).err).msg_code = super::jerror::JERR_MODE_CHANGE as c_int;
    Some(
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as j_common_ptr);
}
/*
 * Module initialization routine for 1-pass color quantization.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_1pass_quantizer(mut cinfo: j_decompress_ptr) {
      /* Flag FS workspace not allocated */
     let mut cquantize:   my_cquantize_ptr =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<my_cquantizer>() as c_ulong,
    ) as my_cquantize_ptr; /* Also flag odither arrays not allocated */
    (*cinfo).cquantize = cquantize as *mut jpeg_color_quantizer;
    (*cquantize).pub_0.start_pass = Some(
        start_pass_1_quant
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
                _: boolean,
            ) -> (),
    );
    (*cquantize).pub_0.finish_pass = Some(
        finish_pass_1_quant as unsafe extern "C" fn(_: j_decompress_ptr) -> (),
    );
    (*cquantize).pub_0.new_color_map = Some(
        new_color_map_1_quant as unsafe extern "C" fn(_: j_decompress_ptr) -> (),
    );
    (*cquantize).fserrors[0] = NULL as FSERRPTR;
    (*cquantize).odither[0] = NULL as ODITHER_MATRIX_PTR;
    /* Make sure my internal arrays won't overflow */
    if (*cinfo).out_color_components > MAX_Q_COMPS {
        (*(*cinfo).err).msg_code = super::jerror::JERR_QUANT_COMPONENTS as c_int;
        (*(*cinfo).err).msg_parm.i[0] = 4i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Make sure colormap indexes can be represented by JSAMPLEs */
    if (*cinfo).desired_number_of_colors > MAXJSAMPLE + 1i32 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_QUANT_MANY_COLORS as c_int;
        (*(*cinfo).err).msg_parm.i[0] = 255i32 + 1i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Create the colormap and color index table. */
    create_colormap(cinfo);
    create_colorindex(cinfo);
    /* Allocate Floyd-Steinberg workspace now if requested.
     * We do this now since it may affect the memory manager's space
     * calculations.  If the user changes to FS dither mode in a later pass, we
     * will allocate the space then, and will possibly overrun the
     * max_memory_to_use setting.
     */
    if  (*cinfo).dither_mode
        ==  JDITHER_FS
    {
        alloc_fs_workspace(cinfo);
    };
}
/* QUANT_1PASS_SUPPORTED */
