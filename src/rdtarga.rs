pub use crate::cderror_h::{
    C2RustUnnamed_91, JERR_BAD_CMAP_FILE, JERR_BMP_BADCMAP, JERR_BMP_BADDEPTH, JERR_BMP_BADHEADER,
    JERR_BMP_BADPLANES, JERR_BMP_COLORSPACE, JERR_BMP_COMPRESSED, JERR_BMP_EMPTY, JERR_BMP_NOT,
    JERR_BMP_OUTOFRANGE, JERR_GIF_BUG, JERR_GIF_CODESIZE, JERR_GIF_COLORSPACE,
    JERR_GIF_IMAGENOTFOUND, JERR_GIF_NOT, JERR_PPM_COLORSPACE, JERR_PPM_NONNUMERIC, JERR_PPM_NOT,
    JERR_PPM_OUTOFRANGE, JERR_TGA_BADCMAP, JERR_TGA_BADPARMS, JERR_TGA_COLORSPACE,
    JERR_TOO_MANY_COLORS, JERR_UNGETC_FAILED, JERR_UNKNOWN_FORMAT, JERR_UNSUPPORTED_FORMAT,
    JMSG_FIRSTADDONCODE, JMSG_LASTADDONCODE, JTRC_BMP, JTRC_BMP_MAPPED, JTRC_BMP_OS2,
    JTRC_BMP_OS2_MAPPED, JTRC_GIF, JTRC_GIF_BADVERSION, JTRC_GIF_EXTENSION, JTRC_GIF_NONSQUARE,
    JTRC_PGM, JTRC_PGM_TEXT, JTRC_PPM, JTRC_PPM_TEXT, JTRC_TGA, JTRC_TGA_GRAY, JTRC_TGA_MAPPED,
    JWRN_GIF_BADDATA, JWRN_GIF_CHAR, JWRN_GIF_ENDCODE, JWRN_GIF_NOMOREDATA,
};
pub use crate::cdjpeg::{
    cd_progress_ptr, cdjpeg_progress_mgr, cjpeg_source_ptr, cjpeg_source_struct,
};
pub use crate::jerror::{
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
pub use crate::jpeglib_h::{
    j_common_ptr, j_compress_ptr, jpeg_c_coef_controller, jpeg_c_main_controller,
    jpeg_c_prep_controller, jpeg_color_converter, jpeg_common_struct, jpeg_comp_master,
    jpeg_component_info, jpeg_compress_struct, jpeg_destination_mgr, jpeg_downsampler,
    jpeg_entropy_encoder, jpeg_error_mgr, jpeg_forward_dct, jpeg_marker_struct, jpeg_marker_writer,
    jpeg_memory_mgr, jpeg_progress_mgr, jpeg_saved_marker_ptr, jpeg_scan_info,
    jvirt_barray_control, jvirt_barray_ptr, jvirt_sarray_control, jvirt_sarray_ptr,
    C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY, JBLOCKROW, JCS_CMYK, JCS_EXT_ABGR,
    JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX,
    JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK,
    JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JHUFF_TBL, JPOOL_IMAGE, JQUANT_TBL, JSAMPARRAY, JSAMPROW,
    J_COLOR_SPACE, J_DCT_METHOD,
};
pub use crate::stddef_h::{size_t, NULL};
pub use crate::stdlib::{
    _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, __off64_t, __off_t, fread, getc, EOF, FILE,
    _IO_FILE,
};
use libc::{self, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};
/* !HAVE_UNSIGNED_CHAR */
/* HAVE_UNSIGNED_CHAR */
/* Private version of data source object */
pub type tga_source_ptr = *mut _tga_source_struct;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _tga_source_struct {
    pub pub_0: cjpeg_source_struct,
    pub cinfo: j_compress_ptr,
    pub colormap: JSAMPARRAY,
    pub whole_image: jvirt_sarray_ptr,
    pub current_row: JDIMENSION,
    pub read_pixel: Option<unsafe extern "C" fn(_: tga_source_ptr) -> ()>,
    pub tga_pixel: [U_CHAR; 4],
    pub pixel_size: c_int,
    pub block_count: c_int,
    pub dup_pixel_count: c_int,
    pub get_pixel_rows:
        Option<unsafe extern "C" fn(_: j_compress_ptr, _: cjpeg_source_ptr) -> JDIMENSION>,
}
/*
 * rdtarga.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * It was modified by The libjpeg-turbo Project to include only code relevant
 * to libjpeg-turbo.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains routines to read input images in Targa format.
 *
 * These routines may need modification for non-Unix environments or
 * specialized applications.  As they stand, they assume input from
 * an ordinary stdio stream.  They further assume that reading begins
 * at the start of the file; start_input may need work if the
 * user interface has already read some data (e.g., to determine that
 * the file is indeed Targa format).
 *
 * Based on code contributed by Lee Daniel Crocker.
 */
/* Macros to deal with unsigned chars as efficiently as compiler allows */
pub type U_CHAR = c_uchar;
pub type tga_source_struct = _tga_source_struct;
/* For expanding 5-bit pixel values to 8-bit with best rounding */
static mut c5to8bits: [UINT8; 32] = [
    0i32 as UINT8,
    8i32 as UINT8,
    16i32 as UINT8,
    25i32 as UINT8,
    33i32 as UINT8,
    41i32 as UINT8,
    49i32 as UINT8,
    58i32 as UINT8,
    66i32 as UINT8,
    74i32 as UINT8,
    82i32 as UINT8,
    90i32 as UINT8,
    99i32 as UINT8,
    107i32 as UINT8,
    115i32 as UINT8,
    123i32 as UINT8,
    132i32 as UINT8,
    140i32 as UINT8,
    148i32 as UINT8,
    156i32 as UINT8,
    165i32 as UINT8,
    173i32 as UINT8,
    181i32 as UINT8,
    189i32 as UINT8,
    197i32 as UINT8,
    206i32 as UINT8,
    214i32 as UINT8,
    222i32 as UINT8,
    230i32 as UINT8,
    239i32 as UINT8,
    247i32 as UINT8,
    255i32 as UINT8,
];
unsafe extern "C" fn read_byte(mut sinfo: tga_source_ptr) -> c_int {
    let mut infile: *mut FILE = (*sinfo).pub_0.input_file;
    let mut c: c_int = 0;
    c = getc(infile);
    if c == EOF {
        (*(*(*sinfo).cinfo).err).msg_code = JERR_INPUT_EOF as c_int;
        (*(*(*sinfo).cinfo).err)
            .error_exit
            .expect("non-null function pointer")((*sinfo).cinfo as j_common_ptr);
    }
    return c;
}
unsafe extern "C" fn read_colormap(
    mut sinfo: tga_source_ptr,
    mut cmaplen: c_int,
    mut mapentrysize: c_int,
) {
    let mut i: c_int = 0;
    if mapentrysize != 24i32 {
        (*(*(*sinfo).cinfo).err).msg_code = JERR_TGA_BADCMAP as c_int;
        (*(*(*sinfo).cinfo).err)
            .error_exit
            .expect("non-null function pointer")((*sinfo).cinfo as j_common_ptr);
    }
    i = 0i32;
    while i < cmaplen {
        *(*(*sinfo).colormap.offset(2isize)).offset(i as isize) = read_byte(sinfo) as JSAMPLE;
        *(*(*sinfo).colormap.offset(1isize)).offset(i as isize) = read_byte(sinfo) as JSAMPLE;
        *(*(*sinfo).colormap.offset(0isize)).offset(i as isize) = read_byte(sinfo) as JSAMPLE;
        i += 1
    }
}
/*
 * read_pixel methods: get a single pixel from Targa file into tga_pixel[]
 */
unsafe extern "C" fn read_non_rle_pixel(mut sinfo: tga_source_ptr) {
    let mut i: c_int = 0;
    i = 0i32;
    while i < (*sinfo).pixel_size {
        (*sinfo).tga_pixel[i as usize] = read_byte(sinfo) as U_CHAR;
        i += 1
    }
}
unsafe extern "C" fn read_rle_pixel(mut sinfo: tga_source_ptr) {
    let mut i: c_int = 0;
    if (*sinfo).dup_pixel_count > 0i32 {
        (*sinfo).dup_pixel_count -= 1;
        return;
    }
    (*sinfo).block_count -= 1;
    if (*sinfo).block_count < 0i32 {
        i = read_byte(sinfo);
        if 0 != i & 0x80i32 {
            (*sinfo).dup_pixel_count = i & 0x7fi32;
            (*sinfo).block_count = 0i32
        } else {
            (*sinfo).block_count = i & 0x7fi32
        }
    }
    i = 0i32;
    while i < (*sinfo).pixel_size {
        (*sinfo).tga_pixel[i as usize] = read_byte(sinfo) as U_CHAR;
        i += 1
    }
}
/*
 * Read one row of pixels.
 *
 * We provide several different versions depending on input file format.
 */
unsafe extern "C" fn get_8bit_gray_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: tga_source_ptr = sinfo as tga_source_ptr;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    ptr = *(*source).pub_0.buffer.offset(0isize);
    col = (*cinfo).image_width;
    while col > 0i32 as c_uint {
        (*source).read_pixel.expect("non-null function pointer")(source);
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        *fresh0 = (*source).tga_pixel[0usize] as c_int as JSAMPLE;
        col = col.wrapping_sub(1)
    }
    return 1i32 as JDIMENSION;
}
unsafe extern "C" fn get_8bit_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: tga_source_ptr = sinfo as tga_source_ptr;
    let mut t: c_int = 0;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut colormap: JSAMPARRAY = (*source).colormap;
    ptr = *(*source).pub_0.buffer.offset(0isize);
    col = (*cinfo).image_width;
    while col > 0i32 as c_uint {
        (*source).read_pixel.expect("non-null function pointer")(source);
        t = (*source).tga_pixel[0usize] as c_int;
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        *fresh1 = *(*colormap.offset(0isize)).offset(t as isize);
        let fresh2 = ptr;
        ptr = ptr.offset(1);
        *fresh2 = *(*colormap.offset(1isize)).offset(t as isize);
        let fresh3 = ptr;
        ptr = ptr.offset(1);
        *fresh3 = *(*colormap.offset(2isize)).offset(t as isize);
        col = col.wrapping_sub(1)
    }
    return 1i32 as JDIMENSION;
}
unsafe extern "C" fn get_16bit_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: tga_source_ptr = sinfo as tga_source_ptr;
    let mut t: c_int = 0;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    ptr = *(*source).pub_0.buffer.offset(0isize);
    col = (*cinfo).image_width;
    while col > 0i32 as c_uint {
        (*source).read_pixel.expect("non-null function pointer")(source);
        t = (*source).tga_pixel[0usize] as c_int;
        t += ((*source).tga_pixel[1usize] as c_int) << 8i32;
        *ptr.offset(2isize) = c5to8bits[(t & 0x1fi32) as usize];
        t >>= 5i32;
        *ptr.offset(1isize) = c5to8bits[(t & 0x1fi32) as usize];
        t >>= 5i32;
        *ptr.offset(0isize) = c5to8bits[(t & 0x1fi32) as usize];
        ptr = ptr.offset(3isize);
        col = col.wrapping_sub(1)
    }
    return 1i32 as JDIMENSION;
}
unsafe extern "C" fn get_24bit_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: tga_source_ptr = sinfo as tga_source_ptr;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    ptr = *(*source).pub_0.buffer.offset(0isize);
    col = (*cinfo).image_width;
    while col > 0i32 as c_uint {
        (*source).read_pixel.expect("non-null function pointer")(source);
        let fresh4 = ptr;
        ptr = ptr.offset(1);
        *fresh4 = (*source).tga_pixel[2usize] as c_int as JSAMPLE;
        let fresh5 = ptr;
        ptr = ptr.offset(1);
        *fresh5 = (*source).tga_pixel[1usize] as c_int as JSAMPLE;
        let fresh6 = ptr;
        ptr = ptr.offset(1);
        *fresh6 = (*source).tga_pixel[0usize] as c_int as JSAMPLE;
        col = col.wrapping_sub(1)
    }
    return 1i32 as JDIMENSION;
}
/*
 * Targa also defines a 32-bit pixel format with order B,G,R,A.
 * We presently ignore the attribute byte, so the code for reading
 * these pixels is identical to the 24-bit routine above.
 * This works because the actual pixel length is only known to read_pixel.
 */
pub const get_32bit_row: unsafe extern "C" fn(
    _: j_compress_ptr,
    _: cjpeg_source_ptr,
) -> JDIMENSION = get_24bit_row;
/*
 * This method is for re-reading the input data in standard top-down
 * row order.  The entire image has already been read into whole_image
 * with proper conversion of pixel format, but it's in a funny row order.
 */
unsafe extern "C" fn get_memory_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: tga_source_ptr = sinfo as tga_source_ptr;
    let mut source_row: JDIMENSION = 0;
    source_row = (*cinfo)
        .image_height
        .wrapping_sub((*source).current_row)
        .wrapping_sub(1i32 as c_uint);
    (*source).pub_0.buffer = (*(*cinfo).mem)
        .access_virt_sarray
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        (*source).whole_image,
        source_row,
        1i32 as JDIMENSION,
        FALSE,
    );
    (*source).current_row = (*source).current_row.wrapping_add(1);
    return 1i32 as JDIMENSION;
}
/*
 * This method loads the image into whole_image during the first call on
 * get_pixel_rows.  The get_pixel_rows pointer is then adjusted to call
 * get_memory_row on subsequent calls.
 */
unsafe extern "C" fn preload_image(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: tga_source_ptr = sinfo as tga_source_ptr;
    let mut row: JDIMENSION = 0;
    let mut progress: cd_progress_ptr = (*cinfo).progress as cd_progress_ptr;
    row = 0i32 as JDIMENSION;
    while row < (*cinfo).image_height {
        if !progress.is_null() {
            (*progress).pub_0.pass_counter = row as c_long;
            (*progress).pub_0.pass_limit = (*cinfo).image_height as c_long;
            (*progress)
                .pub_0
                .progress_monitor
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        (*source).pub_0.buffer = (*(*cinfo).mem)
            .access_virt_sarray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*source).whole_image,
            row,
            1i32 as JDIMENSION,
            TRUE,
        );
        (*source).get_pixel_rows.expect("non-null function pointer")(cinfo, sinfo);
        row = row.wrapping_add(1)
    }
    if !progress.is_null() {
        (*progress).completed_extra_passes += 1
    }
    (*source).pub_0.get_pixel_rows = Some(
        get_memory_row
            as unsafe extern "C" fn(_: j_compress_ptr, _: cjpeg_source_ptr) -> JDIMENSION,
    );
    (*source).current_row = 0i32 as JDIMENSION;
    return get_memory_row(cinfo, sinfo);
}
/*
 * Read the file header; return image size and component count.
 */
unsafe extern "C" fn start_input_tga(mut cinfo: j_compress_ptr, mut sinfo: cjpeg_source_ptr) {
    let mut source: tga_source_ptr = sinfo as tga_source_ptr;
    let mut targaheader: [U_CHAR; 18] = [0; 18];
    let mut idlen: c_int = 0;
    let mut cmaptype: c_int = 0;
    let mut subtype: c_int = 0;
    let mut flags: c_int = 0;
    let mut interlace_type: c_int = 0;
    let mut components: c_int = 0;
    let mut width: c_uint = 0;
    let mut height: c_uint = 0;
    let mut maplen: c_uint = 0;
    let mut is_bottom_up: boolean = 0;
    if !(fread(
        targaheader.as_mut_ptr() as *mut c_void,
        1i32 as size_t,
        18i32 as size_t,
        (*source).pub_0.input_file,
    ) == 18i32 as size_t)
    {
        (*(*cinfo).err).msg_code = JERR_INPUT_EOF as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if targaheader[16usize] as c_int == 15i32 {
        targaheader[16usize] = 16i32 as U_CHAR
    }
    idlen = targaheader[0usize] as c_int;
    cmaptype = targaheader[1usize] as c_int;
    subtype = targaheader[2usize] as c_int;
    maplen = (targaheader[5usize] as c_int as c_uint)
        .wrapping_add((targaheader[(5i32 + 1i32) as usize] as c_int as c_uint) << 8i32);
    width = (targaheader[12usize] as c_int as c_uint)
        .wrapping_add((targaheader[(12i32 + 1i32) as usize] as c_int as c_uint) << 8i32);
    height = (targaheader[14usize] as c_int as c_uint)
        .wrapping_add((targaheader[(14i32 + 1i32) as usize] as c_int as c_uint) << 8i32);
    (*source).pixel_size = targaheader[16usize] as c_int >> 3i32;
    flags = targaheader[17usize] as c_int;
    is_bottom_up = (flags & 0x20i32 == 0i32) as c_int;
    interlace_type = flags >> 6i32;
    if cmaptype > 1i32
        || (*source).pixel_size < 1i32
        || (*source).pixel_size > 4i32
        || targaheader[16usize] as c_int & 7i32 != 0i32
        || interlace_type != 0i32
        || width == 0i32 as c_uint
        || height == 0i32 as c_uint
    {
        (*(*cinfo).err).msg_code = JERR_TGA_BADPARMS as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if subtype > 8i32 {
        (*source).read_pixel =
            Some(read_rle_pixel as unsafe extern "C" fn(_: tga_source_ptr) -> ());
        (*source).dup_pixel_count = 0i32;
        (*source).block_count = (*source).dup_pixel_count;
        subtype -= 8i32
    } else {
        (*source).read_pixel =
            Some(read_non_rle_pixel as unsafe extern "C" fn(_: tga_source_ptr) -> ())
    }
    components = 3i32;
    (*cinfo).in_color_space = JCS_RGB;
    match subtype {
        1 => {
            if (*source).pixel_size == 1i32 && cmaptype == 1i32 {
                (*source).get_pixel_rows = Some(
                    get_8bit_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else {
                (*(*cinfo).err).msg_code = JERR_TGA_BADPARMS as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            (*(*cinfo).err).msg_code = JTRC_TGA_MAPPED as c_int;
            (*(*cinfo).err).msg_parm.i[0usize] = width as c_int;
            (*(*cinfo).err).msg_parm.i[1usize] = height as c_int;
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer")(cinfo as j_common_ptr, 1i32);
        }
        2 => {
            match (*source).pixel_size {
                2 => {
                    (*source).get_pixel_rows = Some(
                        get_16bit_row
                            as unsafe extern "C" fn(
                                _: j_compress_ptr,
                                _: cjpeg_source_ptr,
                            ) -> JDIMENSION,
                    )
                }
                3 => {
                    (*source).get_pixel_rows = Some(
                        get_24bit_row
                            as unsafe extern "C" fn(
                                _: j_compress_ptr,
                                _: cjpeg_source_ptr,
                            ) -> JDIMENSION,
                    )
                }
                4 => (*source).get_pixel_rows = Some(get_32bit_row),
                _ => {
                    (*(*cinfo).err).msg_code = JERR_TGA_BADPARMS as c_int;
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer")(
                        cinfo as j_common_ptr
                    );
                }
            }
            (*(*cinfo).err).msg_code = JTRC_TGA as c_int;
            (*(*cinfo).err).msg_parm.i[0usize] = width as c_int;
            (*(*cinfo).err).msg_parm.i[1usize] = height as c_int;
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer")(cinfo as j_common_ptr, 1i32);
        }
        3 => {
            components = 1i32;
            (*cinfo).in_color_space = JCS_GRAYSCALE;
            if (*source).pixel_size == 1i32 {
                (*source).get_pixel_rows = Some(
                    get_8bit_gray_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            } else {
                (*(*cinfo).err).msg_code = JERR_TGA_BADPARMS as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            (*(*cinfo).err).msg_code = JTRC_TGA_GRAY as c_int;
            (*(*cinfo).err).msg_parm.i[0usize] = width as c_int;
            (*(*cinfo).err).msg_parm.i[1usize] = height as c_int;
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer")(cinfo as j_common_ptr, 1i32);
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_TGA_BADPARMS as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    }
    if 0 != is_bottom_up {
        (*source).whole_image = (*(*cinfo).mem)
            .request_virt_sarray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            FALSE,
            width.wrapping_mul(components as c_uint),
            height,
            1i32 as JDIMENSION,
        );
        if !(*cinfo).progress.is_null() {
            let mut progress: cd_progress_ptr = (*cinfo).progress as cd_progress_ptr;
            (*progress).total_extra_passes += 1
        }
        (*source).pub_0.buffer_height = 1i32 as JDIMENSION;
        (*source).pub_0.get_pixel_rows = Some(
            preload_image
                as unsafe extern "C" fn(_: j_compress_ptr, _: cjpeg_source_ptr) -> JDIMENSION,
        )
    } else {
        (*source).whole_image = NULL as jvirt_sarray_ptr;
        (*source).pub_0.buffer = (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            width.wrapping_mul(components as c_uint),
            1i32 as JDIMENSION,
        );
        (*source).pub_0.buffer_height = 1i32 as JDIMENSION;
        (*source).pub_0.get_pixel_rows = (*source).get_pixel_rows
    }
    loop {
        let fresh7 = idlen;
        idlen = idlen - 1;
        if !(0 != fresh7) {
            break;
        }
        read_byte(source);
    }
    if maplen > 0i32 as c_uint {
        if maplen > 256i32 as c_uint
            || (targaheader[3usize] as c_int as c_uint)
                .wrapping_add((targaheader[(3i32 + 1i32) as usize] as c_int as c_uint) << 8i32)
                != 0i32 as c_uint
        {
            (*(*cinfo).err).msg_code = JERR_TGA_BADCMAP as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        (*source).colormap = (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            maplen,
            3i32 as JDIMENSION,
        );
        read_colormap(source, maplen as c_int, targaheader[7usize] as c_int);
    } else {
        if 0 != cmaptype {
            (*(*cinfo).err).msg_code = JERR_TGA_BADPARMS as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        (*source).colormap = NULL as JSAMPARRAY
    }
    (*cinfo).input_components = components;
    (*cinfo).data_precision = 8i32;
    (*cinfo).image_width = width;
    (*cinfo).image_height = height;
}
/*
 * Finish up at the end of the file.
 */
unsafe extern "C" fn finish_input_tga(mut _cinfo: j_compress_ptr, mut _sinfo: cjpeg_source_ptr) {}
/* no work */
/*
 * The module selection routine for Targa format input.
 */
#[no_mangle]
pub unsafe extern "C" fn jinit_read_targa(mut cinfo: j_compress_ptr) -> cjpeg_source_ptr {
    let mut source: tga_source_ptr = 0 as *mut _tga_source_struct;
    source = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<tga_source_struct>() as c_ulong,
    ) as tga_source_ptr;
    (*source).cinfo = cinfo;
    (*source).pub_0.start_input =
        Some(start_input_tga as unsafe extern "C" fn(_: j_compress_ptr, _: cjpeg_source_ptr) -> ());
    (*source).pub_0.finish_input = Some(
        finish_input_tga as unsafe extern "C" fn(_: j_compress_ptr, _: cjpeg_source_ptr) -> (),
    );
    return source as cjpeg_source_ptr;
}
