#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_transmute)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(ptr_wrapping_offset_from)]
#![feature(main)]


use libc::c_long;use libc::c_uint;use libc::c_int;use libc::c_ulong;use libc::c_void;use libc::c_uchar;use std::prelude::v1;use libc::c_char;use mozjpeg::*;


pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::jconfig_h::JPEG_LIB_VERSION;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_CreateCompress;
pub use crate::jpeglib_h::jpeg_CreateDecompress;
pub use crate::jpeglib_h::jpeg_c_coef_controller;
pub use crate::jpeglib_h::jpeg_c_get_int_param;
pub use crate::jpeglib_h::jpeg_c_int_param_supported;
pub use crate::jpeglib_h::jpeg_c_main_controller;
pub use crate::jpeglib_h::jpeg_c_prep_controller;
pub use crate::jpeglib_h::jpeg_c_set_bool_param;
pub use crate::jpeglib_h::jpeg_c_set_int_param;
pub use crate::jpeglib_h::jpeg_color_converter;
pub use crate::jpeglib_h::jpeg_color_deconverter;
pub use crate::jpeglib_h::jpeg_color_quantizer;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_comp_master;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_copy_critical_parameters;
pub use crate::jpeglib_h::jpeg_d_coef_controller;
pub use crate::jpeglib_h::jpeg_d_main_controller;
pub use crate::jpeglib_h::jpeg_d_post_controller;
pub use crate::jpeglib_h::jpeg_decomp_master;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_destroy_compress;
pub use crate::jpeglib_h::jpeg_destroy_decompress;
pub use crate::jpeglib_h::jpeg_downsampler;
pub use crate::jpeglib_h::jpeg_entropy_decoder;
pub use crate::jpeglib_h::jpeg_entropy_encoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_finish_compress;
pub use crate::jpeglib_h::jpeg_finish_decompress;
pub use crate::jpeglib_h::jpeg_forward_dct;
pub use crate::jpeglib_h::jpeg_input_controller;
pub use crate::jpeglib_h::jpeg_inverse_dct;
pub use crate::jpeglib_h::jpeg_marker_reader;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_marker_writer;
pub use crate::jpeglib_h::jpeg_mem_dest;
pub use crate::jpeglib_h::jpeg_mem_src;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_read_coefficients;
pub use crate::jpeglib_h::jpeg_read_header;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jpeg_simple_progression;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jpeg_std_error;
pub use crate::jpeglib_h::jpeg_stdio_dest;
pub use crate::jpeglib_h::jpeg_stdio_src;
pub use crate::jpeglib_h::jpeg_upsampler;
pub use crate::jpeglib_h::jpeg_write_coefficients;
pub use crate::jpeglib_h::jpeg_write_icc_profile;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_1;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JBOOLEAN_OPTIMIZE_SCANS;
pub use crate::jpeglib_h::JBOOLEAN_OVERSHOOT_DERINGING;
pub use crate::jpeglib_h::JBOOLEAN_TRELLIS_EOB_OPT;
pub use crate::jpeglib_h::JBOOLEAN_TRELLIS_QUANT;
pub use crate::jpeglib_h::JBOOLEAN_TRELLIS_QUANT_DC;
pub use crate::jpeglib_h::JBOOLEAN_TRELLIS_Q_OPT;
pub use crate::jpeglib_h::JBOOLEAN_USE_LAMBDA_WEIGHT_TBL;
pub use crate::jpeglib_h::JBOOLEAN_USE_SCANS_IN_TRELLIS;
pub use crate::jpeglib_h::JCP_FASTEST;
pub use crate::jpeglib_h::JCP_MAX_COMPRESSION;
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
pub use crate::jpeglib_h::JINT_BASE_QUANT_TBL_IDX;
pub use crate::jpeglib_h::JINT_COMPRESS_PROFILE;
pub use crate::jpeglib_h::JINT_DC_SCAN_OPT_MODE;
pub use crate::jpeglib_h::JINT_TRELLIS_FREQ_SPLIT;
pub use crate::jpeglib_h::JINT_TRELLIS_NUM_LOOPS;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_BOOLEAN_PARAM;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::jpeglib_h::J_INT_PARAM;
pub use crate::src::cdjpeg::keymatch;
pub use crate::src::cdjpeg::read_scan_script;
pub use crate::src::cdjpeg::read_stdin;
pub use crate::src::cdjpeg::write_stdout;
pub use crate::src::cdjpeg::EXIT_WARNING;
pub use crate::src::cdjpeg::READ_BINARY;
pub use crate::src::cdjpeg::WRITE_BINARY;
pub use crate::src::transupp::jcopy_markers_execute;
pub use crate::src::transupp::jcopy_markers_setup;
pub use crate::src::transupp::jpeg_transform_info;
pub use crate::src::transupp::jtransform_adjust_parameters;
pub use crate::src::transupp::jtransform_execute_transform;
pub use crate::src::transupp::jtransform_execute_transformation;
pub use crate::src::transupp::jtransform_parse_crop_spec;
pub use crate::src::transupp::jtransform_request_workspace;
pub use crate::src::transupp::JCOPYOPT_ALL;
pub use crate::src::transupp::JCOPYOPT_ALL_EXCEPT_ICC;
pub use crate::src::transupp::JCOPYOPT_COMMENTS;
pub use crate::src::transupp::JCOPYOPT_DEFAULT;
pub use crate::src::transupp::JCOPYOPT_NONE;
pub use crate::src::transupp::JCOPY_OPTION;
pub use crate::src::transupp::JCROP_CODE;
pub use crate::src::transupp::JCROP_FORCE;
pub use crate::src::transupp::JCROP_NEG;
pub use crate::src::transupp::JCROP_POS;
pub use crate::src::transupp::JCROP_UNSET;
pub use crate::src::transupp::JXFORM_CODE;
pub use crate::src::transupp::JXFORM_FLIP_H;
pub use crate::src::transupp::JXFORM_FLIP_V;
pub use crate::src::transupp::JXFORM_NONE;
pub use crate::src::transupp::JXFORM_ROT_180;
pub use crate::src::transupp::JXFORM_ROT_270;
pub use crate::src::transupp::JXFORM_ROT_90;
pub use crate::src::transupp::JXFORM_TRANSPOSE;
pub use crate::src::transupp::JXFORM_TRANSVERSE;
pub use crate::stdlib::exit;
pub use crate::stdlib::fclose;
pub use crate::stdlib::ferror;
pub use crate::stdlib::fopen;
pub use crate::stdlib::fprintf;
pub use crate::stdlib::fread;
pub use crate::stdlib::free;
pub use crate::stdlib::fseek;
pub use crate::stdlib::ftell;
pub use crate::stdlib::fwrite;
pub use crate::stdlib::malloc;
pub use crate::stdlib::realloc;
pub use crate::stdlib::sscanf;
pub use crate::stdlib::stderr;
pub use crate::stdlib::stdin;
pub use crate::stdlib::stdout;
pub use crate::stdlib::EXIT_FAILURE;
pub use crate::stdlib::EXIT_SUCCESS;
pub use crate::stdlib::SEEK_END;
pub use crate::stdlib::SEEK_SET;

pub use crate::jconfigint_h::BUILD;
pub use crate::jconfigint_h::PACKAGE_NAME;
pub use crate::jconfigint_h::VERSION;
pub use crate::jversion_h::JCOPYRIGHT;
pub use crate::jversion_h::JVERSION;
/*
 * jpegtran.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1995-2010, Thomas G. Lane, Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2010, 2014, 2017, D. R. Commander.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains a command-line user interface for JPEG transcoding.
 * It is very similar to cjpeg.c, and partly to djpeg.c, but provides
 * lossless transcoding between different JPEG file formats.  It also
 * provides some lossless and sort-of-lossless transformations of JPEG data.
 */
/* command-line reader for Macintosh */
/*
 * Argument-parsing code.
 * The switch parser is designed to be useful with DOS-style command line
 * syntax, ie, intermixed switches and file names, where only the switches
 * to the left of a given file name affect processing of that file.
 * The main program in this file doesn't actually use this capability...
 */

static mut progname: *const c_char = ::std::ptr::null::< c_char>();
/* program name for error messages */

static mut icc_filename: *mut c_char = ::std::ptr::null::< c_char>() as *mut c_char;
/* for -icc switch */

static mut outfilename: *mut c_char = ::std::ptr::null::< c_char>() as *mut c_char;
/* for -outfile switch */

static mut prefer_smallest: boolean = 0;
/* use smallest of input or result file (if no image-changing options supplied) */

static mut copyoption: JCOPY_OPTION = JCOPYOPT_NONE;
/* -copy switch */

static mut transformoption: jpeg_transform_info =
    jpeg_transform_info{transform:  JXFORM_NONE,
                    perfect:  0,
                    trim:  0,
                    force_grayscale:  0,
                    crop:  0,
                    slow_hflip:  0,
                    crop_width:  0,
                    crop_width_set:  JCROP_UNSET,
                    crop_height:  0,
                    crop_height_set:  JCROP_UNSET,
                    crop_xoffset:  0,
                    crop_xoffset_set:  JCROP_UNSET,
                    crop_yoffset:  0,
                    crop_yoffset_set:  JCROP_UNSET,
                    num_components:  0,
                    workspace_coef_arrays:
                         ::std::ptr::null::< jvirt_barray_ptr>()
            as *mut jvirt_barray_ptr,
                    output_width:  0,
                    output_height:  0,
                    x_crop_offset:  0,
                    y_crop_offset:  0,
                    iMCU_sample_width:  0,
                    iMCU_sample_height:  0,};
/* image transformation options */
#[no_mangle]

pub static mut memsrc: boolean = FALSE;
/* for -memsrc switch */

pub const INPUT_BUF_SIZE: c_int = 4096i32;

unsafe extern "C" fn usage()
/* complain about bad command line */
{
    fprintf(
        stderr,
        
        b"usage: %s [switches] \x00".as_ptr() as *const c_char,
        progname,
    );
    fprintf(
        stderr,
        
        b"[inputfile]\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"Switches (names may be abbreviated):\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -copy none     Copy no extra markers from source file\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -copy comments Copy only comment markers (default)\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -copy all      Copy all extra markers\n\x00".as_ptr() as *const c_char,
    );
    fprintf(stderr,
            
            b"  -optimize      Optimize Huffman table (smaller file, but slow compression, enabled by default)\n\x00".as_ptr() as *const c_char);
    fprintf(
        stderr,
        
        b"  -progressive   Create progressive JPEG file (enabled by default)\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -revert        Revert to standard defaults (instead of mozjpeg defaults)\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -fastcrush     Disable progressive scan optimization\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"Switches for modifying the image:\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -crop WxH+X+Y  Crop to a rectangular subarea\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -grayscale     Reduce to grayscale (omit color data)\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -flip [horizontal|vertical]  Mirror image (left-right or top-bottom)\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -perfect       Fail if there is non-transformable edge blocks\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -rotate [90|180|270]         Rotate image (degrees clockwise)\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -transpose     Transpose image\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -transverse    Transverse transpose image\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -trim          Drop non-transformable edge blocks\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"Switches for advanced users:\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -icc FILE      Embed ICC profile contained in FILE\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -restart N     Set restart interval in rows, or in blocks with B\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -maxmemory N   Maximum memory to use (in kbytes)\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -outfile name  Specify name for output file\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -verbose  or  -debug   Emit debug output\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -version       Print version information and exit\n\x00".as_ptr()
            as *const c_char,
    );
    fprintf(
        stderr,
        
        b"Switches for wizards:\n\x00".as_ptr() as *const c_char,
    );
    fprintf(
        stderr,
        
        b"  -scans FILE    Create multi-scan JPEG per script FILE\n\x00".as_ptr()
            as *const c_char,
    );
    exit(EXIT_FAILURE);
}

unsafe extern "C" fn select_transform(mut transform: JXFORM_CODE)
/* Silly little routine to detect multiple transform options,
 * which we can't handle.
 */
{
    if  transformoption.transform
        ==  JXFORM_NONE
        ||  transformoption.transform ==  transform
    {
        transformoption.transform = transform
    } else {
        fprintf(
            stderr,
            
            b"%s: can only do one image transformation at a time\n\x00".as_ptr()
                as *const c_char,
            progname,
        );
        usage();
    };
}

unsafe extern "C" fn parse_switches(
    mut cinfo: j_compress_ptr,
    mut argc: c_int,
    mut argv: *mut *mut c_char,
    mut last_file_arg_seen: c_int,
    mut for_real: boolean,
) -> c_int
/* Parse optional switches.
 * Returns argv[] index of first file-name argument (== argc if none).
 * Any file names with indexes <= last_file_arg_seen are ignored;
 * they have presumably been processed in a previous iteration.
 * (Pass 0 for last_file_arg_seen on the first or only iteration.)
 * for_real is FALSE on the first (dummy) pass; we may skip any expensive
 * processing.
 */ {
      
    let mut scansarg: *mut c_char = NULL as *mut c_char;
     let mut simple_progressive:   boolean =
     if (*cinfo).num_scans == 0i32 {
        FALSE
    } else {
        TRUE
    };
    icc_filename = NULL as *mut c_char;
    outfilename = NULL as *mut c_char;
    copyoption = JCOPYOPT_DEFAULT as JCOPY_OPTION;
    transformoption.transform = JXFORM_NONE;
    transformoption.perfect = FALSE;
    transformoption.trim = FALSE;
    transformoption.force_grayscale = FALSE;
    transformoption.crop = FALSE;
    transformoption.slow_hflip = FALSE;
    (*(*cinfo).err).trace_level = 0i32;
    prefer_smallest = TRUE;
     let mut argn:   c_int =  1i32;
    while argn < argc {
          let mut arg:   *mut c_char =  *argv.offset(argn as isize);
        if *arg as c_int != '-' as i32 {
            /* Not a switch, must be a file name argument */
            if !(argn <= last_file_arg_seen) {
                break; /* -outfile applies to just one input file */
            }
            outfilename = NULL as *mut c_char
        /* ignore this name if previously processed */
        /* else done parsing switches */
        } else {
            arg = arg.offset(1); /* advance past switch marker character */
            if keymatch(
                arg,
                
                b"arithmetic\x00".as_ptr() as *const c_char,
                1i32,
            ) != 0
            {
                /* Use arithmetic coding. */
                fprintf(
                    stderr,
                    
                    b"%s: sorry, arithmetic coding not supported\n\x00".as_ptr()
                        as *const c_char,
                    progname,
                );
                exit(EXIT_FAILURE);
            } else {
                if keymatch(
                    arg,
                    
                    b"copy\x00".as_ptr() as *const c_char,
                    2i32,
                ) != 0
                {
                    /* Select which extra markers to copy. */
                    argn += 1;
                    if argn >= argc {
                        /* advance to next argument */
                        usage();
                    }
                    if keymatch(
                        *argv.offset(argn as isize),
                        
                        b"none\x00".as_ptr() as *const c_char,
                        1i32,
                    ) != 0
                    {
                        copyoption = JCOPYOPT_NONE
                    } else if keymatch(
                        *argv.offset(argn as isize),
                        
                        b"comments\x00".as_ptr() as *const c_char,
                        1i32,
                    ) != 0
                    {
                        copyoption = JCOPYOPT_COMMENTS
                    } else if keymatch(
                        *argv.offset(argn as isize),
                        
                        b"all\x00".as_ptr() as *const c_char,
                        1i32,
                    ) != 0
                    {
                        copyoption = JCOPYOPT_ALL
                    } else {
                        usage();
                    }
                } else if keymatch(
                    arg,
                    
                    b"crop\x00".as_ptr() as *const c_char,
                    2i32,
                ) != 0
                {
                    /* Perform lossless cropping. */
                    argn += 1;
                    if argn >= argc {
                        /* advance to next argument */
                        usage();
                    }
                    if jtransform_parse_crop_spec(
                        &mut transformoption,
                        *argv.offset(argn as isize),
                    ) == 0
                    {
                        fprintf(
                            stderr,
                            
                            b"%s: bogus -crop argument \'%s\'\n\x00".as_ptr()
                                as *const c_char,
                            progname,
                            *argv.offset(argn as isize),
                        );
                        exit(EXIT_FAILURE);
                    }
                    prefer_smallest = FALSE
                } else if keymatch(
                    arg,
                    
                    b"debug\x00".as_ptr() as *const c_char,
                    1i32,
                ) != 0
                    || keymatch(
                        arg,
                        
                        b"verbose\x00".as_ptr() as *const c_char,
                        1i32,
                    ) != 0
                {
                    /* Enable debug printouts. */
                    /* On first -d, print version identification */
                    static mut printed_version: boolean =
                        FALSE;
                    if printed_version == 0 {
                        fprintf(
                            stderr,
                            
                            b"%s version %s (build %s)\n\x00".as_ptr() as *const c_char,
                            PACKAGE_NAME.as_ptr(),
                            VERSION.as_ptr(),
                            BUILD.as_ptr(),
                        );
                        fprintf(
                            stderr,
                            
                            b"%s\n\n\x00".as_ptr() as *const c_char,
                            JCOPYRIGHT.as_ptr(),
                        );
                        fprintf(
                            stderr,
                            
                            b"Emulating The Independent JPEG Group\'s software, version %s\n\n\x00".as_ptr() as *const c_char,
                            JVERSION.as_ptr(),
                        );
                        printed_version = TRUE
                    }
                    (*(*cinfo).err).trace_level += 1
                } else if keymatch(
                    arg,
                    
                    b"version\x00".as_ptr() as *const c_char,
                    4i32,
                ) != 0
                {
                    fprintf(
                        stderr,
                        
                        b"%s version %s (build %s)\n\x00".as_ptr() as *const c_char,
                        PACKAGE_NAME.as_ptr(),
                        VERSION.as_ptr(),
                        BUILD.as_ptr(),
                    );
                    exit(EXIT_SUCCESS);
                } else {
                    if keymatch(
                        arg,
                        
                        b"flip\x00".as_ptr() as *const c_char,
                        1i32,
                    ) != 0
                    {
                        /* Mirror left-right or top-bottom. */
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        if keymatch(
                            *argv.offset(argn as isize),
                            
                            b"horizontal\x00".as_ptr() as *const c_char,
                            1i32,
                        ) != 0
                        {
                            select_transform(JXFORM_FLIP_H);
                        } else if keymatch(
                            *argv.offset(argn as isize),
                            
                            b"vertical\x00".as_ptr() as *const c_char,
                            1i32,
                        ) != 0
                        {
                            select_transform(JXFORM_FLIP_V);
                        } else {
                            usage();
                        }
                        prefer_smallest = FALSE
                    } else if keymatch(
                        arg,
                        
                        b"fastcrush\x00".as_ptr() as *const c_char,
                        4i32,
                    ) != 0
                    {
                        jpeg_c_set_bool_param(
                            cinfo,
                            JBOOLEAN_OPTIMIZE_SCANS,
                            FALSE,
                        );
                    } else if keymatch(
                        arg,
                        
                        b"grayscale\x00".as_ptr() as *const c_char,
                        1i32,
                    ) != 0
                        || keymatch(
                            arg,
                            
                            b"greyscale\x00".as_ptr() as *const c_char,
                            1i32,
                        ) != 0
                    {
                        /* Force to grayscale. */
                        transformoption.force_grayscale = TRUE;
                        prefer_smallest = FALSE
                    } else if keymatch(
                        arg,
                        
                        b"icc\x00".as_ptr() as *const c_char,
                        1i32,
                    ) != 0
                    {
                        /* Set ICC filename. */
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        icc_filename = *argv.offset(argn as isize)
                    } else if keymatch(
                        arg,
                        
                        b"maxmemory\x00".as_ptr() as *const c_char,
                        3i32,
                    ) != 0
                    {
                        
                         let mut lval:  c_long =  0; let mut ch:  c_char =   'x' as c_char;
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        if sscanf(
                            *argv.offset(argn as isize),
                            
                            b"%ld%c\x00".as_ptr() as *const c_char,
                            &mut lval as *mut c_long,
                            &mut ch as *mut c_char,
                        ) < 1i32
                        {
                            usage();
                        }
                        if ch as c_int == 'm' as i32 || ch as c_int == 'M' as i32 {
                            lval *= 1000i64
                        }
                        (*(*cinfo).mem).max_memory_to_use = lval * 1000i64
                    } else if keymatch(
                        arg,
                        
                        b"optimize\x00".as_ptr() as *const c_char,
                        1i32,
                    ) != 0
                        || keymatch(
                            arg,
                            
                            b"optimise\x00".as_ptr() as *const c_char,
                            1i32,
                        ) != 0
                    {
                        /* Enable entropy parm optimization. */
                        (*cinfo).optimize_coding = TRUE
                    } else if keymatch(
                        arg,
                        
                        b"outfile\x00".as_ptr() as *const c_char,
                        4i32,
                    ) != 0
                    {
                        /* Set output file name. */
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        outfilename = *argv.offset(argn as isize)
                    /* save it away for later use */
                    } else if keymatch(
                        arg,
                        
                        b"perfect\x00".as_ptr() as *const c_char,
                        2i32,
                    ) != 0
                    {
                        /* Fail if there is any partial edge MCUs that the transform can't
                         * handle. */
                        transformoption.perfect = TRUE
                    } else if keymatch(
                        arg,
                        
                        b"progressive\x00".as_ptr() as *const c_char,
                        2i32,
                    ) != 0
                    {
                        /* Select simple progressive mode. */
                        simple_progressive = TRUE;
                        prefer_smallest = FALSE
                    /* We must postpone execution until num_components is known. */
                    } else if keymatch(
                        arg,
                        
                        b"restart\x00".as_ptr() as *const c_char,
                        1i32,
                    ) != 0
                    {
                        
                         let mut lval_0:  c_long =  0; let mut ch_0:  c_char =   'x' as c_char;
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        if sscanf(
                            *argv.offset(argn as isize),
                            
                            b"%ld%c\x00".as_ptr() as *const c_char,
                            &mut lval_0 as *mut c_long,
                            &mut ch_0 as *mut c_char,
                        ) < 1i32
                        {
                            usage();
                        }
                        if lval_0 < 0i64 || lval_0 > 65535i64 {
                            usage();
                        }
                        if ch_0 as c_int == 'b' as i32 || ch_0 as c_int == 'B' as i32 {
                            (*cinfo).restart_interval = lval_0 as c_uint;
                            (*cinfo).restart_in_rows = 0i32
                        /* else prior '-restart n' overrides me */
                        } else {
                            (*cinfo).restart_in_rows = lval_0 as c_int
                            /* restart_interval will be computed during startup */
                        }
                    } else if keymatch(
                        arg,
                        
                        b"revert\x00".as_ptr() as *const c_char,
                        3i32,
                    ) != 0
                    {
                        /* revert to old JPEG default */
                        jpeg_c_set_int_param(
                            cinfo,
                            JINT_COMPRESS_PROFILE,
                            JCP_FASTEST as c_int,
                        );
                        prefer_smallest = FALSE
                    } else if keymatch(
                        arg,
                        
                        b"rotate\x00".as_ptr() as *const c_char,
                        2i32,
                    ) != 0
                    {
                        /* Rotate 90, 180, or 270 degrees (measured clockwise). */
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        if keymatch(
                            *argv.offset(argn as isize),
                            
                            b"90\x00".as_ptr() as *const c_char,
                            2i32,
                        ) != 0
                        {
                            select_transform(JXFORM_ROT_90);
                        } else if keymatch(
                            *argv.offset(argn as isize),
                            
                            b"180\x00".as_ptr() as *const c_char,
                            3i32,
                        ) != 0
                        {
                            select_transform(JXFORM_ROT_180);
                        } else if keymatch(
                            *argv.offset(argn as isize),
                            
                            b"270\x00".as_ptr() as *const c_char,
                            3i32,
                        ) != 0
                        {
                            select_transform(JXFORM_ROT_270);
                        } else {
                            usage();
                        }
                        prefer_smallest = FALSE
                    } else if keymatch(
                        arg,
                        
                        b"scans\x00".as_ptr() as *const c_char,
                        1i32,
                    ) != 0
                    {
                        /* Set scan script. */
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        prefer_smallest = FALSE;
                        scansarg = *argv.offset(argn as isize)
                    /* We must postpone reading the file in case -progressive appears. */
                    } else if keymatch(
                        arg,
                        
                        b"transpose\x00".as_ptr() as *const c_char,
                        1i32,
                    ) != 0
                    {
                        /* Transpose (across UL-to-LR axis). */
                        select_transform(JXFORM_TRANSPOSE);
                        prefer_smallest = FALSE
                    } else if keymatch(
                        arg,
                        
                        b"transverse\x00".as_ptr() as *const c_char,
                        6i32,
                    ) != 0
                    {
                        /* Transverse transpose (across UR-to-LL axis). */
                        select_transform(JXFORM_TRANSVERSE);
                        prefer_smallest = FALSE
                    } else if keymatch(
                        arg,
                        
                        b"trim\x00".as_ptr() as *const c_char,
                        3i32,
                    ) != 0
                    {
                        /* Trim off any partial edge MCUs that the transform can't handle. */
                        transformoption.trim = TRUE;
                        prefer_smallest = FALSE
                    } else {
                        usage();
                        /* bogus switch */
                    }
                }
            }
        }
        argn += 1
    }
    /* Post-switch-scanning cleanup */
    if for_real != 0 {
        if simple_progressive != 0 {
            /* process -progressive; -scans can override */
            jpeg_simple_progression(cinfo);
        }
        if !scansarg.is_null() {
            /* process -scans if it was present */
            if read_scan_script(cinfo, scansarg) == 0 {
                usage();
            }
        }
    }
    return argn;
    /* return index of next arg (file name) */
}
/*
 * The main program.
 */

unsafe fn main_0(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
        let mut fp:  *mut FILE =
     ::std::ptr::null_mut::< FILE>(); let mut insize:  c_ulong =  0u64; let mut outsize:  c_ulong =  0u64; let mut icc_len:  c_long =  0i64;let mut srcinfo: jpeg_decompress_struct =
        jpeg_decompress_struct{err:  ::std::ptr::null_mut::< jpeg_error_mgr>(),
                       mem:  ::std::ptr::null_mut::< jpeg_memory_mgr>(),
                       progress:  ::std::ptr::null_mut::< jpeg_progress_mgr>(),
                       client_data:  ::std::ptr::null_mut::< c_void>(),
                       is_decompressor:  0,
                       global_state:  0,
                       src:  ::std::ptr::null_mut::< jpeg_source_mgr>(),
                       image_width:  0,
                       image_height:  0,
                       num_components:  0,
                       jpeg_color_space:  JCS_UNKNOWN,
                       out_color_space:  JCS_UNKNOWN,
                       scale_num:  0,
                       scale_denom:  0,
                       output_gamma:  0.,
                       buffered_image:  0,
                       raw_data_out:  0,
                       dct_method:  JDCT_ISLOW,
                       do_fancy_upsampling:  0,
                       do_block_smoothing:  0,
                       quantize_colors:  0,
                       dither_mode:  JDITHER_NONE,
                       two_pass_quantize:  0,
                       desired_number_of_colors:  0,
                       enable_1pass_quant:  0,
                       enable_external_quant:  0,
                       enable_2pass_quant:  0,
                       output_width:  0,
                       output_height:  0,
                       out_color_components:  0,
                       output_components:  0,
                       rec_outbuf_height:  0,
                       actual_number_of_colors:  0,
                       colormap:  ::std::ptr::null_mut::< JSAMPROW>(),
                       output_scanline:  0,
                       input_scan_number:  0,
                       input_iMCU_row:  0,
                       output_scan_number:  0,
                       output_iMCU_row:  0,
                       coef_bits:  ::std::ptr::null_mut::< [c_int; 64]>(),
                       quant_tbl_ptrs:
                            [::std::ptr::null_mut::< JQUANT_TBL>(); 4],
                       dc_huff_tbl_ptrs:
                            [::std::ptr::null_mut::< JHUFF_TBL>(); 4],
                       ac_huff_tbl_ptrs:
                            [::std::ptr::null_mut::< JHUFF_TBL>(); 4],
                       data_precision:  0,
                       comp_info:
                            ::std::ptr::null_mut::< jpeg_component_info>(),
                       progressive_mode:  0,
                       arith_code:  0,
                       arith_dc_L:  [0; 16],
                       arith_dc_U:  [0; 16],
                       arith_ac_K:  [0; 16],
                       restart_interval:  0,
                       saw_JFIF_marker:  0,
                       JFIF_major_version:  0,
                       JFIF_minor_version:  0,
                       density_unit:  0,
                       X_density:  0,
                       Y_density:  0,
                       saw_Adobe_marker:  0,
                       Adobe_transform:  0,
                       CCIR601_sampling:  0,
                       marker_list:
                            ::std::ptr::null_mut::< jpeg_marker_struct>(),
                       max_h_samp_factor:  0,
                       max_v_samp_factor:  0,
                       min_DCT_scaled_size:  0,
                       total_iMCU_rows:  0,
                       sample_range_limit:  ::std::ptr::null_mut::< JSAMPLE>(),
                       comps_in_scan:  0,
                       cur_comp_info:
                            [::std::ptr::null_mut::< jpeg_component_info>(); 4],
                       MCUs_per_row:  0,
                       MCU_rows_in_scan:  0,
                       blocks_in_MCU:  0,
                       MCU_membership:  [0; 10],
                       Ss:  0,
                       Se:  0,
                       Ah:  0,
                       Al:  0,
                       unread_marker:  0,
                       master:  ::std::ptr::null_mut::< jpeg_decomp_master>(),
                       main:  ::std::ptr::null_mut::< jpeg_d_main_controller>(),
                       coef:  ::std::ptr::null_mut::< jpeg_d_coef_controller>(),
                       post:  ::std::ptr::null_mut::< jpeg_d_post_controller>(),
                       inputctl:
                            ::std::ptr::null_mut::< jpeg_input_controller>(),
                       marker:  ::std::ptr::null_mut::< jpeg_marker_reader>(),
                       entropy:
                            ::std::ptr::null_mut::< jpeg_entropy_decoder>(),
                       idct:  ::std::ptr::null_mut::< jpeg_inverse_dct>(),
                       upsample:  ::std::ptr::null_mut::< jpeg_upsampler>(),
                       cconvert:
                            ::std::ptr::null_mut::< jpeg_color_deconverter>(),
                       cquantize:
                            ::std::ptr::null_mut::< jpeg_color_quantizer>(),};
    let mut dstinfo: jpeg_compress_struct =
        jpeg_compress_struct{err:  ::std::ptr::null_mut::< jpeg_error_mgr>(),
                     mem:  ::std::ptr::null_mut::< jpeg_memory_mgr>(),
                     progress:  ::std::ptr::null_mut::< jpeg_progress_mgr>(),
                     client_data:  ::std::ptr::null_mut::< c_void>(),
                     is_decompressor:  0,
                     global_state:  0,
                     dest:  ::std::ptr::null_mut::< jpeg_destination_mgr>(),
                     image_width:  0,
                     image_height:  0,
                     input_components:  0,
                     in_color_space:  JCS_UNKNOWN,
                     input_gamma:  0.,
                     data_precision:  0,
                     num_components:  0,
                     jpeg_color_space:  JCS_UNKNOWN,
                     comp_info:  ::std::ptr::null_mut::< jpeg_component_info>(),
                     quant_tbl_ptrs:
                          [::std::ptr::null_mut::< JQUANT_TBL>(); 4],
                     dc_huff_tbl_ptrs:
                          [::std::ptr::null_mut::< JHUFF_TBL>(); 4],
                     ac_huff_tbl_ptrs:
                          [::std::ptr::null_mut::< JHUFF_TBL>(); 4],
                     arith_dc_L:  [0; 16],
                     arith_dc_U:  [0; 16],
                     arith_ac_K:  [0; 16],
                     num_scans:  0,
                     scan_info:  ::std::ptr::null::< jpeg_scan_info>(),
                     raw_data_in:  0,
                     arith_code:  0,
                     optimize_coding:  0,
                     CCIR601_sampling:  0,
                     smoothing_factor:  0,
                     dct_method:  JDCT_ISLOW,
                     restart_interval:  0,
                     restart_in_rows:  0,
                     write_JFIF_header:  0,
                     JFIF_major_version:  0,
                     JFIF_minor_version:  0,
                     density_unit:  0,
                     X_density:  0,
                     Y_density:  0,
                     write_Adobe_marker:  0,
                     next_scanline:  0,
                     progressive_mode:  0,
                     max_h_samp_factor:  0,
                     max_v_samp_factor:  0,
                     total_iMCU_rows:  0,
                     comps_in_scan:  0,
                     cur_comp_info:
                          [::std::ptr::null_mut::< jpeg_component_info>(); 4],
                     MCUs_per_row:  0,
                     MCU_rows_in_scan:  0,
                     blocks_in_MCU:  0,
                     MCU_membership:  [0; 10],
                     Ss:  0,
                     Se:  0,
                     Ah:  0,
                     Al:  0,
                     master:  ::std::ptr::null_mut::< jpeg_comp_master>(),
                     main:  ::std::ptr::null_mut::< jpeg_c_main_controller>(),
                     prep:  ::std::ptr::null_mut::< jpeg_c_prep_controller>(),
                     coef:  ::std::ptr::null_mut::< jpeg_c_coef_controller>(),
                     marker:  ::std::ptr::null_mut::< jpeg_marker_writer>(),
                     cconvert:  ::std::ptr::null_mut::< jpeg_color_converter>(),
                     downsample:  ::std::ptr::null_mut::< jpeg_downsampler>(),
                     fdct:  ::std::ptr::null_mut::< jpeg_forward_dct>(),
                     entropy:  ::std::ptr::null_mut::< jpeg_entropy_encoder>(),
                     script_space:  ::std::ptr::null_mut::< jpeg_scan_info>(),
                     script_space_size:  0,};
    let mut jsrcerr: jpeg_error_mgr = jpeg_error_mgr{error_exit:  None,
               emit_message:  None,
               output_message:  None,
               format_message:  None,
               reset_error_mgr:  None,
               msg_code:  0,
               msg_parm:  C2RustUnnamed_2{i:  [0; 8],},
               trace_level:  0,
               num_warnings:  0,
               jpeg_message_table:  ::std::ptr::null::< *const c_char>(),
               last_jpeg_message:  0,
               addon_message_table:  ::std::ptr::null::< *const c_char>(),
               first_addon_message:  0,
               last_addon_message:  0,};
    let mut jdsterr: jpeg_error_mgr = jpeg_error_mgr{error_exit:  None,
               emit_message:  None,
               output_message:  None,
               format_message:  None,
               reset_error_mgr:  None,
               msg_code:  0,
               msg_parm:  C2RustUnnamed_2{i:  [0; 8],},
               trace_level:  0,
               num_warnings:  0,
               jpeg_message_table:  ::std::ptr::null::< *const c_char>(),
               last_jpeg_message:  0,
               addon_message_table:  ::std::ptr::null::< *const c_char>(),
               first_addon_message:  0,
               last_addon_message:  0,};
    
    
    
    /* We assume all-in-memory processing and can therefore use only a
     * single file pointer for sequential input and output operation.
     */
    
    let mut inbuffer: *mut c_uchar = NULL as *mut c_uchar;
    
    let mut outbuffer: *mut c_uchar = NULL as *mut c_uchar;
    
    
    let mut icc_profile: *mut JOCTET =
        NULL as *mut JOCTET;
    
    /* On Mac, fetch a command line. */
    progname = *argv.offset(0); /* in case C library doesn't provide it */
    if progname.is_null() || *progname.offset(0) as c_int == 0i32 {
        progname =  b"jpegtran\x00".as_ptr() as *const c_char
    }
    /* Initialize the JPEG decompression object with default error handling. */
    srcinfo.err = jpeg_std_error(&mut jsrcerr);
    jpeg_CreateDecompress(
        &mut srcinfo,
        JPEG_LIB_VERSION,
        ::std::mem::size_of::<jpeg_decompress_struct>() as c_ulong,
    );
    /* Initialize the JPEG compression object with default error handling. */
    dstinfo.err = jpeg_std_error(&mut jdsterr);
    jpeg_CreateCompress(
        &mut dstinfo,
        JPEG_LIB_VERSION,
        ::std::mem::size_of::<jpeg_compress_struct>() as c_ulong,
    );
    /* Scan command line to find file names.
     * It is convenient to use just one switch-parsing routine, but the switch
     * values read here are mostly ignored; we will rescan the switches after
     * opening the input file.  Also note that most of the switches affect the
     * destination JPEG object, so we parse into that and then copy over what
     * needs to affects the source too.
     */
     let mut file_index:   c_int =
     parse_switches(&mut dstinfo, argc, argv, 0i32, FALSE);
    jsrcerr.trace_level = jdsterr.trace_level;
    (*srcinfo.mem).max_memory_to_use = (*dstinfo.mem).max_memory_to_use;
    /* Unix style: expect zero or one file name */
    if file_index < argc - 1i32 {
        fprintf(
            stderr,
            
            b"%s: only one input file\n\x00".as_ptr() as *const c_char,
            progname,
        );
        usage();
    }
    /* TWO_FILE_COMMANDLINE */
    /* Open the input file. */
    if file_index < argc {
        fp = fopen(
            *argv.offset(file_index as isize),
            READ_BINARY.as_ptr(),
        );
        if fp.is_null() {
            fprintf(
                stderr,
                
                b"%s: can\'t open %s for reading\n\x00".as_ptr() as *const c_char,
                progname,
                *argv.offset(file_index as isize),
            );
            exit(EXIT_FAILURE);
        }
    } else {
        /* default input file is stdin */
        fp = read_stdin()
    }
    if !icc_filename.is_null() {
          let mut icc_file:   *mut FILE =
     fopen(icc_filename, READ_BINARY.as_ptr());
        if icc_file.is_null() {
            fprintf(
                stderr,
                
                b"%s: can\'t open %s\n\x00".as_ptr() as *const c_char,
                progname,
                icc_filename,
            );
            exit(EXIT_FAILURE);
        }
        if fseek(icc_file, 0i64, SEEK_END) < 0i32
            || {
                icc_len = ftell(icc_file);
                (icc_len) < 1i64
            }
            || fseek(icc_file, 0i64, SEEK_SET) < 0i32
        {
            fprintf(
                stderr,
                
                b"%s: can\'t determine size of %s\n\x00".as_ptr() as *const c_char,
                progname,
                icc_filename,
            );
            exit(EXIT_FAILURE);
        }
        icc_profile =
            malloc(icc_len as c_ulong) as *mut JOCTET;
        if icc_profile.is_null() {
            fprintf(
                stderr,
                
                b"%s: can\'t allocate memory for ICC profile\n\x00".as_ptr()
                    as *const c_char,
                progname,
            );
            fclose(icc_file);
            exit(EXIT_FAILURE);
        }
        if fread(
            icc_profile as *mut c_void,
            icc_len as c_ulong,
            1u64,
            icc_file,
        ) < 1u64
        {
            fprintf(
                stderr,
                
                b"%s: can\'t read ICC profile from %s\n\x00".as_ptr() as *const c_char,
                progname,
                icc_filename,
            );
            free(icc_profile as *mut c_void);
            fclose(icc_file);
            exit(EXIT_FAILURE);
        }
        fclose(icc_file);
        if  copyoption
            ==  JCOPYOPT_ALL
        {
            copyoption = JCOPYOPT_ALL_EXCEPT_ICC
        }
    }
    /* Specify data source for decompression */
    if jpeg_c_int_param_supported(
        &mut dstinfo,
        JINT_COMPRESS_PROFILE,
    ) != 0
        && jpeg_c_get_int_param(
            &mut dstinfo,
            JINT_COMPRESS_PROFILE,
        ) == JCP_MAX_COMPRESSION as c_int
    {
        memsrc = TRUE
    } /* needed to revert to original */
    if memsrc != 0 {
        
        loop {
             inbuffer = realloc(
                inbuffer as *mut c_void,
                
                insize + INPUT_BUF_SIZE as c_ulong,
            ) as *mut c_uchar;
            if inbuffer.is_null() {
                fprintf(
                    stderr,
                    
                    b"%s: memory allocation failure\n\x00".as_ptr() as *const c_char,
                    progname,
                );
                exit(EXIT_FAILURE);
            }
             let mut nbytes:   size_t =
     fread(
                &mut *inbuffer.offset(insize as isize) as *mut c_uchar as *mut c_void,
                1u64,
                4096u64,
                fp,
            );
            if nbytes < INPUT_BUF_SIZE as c_ulong && ferror(fp) != 0 {
                if file_index < argc {
                    fprintf(
                        stderr,
                        
                        b"%s: can\'t read from %s\n\x00".as_ptr() as *const c_char,
                        progname,
                        *argv.offset(file_index as isize),
                    );
                } else {
                    fprintf(
                        stderr,
                        
                        b"%s: can\'t read from stdin\n\x00".as_ptr() as *const c_char,
                        progname,
                    );
                }
            }
            insize +=  nbytes;
            if !(nbytes == INPUT_BUF_SIZE as c_ulong) {
                break;
            }
        }
        jpeg_mem_src(&mut srcinfo, inbuffer, insize);
    } else {
        jpeg_stdio_src(&mut srcinfo, fp);
    }
    /* Enable saving of extra markers that we want to copy */
    jcopy_markers_setup(&mut srcinfo, copyoption);
    /* Read file header */
    jpeg_read_header(&mut srcinfo, TRUE);
    /* Any space needed by a transform option must be requested before
     * jpeg_read_coefficients so that memory allocation will be done right.
     */
    /* Fail right away if -perfect is given and transformation is not perfect.
     */
    if jtransform_request_workspace(&mut srcinfo, &mut transformoption) == 0 {
        fprintf(
            stderr,
            
            b"%s: transformation is not perfect\n\x00".as_ptr() as *const c_char,
            progname,
        );
        exit(EXIT_FAILURE);
    }
     let mut src_coef_arrays:   *mut jvirt_barray_ptr =
     jpeg_read_coefficients(&mut srcinfo);
    /* Initialize destination compression parameters from source values */
    jpeg_copy_critical_parameters(&mut srcinfo, &mut dstinfo);
    /* Adjust destination parameters if required by transform options;
     * also find out which set of coefficient arrays will hold the output.
     */
     let mut dst_coef_arrays:   *mut jvirt_barray_ptr =
     jtransform_adjust_parameters(
        &mut srcinfo,
        &mut dstinfo,
        src_coef_arrays,
        &mut transformoption,
    );
    /* Close input file, if we opened it.
     * Note: we assume that jpeg_read_coefficients consumed all input
     * until JPEG_REACHED_EOI, and that jpeg_finish_decompress will
     * only consume more while (! cinfo->inputctl->eoi_reached).
     * We cannot call jpeg_finish_decompress here since we still need the
     * virtual arrays allocated from the source object for processing.
     */
    if fp != stdin {
        fclose(fp);
    }
    /* Open the output file. */
    if !outfilename.is_null() {
        fp = fopen(outfilename, WRITE_BINARY.as_ptr());
        if fp.is_null() {
            fprintf(
                stderr,
                
                b"%s: can\'t open %s for writing\n\x00".as_ptr() as *const c_char,
                progname,
                outfilename,
            );
            exit(EXIT_FAILURE);
        }
    } else {
        /* default output file is stdout */
        fp = write_stdout()
    }
    /* Adjust default compression parameters by re-parsing the options */
    file_index = parse_switches(&mut dstinfo, argc, argv, 0i32, TRUE);
    /* Specify data destination for compression */
    if jpeg_c_int_param_supported(
        &mut dstinfo,
        JINT_COMPRESS_PROFILE,
    ) != 0
        && jpeg_c_get_int_param(
            &mut dstinfo,
            JINT_COMPRESS_PROFILE,
        ) == JCP_MAX_COMPRESSION as c_int
    {
        jpeg_mem_dest(&mut dstinfo, &mut outbuffer, &mut outsize);
    } else {
        jpeg_stdio_dest(&mut dstinfo, fp);
    }
    /* Start compressor (note no image data is actually written here) */
    jpeg_write_coefficients(&mut dstinfo, dst_coef_arrays);
    /* Copy to the output file any extra markers that we want to preserve */
    jcopy_markers_execute(&mut srcinfo, &mut dstinfo, copyoption);
    if !icc_profile.is_null() {
        jpeg_write_icc_profile(
            &mut dstinfo,
            icc_profile,
            icc_len as c_uint,
        );
    }
    /* Execute image transformation, if any */
    jtransform_execute_transform(
        &mut srcinfo,
        &mut dstinfo,
        src_coef_arrays,
        &mut transformoption,
    );
    /* Finish compression and release memory */
    jpeg_finish_compress(&mut dstinfo);
    if jpeg_c_int_param_supported(
        &mut dstinfo,
        JINT_COMPRESS_PROFILE,
    ) != 0
        && jpeg_c_get_int_param(
            &mut dstinfo,
            JINT_COMPRESS_PROFILE,
        ) == JCP_MAX_COMPRESSION as c_int
    {
         
        let mut buffer: *mut c_uchar = outbuffer;
        let mut size: c_ulong = outsize;
        if prefer_smallest != 0 && insize < size {
            size = insize;
            buffer = inbuffer
        }
         let mut nbytes_0:   size_t =
     fwrite(
            buffer as *const c_void,
            1u64,
            size,
            fp,
        );
        if nbytes_0 < size && ferror(fp) != 0 {
            if file_index < argc {
                fprintf(
                    stderr,
                    
                    b"%s: can\'t write to %s\n\x00".as_ptr() as *const c_char,
                    progname,
                    *argv.offset(file_index as isize),
                );
            } else {
                fprintf(
                    stderr,
                    
                    b"%s: can\'t write to stdout\n\x00".as_ptr() as *const c_char,
                    progname,
                );
            }
        }
    }
    jpeg_destroy_compress(&mut dstinfo);
    jpeg_finish_decompress(&mut srcinfo);
    jpeg_destroy_decompress(&mut srcinfo);
    /* Close output file, if we opened it */
    if fp != stdout {
        fclose(fp);
    }
    free(inbuffer as *mut c_void);
    free(outbuffer as *mut c_void);
    if !icc_profile.is_null() {
        free(icc_profile as *mut c_void);
    }
    /* All done. */
    exit(if jsrcerr.num_warnings + jdsterr.num_warnings != 0 {
        EXIT_WARNING
    } else {
        EXIT_SUCCESS
    });
    /* suppress no-return-value warnings */
}
#[main]
pub fn main() {
     let mut args:  Vec<*mut c_char> =  Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as c_int,
            
            args.as_mut_ptr(),
        ))
    }
}
