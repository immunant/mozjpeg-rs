pub use crate::jmorecfg_h::{
    boolean, FALSE, JCOEF, JDIMENSION, JOCTET, JSAMPLE, MAX_COMPONENTS, TRUE, UINT16, UINT8,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_compress_ptr, jpeg_add_quant_table, jpeg_c_coef_controller,
    jpeg_c_get_int_param, jpeg_c_int_param_supported, jpeg_c_main_controller,
    jpeg_c_prep_controller, jpeg_c_set_bool_param, jpeg_color_converter, jpeg_common_struct,
    jpeg_comp_master, jpeg_component_info, jpeg_compress_struct, jpeg_destination_mgr,
    jpeg_downsampler, jpeg_entropy_encoder, jpeg_error_mgr, jpeg_float_quality_scaling,
    jpeg_forward_dct, jpeg_marker_writer, jpeg_memory_mgr, jpeg_progress_mgr, jpeg_scan_info,
    jvirt_barray_control, jvirt_barray_ptr, jvirt_sarray_control, jvirt_sarray_ptr,
    C2RustUnnamed_2, JCS_YCbCr, DCTSIZE2, JBLOCK, JBLOCKARRAY, JBLOCKROW, JBOOLEAN_OPTIMIZE_SCANS,
    JBOOLEAN_OVERSHOOT_DERINGING, JBOOLEAN_TRELLIS_EOB_OPT, JBOOLEAN_TRELLIS_QUANT,
    JBOOLEAN_TRELLIS_QUANT_DC, JBOOLEAN_TRELLIS_Q_OPT, JBOOLEAN_USE_LAMBDA_WEIGHT_TBL,
    JBOOLEAN_USE_SCANS_IN_TRELLIS, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA,
    JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB,
    JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW,
    JHUFF_TBL, JINT_BASE_QUANT_TBL_IDX, JINT_COMPRESS_PROFILE, JINT_DC_SCAN_OPT_MODE,
    JINT_TRELLIS_FREQ_SPLIT, JINT_TRELLIS_NUM_LOOPS, JPOOL_IMAGE, JQUANT_TBL, JSAMPARRAY, JSAMPROW,
    J_BOOLEAN_PARAM, J_COLOR_SPACE, J_DCT_METHOD, J_INT_PARAM, MAX_COMPS_IN_SCAN, NUM_QUANT_TBLS,
};
pub use crate::stddef_h::{size_t, NULL};
use crate::stdlib::memcpy;
pub use crate::stdlib::{
    C2RustUnnamed_0, _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _ISalnum, _ISalpha,
    _ISblank, _IScntrl, _ISdigit, _ISgraph, _ISlower, _ISprint, _ISpunct, _ISspace, _ISupper,
    _ISxdigit, __ctype_b_loc, __off64_t, __off_t, fclose, fopen, fprintf, getc, sscanf, stderr,
    ungetc, EOF, FILE, _IO_FILE,
};
use libc::{self, c_char, c_float, c_int, c_long, c_uint, c_ulong, c_ushort, c_void};
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
    let mut ch: c_int = 0;
    ch = getc(file);
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
    let mut ch: c_int = 0;
    let mut val: c_long = 0;
    loop
    /* Skip any leading whitespace, detect EOF */
    {
        ch = text_getc(file);
        if ch == EOF {
            *termchar = ch;
            return FALSE;
        }
        if !(*(*__ctype_b_loc()).offset(ch as isize) as c_int
            & _ISspace as c_int as c_ushort as c_int
            != 0)
        {
            break;
        }
    }
    if *(*__ctype_b_loc()).offset(ch as isize) as c_int & _ISdigit as c_int as c_ushort as c_int
        == 0
    {
        *termchar = ch;
        return FALSE;
    }
    val = (ch - '0' as i32) as c_long;
    loop {
        ch = text_getc(file);
        if !(ch != EOF) {
            break;
        }
        if *(*__ctype_b_loc()).offset(ch as isize) as c_int & _ISdigit as c_int as c_ushort as c_int
            == 0
        {
            break;
        }
        val *= 10i32 as c_long;
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
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut tblno: c_int = 0;
    let mut i: c_int = 0;
    let mut termchar: c_int = 0;
    let mut val: c_long = 0;
    let mut table: [c_uint; 64] = [0; 64];
    fp = fopen(filename, b"r\x00" as *const u8 as *const c_char);
    if fp.is_null() {
        fprintf(
            stderr,
            b"Can\'t open table file %s\n\x00" as *const u8 as *const c_char,
            filename,
        );
        return FALSE;
    }
    tblno = 0i32;
    while read_text_integer(fp, &mut val, &mut termchar) != 0 {
        /* read 1st element of table */
        if tblno >= NUM_QUANT_TBLS {
            fprintf(
                stderr,
                b"Too many tables in file %s\n\x00" as *const u8 as *const c_char,
                filename,
            );
            fclose(fp);
            return FALSE;
        }
        table[0] = val as c_uint;
        i = 1i32;
        while i < DCTSIZE2 {
            if read_text_integer(fp, &mut val, &mut termchar) == 0 {
                fprintf(
                    stderr,
                    b"Invalid table data in file %s\n\x00" as *const u8 as *const c_char,
                    filename,
                );
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
        fprintf(
            stderr,
            b"Non-numeric data in file %s\n\x00" as *const u8 as *const c_char,
            filename,
        );
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
    let mut ch: c_int = 0;
    if read_text_integer(file, result, termchar) == 0 {
        return FALSE;
    }
    ch = *termchar;
    while ch != EOF
        && *(*__ctype_b_loc()).offset(ch as isize) as c_int & _ISspace as c_int as c_ushort as c_int
            != 0
    {
        ch = text_getc(file)
    }
    if *(*__ctype_b_loc()).offset(ch as isize) as c_int & _ISdigit as c_int as c_ushort as c_int
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
    let mut current_block: u64;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut scanno: c_int = 0;
    let mut ncomps: c_int = 0;
    let mut termchar: c_int = 0;
    let mut val: c_long = 0;
    let mut scanptr: *mut jpeg_scan_info = 0 as *mut jpeg_scan_info;
    /* quite arbitrary limit */
    let mut scans: [jpeg_scan_info; 100] = [jpeg_scan_info {
        comps_in_scan: 0,
        component_index: [0; 4],
        Ss: 0,
        Se: 0,
        Ah: 0,
        Al: 0,
    }; 100];
    fp = fopen(filename, b"r\x00" as *const u8 as *const c_char);
    if fp.is_null() {
        fprintf(
            stderr,
            b"Can\'t open scan definition file %s\n\x00" as *const u8 as *const c_char,
            filename,
        );
        return FALSE;
    }
    scanptr = scans.as_mut_ptr();
    scanno = 0i32;
    while read_scan_integer(fp, &mut val, &mut termchar) != 0 {
        if scanno >= MAX_SCANS {
            fprintf(
                stderr,
                b"Too many scans defined in file %s\n\x00" as *const u8 as *const c_char,
                filename,
            );
            fclose(fp);
            return FALSE;
        }
        (*scanptr).component_index[0] = val as c_int;
        ncomps = 1i32;
        loop {
            if !(termchar == ' ' as i32) {
                current_block = 1109700713171191020;
                break;
            }
            if ncomps >= MAX_COMPS_IN_SCAN {
                fprintf(
                    stderr,
                    b"Too many components in one scan in file %s\n\x00" as *const u8
                        as *const c_char,
                    filename,
                );
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
        fprintf(
            stderr,
            b"Invalid scan entry format in file %s\n\x00" as *const u8 as *const c_char,
            filename,
        );
        fclose(fp);
        return FALSE;
    }
    if termchar != EOF {
        fprintf(
            stderr,
            b"Non-numeric data in file %s\n\x00" as *const u8 as *const c_char,
            filename,
        );
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
            (scanno as c_ulong).wrapping_mul(::std::mem::size_of::<jpeg_scan_info>() as c_ulong),
        ) as *mut jpeg_scan_info;
        memcpy(
            scanptr as *mut c_void,
            scans.as_mut_ptr() as *const c_void,
            (scanno as c_ulong).wrapping_mul(::std::mem::size_of::<jpeg_scan_info>() as c_ulong),
        );
        (*cinfo).scan_info = scanptr;
        (*cinfo).num_scans = scanno;
        /* Disable scan optimization if using custom scan */
        jpeg_c_set_bool_param(cinfo, JBOOLEAN_OPTIMIZE_SCANS, FALSE);
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
        16i32 as c_uint,
        11i32 as c_uint,
        10i32 as c_uint,
        16i32 as c_uint,
        24i32 as c_uint,
        40i32 as c_uint,
        51i32 as c_uint,
        61i32 as c_uint,
        12i32 as c_uint,
        12i32 as c_uint,
        14i32 as c_uint,
        19i32 as c_uint,
        26i32 as c_uint,
        58i32 as c_uint,
        60i32 as c_uint,
        55i32 as c_uint,
        14i32 as c_uint,
        13i32 as c_uint,
        16i32 as c_uint,
        24i32 as c_uint,
        40i32 as c_uint,
        57i32 as c_uint,
        69i32 as c_uint,
        56i32 as c_uint,
        14i32 as c_uint,
        17i32 as c_uint,
        22i32 as c_uint,
        29i32 as c_uint,
        51i32 as c_uint,
        87i32 as c_uint,
        80i32 as c_uint,
        62i32 as c_uint,
        18i32 as c_uint,
        22i32 as c_uint,
        37i32 as c_uint,
        56i32 as c_uint,
        68i32 as c_uint,
        109i32 as c_uint,
        103i32 as c_uint,
        77i32 as c_uint,
        24i32 as c_uint,
        35i32 as c_uint,
        55i32 as c_uint,
        64i32 as c_uint,
        81i32 as c_uint,
        104i32 as c_uint,
        113i32 as c_uint,
        92i32 as c_uint,
        49i32 as c_uint,
        64i32 as c_uint,
        78i32 as c_uint,
        87i32 as c_uint,
        103i32 as c_uint,
        121i32 as c_uint,
        120i32 as c_uint,
        101i32 as c_uint,
        72i32 as c_uint,
        92i32 as c_uint,
        95i32 as c_uint,
        98i32 as c_uint,
        112i32 as c_uint,
        100i32 as c_uint,
        103i32 as c_uint,
        99i32 as c_uint,
    ],
    [
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
    ],
    [
        12i32 as c_uint,
        17i32 as c_uint,
        20i32 as c_uint,
        21i32 as c_uint,
        30i32 as c_uint,
        34i32 as c_uint,
        56i32 as c_uint,
        63i32 as c_uint,
        18i32 as c_uint,
        20i32 as c_uint,
        20i32 as c_uint,
        26i32 as c_uint,
        28i32 as c_uint,
        51i32 as c_uint,
        61i32 as c_uint,
        55i32 as c_uint,
        19i32 as c_uint,
        20i32 as c_uint,
        21i32 as c_uint,
        26i32 as c_uint,
        33i32 as c_uint,
        58i32 as c_uint,
        69i32 as c_uint,
        55i32 as c_uint,
        26i32 as c_uint,
        26i32 as c_uint,
        26i32 as c_uint,
        30i32 as c_uint,
        46i32 as c_uint,
        87i32 as c_uint,
        86i32 as c_uint,
        66i32 as c_uint,
        31i32 as c_uint,
        33i32 as c_uint,
        36i32 as c_uint,
        40i32 as c_uint,
        46i32 as c_uint,
        96i32 as c_uint,
        100i32 as c_uint,
        73i32 as c_uint,
        40i32 as c_uint,
        35i32 as c_uint,
        46i32 as c_uint,
        62i32 as c_uint,
        81i32 as c_uint,
        100i32 as c_uint,
        111i32 as c_uint,
        91i32 as c_uint,
        46i32 as c_uint,
        66i32 as c_uint,
        76i32 as c_uint,
        86i32 as c_uint,
        102i32 as c_uint,
        121i32 as c_uint,
        120i32 as c_uint,
        101i32 as c_uint,
        68i32 as c_uint,
        90i32 as c_uint,
        90i32 as c_uint,
        96i32 as c_uint,
        113i32 as c_uint,
        102i32 as c_uint,
        105i32 as c_uint,
        103i32 as c_uint,
    ],
    [
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        18i32 as c_uint,
        25i32 as c_uint,
        37i32 as c_uint,
        56i32 as c_uint,
        85i32 as c_uint,
        16i32 as c_uint,
        17i32 as c_uint,
        20i32 as c_uint,
        27i32 as c_uint,
        34i32 as c_uint,
        40i32 as c_uint,
        53i32 as c_uint,
        75i32 as c_uint,
        16i32 as c_uint,
        20i32 as c_uint,
        24i32 as c_uint,
        31i32 as c_uint,
        43i32 as c_uint,
        62i32 as c_uint,
        91i32 as c_uint,
        135i32 as c_uint,
        18i32 as c_uint,
        27i32 as c_uint,
        31i32 as c_uint,
        40i32 as c_uint,
        53i32 as c_uint,
        74i32 as c_uint,
        106i32 as c_uint,
        156i32 as c_uint,
        25i32 as c_uint,
        34i32 as c_uint,
        43i32 as c_uint,
        53i32 as c_uint,
        69i32 as c_uint,
        94i32 as c_uint,
        131i32 as c_uint,
        189i32 as c_uint,
        37i32 as c_uint,
        40i32 as c_uint,
        62i32 as c_uint,
        74i32 as c_uint,
        94i32 as c_uint,
        124i32 as c_uint,
        169i32 as c_uint,
        238i32 as c_uint,
        56i32 as c_uint,
        53i32 as c_uint,
        91i32 as c_uint,
        106i32 as c_uint,
        131i32 as c_uint,
        169i32 as c_uint,
        226i32 as c_uint,
        311i32 as c_uint,
        85i32 as c_uint,
        75i32 as c_uint,
        135i32 as c_uint,
        156i32 as c_uint,
        189i32 as c_uint,
        238i32 as c_uint,
        311i32 as c_uint,
        418i32 as c_uint,
    ],
    [
        9i32 as c_uint,
        10i32 as c_uint,
        12i32 as c_uint,
        14i32 as c_uint,
        27i32 as c_uint,
        32i32 as c_uint,
        51i32 as c_uint,
        62i32 as c_uint,
        11i32 as c_uint,
        12i32 as c_uint,
        14i32 as c_uint,
        19i32 as c_uint,
        27i32 as c_uint,
        44i32 as c_uint,
        59i32 as c_uint,
        73i32 as c_uint,
        12i32 as c_uint,
        14i32 as c_uint,
        18i32 as c_uint,
        25i32 as c_uint,
        42i32 as c_uint,
        59i32 as c_uint,
        79i32 as c_uint,
        78i32 as c_uint,
        17i32 as c_uint,
        18i32 as c_uint,
        25i32 as c_uint,
        42i32 as c_uint,
        61i32 as c_uint,
        92i32 as c_uint,
        87i32 as c_uint,
        92i32 as c_uint,
        23i32 as c_uint,
        28i32 as c_uint,
        42i32 as c_uint,
        75i32 as c_uint,
        79i32 as c_uint,
        112i32 as c_uint,
        112i32 as c_uint,
        99i32 as c_uint,
        40i32 as c_uint,
        42i32 as c_uint,
        59i32 as c_uint,
        84i32 as c_uint,
        88i32 as c_uint,
        124i32 as c_uint,
        132i32 as c_uint,
        111i32 as c_uint,
        42i32 as c_uint,
        64i32 as c_uint,
        78i32 as c_uint,
        95i32 as c_uint,
        105i32 as c_uint,
        126i32 as c_uint,
        125i32 as c_uint,
        99i32 as c_uint,
        70i32 as c_uint,
        75i32 as c_uint,
        100i32 as c_uint,
        102i32 as c_uint,
        116i32 as c_uint,
        100i32 as c_uint,
        107i32 as c_uint,
        98i32 as c_uint,
    ],
    [
        10i32 as c_uint,
        12i32 as c_uint,
        14i32 as c_uint,
        19i32 as c_uint,
        26i32 as c_uint,
        38i32 as c_uint,
        57i32 as c_uint,
        86i32 as c_uint,
        12i32 as c_uint,
        18i32 as c_uint,
        21i32 as c_uint,
        28i32 as c_uint,
        35i32 as c_uint,
        41i32 as c_uint,
        54i32 as c_uint,
        76i32 as c_uint,
        14i32 as c_uint,
        21i32 as c_uint,
        25i32 as c_uint,
        32i32 as c_uint,
        44i32 as c_uint,
        63i32 as c_uint,
        92i32 as c_uint,
        136i32 as c_uint,
        19i32 as c_uint,
        28i32 as c_uint,
        32i32 as c_uint,
        41i32 as c_uint,
        54i32 as c_uint,
        75i32 as c_uint,
        107i32 as c_uint,
        157i32 as c_uint,
        26i32 as c_uint,
        35i32 as c_uint,
        44i32 as c_uint,
        54i32 as c_uint,
        70i32 as c_uint,
        95i32 as c_uint,
        132i32 as c_uint,
        190i32 as c_uint,
        38i32 as c_uint,
        41i32 as c_uint,
        63i32 as c_uint,
        75i32 as c_uint,
        95i32 as c_uint,
        125i32 as c_uint,
        170i32 as c_uint,
        239i32 as c_uint,
        57i32 as c_uint,
        54i32 as c_uint,
        92i32 as c_uint,
        107i32 as c_uint,
        132i32 as c_uint,
        170i32 as c_uint,
        227i32 as c_uint,
        312i32 as c_uint,
        86i32 as c_uint,
        76i32 as c_uint,
        136i32 as c_uint,
        157i32 as c_uint,
        190i32 as c_uint,
        239i32 as c_uint,
        312i32 as c_uint,
        419i32 as c_uint,
    ],
    [
        7i32 as c_uint,
        8i32 as c_uint,
        10i32 as c_uint,
        14i32 as c_uint,
        23i32 as c_uint,
        44i32 as c_uint,
        95i32 as c_uint,
        241i32 as c_uint,
        8i32 as c_uint,
        8i32 as c_uint,
        11i32 as c_uint,
        15i32 as c_uint,
        25i32 as c_uint,
        47i32 as c_uint,
        102i32 as c_uint,
        255i32 as c_uint,
        10i32 as c_uint,
        11i32 as c_uint,
        13i32 as c_uint,
        19i32 as c_uint,
        31i32 as c_uint,
        58i32 as c_uint,
        127i32 as c_uint,
        255i32 as c_uint,
        14i32 as c_uint,
        15i32 as c_uint,
        19i32 as c_uint,
        27i32 as c_uint,
        44i32 as c_uint,
        83i32 as c_uint,
        181i32 as c_uint,
        255i32 as c_uint,
        23i32 as c_uint,
        25i32 as c_uint,
        31i32 as c_uint,
        44i32 as c_uint,
        72i32 as c_uint,
        136i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        44i32 as c_uint,
        47i32 as c_uint,
        58i32 as c_uint,
        83i32 as c_uint,
        136i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        95i32 as c_uint,
        102i32 as c_uint,
        127i32 as c_uint,
        181i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        241i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
    ],
    [
        15i32 as c_uint,
        11i32 as c_uint,
        11i32 as c_uint,
        12i32 as c_uint,
        15i32 as c_uint,
        19i32 as c_uint,
        25i32 as c_uint,
        32i32 as c_uint,
        11i32 as c_uint,
        13i32 as c_uint,
        10i32 as c_uint,
        10i32 as c_uint,
        12i32 as c_uint,
        15i32 as c_uint,
        19i32 as c_uint,
        24i32 as c_uint,
        11i32 as c_uint,
        10i32 as c_uint,
        14i32 as c_uint,
        14i32 as c_uint,
        16i32 as c_uint,
        18i32 as c_uint,
        22i32 as c_uint,
        27i32 as c_uint,
        12i32 as c_uint,
        10i32 as c_uint,
        14i32 as c_uint,
        18i32 as c_uint,
        21i32 as c_uint,
        24i32 as c_uint,
        28i32 as c_uint,
        33i32 as c_uint,
        15i32 as c_uint,
        12i32 as c_uint,
        16i32 as c_uint,
        21i32 as c_uint,
        26i32 as c_uint,
        31i32 as c_uint,
        36i32 as c_uint,
        42i32 as c_uint,
        19i32 as c_uint,
        15i32 as c_uint,
        18i32 as c_uint,
        24i32 as c_uint,
        31i32 as c_uint,
        38i32 as c_uint,
        45i32 as c_uint,
        53i32 as c_uint,
        25i32 as c_uint,
        19i32 as c_uint,
        22i32 as c_uint,
        28i32 as c_uint,
        36i32 as c_uint,
        45i32 as c_uint,
        55i32 as c_uint,
        65i32 as c_uint,
        32i32 as c_uint,
        24i32 as c_uint,
        27i32 as c_uint,
        33i32 as c_uint,
        42i32 as c_uint,
        53i32 as c_uint,
        65i32 as c_uint,
        77i32 as c_uint,
    ],
    [
        14i32 as c_uint,
        10i32 as c_uint,
        11i32 as c_uint,
        14i32 as c_uint,
        19i32 as c_uint,
        25i32 as c_uint,
        34i32 as c_uint,
        45i32 as c_uint,
        10i32 as c_uint,
        11i32 as c_uint,
        11i32 as c_uint,
        12i32 as c_uint,
        15i32 as c_uint,
        20i32 as c_uint,
        26i32 as c_uint,
        33i32 as c_uint,
        11i32 as c_uint,
        11i32 as c_uint,
        15i32 as c_uint,
        18i32 as c_uint,
        21i32 as c_uint,
        25i32 as c_uint,
        31i32 as c_uint,
        38i32 as c_uint,
        14i32 as c_uint,
        12i32 as c_uint,
        18i32 as c_uint,
        24i32 as c_uint,
        28i32 as c_uint,
        33i32 as c_uint,
        39i32 as c_uint,
        47i32 as c_uint,
        19i32 as c_uint,
        15i32 as c_uint,
        21i32 as c_uint,
        28i32 as c_uint,
        36i32 as c_uint,
        43i32 as c_uint,
        51i32 as c_uint,
        59i32 as c_uint,
        25i32 as c_uint,
        20i32 as c_uint,
        25i32 as c_uint,
        33i32 as c_uint,
        43i32 as c_uint,
        54i32 as c_uint,
        64i32 as c_uint,
        74i32 as c_uint,
        34i32 as c_uint,
        26i32 as c_uint,
        31i32 as c_uint,
        39i32 as c_uint,
        51i32 as c_uint,
        64i32 as c_uint,
        77i32 as c_uint,
        91i32 as c_uint,
        45i32 as c_uint,
        33i32 as c_uint,
        38i32 as c_uint,
        47i32 as c_uint,
        59i32 as c_uint,
        74i32 as c_uint,
        91i32 as c_uint,
        108i32 as c_uint,
    ],
];

static mut std_chrominance_quant_tbl: [[c_uint; 64]; 9] = [
    [
        17i32 as c_uint,
        18i32 as c_uint,
        24i32 as c_uint,
        47i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        18i32 as c_uint,
        21i32 as c_uint,
        26i32 as c_uint,
        66i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        24i32 as c_uint,
        26i32 as c_uint,
        56i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        47i32 as c_uint,
        66i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
    ],
    [
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
    ],
    [
        8i32 as c_uint,
        12i32 as c_uint,
        15i32 as c_uint,
        15i32 as c_uint,
        86i32 as c_uint,
        96i32 as c_uint,
        96i32 as c_uint,
        98i32 as c_uint,
        13i32 as c_uint,
        13i32 as c_uint,
        15i32 as c_uint,
        26i32 as c_uint,
        90i32 as c_uint,
        96i32 as c_uint,
        99i32 as c_uint,
        98i32 as c_uint,
        12i32 as c_uint,
        15i32 as c_uint,
        18i32 as c_uint,
        96i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        17i32 as c_uint,
        16i32 as c_uint,
        90i32 as c_uint,
        96i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        96i32 as c_uint,
        96i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
    ],
    [
        16i32 as c_uint,
        16i32 as c_uint,
        16i32 as c_uint,
        18i32 as c_uint,
        25i32 as c_uint,
        37i32 as c_uint,
        56i32 as c_uint,
        85i32 as c_uint,
        16i32 as c_uint,
        17i32 as c_uint,
        20i32 as c_uint,
        27i32 as c_uint,
        34i32 as c_uint,
        40i32 as c_uint,
        53i32 as c_uint,
        75i32 as c_uint,
        16i32 as c_uint,
        20i32 as c_uint,
        24i32 as c_uint,
        31i32 as c_uint,
        43i32 as c_uint,
        62i32 as c_uint,
        91i32 as c_uint,
        135i32 as c_uint,
        18i32 as c_uint,
        27i32 as c_uint,
        31i32 as c_uint,
        40i32 as c_uint,
        53i32 as c_uint,
        74i32 as c_uint,
        106i32 as c_uint,
        156i32 as c_uint,
        25i32 as c_uint,
        34i32 as c_uint,
        43i32 as c_uint,
        53i32 as c_uint,
        69i32 as c_uint,
        94i32 as c_uint,
        131i32 as c_uint,
        189i32 as c_uint,
        37i32 as c_uint,
        40i32 as c_uint,
        62i32 as c_uint,
        74i32 as c_uint,
        94i32 as c_uint,
        124i32 as c_uint,
        169i32 as c_uint,
        238i32 as c_uint,
        56i32 as c_uint,
        53i32 as c_uint,
        91i32 as c_uint,
        106i32 as c_uint,
        131i32 as c_uint,
        169i32 as c_uint,
        226i32 as c_uint,
        311i32 as c_uint,
        85i32 as c_uint,
        75i32 as c_uint,
        135i32 as c_uint,
        156i32 as c_uint,
        189i32 as c_uint,
        238i32 as c_uint,
        311i32 as c_uint,
        418i32 as c_uint,
    ],
    [
        9i32 as c_uint,
        10i32 as c_uint,
        17i32 as c_uint,
        19i32 as c_uint,
        62i32 as c_uint,
        89i32 as c_uint,
        91i32 as c_uint,
        97i32 as c_uint,
        12i32 as c_uint,
        13i32 as c_uint,
        18i32 as c_uint,
        29i32 as c_uint,
        84i32 as c_uint,
        91i32 as c_uint,
        88i32 as c_uint,
        98i32 as c_uint,
        14i32 as c_uint,
        19i32 as c_uint,
        29i32 as c_uint,
        93i32 as c_uint,
        95i32 as c_uint,
        95i32 as c_uint,
        98i32 as c_uint,
        97i32 as c_uint,
        20i32 as c_uint,
        26i32 as c_uint,
        84i32 as c_uint,
        88i32 as c_uint,
        95i32 as c_uint,
        95i32 as c_uint,
        98i32 as c_uint,
        94i32 as c_uint,
        26i32 as c_uint,
        86i32 as c_uint,
        91i32 as c_uint,
        93i32 as c_uint,
        97i32 as c_uint,
        99i32 as c_uint,
        98i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        100i32 as c_uint,
        98i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        97i32 as c_uint,
        97i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        99i32 as c_uint,
        97i32 as c_uint,
        99i32 as c_uint,
    ],
    [
        10i32 as c_uint,
        12i32 as c_uint,
        14i32 as c_uint,
        19i32 as c_uint,
        26i32 as c_uint,
        38i32 as c_uint,
        57i32 as c_uint,
        86i32 as c_uint,
        12i32 as c_uint,
        18i32 as c_uint,
        21i32 as c_uint,
        28i32 as c_uint,
        35i32 as c_uint,
        41i32 as c_uint,
        54i32 as c_uint,
        76i32 as c_uint,
        14i32 as c_uint,
        21i32 as c_uint,
        25i32 as c_uint,
        32i32 as c_uint,
        44i32 as c_uint,
        63i32 as c_uint,
        92i32 as c_uint,
        136i32 as c_uint,
        19i32 as c_uint,
        28i32 as c_uint,
        32i32 as c_uint,
        41i32 as c_uint,
        54i32 as c_uint,
        75i32 as c_uint,
        107i32 as c_uint,
        157i32 as c_uint,
        26i32 as c_uint,
        35i32 as c_uint,
        44i32 as c_uint,
        54i32 as c_uint,
        70i32 as c_uint,
        95i32 as c_uint,
        132i32 as c_uint,
        190i32 as c_uint,
        38i32 as c_uint,
        41i32 as c_uint,
        63i32 as c_uint,
        75i32 as c_uint,
        95i32 as c_uint,
        125i32 as c_uint,
        170i32 as c_uint,
        239i32 as c_uint,
        57i32 as c_uint,
        54i32 as c_uint,
        92i32 as c_uint,
        107i32 as c_uint,
        132i32 as c_uint,
        170i32 as c_uint,
        227i32 as c_uint,
        312i32 as c_uint,
        86i32 as c_uint,
        76i32 as c_uint,
        136i32 as c_uint,
        157i32 as c_uint,
        190i32 as c_uint,
        239i32 as c_uint,
        312i32 as c_uint,
        419i32 as c_uint,
    ],
    [
        7i32 as c_uint,
        8i32 as c_uint,
        10i32 as c_uint,
        14i32 as c_uint,
        23i32 as c_uint,
        44i32 as c_uint,
        95i32 as c_uint,
        241i32 as c_uint,
        8i32 as c_uint,
        8i32 as c_uint,
        11i32 as c_uint,
        15i32 as c_uint,
        25i32 as c_uint,
        47i32 as c_uint,
        102i32 as c_uint,
        255i32 as c_uint,
        10i32 as c_uint,
        11i32 as c_uint,
        13i32 as c_uint,
        19i32 as c_uint,
        31i32 as c_uint,
        58i32 as c_uint,
        127i32 as c_uint,
        255i32 as c_uint,
        14i32 as c_uint,
        15i32 as c_uint,
        19i32 as c_uint,
        27i32 as c_uint,
        44i32 as c_uint,
        83i32 as c_uint,
        181i32 as c_uint,
        255i32 as c_uint,
        23i32 as c_uint,
        25i32 as c_uint,
        31i32 as c_uint,
        44i32 as c_uint,
        72i32 as c_uint,
        136i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        44i32 as c_uint,
        47i32 as c_uint,
        58i32 as c_uint,
        83i32 as c_uint,
        136i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        95i32 as c_uint,
        102i32 as c_uint,
        127i32 as c_uint,
        181i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        241i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
        255i32 as c_uint,
    ],
    [
        15i32 as c_uint,
        11i32 as c_uint,
        11i32 as c_uint,
        12i32 as c_uint,
        15i32 as c_uint,
        19i32 as c_uint,
        25i32 as c_uint,
        32i32 as c_uint,
        11i32 as c_uint,
        13i32 as c_uint,
        10i32 as c_uint,
        10i32 as c_uint,
        12i32 as c_uint,
        15i32 as c_uint,
        19i32 as c_uint,
        24i32 as c_uint,
        11i32 as c_uint,
        10i32 as c_uint,
        14i32 as c_uint,
        14i32 as c_uint,
        16i32 as c_uint,
        18i32 as c_uint,
        22i32 as c_uint,
        27i32 as c_uint,
        12i32 as c_uint,
        10i32 as c_uint,
        14i32 as c_uint,
        18i32 as c_uint,
        21i32 as c_uint,
        24i32 as c_uint,
        28i32 as c_uint,
        33i32 as c_uint,
        15i32 as c_uint,
        12i32 as c_uint,
        16i32 as c_uint,
        21i32 as c_uint,
        26i32 as c_uint,
        31i32 as c_uint,
        36i32 as c_uint,
        42i32 as c_uint,
        19i32 as c_uint,
        15i32 as c_uint,
        18i32 as c_uint,
        24i32 as c_uint,
        31i32 as c_uint,
        38i32 as c_uint,
        45i32 as c_uint,
        53i32 as c_uint,
        25i32 as c_uint,
        19i32 as c_uint,
        22i32 as c_uint,
        28i32 as c_uint,
        36i32 as c_uint,
        45i32 as c_uint,
        55i32 as c_uint,
        65i32 as c_uint,
        32i32 as c_uint,
        24i32 as c_uint,
        27i32 as c_uint,
        33i32 as c_uint,
        42i32 as c_uint,
        53i32 as c_uint,
        65i32 as c_uint,
        77i32 as c_uint,
    ],
    [
        14i32 as c_uint,
        10i32 as c_uint,
        11i32 as c_uint,
        14i32 as c_uint,
        19i32 as c_uint,
        25i32 as c_uint,
        34i32 as c_uint,
        45i32 as c_uint,
        10i32 as c_uint,
        11i32 as c_uint,
        11i32 as c_uint,
        12i32 as c_uint,
        15i32 as c_uint,
        20i32 as c_uint,
        26i32 as c_uint,
        33i32 as c_uint,
        11i32 as c_uint,
        11i32 as c_uint,
        15i32 as c_uint,
        18i32 as c_uint,
        21i32 as c_uint,
        25i32 as c_uint,
        31i32 as c_uint,
        38i32 as c_uint,
        14i32 as c_uint,
        12i32 as c_uint,
        18i32 as c_uint,
        24i32 as c_uint,
        28i32 as c_uint,
        33i32 as c_uint,
        39i32 as c_uint,
        47i32 as c_uint,
        19i32 as c_uint,
        15i32 as c_uint,
        21i32 as c_uint,
        28i32 as c_uint,
        36i32 as c_uint,
        43i32 as c_uint,
        51i32 as c_uint,
        59i32 as c_uint,
        25i32 as c_uint,
        20i32 as c_uint,
        25i32 as c_uint,
        33i32 as c_uint,
        43i32 as c_uint,
        54i32 as c_uint,
        64i32 as c_uint,
        74i32 as c_uint,
        34i32 as c_uint,
        26i32 as c_uint,
        31i32 as c_uint,
        39i32 as c_uint,
        51i32 as c_uint,
        64i32 as c_uint,
        77i32 as c_uint,
        91i32 as c_uint,
        45i32 as c_uint,
        33i32 as c_uint,
        38i32 as c_uint,
        47i32 as c_uint,
        59i32 as c_uint,
        74i32 as c_uint,
        91i32 as c_uint,
        108i32 as c_uint,
    ],
];

unsafe extern "C" fn jpeg_default_qtables(mut cinfo: j_compress_ptr, mut force_baseline: boolean) {
    let mut quant_tbl_master_idx: c_int = 0i32;
    if jpeg_c_int_param_supported(cinfo, JINT_BASE_QUANT_TBL_IDX) != 0 {
        quant_tbl_master_idx = jpeg_c_get_int_param(cinfo, JINT_BASE_QUANT_TBL_IDX)
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
    let mut val: c_float = 75.0f32; /* default value */
    let mut tblno: c_int = 0; /* if not set by sscanf, will be ',' */
    let mut ch: c_char = 0;
    tblno = 0i32;
    while tblno < NUM_QUANT_TBLS {
        if *arg != 0 {
            ch = ',' as i32 as c_char;
            if sscanf(
                arg,
                b"%f%c\x00" as *const u8 as *const c_char,
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
            q_scale_factor[tblno as usize] = jpeg_float_quality_scaling(val) as c_int;
            /* advance to next segment of arg string */
            while *arg as c_int != 0 && {
                let fresh0 = arg;
                arg = arg.offset(1);
                (*fresh0 as c_int) != ',' as i32
            } {}
        } else {
            /* reached end of parameter, set remaining factors to last value */
            q_scale_factor[tblno as usize] = jpeg_float_quality_scaling(val) as c_int
        }
        tblno += 1
    }
    jpeg_default_qtables(cinfo, force_baseline);
    /* For some images chroma subsampling significantly degrades color quality,
    making it impossible to achieve high visual quality regardless of quality setting.
    To make the quality setting more intuitive, disable subsampling when high-quality
    color is desired. */
    if val >= 90i32 as c_float {
        set_sample_factors(
            cinfo,
            b"1x1\x00" as *const u8 as *const c_char as *mut c_char,
        );
    } else if val >= 80i32 as c_float {
        set_sample_factors(
            cinfo,
            b"2x1\x00" as *const u8 as *const c_char as *mut c_char,
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
    let mut val: c_int = 0i32; /* default table # */
    let mut ci: c_int = 0; /* if not set by sscanf, will be ',' */
    let mut ch: c_char = 0;
    ci = 0i32;
    while ci < MAX_COMPONENTS {
        if *arg != 0 {
            ch = ',' as i32 as c_char;
            if sscanf(
                arg,
                b"%d%c\x00" as *const u8 as *const c_char,
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
                fprintf(
                    stderr,
                    b"JPEG quantization tables are numbered 0..%d\n\x00" as *const u8
                        as *const c_char,
                    NUM_QUANT_TBLS - 1i32,
                );
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
    let mut ci: c_int = 0; /* if not set by sscanf, will be ',' */
    let mut val1: c_int = 0;
    let mut val2: c_int = 0;
    let mut ch1: c_char = 0;
    let mut ch2: c_char = 0;
    ci = 0i32;
    while ci < MAX_COMPONENTS {
        if *arg != 0 {
            ch2 = ',' as i32 as c_char;
            if sscanf(
                arg,
                b"%d%c%d%c\x00" as *const u8 as *const c_char,
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
                fprintf(
                    stderr,
                    b"JPEG sampling factors must be 1..4\n\x00" as *const u8 as *const c_char,
                );
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
