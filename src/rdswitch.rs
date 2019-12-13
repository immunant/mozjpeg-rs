use ::libc;

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
pub use crate::jpegint_h::jpeg_c_coef_controller;
pub use crate::jpegint_h::jpeg_c_main_controller;
pub use crate::jpegint_h::jpeg_c_prep_controller;
pub use crate::jpegint_h::jpeg_color_converter;
pub use crate::jpegint_h::jpeg_comp_master;
pub use crate::jpegint_h::jpeg_downsampler;
pub use crate::jpegint_h::jpeg_entropy_encoder;
pub use crate::jpegint_h::jpeg_forward_dct;
pub use crate::jpegint_h::jpeg_marker_writer;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_error_mgr;
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
pub use crate::src::jcext::jpeg_c_get_int_param;
pub use crate::src::jcext::jpeg_c_int_param_supported;
pub use crate::src::jcext::jpeg_c_set_bool_param;
pub use crate::src::jcparam::jpeg_add_quant_table;
pub use crate::src::jcparam::jpeg_float_quality_scaling;
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

unsafe extern "C" fn text_getc(mut file: *mut crate::stdlib::FILE) -> libc::c_int
/* Read next char, skipping over any comments (# to end of line) */
/* A comment/newline sequence is returned as a newline */ {
    let mut ch: libc::c_int = 0;
    ch = crate::stdlib::getc(file);
    if ch == '#' as i32 {
        loop {
            ch = crate::stdlib::getc(file);
            if !(ch != '\n' as i32 && ch != crate::stdlib::EOF) {
                break;
            }
        }
    }
    return ch;
}

unsafe extern "C" fn read_text_integer(
    mut file: *mut crate::stdlib::FILE,
    mut result: *mut libc::c_long,
    mut termchar: *mut libc::c_int,
) -> crate::jmorecfg_h::boolean
/* Read an unsigned decimal integer from a file, store it in result */
/* Reads one trailing character after the integer; returns it in termchar */ {
    let mut ch: libc::c_int = 0;
    let mut val: libc::c_long = 0;
    loop
    /* Skip any leading whitespace, detect EOF */
    {
        ch = text_getc(file);
        if ch == crate::stdlib::EOF {
            *termchar = ch;
            return crate::jmorecfg_h::FALSE;
        }
        if !(*(*crate::stdlib::__ctype_b_loc()).offset(ch as isize) as libc::c_int
            & crate::stdlib::_ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0)
        {
            break;
        }
    }
    if *(*crate::stdlib::__ctype_b_loc()).offset(ch as isize) as libc::c_int
        & crate::stdlib::_ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        == 0
    {
        *termchar = ch;
        return crate::jmorecfg_h::FALSE;
    }
    val = (ch - '0' as i32) as libc::c_long;
    loop {
        ch = text_getc(file);
        if !(ch != crate::stdlib::EOF) {
            break;
        }
        if *(*crate::stdlib::__ctype_b_loc()).offset(ch as isize) as libc::c_int
            & crate::stdlib::_ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            break;
        }
        val *= 10 as libc::c_int as libc::c_long;
        val += (ch - '0' as i32) as libc::c_long
    }
    *result = val;
    *termchar = ch;
    return crate::jmorecfg_h::TRUE;
}

static mut q_scale_factor: [libc::c_int; 4] = [
    100 as libc::c_int,
    100 as libc::c_int,
    100 as libc::c_int,
    100 as libc::c_int,
];
#[no_mangle]

pub unsafe extern "C" fn read_quant_tables(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut filename: *mut libc::c_char,
    mut force_baseline: crate::jmorecfg_h::boolean,
) -> crate::jmorecfg_h::boolean
/* Read a set of quantization tables from the specified file.
 * The file is plain ASCII text: decimal numbers with whitespace between.
 * Comments preceded by '#' may be included in the file.
 * There may be one to NUM_QUANT_TBLS tables in the file, each of 64 values.
 * The tables are implicitly numbered 0,1,etc.
 * NOTE: does not affect the qslots mapping, which will default to selecting
 * table 0 for luminance (or primary) components, 1 for chrominance components.
 * You must use -qslots if you want a different component->table mapping.
 */ {
    let mut fp: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    let mut tblno: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut termchar: libc::c_int = 0;
    let mut val: libc::c_long = 0;
    let mut table: [libc::c_uint; 64] = [0; 64];
    fp = crate::stdlib::fopen(filename, b"r\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"Can\'t open table file %s\n\x00" as *const u8 as *const libc::c_char,
            filename,
        );
        return crate::jmorecfg_h::FALSE;
    }
    tblno = 0 as libc::c_int;
    while read_text_integer(fp, &mut val, &mut termchar) != 0 {
        /* read 1st element of table */
        if tblno >= crate::jpeglib_h::NUM_QUANT_TBLS {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"Too many tables in file %s\n\x00" as *const u8 as *const libc::c_char,
                filename,
            );
            crate::stdlib::fclose(fp);
            return crate::jmorecfg_h::FALSE;
        }
        table[0 as libc::c_int as usize] = val as libc::c_uint;
        i = 1 as libc::c_int;
        while i < crate::jpeglib_h::DCTSIZE2 {
            if read_text_integer(fp, &mut val, &mut termchar) == 0 {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"Invalid table data in file %s\n\x00" as *const u8 as *const libc::c_char,
                    filename,
                );
                crate::stdlib::fclose(fp);
                return crate::jmorecfg_h::FALSE;
            }
            table[i as usize] = val as libc::c_uint;
            i += 1
        }
        crate::src::jcparam::jpeg_add_quant_table(
            cinfo,
            tblno,
            table.as_mut_ptr(),
            q_scale_factor[tblno as usize],
            force_baseline,
        );
        tblno += 1
    }
    if termchar != crate::stdlib::EOF {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"Non-numeric data in file %s\n\x00" as *const u8 as *const libc::c_char,
            filename,
        );
        crate::stdlib::fclose(fp);
        return crate::jmorecfg_h::FALSE;
    }
    crate::stdlib::fclose(fp);
    return crate::jmorecfg_h::TRUE;
}

unsafe extern "C" fn read_scan_integer(
    mut file: *mut crate::stdlib::FILE,
    mut result: *mut libc::c_long,
    mut termchar: *mut libc::c_int,
) -> crate::jmorecfg_h::boolean
/* Variant of read_text_integer that always looks for a non-space termchar;
 * this simplifies parsing of punctuation in scan scripts.
 */ {
    let mut ch: libc::c_int = 0;
    if read_text_integer(file, result, termchar) == 0 {
        return crate::jmorecfg_h::FALSE;
    }
    ch = *termchar;
    while ch != crate::stdlib::EOF
        && *(*crate::stdlib::__ctype_b_loc()).offset(ch as isize) as libc::c_int
            & crate::stdlib::_ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        ch = text_getc(file)
    }
    if *(*crate::stdlib::__ctype_b_loc()).offset(ch as isize) as libc::c_int
        & crate::stdlib::_ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        != 0
    {
        /* oops, put it back */
        if crate::stdlib::ungetc(ch, file) == crate::stdlib::EOF {
            return crate::jmorecfg_h::FALSE;
        }
        ch = ' ' as i32
    } else if ch != crate::stdlib::EOF && ch != ';' as i32 && ch != ':' as i32 {
        ch = ' ' as i32
    }
    *termchar = ch;
    return crate::jmorecfg_h::TRUE;
}
#[no_mangle]

pub unsafe extern "C" fn read_scan_script(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut filename: *mut libc::c_char,
) -> crate::jmorecfg_h::boolean
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
    let mut current_block: u64;
    let mut fp: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    let mut scanno: libc::c_int = 0;
    let mut ncomps: libc::c_int = 0;
    let mut termchar: libc::c_int = 0;
    let mut val: libc::c_long = 0;
    let mut scanptr: *mut crate::jpeglib_h::jpeg_scan_info =
        0 as *mut crate::jpeglib_h::jpeg_scan_info;
    /* quite arbitrary limit */
    let mut scans: [crate::jpeglib_h::jpeg_scan_info; 100] = [crate::jpeglib_h::jpeg_scan_info {
        comps_in_scan: 0,
        component_index: [0; 4],
        Ss: 0,
        Se: 0,
        Ah: 0,
        Al: 0,
    }; 100];
    fp = crate::stdlib::fopen(filename, b"r\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"Can\'t open scan definition file %s\n\x00" as *const u8 as *const libc::c_char,
            filename,
        );
        return crate::jmorecfg_h::FALSE;
    }
    scanptr = scans.as_mut_ptr();
    scanno = 0 as libc::c_int;
    while read_scan_integer(fp, &mut val, &mut termchar) != 0 {
        if scanno >= MAX_SCANS {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"Too many scans defined in file %s\n\x00" as *const u8 as *const libc::c_char,
                filename,
            );
            crate::stdlib::fclose(fp);
            return crate::jmorecfg_h::FALSE;
        }
        (*scanptr).component_index[0 as libc::c_int as usize] = val as libc::c_int;
        ncomps = 1 as libc::c_int;
        loop {
            if !(termchar == ' ' as i32) {
                current_block = 1109700713171191020;
                break;
            }
            if ncomps >= crate::jpeglib_h::MAX_COMPS_IN_SCAN {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"Too many components in one scan in file %s\n\x00" as *const u8
                        as *const libc::c_char,
                    filename,
                );
                crate::stdlib::fclose(fp);
                return crate::jmorecfg_h::FALSE;
            }
            if read_scan_integer(fp, &mut val, &mut termchar) == 0 {
                current_block = 9520589643232431964;
                break;
            }
            (*scanptr).component_index[ncomps as usize] = val as libc::c_int;
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
                        (*scanptr).Ss = val as libc::c_int;
                        if read_scan_integer(fp, &mut val, &mut termchar) == 0
                            || termchar != ' ' as i32
                        {
                            current_block = 9520589643232431964;
                        } else {
                            (*scanptr).Se = val as libc::c_int;
                            if read_scan_integer(fp, &mut val, &mut termchar) == 0
                                || termchar != ' ' as i32
                            {
                                current_block = 9520589643232431964;
                            } else {
                                (*scanptr).Ah = val as libc::c_int;
                                if read_scan_integer(fp, &mut val, &mut termchar) == 0 {
                                    current_block = 9520589643232431964;
                                } else {
                                    (*scanptr).Al = val as libc::c_int;
                                    current_block = 8845338526596852646;
                                }
                            }
                        }
                    }
                } else {
                    /* set non-progressive parameters */
                    (*scanptr).Ss = 0 as libc::c_int;
                    (*scanptr).Se = crate::jpeglib_h::DCTSIZE2 - 1 as libc::c_int;
                    (*scanptr).Ah = 0 as libc::c_int;
                    (*scanptr).Al = 0 as libc::c_int;
                    current_block = 8845338526596852646;
                }
                match current_block {
                    9520589643232431964 => {}
                    _ => {
                        if !(termchar != ';' as i32 && termchar != crate::stdlib::EOF) {
                            scanptr = scanptr.offset(1);
                            scanno += 1;
                            continue;
                        }
                    }
                }
            }
            _ => {}
        }
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"Invalid scan entry format in file %s\n\x00" as *const u8 as *const libc::c_char,
            filename,
        );
        crate::stdlib::fclose(fp);
        return crate::jmorecfg_h::FALSE;
    }
    if termchar != crate::stdlib::EOF {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"Non-numeric data in file %s\n\x00" as *const u8 as *const libc::c_char,
            filename,
        );
        crate::stdlib::fclose(fp);
        return crate::jmorecfg_h::FALSE;
    }
    if scanno > 0 as libc::c_int {
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
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            (scanno as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::jpeglib_h::jpeg_scan_info,
            >() as libc::c_ulong),
        ) as *mut crate::jpeglib_h::jpeg_scan_info;
        crate::stdlib::memcpy(
            scanptr as *mut libc::c_void,
            scans.as_mut_ptr() as *const libc::c_void,
            (scanno as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::jpeglib_h::jpeg_scan_info,
            >() as libc::c_ulong),
        );
        (*cinfo).scan_info = scanptr;
        (*cinfo).num_scans = scanno;
        /* Disable scan optimization if using custom scan */
        crate::src::jcext::jpeg_c_set_bool_param(
            cinfo,
            crate::jpeglib_h::JBOOLEAN_OPTIMIZE_SCANS,
            crate::jmorecfg_h::FALSE,
        );
    }
    crate::stdlib::fclose(fp);
    return crate::jmorecfg_h::TRUE;
}

pub const MAX_SCANS: libc::c_int = 100 as libc::c_int;
/* C_MULTISCAN_FILES_SUPPORTED */
/* These are the sample quantization tables given in Annex K (Clause K.1) of
 * Recommendation ITU-T T.81 (1992) | ISO/IEC 10918-1:1994.
 * The spec says that the values given produce "good" quality, and
 * when divided by 2, "very good" quality.
 */

static mut std_luminance_quant_tbl: [[libc::c_uint; 64]; 9] = [
    [
        16 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        24 as libc::c_int as libc::c_uint,
        40 as libc::c_int as libc::c_uint,
        51 as libc::c_int as libc::c_uint,
        61 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        58 as libc::c_int as libc::c_uint,
        60 as libc::c_int as libc::c_uint,
        55 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        13 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        24 as libc::c_int as libc::c_uint,
        40 as libc::c_int as libc::c_uint,
        57 as libc::c_int as libc::c_uint,
        69 as libc::c_int as libc::c_uint,
        56 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        17 as libc::c_int as libc::c_uint,
        22 as libc::c_int as libc::c_uint,
        29 as libc::c_int as libc::c_uint,
        51 as libc::c_int as libc::c_uint,
        87 as libc::c_int as libc::c_uint,
        80 as libc::c_int as libc::c_uint,
        62 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        22 as libc::c_int as libc::c_uint,
        37 as libc::c_int as libc::c_uint,
        56 as libc::c_int as libc::c_uint,
        68 as libc::c_int as libc::c_uint,
        109 as libc::c_int as libc::c_uint,
        103 as libc::c_int as libc::c_uint,
        77 as libc::c_int as libc::c_uint,
        24 as libc::c_int as libc::c_uint,
        35 as libc::c_int as libc::c_uint,
        55 as libc::c_int as libc::c_uint,
        64 as libc::c_int as libc::c_uint,
        81 as libc::c_int as libc::c_uint,
        104 as libc::c_int as libc::c_uint,
        113 as libc::c_int as libc::c_uint,
        92 as libc::c_int as libc::c_uint,
        49 as libc::c_int as libc::c_uint,
        64 as libc::c_int as libc::c_uint,
        78 as libc::c_int as libc::c_uint,
        87 as libc::c_int as libc::c_uint,
        103 as libc::c_int as libc::c_uint,
        121 as libc::c_int as libc::c_uint,
        120 as libc::c_int as libc::c_uint,
        101 as libc::c_int as libc::c_uint,
        72 as libc::c_int as libc::c_uint,
        92 as libc::c_int as libc::c_uint,
        95 as libc::c_int as libc::c_uint,
        98 as libc::c_int as libc::c_uint,
        112 as libc::c_int as libc::c_uint,
        100 as libc::c_int as libc::c_uint,
        103 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
    ],
    [
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
    ],
    [
        12 as libc::c_int as libc::c_uint,
        17 as libc::c_int as libc::c_uint,
        20 as libc::c_int as libc::c_uint,
        21 as libc::c_int as libc::c_uint,
        30 as libc::c_int as libc::c_uint,
        34 as libc::c_int as libc::c_uint,
        56 as libc::c_int as libc::c_uint,
        63 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        20 as libc::c_int as libc::c_uint,
        20 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        28 as libc::c_int as libc::c_uint,
        51 as libc::c_int as libc::c_uint,
        61 as libc::c_int as libc::c_uint,
        55 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        20 as libc::c_int as libc::c_uint,
        21 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        33 as libc::c_int as libc::c_uint,
        58 as libc::c_int as libc::c_uint,
        69 as libc::c_int as libc::c_uint,
        55 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        30 as libc::c_int as libc::c_uint,
        46 as libc::c_int as libc::c_uint,
        87 as libc::c_int as libc::c_uint,
        86 as libc::c_int as libc::c_uint,
        66 as libc::c_int as libc::c_uint,
        31 as libc::c_int as libc::c_uint,
        33 as libc::c_int as libc::c_uint,
        36 as libc::c_int as libc::c_uint,
        40 as libc::c_int as libc::c_uint,
        46 as libc::c_int as libc::c_uint,
        96 as libc::c_int as libc::c_uint,
        100 as libc::c_int as libc::c_uint,
        73 as libc::c_int as libc::c_uint,
        40 as libc::c_int as libc::c_uint,
        35 as libc::c_int as libc::c_uint,
        46 as libc::c_int as libc::c_uint,
        62 as libc::c_int as libc::c_uint,
        81 as libc::c_int as libc::c_uint,
        100 as libc::c_int as libc::c_uint,
        111 as libc::c_int as libc::c_uint,
        91 as libc::c_int as libc::c_uint,
        46 as libc::c_int as libc::c_uint,
        66 as libc::c_int as libc::c_uint,
        76 as libc::c_int as libc::c_uint,
        86 as libc::c_int as libc::c_uint,
        102 as libc::c_int as libc::c_uint,
        121 as libc::c_int as libc::c_uint,
        120 as libc::c_int as libc::c_uint,
        101 as libc::c_int as libc::c_uint,
        68 as libc::c_int as libc::c_uint,
        90 as libc::c_int as libc::c_uint,
        90 as libc::c_int as libc::c_uint,
        96 as libc::c_int as libc::c_uint,
        113 as libc::c_int as libc::c_uint,
        102 as libc::c_int as libc::c_uint,
        105 as libc::c_int as libc::c_uint,
        103 as libc::c_int as libc::c_uint,
    ],
    [
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        37 as libc::c_int as libc::c_uint,
        56 as libc::c_int as libc::c_uint,
        85 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        17 as libc::c_int as libc::c_uint,
        20 as libc::c_int as libc::c_uint,
        27 as libc::c_int as libc::c_uint,
        34 as libc::c_int as libc::c_uint,
        40 as libc::c_int as libc::c_uint,
        53 as libc::c_int as libc::c_uint,
        75 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        20 as libc::c_int as libc::c_uint,
        24 as libc::c_int as libc::c_uint,
        31 as libc::c_int as libc::c_uint,
        43 as libc::c_int as libc::c_uint,
        62 as libc::c_int as libc::c_uint,
        91 as libc::c_int as libc::c_uint,
        135 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        27 as libc::c_int as libc::c_uint,
        31 as libc::c_int as libc::c_uint,
        40 as libc::c_int as libc::c_uint,
        53 as libc::c_int as libc::c_uint,
        74 as libc::c_int as libc::c_uint,
        106 as libc::c_int as libc::c_uint,
        156 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        34 as libc::c_int as libc::c_uint,
        43 as libc::c_int as libc::c_uint,
        53 as libc::c_int as libc::c_uint,
        69 as libc::c_int as libc::c_uint,
        94 as libc::c_int as libc::c_uint,
        131 as libc::c_int as libc::c_uint,
        189 as libc::c_int as libc::c_uint,
        37 as libc::c_int as libc::c_uint,
        40 as libc::c_int as libc::c_uint,
        62 as libc::c_int as libc::c_uint,
        74 as libc::c_int as libc::c_uint,
        94 as libc::c_int as libc::c_uint,
        124 as libc::c_int as libc::c_uint,
        169 as libc::c_int as libc::c_uint,
        238 as libc::c_int as libc::c_uint,
        56 as libc::c_int as libc::c_uint,
        53 as libc::c_int as libc::c_uint,
        91 as libc::c_int as libc::c_uint,
        106 as libc::c_int as libc::c_uint,
        131 as libc::c_int as libc::c_uint,
        169 as libc::c_int as libc::c_uint,
        226 as libc::c_int as libc::c_uint,
        311 as libc::c_int as libc::c_uint,
        85 as libc::c_int as libc::c_uint,
        75 as libc::c_int as libc::c_uint,
        135 as libc::c_int as libc::c_uint,
        156 as libc::c_int as libc::c_uint,
        189 as libc::c_int as libc::c_uint,
        238 as libc::c_int as libc::c_uint,
        311 as libc::c_int as libc::c_uint,
        418 as libc::c_int as libc::c_uint,
    ],
    [
        9 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        27 as libc::c_int as libc::c_uint,
        32 as libc::c_int as libc::c_uint,
        51 as libc::c_int as libc::c_uint,
        62 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        27 as libc::c_int as libc::c_uint,
        44 as libc::c_int as libc::c_uint,
        59 as libc::c_int as libc::c_uint,
        73 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        42 as libc::c_int as libc::c_uint,
        59 as libc::c_int as libc::c_uint,
        79 as libc::c_int as libc::c_uint,
        78 as libc::c_int as libc::c_uint,
        17 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        42 as libc::c_int as libc::c_uint,
        61 as libc::c_int as libc::c_uint,
        92 as libc::c_int as libc::c_uint,
        87 as libc::c_int as libc::c_uint,
        92 as libc::c_int as libc::c_uint,
        23 as libc::c_int as libc::c_uint,
        28 as libc::c_int as libc::c_uint,
        42 as libc::c_int as libc::c_uint,
        75 as libc::c_int as libc::c_uint,
        79 as libc::c_int as libc::c_uint,
        112 as libc::c_int as libc::c_uint,
        112 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        40 as libc::c_int as libc::c_uint,
        42 as libc::c_int as libc::c_uint,
        59 as libc::c_int as libc::c_uint,
        84 as libc::c_int as libc::c_uint,
        88 as libc::c_int as libc::c_uint,
        124 as libc::c_int as libc::c_uint,
        132 as libc::c_int as libc::c_uint,
        111 as libc::c_int as libc::c_uint,
        42 as libc::c_int as libc::c_uint,
        64 as libc::c_int as libc::c_uint,
        78 as libc::c_int as libc::c_uint,
        95 as libc::c_int as libc::c_uint,
        105 as libc::c_int as libc::c_uint,
        126 as libc::c_int as libc::c_uint,
        125 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        70 as libc::c_int as libc::c_uint,
        75 as libc::c_int as libc::c_uint,
        100 as libc::c_int as libc::c_uint,
        102 as libc::c_int as libc::c_uint,
        116 as libc::c_int as libc::c_uint,
        100 as libc::c_int as libc::c_uint,
        107 as libc::c_int as libc::c_uint,
        98 as libc::c_int as libc::c_uint,
    ],
    [
        10 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        38 as libc::c_int as libc::c_uint,
        57 as libc::c_int as libc::c_uint,
        86 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        21 as libc::c_int as libc::c_uint,
        28 as libc::c_int as libc::c_uint,
        35 as libc::c_int as libc::c_uint,
        41 as libc::c_int as libc::c_uint,
        54 as libc::c_int as libc::c_uint,
        76 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        21 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        32 as libc::c_int as libc::c_uint,
        44 as libc::c_int as libc::c_uint,
        63 as libc::c_int as libc::c_uint,
        92 as libc::c_int as libc::c_uint,
        136 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        28 as libc::c_int as libc::c_uint,
        32 as libc::c_int as libc::c_uint,
        41 as libc::c_int as libc::c_uint,
        54 as libc::c_int as libc::c_uint,
        75 as libc::c_int as libc::c_uint,
        107 as libc::c_int as libc::c_uint,
        157 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        35 as libc::c_int as libc::c_uint,
        44 as libc::c_int as libc::c_uint,
        54 as libc::c_int as libc::c_uint,
        70 as libc::c_int as libc::c_uint,
        95 as libc::c_int as libc::c_uint,
        132 as libc::c_int as libc::c_uint,
        190 as libc::c_int as libc::c_uint,
        38 as libc::c_int as libc::c_uint,
        41 as libc::c_int as libc::c_uint,
        63 as libc::c_int as libc::c_uint,
        75 as libc::c_int as libc::c_uint,
        95 as libc::c_int as libc::c_uint,
        125 as libc::c_int as libc::c_uint,
        170 as libc::c_int as libc::c_uint,
        239 as libc::c_int as libc::c_uint,
        57 as libc::c_int as libc::c_uint,
        54 as libc::c_int as libc::c_uint,
        92 as libc::c_int as libc::c_uint,
        107 as libc::c_int as libc::c_uint,
        132 as libc::c_int as libc::c_uint,
        170 as libc::c_int as libc::c_uint,
        227 as libc::c_int as libc::c_uint,
        312 as libc::c_int as libc::c_uint,
        86 as libc::c_int as libc::c_uint,
        76 as libc::c_int as libc::c_uint,
        136 as libc::c_int as libc::c_uint,
        157 as libc::c_int as libc::c_uint,
        190 as libc::c_int as libc::c_uint,
        239 as libc::c_int as libc::c_uint,
        312 as libc::c_int as libc::c_uint,
        419 as libc::c_int as libc::c_uint,
    ],
    [
        7 as libc::c_int as libc::c_uint,
        8 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        23 as libc::c_int as libc::c_uint,
        44 as libc::c_int as libc::c_uint,
        95 as libc::c_int as libc::c_uint,
        241 as libc::c_int as libc::c_uint,
        8 as libc::c_int as libc::c_uint,
        8 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        47 as libc::c_int as libc::c_uint,
        102 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        13 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        31 as libc::c_int as libc::c_uint,
        58 as libc::c_int as libc::c_uint,
        127 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        27 as libc::c_int as libc::c_uint,
        44 as libc::c_int as libc::c_uint,
        83 as libc::c_int as libc::c_uint,
        181 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        23 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        31 as libc::c_int as libc::c_uint,
        44 as libc::c_int as libc::c_uint,
        72 as libc::c_int as libc::c_uint,
        136 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        44 as libc::c_int as libc::c_uint,
        47 as libc::c_int as libc::c_uint,
        58 as libc::c_int as libc::c_uint,
        83 as libc::c_int as libc::c_uint,
        136 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        95 as libc::c_int as libc::c_uint,
        102 as libc::c_int as libc::c_uint,
        127 as libc::c_int as libc::c_uint,
        181 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        241 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
    ],
    [
        15 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        32 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        13 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        24 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        22 as libc::c_int as libc::c_uint,
        27 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        21 as libc::c_int as libc::c_uint,
        24 as libc::c_int as libc::c_uint,
        28 as libc::c_int as libc::c_uint,
        33 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        21 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        31 as libc::c_int as libc::c_uint,
        36 as libc::c_int as libc::c_uint,
        42 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        24 as libc::c_int as libc::c_uint,
        31 as libc::c_int as libc::c_uint,
        38 as libc::c_int as libc::c_uint,
        45 as libc::c_int as libc::c_uint,
        53 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        22 as libc::c_int as libc::c_uint,
        28 as libc::c_int as libc::c_uint,
        36 as libc::c_int as libc::c_uint,
        45 as libc::c_int as libc::c_uint,
        55 as libc::c_int as libc::c_uint,
        65 as libc::c_int as libc::c_uint,
        32 as libc::c_int as libc::c_uint,
        24 as libc::c_int as libc::c_uint,
        27 as libc::c_int as libc::c_uint,
        33 as libc::c_int as libc::c_uint,
        42 as libc::c_int as libc::c_uint,
        53 as libc::c_int as libc::c_uint,
        65 as libc::c_int as libc::c_uint,
        77 as libc::c_int as libc::c_uint,
    ],
    [
        14 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        34 as libc::c_int as libc::c_uint,
        45 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        20 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        33 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        21 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        31 as libc::c_int as libc::c_uint,
        38 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        24 as libc::c_int as libc::c_uint,
        28 as libc::c_int as libc::c_uint,
        33 as libc::c_int as libc::c_uint,
        39 as libc::c_int as libc::c_uint,
        47 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        21 as libc::c_int as libc::c_uint,
        28 as libc::c_int as libc::c_uint,
        36 as libc::c_int as libc::c_uint,
        43 as libc::c_int as libc::c_uint,
        51 as libc::c_int as libc::c_uint,
        59 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        20 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        33 as libc::c_int as libc::c_uint,
        43 as libc::c_int as libc::c_uint,
        54 as libc::c_int as libc::c_uint,
        64 as libc::c_int as libc::c_uint,
        74 as libc::c_int as libc::c_uint,
        34 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        31 as libc::c_int as libc::c_uint,
        39 as libc::c_int as libc::c_uint,
        51 as libc::c_int as libc::c_uint,
        64 as libc::c_int as libc::c_uint,
        77 as libc::c_int as libc::c_uint,
        91 as libc::c_int as libc::c_uint,
        45 as libc::c_int as libc::c_uint,
        33 as libc::c_int as libc::c_uint,
        38 as libc::c_int as libc::c_uint,
        47 as libc::c_int as libc::c_uint,
        59 as libc::c_int as libc::c_uint,
        74 as libc::c_int as libc::c_uint,
        91 as libc::c_int as libc::c_uint,
        108 as libc::c_int as libc::c_uint,
    ],
];

static mut std_chrominance_quant_tbl: [[libc::c_uint; 64]; 9] = [
    [
        17 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        24 as libc::c_int as libc::c_uint,
        47 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        21 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        66 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        24 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        56 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        47 as libc::c_int as libc::c_uint,
        66 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
    ],
    [
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
    ],
    [
        8 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        86 as libc::c_int as libc::c_uint,
        96 as libc::c_int as libc::c_uint,
        96 as libc::c_int as libc::c_uint,
        98 as libc::c_int as libc::c_uint,
        13 as libc::c_int as libc::c_uint,
        13 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        90 as libc::c_int as libc::c_uint,
        96 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        98 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        96 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        17 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        90 as libc::c_int as libc::c_uint,
        96 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        96 as libc::c_int as libc::c_uint,
        96 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
    ],
    [
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        37 as libc::c_int as libc::c_uint,
        56 as libc::c_int as libc::c_uint,
        85 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        17 as libc::c_int as libc::c_uint,
        20 as libc::c_int as libc::c_uint,
        27 as libc::c_int as libc::c_uint,
        34 as libc::c_int as libc::c_uint,
        40 as libc::c_int as libc::c_uint,
        53 as libc::c_int as libc::c_uint,
        75 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        20 as libc::c_int as libc::c_uint,
        24 as libc::c_int as libc::c_uint,
        31 as libc::c_int as libc::c_uint,
        43 as libc::c_int as libc::c_uint,
        62 as libc::c_int as libc::c_uint,
        91 as libc::c_int as libc::c_uint,
        135 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        27 as libc::c_int as libc::c_uint,
        31 as libc::c_int as libc::c_uint,
        40 as libc::c_int as libc::c_uint,
        53 as libc::c_int as libc::c_uint,
        74 as libc::c_int as libc::c_uint,
        106 as libc::c_int as libc::c_uint,
        156 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        34 as libc::c_int as libc::c_uint,
        43 as libc::c_int as libc::c_uint,
        53 as libc::c_int as libc::c_uint,
        69 as libc::c_int as libc::c_uint,
        94 as libc::c_int as libc::c_uint,
        131 as libc::c_int as libc::c_uint,
        189 as libc::c_int as libc::c_uint,
        37 as libc::c_int as libc::c_uint,
        40 as libc::c_int as libc::c_uint,
        62 as libc::c_int as libc::c_uint,
        74 as libc::c_int as libc::c_uint,
        94 as libc::c_int as libc::c_uint,
        124 as libc::c_int as libc::c_uint,
        169 as libc::c_int as libc::c_uint,
        238 as libc::c_int as libc::c_uint,
        56 as libc::c_int as libc::c_uint,
        53 as libc::c_int as libc::c_uint,
        91 as libc::c_int as libc::c_uint,
        106 as libc::c_int as libc::c_uint,
        131 as libc::c_int as libc::c_uint,
        169 as libc::c_int as libc::c_uint,
        226 as libc::c_int as libc::c_uint,
        311 as libc::c_int as libc::c_uint,
        85 as libc::c_int as libc::c_uint,
        75 as libc::c_int as libc::c_uint,
        135 as libc::c_int as libc::c_uint,
        156 as libc::c_int as libc::c_uint,
        189 as libc::c_int as libc::c_uint,
        238 as libc::c_int as libc::c_uint,
        311 as libc::c_int as libc::c_uint,
        418 as libc::c_int as libc::c_uint,
    ],
    [
        9 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        17 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        62 as libc::c_int as libc::c_uint,
        89 as libc::c_int as libc::c_uint,
        91 as libc::c_int as libc::c_uint,
        97 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        13 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        29 as libc::c_int as libc::c_uint,
        84 as libc::c_int as libc::c_uint,
        91 as libc::c_int as libc::c_uint,
        88 as libc::c_int as libc::c_uint,
        98 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        29 as libc::c_int as libc::c_uint,
        93 as libc::c_int as libc::c_uint,
        95 as libc::c_int as libc::c_uint,
        95 as libc::c_int as libc::c_uint,
        98 as libc::c_int as libc::c_uint,
        97 as libc::c_int as libc::c_uint,
        20 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        84 as libc::c_int as libc::c_uint,
        88 as libc::c_int as libc::c_uint,
        95 as libc::c_int as libc::c_uint,
        95 as libc::c_int as libc::c_uint,
        98 as libc::c_int as libc::c_uint,
        94 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        86 as libc::c_int as libc::c_uint,
        91 as libc::c_int as libc::c_uint,
        93 as libc::c_int as libc::c_uint,
        97 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        98 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        100 as libc::c_int as libc::c_uint,
        98 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        97 as libc::c_int as libc::c_uint,
        97 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
        97 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
    ],
    [
        10 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        38 as libc::c_int as libc::c_uint,
        57 as libc::c_int as libc::c_uint,
        86 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        21 as libc::c_int as libc::c_uint,
        28 as libc::c_int as libc::c_uint,
        35 as libc::c_int as libc::c_uint,
        41 as libc::c_int as libc::c_uint,
        54 as libc::c_int as libc::c_uint,
        76 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        21 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        32 as libc::c_int as libc::c_uint,
        44 as libc::c_int as libc::c_uint,
        63 as libc::c_int as libc::c_uint,
        92 as libc::c_int as libc::c_uint,
        136 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        28 as libc::c_int as libc::c_uint,
        32 as libc::c_int as libc::c_uint,
        41 as libc::c_int as libc::c_uint,
        54 as libc::c_int as libc::c_uint,
        75 as libc::c_int as libc::c_uint,
        107 as libc::c_int as libc::c_uint,
        157 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        35 as libc::c_int as libc::c_uint,
        44 as libc::c_int as libc::c_uint,
        54 as libc::c_int as libc::c_uint,
        70 as libc::c_int as libc::c_uint,
        95 as libc::c_int as libc::c_uint,
        132 as libc::c_int as libc::c_uint,
        190 as libc::c_int as libc::c_uint,
        38 as libc::c_int as libc::c_uint,
        41 as libc::c_int as libc::c_uint,
        63 as libc::c_int as libc::c_uint,
        75 as libc::c_int as libc::c_uint,
        95 as libc::c_int as libc::c_uint,
        125 as libc::c_int as libc::c_uint,
        170 as libc::c_int as libc::c_uint,
        239 as libc::c_int as libc::c_uint,
        57 as libc::c_int as libc::c_uint,
        54 as libc::c_int as libc::c_uint,
        92 as libc::c_int as libc::c_uint,
        107 as libc::c_int as libc::c_uint,
        132 as libc::c_int as libc::c_uint,
        170 as libc::c_int as libc::c_uint,
        227 as libc::c_int as libc::c_uint,
        312 as libc::c_int as libc::c_uint,
        86 as libc::c_int as libc::c_uint,
        76 as libc::c_int as libc::c_uint,
        136 as libc::c_int as libc::c_uint,
        157 as libc::c_int as libc::c_uint,
        190 as libc::c_int as libc::c_uint,
        239 as libc::c_int as libc::c_uint,
        312 as libc::c_int as libc::c_uint,
        419 as libc::c_int as libc::c_uint,
    ],
    [
        7 as libc::c_int as libc::c_uint,
        8 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        23 as libc::c_int as libc::c_uint,
        44 as libc::c_int as libc::c_uint,
        95 as libc::c_int as libc::c_uint,
        241 as libc::c_int as libc::c_uint,
        8 as libc::c_int as libc::c_uint,
        8 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        47 as libc::c_int as libc::c_uint,
        102 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        13 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        31 as libc::c_int as libc::c_uint,
        58 as libc::c_int as libc::c_uint,
        127 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        27 as libc::c_int as libc::c_uint,
        44 as libc::c_int as libc::c_uint,
        83 as libc::c_int as libc::c_uint,
        181 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        23 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        31 as libc::c_int as libc::c_uint,
        44 as libc::c_int as libc::c_uint,
        72 as libc::c_int as libc::c_uint,
        136 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        44 as libc::c_int as libc::c_uint,
        47 as libc::c_int as libc::c_uint,
        58 as libc::c_int as libc::c_uint,
        83 as libc::c_int as libc::c_uint,
        136 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        95 as libc::c_int as libc::c_uint,
        102 as libc::c_int as libc::c_uint,
        127 as libc::c_int as libc::c_uint,
        181 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        241 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
        255 as libc::c_int as libc::c_uint,
    ],
    [
        15 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        32 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        13 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        24 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        22 as libc::c_int as libc::c_uint,
        27 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        21 as libc::c_int as libc::c_uint,
        24 as libc::c_int as libc::c_uint,
        28 as libc::c_int as libc::c_uint,
        33 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        21 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        31 as libc::c_int as libc::c_uint,
        36 as libc::c_int as libc::c_uint,
        42 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        24 as libc::c_int as libc::c_uint,
        31 as libc::c_int as libc::c_uint,
        38 as libc::c_int as libc::c_uint,
        45 as libc::c_int as libc::c_uint,
        53 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        22 as libc::c_int as libc::c_uint,
        28 as libc::c_int as libc::c_uint,
        36 as libc::c_int as libc::c_uint,
        45 as libc::c_int as libc::c_uint,
        55 as libc::c_int as libc::c_uint,
        65 as libc::c_int as libc::c_uint,
        32 as libc::c_int as libc::c_uint,
        24 as libc::c_int as libc::c_uint,
        27 as libc::c_int as libc::c_uint,
        33 as libc::c_int as libc::c_uint,
        42 as libc::c_int as libc::c_uint,
        53 as libc::c_int as libc::c_uint,
        65 as libc::c_int as libc::c_uint,
        77 as libc::c_int as libc::c_uint,
    ],
    [
        14 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        34 as libc::c_int as libc::c_uint,
        45 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        20 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        33 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        21 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        31 as libc::c_int as libc::c_uint,
        38 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        24 as libc::c_int as libc::c_uint,
        28 as libc::c_int as libc::c_uint,
        33 as libc::c_int as libc::c_uint,
        39 as libc::c_int as libc::c_uint,
        47 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
        21 as libc::c_int as libc::c_uint,
        28 as libc::c_int as libc::c_uint,
        36 as libc::c_int as libc::c_uint,
        43 as libc::c_int as libc::c_uint,
        51 as libc::c_int as libc::c_uint,
        59 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        20 as libc::c_int as libc::c_uint,
        25 as libc::c_int as libc::c_uint,
        33 as libc::c_int as libc::c_uint,
        43 as libc::c_int as libc::c_uint,
        54 as libc::c_int as libc::c_uint,
        64 as libc::c_int as libc::c_uint,
        74 as libc::c_int as libc::c_uint,
        34 as libc::c_int as libc::c_uint,
        26 as libc::c_int as libc::c_uint,
        31 as libc::c_int as libc::c_uint,
        39 as libc::c_int as libc::c_uint,
        51 as libc::c_int as libc::c_uint,
        64 as libc::c_int as libc::c_uint,
        77 as libc::c_int as libc::c_uint,
        91 as libc::c_int as libc::c_uint,
        45 as libc::c_int as libc::c_uint,
        33 as libc::c_int as libc::c_uint,
        38 as libc::c_int as libc::c_uint,
        47 as libc::c_int as libc::c_uint,
        59 as libc::c_int as libc::c_uint,
        74 as libc::c_int as libc::c_uint,
        91 as libc::c_int as libc::c_uint,
        108 as libc::c_int as libc::c_uint,
    ],
];

unsafe extern "C" fn jpeg_default_qtables(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut force_baseline: crate::jmorecfg_h::boolean,
) {
    let mut quant_tbl_master_idx: libc::c_int = 0 as libc::c_int;
    if crate::src::jcext::jpeg_c_int_param_supported(
        cinfo,
        crate::jpeglib_h::JINT_BASE_QUANT_TBL_IDX,
    ) != 0
    {
        quant_tbl_master_idx = crate::src::jcext::jpeg_c_get_int_param(
            cinfo,
            crate::jpeglib_h::JINT_BASE_QUANT_TBL_IDX,
        )
    }
    crate::src::jcparam::jpeg_add_quant_table(
        cinfo,
        0 as libc::c_int,
        std_luminance_quant_tbl[quant_tbl_master_idx as usize].as_ptr(),
        q_scale_factor[0 as libc::c_int as usize],
        force_baseline,
    );
    crate::src::jcparam::jpeg_add_quant_table(
        cinfo,
        1 as libc::c_int,
        std_chrominance_quant_tbl[quant_tbl_master_idx as usize].as_ptr(),
        q_scale_factor[1 as libc::c_int as usize],
        force_baseline,
    );
}
#[no_mangle]

pub unsafe extern "C" fn set_quality_ratings(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut arg: *mut libc::c_char,
    mut force_baseline: crate::jmorecfg_h::boolean,
) -> crate::jmorecfg_h::boolean
/* Process a quality-ratings parameter string, of the form
 *     N[,N,...]
 * If there are more q-table slots than parameters, the last value is replicated.
 */ {
    let mut val: libc::c_float = 75.0f32; /* default value */
    let mut tblno: libc::c_int = 0; /* if not set by sscanf, will be ',' */
    let mut ch: libc::c_char = 0;
    tblno = 0 as libc::c_int;
    while tblno < crate::jpeglib_h::NUM_QUANT_TBLS {
        if *arg != 0 {
            ch = ',' as i32 as libc::c_char;
            if crate::stdlib::sscanf(
                arg,
                b"%f%c\x00" as *const u8 as *const libc::c_char,
                &mut val as *mut libc::c_float,
                &mut ch as *mut libc::c_char,
            ) < 1 as libc::c_int
            {
                return crate::jmorecfg_h::FALSE;
            }
            if ch as libc::c_int != ',' as i32 {
                /* syntax check */
                return crate::jmorecfg_h::FALSE;
            }
            /* Convert user 0-100 rating to percentage scaling */
            q_scale_factor[tblno as usize] =
                crate::src::jcparam::jpeg_float_quality_scaling(val) as libc::c_int;
            /* advance to next segment of arg string */
            while *arg as libc::c_int != 0 && {
                let fresh0 = arg;
                arg = arg.offset(1);
                (*fresh0 as libc::c_int) != ',' as i32
            } {}
        } else {
            /* reached end of parameter, set remaining factors to last value */
            q_scale_factor[tblno as usize] =
                crate::src::jcparam::jpeg_float_quality_scaling(val) as libc::c_int
        }
        tblno += 1
    }
    jpeg_default_qtables(cinfo, force_baseline);
    /* For some images chroma subsampling significantly degrades color quality,
    making it impossible to achieve high visual quality regardless of quality setting.
    To make the quality setting more intuitive, disable subsampling when high-quality
    color is desired. */
    if val >= 90 as libc::c_int as libc::c_float {
        set_sample_factors(
            cinfo,
            b"1x1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else if val >= 80 as libc::c_int as libc::c_float {
        set_sample_factors(
            cinfo,
            b"2x1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return crate::jmorecfg_h::TRUE;
}
#[no_mangle]

pub unsafe extern "C" fn set_quant_slots(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut arg: *mut libc::c_char,
) -> crate::jmorecfg_h::boolean
/* Process a quantization-table-selectors parameter string, of the form
 *     N[,N,...]
 * If there are more components than parameters, the last value is replicated.
 */ {
    let mut val: libc::c_int = 0 as libc::c_int; /* default table # */
    let mut ci: libc::c_int = 0; /* if not set by sscanf, will be ',' */
    let mut ch: libc::c_char = 0;
    ci = 0 as libc::c_int;
    while ci < crate::jmorecfg_h::MAX_COMPONENTS {
        if *arg != 0 {
            ch = ',' as i32 as libc::c_char;
            if crate::stdlib::sscanf(
                arg,
                b"%d%c\x00" as *const u8 as *const libc::c_char,
                &mut val as *mut libc::c_int,
                &mut ch as *mut libc::c_char,
            ) < 1 as libc::c_int
            {
                return crate::jmorecfg_h::FALSE;
            }
            if ch as libc::c_int != ',' as i32 {
                /* syntax check */
                return crate::jmorecfg_h::FALSE;
            }
            if val < 0 as libc::c_int || val >= crate::jpeglib_h::NUM_QUANT_TBLS {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"JPEG quantization tables are numbered 0..%d\n\x00" as *const u8
                        as *const libc::c_char,
                    crate::jpeglib_h::NUM_QUANT_TBLS - 1 as libc::c_int,
                );
                return crate::jmorecfg_h::FALSE;
            }
            (*(*cinfo).comp_info.offset(ci as isize)).quant_tbl_no = val;
            /* advance to next segment of arg string */
            while *arg as libc::c_int != 0 && {
                let fresh1 = arg;
                arg = arg.offset(1);
                (*fresh1 as libc::c_int) != ',' as i32
            } {}
        } else {
            /* reached end of parameter, set remaining components to last table */
            (*(*cinfo).comp_info.offset(ci as isize)).quant_tbl_no = val
        }
        ci += 1
    }
    return crate::jmorecfg_h::TRUE;
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
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut arg: *mut libc::c_char,
) -> crate::jmorecfg_h::boolean
/* Process a sample-factors parameter string, of the form
 *     HxV[,HxV,...]
 * If there are more components than parameters, "1x1" is assumed for the rest.
 */ {
    let mut ci: libc::c_int = 0; /* if not set by sscanf, will be ',' */
    let mut val1: libc::c_int = 0;
    let mut val2: libc::c_int = 0;
    let mut ch1: libc::c_char = 0;
    let mut ch2: libc::c_char = 0;
    ci = 0 as libc::c_int;
    while ci < crate::jmorecfg_h::MAX_COMPONENTS {
        if *arg != 0 {
            ch2 = ',' as i32 as libc::c_char;
            if crate::stdlib::sscanf(
                arg,
                b"%d%c%d%c\x00" as *const u8 as *const libc::c_char,
                &mut val1 as *mut libc::c_int,
                &mut ch1 as *mut libc::c_char,
                &mut val2 as *mut libc::c_int,
                &mut ch2 as *mut libc::c_char,
            ) < 3 as libc::c_int
            {
                return crate::jmorecfg_h::FALSE;
            }
            if ch1 as libc::c_int != 'x' as i32 && ch1 as libc::c_int != 'X' as i32
                || ch2 as libc::c_int != ',' as i32
            {
                /* syntax check */
                return crate::jmorecfg_h::FALSE;
            }
            if val1 <= 0 as libc::c_int
                || val1 > 4 as libc::c_int
                || val2 <= 0 as libc::c_int
                || val2 > 4 as libc::c_int
            {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"JPEG sampling factors must be 1..4\n\x00" as *const u8 as *const libc::c_char,
                );
                return crate::jmorecfg_h::FALSE;
            }
            (*(*cinfo).comp_info.offset(ci as isize)).h_samp_factor = val1;
            (*(*cinfo).comp_info.offset(ci as isize)).v_samp_factor = val2;
            /* advance to next segment of arg string */
            while *arg as libc::c_int != 0 && {
                let fresh2 = arg;
                arg = arg.offset(1);
                (*fresh2 as libc::c_int) != ',' as i32
            } {}
        } else {
            /* reached end of parameter, set remaining components to 1x1 sampling */
            (*(*cinfo).comp_info.offset(ci as isize)).h_samp_factor = 1 as libc::c_int;
            (*(*cinfo).comp_info.offset(ci as isize)).v_samp_factor = 1 as libc::c_int
        }
        ci += 1
    }
    return crate::jmorecfg_h::TRUE;
}
