use libc::c_uint;use std::ffi::CStr;use libc::c_ulong;use libc::c_long;use libc::c_void;use libc::c_float;use libc::c_int;use libc::c_ushort;use libc::c_char;use libc;

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

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::MAX_COMPONENTS;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_add_quant_table;
pub use crate::jpeglib_h::jpeg_c_coef_controller;
pub use crate::jpeglib_h::jpeg_c_get_int_param;
pub use crate::jpeglib_h::jpeg_c_int_param_supported;
pub use crate::jpeglib_h::jpeg_c_main_controller;
pub use crate::jpeglib_h::jpeg_c_prep_controller;
pub use crate::jpeglib_h::jpeg_c_set_bool_param;
pub use crate::jpeglib_h::jpeg_color_converter;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_comp_master;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_downsampler;
pub use crate::jpeglib_h::jpeg_entropy_encoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_float_quality_scaling;
pub use crate::jpeglib_h::jpeg_forward_dct;
pub use crate::jpeglib_h::jpeg_marker_writer;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::DCTSIZE2;
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
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JINT_BASE_QUANT_TBL_IDX;
pub use crate::jpeglib_h::JINT_COMPRESS_PROFILE;
pub use crate::jpeglib_h::JINT_DC_SCAN_OPT_MODE;
pub use crate::jpeglib_h::JINT_TRELLIS_FREQ_SPLIT;
pub use crate::jpeglib_h::JINT_TRELLIS_NUM_LOOPS;
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_BOOLEAN_PARAM;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_INT_PARAM;
pub use crate::jpeglib_h::MAX_COMPS_IN_SCAN;
pub use crate::jpeglib_h::NUM_QUANT_TBLS;
pub use crate::stdlib::C2RustUnnamed_0;
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
pub use crate::stdlib::fclose;
pub use crate::stdlib::fopen;
pub use crate::stdlib::fprintf;
pub use crate::stdlib::getc;
use crate::stdlib::memcpy;
pub use crate::stdlib::sscanf;
pub use crate::stdlib::stderr;
pub use crate::stdlib::ungetc;
pub use crate::stdlib::EOF;
/*
 * rdswitch.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2010, 2018, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains routines to process some of cjpeg's more complicated
 * command-line switches.  Switches processed here are:
 *      -qtables file           Read quantization tables from text file
 *      -scans file             Read scan script from text file
 *      -quality N[,N,...]      Set quality ratings
 *      -qslots N[,N,...]       Set component quantization table selectors
 *      -sample HxV[,HxV,...]   Set component sampling factors
 */
/* to declare isdigit(), isspace() */

unsafe extern "C" fn text_getc(mut file: *mut FILE) -> c_int
/* Read next char, skipping over any comments (# to end of line) */
/* A comment/newline sequence is returned as a newline */ {
     
     let mut ch:   c_int =  getc(file);
    if ch == '#' as i32 {
        loop {
            ch = getc(file);
            if !(ch != '\n' as i32 && ch != EOF) {
                break;
            }
        }
    }
    return ch;
}

unsafe extern "C" fn read_text_integer(
    mut file: *mut FILE,
    mut result: *mut c_long,
    mut termchar: *mut c_int,
) -> boolean
/* Read an unsigned decimal integer from a file, store it in result */
/* Reads one trailing character after the integer; returns it in termchar */ {
    
     let mut ch:  c_int =  0; 
    loop
    /* Skip any leading whitespace, detect EOF */
    {
        ch = text_getc(file);
        if ch == EOF {
            *termchar = ch;
            return FALSE;
        }
        if !(*(*__ctype_b_loc()).offset(ch as isize) as c_int
            &  _ISspace as c_ushort as c_int
            != 0)
        {
            break;
        }
    }
    if *(*__ctype_b_loc()).offset(ch as isize) as c_int
        &  _ISdigit as c_ushort as c_int
        == 0
    {
        *termchar = ch;
        return FALSE;
    }
     let mut val:   c_long =  (ch - '0' as i32) as c_long;
    loop {
        ch = text_getc(file);
        if !(ch != EOF) {
            break;
        }
        if *(*__ctype_b_loc()).offset(ch as isize) as c_int
            &  _ISdigit as c_ushort as c_int
            == 0
        {
            break;
        }
        val *= 10i64;
        val += (ch - '0' as i32) as c_long
    }
    *result = val;
    *termchar = ch;
    return TRUE;
}

static mut q_scale_factor: [c_int; 4] = [100i32, 100i32, 100i32, 100i32];
#[no_mangle]

pub unsafe extern "C" fn read_quant_tables(
    mut cinfo: j_compress_ptr,
    mut filename: *mut c_char,
    mut force_baseline: boolean,
) -> boolean
/* Read a set of quantization tables from the specified file.
 * The file is plain ASCII text: decimal numbers with whitespace between.
 * Comments preceded by '#' may be included in the file.
 * There may be one to NUM_QUANT_TBLS tables in the file, each of 64 values.
 * The tables are implicitly numbered 0,1,etc.
 * NOTE: does not affect the qslots mapping, which will default to selecting
 * table 0 for luminance (or primary) components, 1 for chrominance components.
 * You must use -qslots if you want a different component->table mapping.
 */ {
    
    
    
    
    
       let mut termchar:  c_int =  0; let mut val:  c_long =  0;
     let mut fp:   *mut FILE =
     fopen(filename,  b"r\x00".as_ptr() as *const c_char);
    if fp.is_null() {
                eprintln!("Can\'t open table file {:}",
          { CStr::from_ptr(filename as *const c_char).to_str().unwrap() });
        return FALSE;
    }
     let mut tblno:   c_int =  0i32;
    while read_text_integer(fp, &mut val, &mut termchar) != 0 {
         let mut table:  [c_uint; 64] =  [0; 64];if tblno >= NUM_QUANT_TBLS {
                    eprintln!("Too many tables in file {:}",
          { CStr::from_ptr(filename as *const c_char).to_str().unwrap() });
            fclose(fp);
            return FALSE;
        }
        table[0] = val as c_uint;
         let mut i:   c_int =  1i32;
        while i < DCTSIZE2 {
            if read_text_integer(fp, &mut val, &mut termchar) == 0 {
                        eprintln!("Invalid table data in file {:}",
          { CStr::from_ptr(filename as *const c_char).to_str().unwrap() });
                fclose(fp);
                return FALSE;
            }
            table[i as usize] = val as c_uint;
            i += 1
        }
        jpeg_add_quant_table(
            cinfo,
            tblno,
            table.as_mut_ptr(),
            q_scale_factor[tblno as usize],
            force_baseline,
        );
        tblno += 1
    }
    if termchar != EOF {
                eprintln!("Non-numeric data in file {:}",
          { CStr::from_ptr(filename as *const c_char).to_str().unwrap() });
        fclose(fp);
        return FALSE;
    }
    fclose(fp);
    return TRUE;
}

unsafe extern "C" fn read_scan_integer(
    mut file: *mut FILE,
    mut result: *mut c_long,
    mut termchar: *mut c_int,
) -> boolean
/* Variant of read_text_integer that always looks for a non-space termchar;
 * this simplifies parsing of punctuation in scan scripts.
 */ {
     
    if read_text_integer(file, result, termchar) == 0 {
        return FALSE;
    }
     let mut ch:   c_int =  *termchar;
    while ch != EOF
        && *(*__ctype_b_loc()).offset(ch as isize) as c_int
            &  _ISspace as c_ushort as c_int
            != 0
    {
        ch = text_getc(file)
    }
    if *(*__ctype_b_loc()).offset(ch as isize) as c_int
        &  _ISdigit as c_ushort as c_int
        != 0
    {
        /* oops, put it back */
        if ungetc(ch, file) == EOF {
            return FALSE;
        }
        ch = ' ' as i32
    } else if ch != EOF && ch != ';' as i32 && ch != ':' as i32 {
        ch = ' ' as i32
    }
    *termchar = ch;
    return TRUE;
}
#[no_mangle]

pub unsafe extern "C" fn read_scan_script(
    mut cinfo: j_compress_ptr,
    mut filename: *mut c_char,
) -> boolean
/* Any separators other than ';' and ':' are ignored;
     * this allows user to insert commas, etc, if desired.
     */
 /* Read a scan script from the specified text file.
 * Each entry in the file defines one scan to be emitted.
 * Entries are separated by semicolons ';'.
 * An entry contains one to four component indexes,
 * optionally followed by a colon ':' and four progressive-JPEG parameters.
 * The component indexes denote which component(s) are to be transmitted
 * in the current scan.  The first component has index 0.
 * Sequential JPEG is used if the progressive-JPEG parameters are omitted.
 * The file is free format text: any whitespace may appear between numbers
 * and the ':' and ';' punctuation marks.  Also, other punctuation (such
 * as commas or dashes) can be placed between numbers if desired.
 * Comments preceded by '#' may be included in the file.
 * Note: we do very little validity checking here;
 * jcmaster.c will validate the script parameters.
 */ {
    
    
    
    
    
    
    
       let mut termchar:  c_int =  0; let mut val:  c_long =  0;  let mut scans:  [jpeg_scan_info; 100] =
     [jpeg_scan_info{comps_in_scan:  0,
               component_index:  [0; 4],
               Ss:  0,
               Se:  0,
               Ah:  0,
               Al:  0,}; 100];
     let mut fp:   *mut FILE =
     fopen(filename,  b"r\x00".as_ptr() as *const c_char);
    if fp.is_null() {
                eprintln!("Can\'t open scan definition file {:}",
          { CStr::from_ptr(filename as *const c_char).to_str().unwrap() });
        return FALSE;
    }
    
     let mut scanptr:   *mut jpeg_scan_info =  scans.as_mut_ptr(); let mut scanno:   c_int =  0i32;
    while read_scan_integer(fp, &mut val, &mut termchar) != 0 {
         let mut current_block:  u64; if scanno >= MAX_SCANS {
                    eprintln!("Too many scans defined in file {:}",
          { CStr::from_ptr(filename as *const c_char).to_str().unwrap() });
            fclose(fp);
            return FALSE;
        }
        (*scanptr).component_index[0] = val as c_int;
         let mut ncomps:   c_int =  1i32;
        loop {
            if !(termchar == ' ' as i32) {
                current_block = 1109700713171191020;
                break;
            }
            if ncomps >= MAX_COMPS_IN_SCAN {
                        eprintln!("Too many components in one scan in file {:}",
          { CStr::from_ptr(filename as *const c_char).to_str().unwrap() });
                fclose(fp);
                return FALSE;
            }
            if read_scan_integer(fp, &mut val, &mut termchar) == 0 {
                current_block = 9520589643232431964;
                break;
            }
            (*scanptr).component_index[ncomps as usize] = val as c_int;
            ncomps += 1
        }
        match current_block {
            1109700713171191020 => {
                (*scanptr).comps_in_scan = ncomps;
                if termchar == ':' as i32 {
                    if read_scan_integer(fp, &mut val, &mut termchar) == 0 || termchar != ' ' as i32
                    {
                        current_block = 9520589643232431964;
                    } else {
                        (*scanptr).Ss = val as c_int;
                        if read_scan_integer(fp, &mut val, &mut termchar) == 0
                            || termchar != ' ' as i32
                        {
                            current_block = 9520589643232431964;
                        } else {
                            (*scanptr).Se = val as c_int;
                            if read_scan_integer(fp, &mut val, &mut termchar) == 0
                                || termchar != ' ' as i32
                            {
                                current_block = 9520589643232431964;
                            } else {
                                (*scanptr).Ah = val as c_int;
                                if read_scan_integer(fp, &mut val, &mut termchar) == 0 {
                                    current_block = 9520589643232431964;
                                } else {
                                    (*scanptr).Al = val as c_int;
                                    current_block = 8845338526596852646;
                                }
                            }
                        }
                    }
                } else {
                    /* set non-progressive parameters */
                    (*scanptr).Ss = 0i32;
                    (*scanptr).Se = DCTSIZE2 - 1i32;
                    (*scanptr).Ah = 0i32;
                    (*scanptr).Al = 0i32;
                    current_block = 8845338526596852646;
                }
                match current_block {
                    9520589643232431964 => {}
                    _ => {
                        if !(termchar != ';' as i32 && termchar != EOF) {
                            scanptr = scanptr.offset(1);
                            scanno += 1;
                            continue;
                        }
                    }
                }
            }
            _ => {}
        }
                eprintln!("Invalid scan entry format in file {:}",
          { CStr::from_ptr(filename as *const c_char).to_str().unwrap() });
        fclose(fp);
        return FALSE;
    }
    if termchar != EOF {
                eprintln!("Non-numeric data in file {:}",
          { CStr::from_ptr(filename as *const c_char).to_str().unwrap() });
        fclose(fp);
        return FALSE;
    }
    if scanno > 0i32 {
        /* Stash completed scan list in cinfo structure.
         * NOTE: for cjpeg's use, JPOOL_IMAGE is the right lifetime for this data,
         * but if you want to compress multiple images you'd want JPOOL_PERMANENT.
         */
        scanptr = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            scanno as c_ulong *
    ::std::mem::size_of::<
                jpeg_scan_info,
            >() as c_ulong,
        ) as *mut jpeg_scan_info;
        memcpy(
            scanptr as *mut c_void,
            scans.as_mut_ptr() as *const c_void,
            scanno as c_ulong *
    ::std::mem::size_of::<
                jpeg_scan_info,
            >() as c_ulong,
        );
        (*cinfo).scan_info = scanptr;
        (*cinfo).num_scans = scanno;
        /* Disable scan optimization if using custom scan */
        jpeg_c_set_bool_param(
            cinfo,
            JBOOLEAN_OPTIMIZE_SCANS,
            FALSE,
        );
    }
    fclose(fp);
    return TRUE;
}

pub const MAX_SCANS: c_int = 100i32;
/* C_MULTISCAN_FILES_SUPPORTED */
/* These are the sample quantization tables given in Annex K (Clause K.1) of
 * Recommendation ITU-T T.81 (1992) | ISO/IEC 10918-1:1994.
 * The spec says that the values given produce "good" quality, and
 * when divided by 2, "very good" quality.
 */

static mut std_luminance_quant_tbl: [[c_uint; 64]; 9] = [
    [
        16u32,
        11u32,
        10u32,
        16u32,
        24u32,
        40u32,
        51u32,
        61u32,
        12u32,
        12u32,
        14u32,
        19u32,
        26u32,
        58u32,
        60u32,
        55u32,
        14u32,
        13u32,
        16u32,
        24u32,
        40u32,
        57u32,
        69u32,
        56u32,
        14u32,
        17u32,
        22u32,
        29u32,
        51u32,
        87u32,
        80u32,
        62u32,
        18u32,
        22u32,
        37u32,
        56u32,
        68u32,
        109u32,
        103u32,
        77u32,
        24u32,
        35u32,
        55u32,
        64u32,
        81u32,
        104u32,
        113u32,
        92u32,
        49u32,
        64u32,
        78u32,
        87u32,
        103u32,
        121u32,
        120u32,
        101u32,
        72u32,
        92u32,
        95u32,
        98u32,
        112u32,
        100u32,
        103u32,
        99u32,
    ],
    [
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
    ],
    [
        12u32,
        17u32,
        20u32,
        21u32,
        30u32,
        34u32,
        56u32,
        63u32,
        18u32,
        20u32,
        20u32,
        26u32,
        28u32,
        51u32,
        61u32,
        55u32,
        19u32,
        20u32,
        21u32,
        26u32,
        33u32,
        58u32,
        69u32,
        55u32,
        26u32,
        26u32,
        26u32,
        30u32,
        46u32,
        87u32,
        86u32,
        66u32,
        31u32,
        33u32,
        36u32,
        40u32,
        46u32,
        96u32,
        100u32,
        73u32,
        40u32,
        35u32,
        46u32,
        62u32,
        81u32,
        100u32,
        111u32,
        91u32,
        46u32,
        66u32,
        76u32,
        86u32,
        102u32,
        121u32,
        120u32,
        101u32,
        68u32,
        90u32,
        90u32,
        96u32,
        113u32,
        102u32,
        105u32,
        103u32,
    ],
    [
        16u32,
        16u32,
        16u32,
        18u32,
        25u32,
        37u32,
        56u32,
        85u32,
        16u32,
        17u32,
        20u32,
        27u32,
        34u32,
        40u32,
        53u32,
        75u32,
        16u32,
        20u32,
        24u32,
        31u32,
        43u32,
        62u32,
        91u32,
        135u32,
        18u32,
        27u32,
        31u32,
        40u32,
        53u32,
        74u32,
        106u32,
        156u32,
        25u32,
        34u32,
        43u32,
        53u32,
        69u32,
        94u32,
        131u32,
        189u32,
        37u32,
        40u32,
        62u32,
        74u32,
        94u32,
        124u32,
        169u32,
        238u32,
        56u32,
        53u32,
        91u32,
        106u32,
        131u32,
        169u32,
        226u32,
        311u32,
        85u32,
        75u32,
        135u32,
        156u32,
        189u32,
        238u32,
        311u32,
        418u32,
    ],
    [
        9u32,
        10u32,
        12u32,
        14u32,
        27u32,
        32u32,
        51u32,
        62u32,
        11u32,
        12u32,
        14u32,
        19u32,
        27u32,
        44u32,
        59u32,
        73u32,
        12u32,
        14u32,
        18u32,
        25u32,
        42u32,
        59u32,
        79u32,
        78u32,
        17u32,
        18u32,
        25u32,
        42u32,
        61u32,
        92u32,
        87u32,
        92u32,
        23u32,
        28u32,
        42u32,
        75u32,
        79u32,
        112u32,
        112u32,
        99u32,
        40u32,
        42u32,
        59u32,
        84u32,
        88u32,
        124u32,
        132u32,
        111u32,
        42u32,
        64u32,
        78u32,
        95u32,
        105u32,
        126u32,
        125u32,
        99u32,
        70u32,
        75u32,
        100u32,
        102u32,
        116u32,
        100u32,
        107u32,
        98u32,
    ],
    [
        10u32,
        12u32,
        14u32,
        19u32,
        26u32,
        38u32,
        57u32,
        86u32,
        12u32,
        18u32,
        21u32,
        28u32,
        35u32,
        41u32,
        54u32,
        76u32,
        14u32,
        21u32,
        25u32,
        32u32,
        44u32,
        63u32,
        92u32,
        136u32,
        19u32,
        28u32,
        32u32,
        41u32,
        54u32,
        75u32,
        107u32,
        157u32,
        26u32,
        35u32,
        44u32,
        54u32,
        70u32,
        95u32,
        132u32,
        190u32,
        38u32,
        41u32,
        63u32,
        75u32,
        95u32,
        125u32,
        170u32,
        239u32,
        57u32,
        54u32,
        92u32,
        107u32,
        132u32,
        170u32,
        227u32,
        312u32,
        86u32,
        76u32,
        136u32,
        157u32,
        190u32,
        239u32,
        312u32,
        419u32,
    ],
    [
        7u32,
        8u32,
        10u32,
        14u32,
        23u32,
        44u32,
        95u32,
        241u32,
        8u32,
        8u32,
        11u32,
        15u32,
        25u32,
        47u32,
        102u32,
        255u32,
        10u32,
        11u32,
        13u32,
        19u32,
        31u32,
        58u32,
        127u32,
        255u32,
        14u32,
        15u32,
        19u32,
        27u32,
        44u32,
        83u32,
        181u32,
        255u32,
        23u32,
        25u32,
        31u32,
        44u32,
        72u32,
        136u32,
        255u32,
        255u32,
        44u32,
        47u32,
        58u32,
        83u32,
        136u32,
        255u32,
        255u32,
        255u32,
        95u32,
        102u32,
        127u32,
        181u32,
        255u32,
        255u32,
        255u32,
        255u32,
        241u32,
        255u32,
        255u32,
        255u32,
        255u32,
        255u32,
        255u32,
        255u32,
    ],
    [
        15u32,
        11u32,
        11u32,
        12u32,
        15u32,
        19u32,
        25u32,
        32u32,
        11u32,
        13u32,
        10u32,
        10u32,
        12u32,
        15u32,
        19u32,
        24u32,
        11u32,
        10u32,
        14u32,
        14u32,
        16u32,
        18u32,
        22u32,
        27u32,
        12u32,
        10u32,
        14u32,
        18u32,
        21u32,
        24u32,
        28u32,
        33u32,
        15u32,
        12u32,
        16u32,
        21u32,
        26u32,
        31u32,
        36u32,
        42u32,
        19u32,
        15u32,
        18u32,
        24u32,
        31u32,
        38u32,
        45u32,
        53u32,
        25u32,
        19u32,
        22u32,
        28u32,
        36u32,
        45u32,
        55u32,
        65u32,
        32u32,
        24u32,
        27u32,
        33u32,
        42u32,
        53u32,
        65u32,
        77u32,
    ],
    [
        14u32,
        10u32,
        11u32,
        14u32,
        19u32,
        25u32,
        34u32,
        45u32,
        10u32,
        11u32,
        11u32,
        12u32,
        15u32,
        20u32,
        26u32,
        33u32,
        11u32,
        11u32,
        15u32,
        18u32,
        21u32,
        25u32,
        31u32,
        38u32,
        14u32,
        12u32,
        18u32,
        24u32,
        28u32,
        33u32,
        39u32,
        47u32,
        19u32,
        15u32,
        21u32,
        28u32,
        36u32,
        43u32,
        51u32,
        59u32,
        25u32,
        20u32,
        25u32,
        33u32,
        43u32,
        54u32,
        64u32,
        74u32,
        34u32,
        26u32,
        31u32,
        39u32,
        51u32,
        64u32,
        77u32,
        91u32,
        45u32,
        33u32,
        38u32,
        47u32,
        59u32,
        74u32,
        91u32,
        108u32,
    ],
];

static mut std_chrominance_quant_tbl: [[c_uint; 64]; 9] = [
    [
        17u32,
        18u32,
        24u32,
        47u32,
        99u32,
        99u32,
        99u32,
        99u32,
        18u32,
        21u32,
        26u32,
        66u32,
        99u32,
        99u32,
        99u32,
        99u32,
        24u32,
        26u32,
        56u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        47u32,
        66u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
    ],
    [
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
        16u32,
    ],
    [
        8u32,
        12u32,
        15u32,
        15u32,
        86u32,
        96u32,
        96u32,
        98u32,
        13u32,
        13u32,
        15u32,
        26u32,
        90u32,
        96u32,
        99u32,
        98u32,
        12u32,
        15u32,
        18u32,
        96u32,
        99u32,
        99u32,
        99u32,
        99u32,
        17u32,
        16u32,
        90u32,
        96u32,
        99u32,
        99u32,
        99u32,
        99u32,
        96u32,
        96u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
    ],
    [
        16u32,
        16u32,
        16u32,
        18u32,
        25u32,
        37u32,
        56u32,
        85u32,
        16u32,
        17u32,
        20u32,
        27u32,
        34u32,
        40u32,
        53u32,
        75u32,
        16u32,
        20u32,
        24u32,
        31u32,
        43u32,
        62u32,
        91u32,
        135u32,
        18u32,
        27u32,
        31u32,
        40u32,
        53u32,
        74u32,
        106u32,
        156u32,
        25u32,
        34u32,
        43u32,
        53u32,
        69u32,
        94u32,
        131u32,
        189u32,
        37u32,
        40u32,
        62u32,
        74u32,
        94u32,
        124u32,
        169u32,
        238u32,
        56u32,
        53u32,
        91u32,
        106u32,
        131u32,
        169u32,
        226u32,
        311u32,
        85u32,
        75u32,
        135u32,
        156u32,
        189u32,
        238u32,
        311u32,
        418u32,
    ],
    [
        9u32,
        10u32,
        17u32,
        19u32,
        62u32,
        89u32,
        91u32,
        97u32,
        12u32,
        13u32,
        18u32,
        29u32,
        84u32,
        91u32,
        88u32,
        98u32,
        14u32,
        19u32,
        29u32,
        93u32,
        95u32,
        95u32,
        98u32,
        97u32,
        20u32,
        26u32,
        84u32,
        88u32,
        95u32,
        95u32,
        98u32,
        94u32,
        26u32,
        86u32,
        91u32,
        93u32,
        97u32,
        99u32,
        98u32,
        99u32,
        99u32,
        100u32,
        98u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        99u32,
        97u32,
        97u32,
        99u32,
        99u32,
        99u32,
        99u32,
        97u32,
        99u32,
    ],
    [
        10u32,
        12u32,
        14u32,
        19u32,
        26u32,
        38u32,
        57u32,
        86u32,
        12u32,
        18u32,
        21u32,
        28u32,
        35u32,
        41u32,
        54u32,
        76u32,
        14u32,
        21u32,
        25u32,
        32u32,
        44u32,
        63u32,
        92u32,
        136u32,
        19u32,
        28u32,
        32u32,
        41u32,
        54u32,
        75u32,
        107u32,
        157u32,
        26u32,
        35u32,
        44u32,
        54u32,
        70u32,
        95u32,
        132u32,
        190u32,
        38u32,
        41u32,
        63u32,
        75u32,
        95u32,
        125u32,
        170u32,
        239u32,
        57u32,
        54u32,
        92u32,
        107u32,
        132u32,
        170u32,
        227u32,
        312u32,
        86u32,
        76u32,
        136u32,
        157u32,
        190u32,
        239u32,
        312u32,
        419u32,
    ],
    [
        7u32,
        8u32,
        10u32,
        14u32,
        23u32,
        44u32,
        95u32,
        241u32,
        8u32,
        8u32,
        11u32,
        15u32,
        25u32,
        47u32,
        102u32,
        255u32,
        10u32,
        11u32,
        13u32,
        19u32,
        31u32,
        58u32,
        127u32,
        255u32,
        14u32,
        15u32,
        19u32,
        27u32,
        44u32,
        83u32,
        181u32,
        255u32,
        23u32,
        25u32,
        31u32,
        44u32,
        72u32,
        136u32,
        255u32,
        255u32,
        44u32,
        47u32,
        58u32,
        83u32,
        136u32,
        255u32,
        255u32,
        255u32,
        95u32,
        102u32,
        127u32,
        181u32,
        255u32,
        255u32,
        255u32,
        255u32,
        241u32,
        255u32,
        255u32,
        255u32,
        255u32,
        255u32,
        255u32,
        255u32,
    ],
    [
        15u32,
        11u32,
        11u32,
        12u32,
        15u32,
        19u32,
        25u32,
        32u32,
        11u32,
        13u32,
        10u32,
        10u32,
        12u32,
        15u32,
        19u32,
        24u32,
        11u32,
        10u32,
        14u32,
        14u32,
        16u32,
        18u32,
        22u32,
        27u32,
        12u32,
        10u32,
        14u32,
        18u32,
        21u32,
        24u32,
        28u32,
        33u32,
        15u32,
        12u32,
        16u32,
        21u32,
        26u32,
        31u32,
        36u32,
        42u32,
        19u32,
        15u32,
        18u32,
        24u32,
        31u32,
        38u32,
        45u32,
        53u32,
        25u32,
        19u32,
        22u32,
        28u32,
        36u32,
        45u32,
        55u32,
        65u32,
        32u32,
        24u32,
        27u32,
        33u32,
        42u32,
        53u32,
        65u32,
        77u32,
    ],
    [
        14u32,
        10u32,
        11u32,
        14u32,
        19u32,
        25u32,
        34u32,
        45u32,
        10u32,
        11u32,
        11u32,
        12u32,
        15u32,
        20u32,
        26u32,
        33u32,
        11u32,
        11u32,
        15u32,
        18u32,
        21u32,
        25u32,
        31u32,
        38u32,
        14u32,
        12u32,
        18u32,
        24u32,
        28u32,
        33u32,
        39u32,
        47u32,
        19u32,
        15u32,
        21u32,
        28u32,
        36u32,
        43u32,
        51u32,
        59u32,
        25u32,
        20u32,
        25u32,
        33u32,
        43u32,
        54u32,
        64u32,
        74u32,
        34u32,
        26u32,
        31u32,
        39u32,
        51u32,
        64u32,
        77u32,
        91u32,
        45u32,
        33u32,
        38u32,
        47u32,
        59u32,
        74u32,
        91u32,
        108u32,
    ],
];

unsafe extern "C" fn jpeg_default_qtables(
    mut cinfo: j_compress_ptr,
    mut force_baseline: boolean,
) {
     let mut quant_tbl_master_idx:  c_int =  0i32;
    if jpeg_c_int_param_supported(
        cinfo,
        JINT_BASE_QUANT_TBL_IDX,
    ) != 0
    {
        quant_tbl_master_idx =
            jpeg_c_get_int_param(cinfo, JINT_BASE_QUANT_TBL_IDX)
    }
    jpeg_add_quant_table(
        cinfo,
        0i32,
        std_luminance_quant_tbl[quant_tbl_master_idx as usize].as_ptr(),
        q_scale_factor[0],
        force_baseline,
    );
    jpeg_add_quant_table(
        cinfo,
        1i32,
        std_chrominance_quant_tbl[quant_tbl_master_idx as usize].as_ptr(),
        q_scale_factor[1],
        force_baseline,
    );
}
#[no_mangle]

pub unsafe extern "C" fn set_quality_ratings(
    mut cinfo: j_compress_ptr,
    mut arg: *mut c_char,
    mut force_baseline: boolean,
) -> boolean
/* Process a quality-ratings parameter string, of the form
 *     N[,N,...]
 * If there are more q-table slots than parameters, the last value is replicated.
 */ {
     /* default value */
     /* if not set by sscanf, will be ',' */
     let mut val:  c_float =  75.0f32; 
     let mut tblno:   c_int =  0i32;
    while tblno < NUM_QUANT_TBLS {
        if *arg != 0 {
              let mut ch:   c_char =   ',' as c_char;
            if sscanf(
                arg,
                
                b"%f%c\x00".as_ptr() as *const c_char,
                &mut val as *mut c_float,
                &mut ch as *mut c_char,
            ) < 1i32
            {
                return FALSE;
            }
            if ch as c_int != ',' as i32 {
                /* syntax check */
                return FALSE;
            }
            /* Convert user 0-100 rating to percentage scaling */
            q_scale_factor[tblno as usize] =
                jpeg_float_quality_scaling(val) as c_int;
            /* advance to next segment of arg string */
            while *arg as c_int != 0 && {
                let fresh0 = arg;
                arg = arg.offset(1);
                (*fresh0 as c_int) != ',' as i32
            } {}
        } else {
            /* reached end of parameter, set remaining factors to last value */
            q_scale_factor[tblno as usize] =
                jpeg_float_quality_scaling(val) as c_int
        }
        tblno += 1
    }
    jpeg_default_qtables(cinfo, force_baseline);
    /* For some images chroma subsampling significantly degrades color quality,
    making it impossible to achieve high visual quality regardless of quality setting.
    To make the quality setting more intuitive, disable subsampling when high-quality
    color is desired. */
    if val >= 90f32 {
        set_sample_factors(
            cinfo,
            
            
            
            b"1x1\x00".as_ptr() as *mut c_char,
        );
    } else if val >= 80f32 {
        set_sample_factors(
            cinfo,
            
            
            
            b"2x1\x00".as_ptr() as *mut c_char,
        );
    }
    return TRUE;
}
#[no_mangle]

pub unsafe extern "C" fn set_quant_slots(
    mut cinfo: j_compress_ptr,
    mut arg: *mut c_char,
) -> boolean
/* Process a quantization-table-selectors parameter string, of the form
 *     N[,N,...]
 * If there are more components than parameters, the last value is replicated.
 */ {
     
     let mut ci:   c_int =  0i32;
    while ci < MAX_COMPONENTS {
         let mut val:  c_int =  0i32;if *arg != 0 {
              let mut ch:   c_char =   ',' as c_char;
            if sscanf(
                arg,
                
                b"%d%c\x00".as_ptr() as *const c_char,
                &mut val as *mut c_int,
                &mut ch as *mut c_char,
            ) < 1i32
            {
                return FALSE;
            }
            if ch as c_int != ',' as i32 {
                /* syntax check */
                return FALSE;
            }
            if val < 0i32 || val >= NUM_QUANT_TBLS {
                        eprintln!("JPEG quantization tables are numbered 0..{:}",
          NUM_QUANT_TBLS - 1i32);
                return FALSE;
            }
            (*(*cinfo).comp_info.offset(ci as isize)).quant_tbl_no = val;
            /* advance to next segment of arg string */
            while *arg as c_int != 0 && {
                let fresh1 = arg;
                arg = arg.offset(1);
                (*fresh1 as c_int) != ',' as i32
            } {}
        } else {
            /* reached end of parameter, set remaining components to last table */
            (*(*cinfo).comp_info.offset(ci as isize)).quant_tbl_no = val
        }
        ci += 1
    }
    return TRUE;
}
/*
 * cdjpeg.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2017, D. R. Commander.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README.ijg file.
 *
 * This file contains common declarations for the sample applications
 * cjpeg and djpeg.  It is NOT used by the core JPEG library.
 */
/* define proper options in jconfig.h */
/* cjpeg.c,djpeg.c need to see xxx_SUPPORTED */
/*
 * Object interface for cjpeg's source file decoding modules
 */
/*
 * Object interface for djpeg's output file encoding modules
 */
/* start_output is called after jpeg_start_decompress finishes.
 * The color map will be ready at this time, if one is needed.
 */
/* Emit the specified number of pixel rows from the buffer. */
/* Finish up at the end of the image. */
/* Re-calculate buffer dimensions based on output dimensions (for use with
partial image decompression.)  If this is NULL, then the output format
does not support partial image decompression (BMP and RLE, in particular,
cannot support partial decompression because they use an inversion buffer
to write the image in bottom-up order.) */
/* Target file spec; filled in by djpeg.c after object is created. */
/* Output pixel-row buffer.  Created by module init or start_output.
 * Width is cinfo->output_width * cinfo->output_components;
 * height is buffer_height.
 */
/*
 * cjpeg/djpeg may need to perform extra passes to convert to or from
 * the source/destination file format.  The JPEG library does not know
 * about these passes, but we'd like them to be counted by the progress
 * monitor.  We use an expanded progress monitor object to hold the
 * additional pass count.
 */
/* fields known to JPEG library */
/* extra passes completed */
/* total extra */
/* last printed percentage stored here to avoid multiple printouts */
/* Module selection routines for I/O modules. */
/* cjpeg support routines (in rdswitch.c) */
#[no_mangle]

pub unsafe extern "C" fn set_sample_factors(
    mut cinfo: j_compress_ptr,
    mut arg: *mut c_char,
) -> boolean
/* Process a sample-factors parameter string, of the form
 *     HxV[,HxV,...]
 * If there are more components than parameters, "1x1" is assumed for the rest.
 */ {
     
     let mut ci:   c_int =  0i32;
    while ci < MAX_COMPONENTS {
        if *arg != 0 {
             let mut val1:  c_int =  0; let mut val2:  c_int =  0; let mut ch1:  c_char =  0;  let mut ch2:   c_char =   ',' as c_char;
            if sscanf(
                arg,
                
                b"%d%c%d%c\x00".as_ptr() as *const c_char,
                &mut val1 as *mut c_int,
                &mut ch1 as *mut c_char,
                &mut val2 as *mut c_int,
                &mut ch2 as *mut c_char,
            ) < 3i32
            {
                return FALSE;
            }
            if ch1 as c_int != 'x' as i32 && ch1 as c_int != 'X' as i32
                || ch2 as c_int != ',' as i32
            {
                /* syntax check */
                return FALSE;
            }
            if val1 <= 0i32 || val1 > 4i32 || val2 <= 0i32 || val2 > 4i32 {
                 eprintln!("JPEG sampling factors must be 1..4");
                return FALSE;
            }
            (*(*cinfo).comp_info.offset(ci as isize)).h_samp_factor = val1;
            (*(*cinfo).comp_info.offset(ci as isize)).v_samp_factor = val2;
            /* advance to next segment of arg string */
            while *arg as c_int != 0 && {
                let fresh2 = arg;
                arg = arg.offset(1);
                (*fresh2 as c_int) != ',' as i32
            } {}
        } else {
            /* reached end of parameter, set remaining components to 1x1 sampling */
            (*(*cinfo).comp_info.offset(ci as isize)).h_samp_factor = 1i32;
            (*(*cinfo).comp_info.offset(ci as isize)).v_samp_factor = 1i32
        }
        ci += 1
    }
    return TRUE;
}
