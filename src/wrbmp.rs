use libc::{c_ushort, c_ulong, c_char, c_long, c_int, intptr_t, c_void, c_uint,
           self};

#[header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/jmorecfg.h:24"]
pub mod jmorecfg_h {

    use crate::jmorecfg_h::{EXT_RGB_PIXELSIZE, EXT_RGBX_PIXELSIZE,
                        EXT_XBGR_PIXELSIZE, EXT_BGR_PIXELSIZE,
                        EXT_BGRX_PIXELSIZE, EXT_XRGB_PIXELSIZE,
                        RGB_PIXELSIZE};use libc::c_int;pub static mut rgb_pixelsize: [c_int; 17] = [
        -1i32,
        -1i32,
        RGB_PIXELSIZE,
        -1i32,
        -1i32,
        -1i32,
        EXT_RGB_PIXELSIZE,
        EXT_RGBX_PIXELSIZE,
        EXT_BGR_PIXELSIZE,
        EXT_BGRX_PIXELSIZE,
        EXT_XBGR_PIXELSIZE,
        EXT_XRGB_PIXELSIZE,
        EXT_RGBX_PIXELSIZE,
        EXT_BGRX_PIXELSIZE,
        EXT_XBGR_PIXELSIZE,
        EXT_XRGB_PIXELSIZE,
        -1i32,
    ];

    /* JPEG_INTERNAL_OPTIONS */
    /* FAST_FLOAT should be either float or double, whichever is done faster
     * by your compiler.  (Note that this type is only used in the floating point
     * DCT routines, so it only matters if you've defined DCT_FLOAT_SUPPORTED.)
     */
    /* prefer 16-bit with SIMD for parellelism */
    /* On some machines (notably 68000 series) "int" is 32 bits, but multiplying
     * two 16-bit shorts is faster than multiplying two ints.  Define MULTIPLIER
     * as short on such a machine.  MULTIPLIER must be at least 16 bits wide.
     */
    /* Definitions for speed-related optimizations. */
}


























































































































































































































































































































use crate::stdlib::{ferror, fflush, fwrite, memcpy, memset, putc};pub use crate::cderror_h::{C2RustUnnamed_4, JERR_BAD_CMAP_FILE,
                           JERR_BMP_BADCMAP, JERR_BMP_BADDEPTH,
                           JERR_BMP_BADHEADER, JERR_BMP_BADPLANES,
                           JERR_BMP_COLORSPACE, JERR_BMP_COMPRESSED,
                           JERR_BMP_EMPTY, JERR_BMP_NOT, JERR_BMP_OUTOFRANGE,
                           JERR_GIF_BUG, JERR_GIF_CODESIZE,
                           JERR_GIF_COLORSPACE, JERR_GIF_IMAGENOTFOUND,
                           JERR_GIF_NOT, JERR_PPM_COLORSPACE,
                           JERR_PPM_NONNUMERIC, JERR_PPM_NOT,
                           JERR_PPM_OUTOFRANGE, JERR_TGA_BADCMAP,
                           JERR_TGA_BADPARMS, JERR_TGA_COLORSPACE,
                           JERR_TOO_MANY_COLORS, JERR_UNGETC_FAILED,
                           JERR_UNKNOWN_FORMAT, JERR_UNSUPPORTED_FORMAT,
                           JMSG_FIRSTADDONCODE, JMSG_LASTADDONCODE, JTRC_BMP,
                           JTRC_BMP_MAPPED, JTRC_BMP_OS2, JTRC_BMP_OS2_MAPPED,
                           JTRC_GIF, JTRC_GIF_BADVERSION, JTRC_GIF_EXTENSION,
                           JTRC_GIF_NONSQUARE, JTRC_PGM, JTRC_PGM_TEXT,
                           JTRC_PPM, JTRC_PPM_TEXT, JTRC_TGA, JTRC_TGA_GRAY,
                           JTRC_TGA_MAPPED, JWRN_GIF_BADDATA, JWRN_GIF_CHAR,
                           JWRN_GIF_ENDCODE, JWRN_GIF_NOMOREDATA};pub use crate::jpegint_h::{inverse_DCT_method_ptr, JBUF_CRANK_DEST,
                           JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS,
                           JBUF_SAVE_SOURCE, J_BUF_MODE};pub use crate::stdlib::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data,
                        __off64_t, __off_t, FILE, _IO_FILE};pub use crate::jmorecfg_h::{boolean, rgb_blue, rgb_green, rgb_red,
                            EXT_BGRX_BLUE, EXT_BGRX_GREEN, EXT_BGRX_PIXELSIZE,
                            EXT_BGRX_RED, EXT_BGR_BLUE, EXT_BGR_GREEN,
                            EXT_BGR_PIXELSIZE, EXT_BGR_RED, EXT_RGBX_BLUE,
                            EXT_RGBX_GREEN, EXT_RGBX_PIXELSIZE, EXT_RGBX_RED,
                            EXT_RGB_BLUE, EXT_RGB_GREEN, EXT_RGB_PIXELSIZE,
                            EXT_RGB_RED, EXT_XBGR_BLUE, EXT_XBGR_GREEN,
                            EXT_XBGR_PIXELSIZE, EXT_XBGR_RED, EXT_XRGB_BLUE,
                            EXT_XRGB_GREEN, EXT_XRGB_PIXELSIZE, EXT_XRGB_RED,
                            FALSE, JCOEF, JDIMENSION, JOCTET, JSAMPLE,
                            RGB_BLUE, RGB_GREEN, RGB_PIXELSIZE, RGB_RED, TRUE,
                            UINT16, UINT8};pub use crate::cmyk_h::cmyk_to_rgb;pub use crate::jpeglib_h::{j_common_ptr, j_decompress_ptr,
                           jpeg_calc_output_dimensions,
                           jpeg_color_deconverter, jpeg_color_quantizer,
                           jpeg_common_struct, jpeg_component_info,
                           jpeg_d_coef_controller, jpeg_d_main_controller,
                           jpeg_d_post_controller, jpeg_decomp_master,
                           jpeg_decompress_struct, jpeg_entropy_decoder,
                           jpeg_error_mgr, jpeg_input_controller,
                           jpeg_inverse_dct, jpeg_marker_parser_method,
                           jpeg_marker_reader, jpeg_marker_struct,
                           jpeg_memory_mgr, jpeg_progress_mgr,
                           jpeg_saved_marker_ptr, jpeg_source_mgr,
                           jpeg_upsampler, jvirt_barray_control,
                           jvirt_barray_ptr, jvirt_sarray_control,
                           jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr,
                           JBLOCK, JBLOCKARRAY, JBLOCKROW, JCOEFPTR, JCS_CMYK,
                           JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR,
                           JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB,
                           JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR,
                           JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565,
                           JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT, JDCT_IFAST,
                           JDCT_ISLOW, JDITHER_FS, JDITHER_NONE,
                           JDITHER_ORDERED, JHUFF_TBL, JPOOL_IMAGE,
                           JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW,
                           J_COLOR_SPACE, J_DCT_METHOD, J_DITHER_MODE};pub use crate::stddef_h::{size_t, NULL};pub use super::jerror::{C2RustUnnamed_3, JERR_ARITH_NOTIMPL,
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
                        JWRN_TOO_MUCH_DATA};pub use super::cdjpeg::{cd_progress_ptr, cdjpeg_progress_mgr, djpeg_dest_ptr,
                        djpeg_dest_struct};pub use jmorecfg_h::rgb_pixelsize;

pub type bmp_dest_ptr = *mut bmp_dest_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmp_dest_struct {
    pub pub_0: super::cdjpeg::djpeg_dest_struct,
    pub is_os2: boolean,
    pub whole_image: jvirt_sarray_ptr,
    pub data_width: JDIMENSION,
    pub row_width: JDIMENSION,
    pub pad_bytes: c_int,
    pub cur_output_row: JDIMENSION,
    pub use_inversion_array: boolean,
    pub iobuffer: *mut JSAMPLE,
}
#[inline(always)]

unsafe extern "C" fn is_big_endian() -> boolean {
     let mut test_value:  c_int =  1i32;
    if *(&mut test_value as *mut c_int as *mut c_char) as c_int != 1i32 {
        return TRUE;
    }
    return FALSE;
}
/*
 * Write some pixel data.
 * In this module rows_supplied will always be 1.
 */

unsafe extern "C" fn put_pixel_rows(
    mut cinfo: j_decompress_ptr,
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
    mut rows_supplied: JDIMENSION,
)
/* This version is for writing 24-bit pixels */
{
      let mut outptr:  JSAMPROW =
     ::std::ptr::null_mut::< JSAMPLE>(); let mut col:  JDIMENSION =  0; let mut dest: bmp_dest_ptr = dinfo as bmp_dest_ptr;
    
    
    
    
    
    if (*dest).use_inversion_array != 0 {
         let mut image_ptr:   JSAMPARRAY =
     Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*dest).whole_image,
            (*dest).cur_output_row,
            1u32,
            TRUE,
        );
        (*dest).cur_output_row =  (*dest).cur_output_row + 1;
        outptr = *image_ptr.offset(0)
    } else {
        outptr = (*dest).iobuffer
    }
    /* Transfer data.  Note destination values must be in BGR order
     * (even though Microsoft's own documents say the opposite).
     */
     let mut inptr:   JSAMPROW =  *(*dest).pub_0.buffer.offset(0);
    if  (*cinfo).out_color_space
        ==  JCS_EXT_BGR
    {
        memcpy(
            outptr as *mut c_void,
            inptr as *const c_void,
            (*dest).row_width as size_t,
        );
        outptr = outptr.offset(((*cinfo).output_width * 3u32) as isize)
    } else if  (*cinfo).out_color_space
        ==  JCS_RGB565
    {
        let mut big_endian: boolean = is_big_endian();
        let mut inptr2: *mut c_ushort = inptr as *mut c_ushort;
        col = (*cinfo).output_width;
        while col > 0u32 {
            if big_endian != 0 {
                *outptr.offset(0) =
                    (*inptr2 as c_int >> 5i32 & 0xf8i32) as JSAMPLE;
                *outptr.offset(1) = ((*inptr2 as c_int) << 5i32 & 0xe0i32
                    | *inptr2 as c_int >> 11i32 & 0x1ci32)
                    as JSAMPLE;
                *outptr.offset(2) = (*inptr2 as c_int & 0xf8i32) as JSAMPLE
            } else {
                *outptr.offset(0) =
                    ((*inptr2 as c_int) << 3i32 & 0xf8i32) as JSAMPLE;
                *outptr.offset(1) =
                    (*inptr2 as c_int >> 3i32 & 0xfci32) as JSAMPLE;
                *outptr.offset(2) =
                    (*inptr2 as c_int >> 8i32 & 0xf8i32) as JSAMPLE
            }
            outptr = outptr.offset(3);
            inptr2 = inptr2.offset(1);
            col -=  1
        }
    } else if  (*cinfo).out_color_space
        ==  JCS_CMYK
    {
        col = (*cinfo).output_width;
        while col > 0u32 {
            /* can omit GETJSAMPLE() safely */
            let fresh0 = inptr;
            inptr = inptr.offset(1);
            let mut c: JSAMPLE = *fresh0;
            let fresh1 = inptr;
            inptr = inptr.offset(1);
            let mut m: JSAMPLE = *fresh1;
            let fresh2 = inptr;
            inptr = inptr.offset(1);
            let mut y: JSAMPLE = *fresh2;
            let fresh3 = inptr;
            inptr = inptr.offset(1);
            let mut k: JSAMPLE = *fresh3;
            cmyk_to_rgb(c, m, y, k, outptr.offset(2), outptr.offset(1), outptr);
            outptr = outptr.offset(3);
            col -=  1
        }
    } else {
        let mut rindex: c_int = rgb_red[(*cinfo).out_color_space as usize];
        let mut gindex: c_int =
            rgb_green[(*cinfo).out_color_space as usize];
        let mut bindex: c_int =
            rgb_blue[(*cinfo).out_color_space as usize];
        let mut ps: c_int = rgb_pixelsize[(*cinfo).out_color_space as usize];
        col = (*cinfo).output_width;
        while col > 0u32 {
            /* can omit GETJSAMPLE() safely */
            *outptr.offset(0) = *inptr.offset(bindex as isize);
            *outptr.offset(1) = *inptr.offset(gindex as isize);
            *outptr.offset(2) = *inptr.offset(rindex as isize);
            outptr = outptr.offset(3);
            inptr = inptr.offset(ps as isize);
            col -=  1
        }
    }
     let mut pad:   c_int =  (*dest).pad_bytes;
    loop {
        pad -= 1;
        if !(pad >= 0i32) {
            break;
        }
        let fresh4 = outptr;
        outptr = outptr.offset(1);
        *fresh4 = 0u8
    }
    if (*dest).use_inversion_array == 0 {
        fwrite(
            (*dest).iobuffer as *const c_void,
            1u64,
            (*dest).row_width as size_t,
            (*dest).pub_0.output_file,
        );
    };
}

unsafe extern "C" fn put_gray_rows(
    mut cinfo: j_decompress_ptr,
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
    mut rows_supplied: JDIMENSION,
)
/* This version is for grayscale OR quantized color output */
{
      let mut outptr:  JSAMPROW =
     ::std::ptr::null_mut::< JSAMPLE>(); let mut dest: bmp_dest_ptr = dinfo as bmp_dest_ptr;
    
    
    
    
    if (*dest).use_inversion_array != 0 {
         let mut image_ptr:   JSAMPARRAY =
     Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*dest).whole_image,
            (*dest).cur_output_row,
            1u32,
            TRUE,
        );
        (*dest).cur_output_row =  (*dest).cur_output_row + 1;
        outptr = *image_ptr.offset(0)
    } else {
        outptr = (*dest).iobuffer
    }
     let mut inptr:   JSAMPROW =  *(*dest).pub_0.buffer.offset(0);
    memcpy(
        outptr as *mut c_void,
        inptr as *const c_void,
        (*cinfo).output_width as size_t,
    );
    outptr = outptr.offset((*cinfo).output_width as isize);
     let mut pad:   c_int =  (*dest).pad_bytes;
    loop {
        pad -= 1;
        if !(pad >= 0i32) {
            break;
        }
        let fresh5 = outptr;
        outptr = outptr.offset(1);
        *fresh5 = 0u8
    }
    if (*dest).use_inversion_array == 0 {
        fwrite(
            (*dest).iobuffer as *const c_void,
            1u64,
            (*dest).row_width as size_t,
            (*dest).pub_0.output_file,
        );
    };
}
/*
 * Finish up at the end of the file.
 *
 * Here is where we really output the BMP file.
 *
 * First, routines to write the Windows and OS/2 variants of the file header.
 */

unsafe extern "C" fn write_bmp_header(
    mut cinfo: j_decompress_ptr,
    mut dest: bmp_dest_ptr,
)
/* Write a Windows-style BMP file header, including colormap if needed */
{
    
    
    
    
    
     let mut bmpfileheader:  [c_char; 14] =  [0; 14]; let mut bmpinfoheader:  [c_char; 40] =  [0; 40];   let mut bits_per_pixel:  c_int =  0; let mut cmap_entries:  c_int =  0;
    /* Compute colormap size and total file size */
    if  (*cinfo).out_color_space
        ==  JCS_RGB
        ||  (*cinfo).out_color_space
            >=  JCS_EXT_RGB
            &&  (*cinfo).out_color_space
                <=  JCS_EXT_ARGB
    {
        if (*cinfo).quantize_colors != 0 {
            /* Colormapped RGB */
            bits_per_pixel = 8i32;
            cmap_entries = 256i32
        } else {
            /* Unquantized, full color RGB */
            bits_per_pixel = 24i32;
            cmap_entries = 0i32
        }
    } else if  (*cinfo).out_color_space
        ==  JCS_RGB565
        ||  (*cinfo).out_color_space
            ==  JCS_CMYK
    {
        bits_per_pixel = 24i32;
        cmap_entries = 0i32
    } else {
        /* Grayscale output.  We need to fake a 256-entry colormap. */
        bits_per_pixel = 8i32;
        cmap_entries = 256i32
    }
     /* Header and colormap */
     let mut headersize:   c_long =
     (14i32 + 40i32 + cmap_entries * 4i32) as c_long; let mut bfSize:   c_long =
    
        headersize + (*dest).row_width as c_long * (*cinfo).output_height as c_long;
    /* Set unused fields of header to 0 */
    memset(
        bmpfileheader.as_mut_ptr() as *mut c_void,
        0i32,
        ::std::mem::size_of::<[c_char; 14]>() as c_ulong,
    );
    memset(
        bmpinfoheader.as_mut_ptr() as *mut c_void,
        0i32,
        ::std::mem::size_of::<[c_char; 40]>() as c_ulong,
    );
    /* Fill the file header */
    bmpfileheader[0] = 0x42i8; /* first 2 bytes are ASCII 'B', 'M' */
    bmpfileheader[1] = 0x4di8; /* bfSize */
    bmpfileheader[2] = (bfSize & 0xffi64) as c_char;
    bmpfileheader[(2i32 + 1i32) as usize] =
        (bfSize >> 8i32 & 0xffi64) as c_char;
    bmpfileheader[(2i32 + 2i32) as usize] =
        (bfSize >> 16i32 & 0xffi64) as c_char;
    bmpfileheader[(2i32 + 3i32) as usize] =
        (bfSize >> 24i32 & 0xffi64) as c_char;
    /* we leave bfReserved1 & bfReserved2 = 0 */
    bmpfileheader[10] = (headersize & 0xffi64) as c_char; /* bfOffBits */
    bmpfileheader[(10i32 + 1i32) as usize] =
        (headersize >> 8i32 & 0xffi64) as c_char;
    bmpfileheader[(10i32 + 2i32) as usize] =
        (headersize >> 16i32 & 0xffi64) as c_char;
    bmpfileheader[(10i32 + 3i32) as usize] =
        (headersize >> 24i32 & 0xffi64) as c_char;
    /* Fill the info header (Microsoft calls this a BITMAPINFOHEADER) */
    bmpinfoheader[0] = (40i32 & 0xffi32) as c_char; /* biSize */
    bmpinfoheader[(0i32 + 1i32) as usize] = (40i32 >> 8i32 & 0xffi32) as c_char; /* biWidth */
    bmpinfoheader[4] = ((*cinfo).output_width & 0xffu32) as c_char; /* biHeight */
    bmpinfoheader[(4i32 + 1i32) as usize] =
        ((*cinfo).output_width >> 8i32 & 0xffu32) as c_char; /* biPlanes - must be 1 */
    bmpinfoheader[(4i32 + 2i32) as usize] =
        ((*cinfo).output_width >> 16i32 & 0xffu32) as c_char; /* biBitCount */
    bmpinfoheader[(4i32 + 3i32) as usize] =
        ((*cinfo).output_width >> 24i32 & 0xffu32) as c_char;
    bmpinfoheader[8] = ((*cinfo).output_height & 0xffu32) as c_char;
    bmpinfoheader[(8i32 + 1i32) as usize] =
        ((*cinfo).output_height >> 8i32 & 0xffu32) as c_char;
    bmpinfoheader[(8i32 + 2i32) as usize] =
        ((*cinfo).output_height >> 16i32 & 0xffu32) as c_char;
    bmpinfoheader[(8i32 + 3i32) as usize] =
        ((*cinfo).output_height >> 24i32 & 0xffu32) as c_char;
    bmpinfoheader[12] = (1i32 & 0xffi32) as c_char;
    bmpinfoheader[(12i32 + 1i32) as usize] = (1i32 >> 8i32 & 0xffi32) as c_char;
    bmpinfoheader[14] = (bits_per_pixel & 0xffi32) as c_char;
    bmpinfoheader[(14i32 + 1i32) as usize] = (bits_per_pixel >> 8i32 & 0xffi32) as c_char;
    /* we leave biCompression = 0, for none */
    /* we leave biSizeImage = 0; this is correct for uncompressed data */
    if (*cinfo).density_unit as c_int == 2i32 {
        /* if have density in dots/cm, then */
        bmpinfoheader[24] = (((*cinfo).X_density as c_int * 100i32) as c_long
            & 0xffi64) as c_char; /* XPels/M */
        bmpinfoheader[(24i32 + 1i32) as usize] =
            (((*cinfo).X_density as c_int * 100i32) as c_long >> 8i32
                & 0xffi64) as c_char;
        bmpinfoheader[(24i32 + 2i32) as usize] =
            (((*cinfo).X_density as c_int * 100i32) as c_long >> 16i32
                & 0xffi64) as c_char;
        bmpinfoheader[(24i32 + 3i32) as usize] =
            (((*cinfo).X_density as c_int * 100i32) as c_long >> 24i32
                & 0xffi64) as c_char;
        bmpinfoheader[28] = (((*cinfo).Y_density as c_int * 100i32) as c_long
            & 0xffi64) as c_char;
        bmpinfoheader[(28i32 + 1i32) as usize] =
            (((*cinfo).Y_density as c_int * 100i32) as c_long >> 8i32
                & 0xffi64) as c_char;
        bmpinfoheader[(28i32 + 2i32) as usize] =
            (((*cinfo).Y_density as c_int * 100i32) as c_long >> 16i32
                & 0xffi64) as c_char;
        bmpinfoheader[(28i32 + 3i32) as usize] =
            (((*cinfo).Y_density as c_int * 100i32) as c_long >> 24i32
                & 0xffi64) as c_char
        /* XPels/M */
    } /* biClrUsed */
    bmpinfoheader[32] = (cmap_entries & 0xffi32) as c_char;
    bmpinfoheader[(32i32 + 1i32) as usize] = (cmap_entries >> 8i32 & 0xffi32) as c_char;
    /* we leave biClrImportant = 0 */
    if fwrite(
        bmpfileheader.as_mut_ptr() as *const c_void,
        1u64,
        14u64,
        (*dest).pub_0.output_file,
    ) != 14u64
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_FILE_WRITE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if fwrite(
        bmpinfoheader.as_mut_ptr() as *const c_void,
        1u64,
        40u64,
        (*dest).pub_0.output_file,
    ) != 40u64
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_FILE_WRITE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if cmap_entries > 0i32 {
        write_colormap(cinfo, dest, cmap_entries, 4i32);
    };
}

unsafe extern "C" fn write_os2_header(
    mut cinfo: j_decompress_ptr,
    mut dest: bmp_dest_ptr,
)
/* Write an OS2-style BMP file header, including colormap if needed */
{
    
    
    
    
    
     let mut bmpfileheader:  [c_char; 14] =  [0; 14]; let mut bmpcoreheader:  [c_char; 12] =  [0; 12];   let mut bits_per_pixel:  c_int =  0; let mut cmap_entries:  c_int =  0;
    /* Compute colormap size and total file size */
    if  (*cinfo).out_color_space
        ==  JCS_RGB
        ||  (*cinfo).out_color_space
            >=  JCS_EXT_RGB
            &&  (*cinfo).out_color_space
                <=  JCS_EXT_ARGB
    {
        if (*cinfo).quantize_colors != 0 {
            /* Colormapped RGB */
            bits_per_pixel = 8i32;
            cmap_entries = 256i32
        } else {
            /* Unquantized, full color RGB */
            bits_per_pixel = 24i32;
            cmap_entries = 0i32
        }
    } else if  (*cinfo).out_color_space
        ==  JCS_RGB565
        ||  (*cinfo).out_color_space
            ==  JCS_CMYK
    {
        bits_per_pixel = 24i32;
        cmap_entries = 0i32
    } else {
        /* Grayscale output.  We need to fake a 256-entry colormap. */
        bits_per_pixel = 8i32;
        cmap_entries = 256i32
    }
    /* File size */
     /* Header and colormap */
     let mut headersize:   c_long =
     (14i32 + 12i32 + cmap_entries * 3i32) as c_long; let mut bfSize:   c_long =
    
        headersize + (*dest).row_width as c_long * (*cinfo).output_height as c_long;
    /* Set unused fields of header to 0 */
    memset(
        bmpfileheader.as_mut_ptr() as *mut c_void,
        0i32,
        ::std::mem::size_of::<[c_char; 14]>() as c_ulong,
    );
    memset(
        bmpcoreheader.as_mut_ptr() as *mut c_void,
        0i32,
        ::std::mem::size_of::<[c_char; 12]>() as c_ulong,
    );
    /* Fill the file header */
    bmpfileheader[0] = 0x42i8; /* first 2 bytes are ASCII 'B', 'M' */
    bmpfileheader[1] = 0x4di8; /* bfSize */
    bmpfileheader[2] = (bfSize & 0xffi64) as c_char;
    bmpfileheader[(2i32 + 1i32) as usize] =
        (bfSize >> 8i32 & 0xffi64) as c_char;
    bmpfileheader[(2i32 + 2i32) as usize] =
        (bfSize >> 16i32 & 0xffi64) as c_char;
    bmpfileheader[(2i32 + 3i32) as usize] =
        (bfSize >> 24i32 & 0xffi64) as c_char;
    /* we leave bfReserved1 & bfReserved2 = 0 */
    bmpfileheader[10] = (headersize & 0xffi64) as c_char; /* bfOffBits */
    bmpfileheader[(10i32 + 1i32) as usize] =
        (headersize >> 8i32 & 0xffi64) as c_char;
    bmpfileheader[(10i32 + 2i32) as usize] =
        (headersize >> 16i32 & 0xffi64) as c_char;
    bmpfileheader[(10i32 + 3i32) as usize] =
        (headersize >> 24i32 & 0xffi64) as c_char;
    /* Fill the info header (Microsoft calls this a BITMAPCOREHEADER) */
    bmpcoreheader[0] = (12i32 & 0xffi32) as c_char; /* bcSize */
    bmpcoreheader[(0i32 + 1i32) as usize] = (12i32 >> 8i32 & 0xffi32) as c_char; /* bcWidth */
    bmpcoreheader[4] = ((*cinfo).output_width & 0xffu32) as c_char; /* bcHeight */
    bmpcoreheader[(4i32 + 1i32) as usize] =
        ((*cinfo).output_width >> 8i32 & 0xffu32) as c_char; /* bcPlanes - must be 1 */
    bmpcoreheader[6] = ((*cinfo).output_height & 0xffu32) as c_char; /* bcBitCount */
    bmpcoreheader[(6i32 + 1i32) as usize] =
        ((*cinfo).output_height >> 8i32 & 0xffu32) as c_char;
    bmpcoreheader[8] = (1i32 & 0xffi32) as c_char;
    bmpcoreheader[(8i32 + 1i32) as usize] = (1i32 >> 8i32 & 0xffi32) as c_char;
    bmpcoreheader[10] = (bits_per_pixel & 0xffi32) as c_char;
    bmpcoreheader[(10i32 + 1i32) as usize] = (bits_per_pixel >> 8i32 & 0xffi32) as c_char;
    if fwrite(
        bmpfileheader.as_mut_ptr() as *const c_void,
        1u64,
        14u64,
        (*dest).pub_0.output_file,
    ) != 14u64
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_FILE_WRITE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if fwrite(
        bmpcoreheader.as_mut_ptr() as *const c_void,
        1u64,
        12u64,
        (*dest).pub_0.output_file,
    ) != 12u64
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_FILE_WRITE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if cmap_entries > 0i32 {
        write_colormap(cinfo, dest, cmap_entries, 3i32);
    };
}
/* Forward declarations */
/*
 * Write the colormap.
 * Windows uses BGR0 map entries; OS/2 uses BGR entries.
 */

unsafe extern "C" fn write_colormap(
    mut cinfo: j_decompress_ptr,
    mut dest: bmp_dest_ptr,
    mut map_colors: c_int,
    mut map_entry_size: c_int,
) {
     let mut i:  c_int =  0;let mut colormap: JSAMPARRAY = (*cinfo).colormap;
    let mut num_colors: c_int = (*cinfo).actual_number_of_colors;
    let mut outfile: *mut FILE = (*dest).pub_0.output_file;
    
    if !colormap.is_null() {
        if (*cinfo).out_color_components == 3i32 {
            /* Normal case with RGB colormap */
            i = 0i32;
            while i < num_colors {
                putc(
                    *(*colormap.offset(2)).offset(i as isize) as c_int,
                    outfile,
                );
                putc(
                    *(*colormap.offset(1)).offset(i as isize) as c_int,
                    outfile,
                );
                putc(
                    *(*colormap.offset(0)).offset(i as isize) as c_int,
                    outfile,
                );
                if map_entry_size == 4i32 {
                    putc(0i32, outfile);
                }
                i += 1
            }
        } else {
            /* Grayscale colormap (only happens with grayscale quantization) */
            i = 0i32;
            while i < num_colors {
                putc(
                    *(*colormap.offset(0)).offset(i as isize) as c_int,
                    outfile,
                );
                putc(
                    *(*colormap.offset(0)).offset(i as isize) as c_int,
                    outfile,
                );
                putc(
                    *(*colormap.offset(0)).offset(i as isize) as c_int,
                    outfile,
                );
                if map_entry_size == 4i32 {
                    putc(0i32, outfile);
                }
                i += 1
            }
        }
    } else {
        /* If no colormap, must be grayscale data.  Generate a linear "map". */
        i = 0i32;
        while i < 256i32 {
            putc(i, outfile);
            putc(i, outfile);
            putc(i, outfile);
            if map_entry_size == 4i32 {
                putc(0i32, outfile);
            }
            i += 1
        }
    }
    /* Pad colormap with zeros to ensure specified number of colormap entries */
    if i > map_colors {
        (*(*cinfo).err).msg_code = JERR_TOO_MANY_COLORS as c_int;
        (*(*cinfo).err).msg_parm.i[0] = i;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    while i < map_colors {
        putc(0i32, outfile);
        putc(0i32, outfile);
        putc(0i32, outfile);
        if map_entry_size == 4i32 {
            putc(0i32, outfile);
        }
        i += 1
    }
}
/*
 * Startup: write the file header unless the inversion array is being used.
 */

unsafe extern "C" fn start_output_bmp(
    mut cinfo: j_decompress_ptr,
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
) {
    let mut dest: bmp_dest_ptr = dinfo as bmp_dest_ptr;
    if (*dest).use_inversion_array == 0 {
        /* Write the header and colormap */
        if (*dest).is_os2 != 0 {
            write_os2_header(cinfo, dest);
        } else {
            write_bmp_header(cinfo, dest);
        }
    };
}

unsafe extern "C" fn finish_output_bmp(
    mut cinfo: j_decompress_ptr,
    mut dinfo: super::cdjpeg::djpeg_dest_ptr,
) {
    let mut dest: bmp_dest_ptr = dinfo as bmp_dest_ptr;
    let mut outfile: *mut FILE = (*dest).pub_0.output_file;
    
    
    
    
    let mut progress: super::cdjpeg::cd_progress_ptr =
        (*cinfo).progress as super::cdjpeg::cd_progress_ptr;
    if (*dest).use_inversion_array != 0 {
        /* Write the header and colormap */
         if (*dest).is_os2 != 0 {
            write_os2_header(cinfo, dest);
        } else {
            write_bmp_header(cinfo, dest);
        }
        /* Write the file body from our virtual array */
         let mut row:   JDIMENSION =  (*cinfo).output_height;
        while row > 0u32 {
               if !progress.is_null() {
                (*progress).pub_0.pass_counter =
                    (
                    (*cinfo).output_height - row) as c_long;
                (*progress).pub_0.pass_limit = (*cinfo).output_height as c_long;
                Some(
                    (*progress)
                        .pub_0
                        .progress_monitor
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            
            
             let mut image_ptr:   JSAMPARRAY =
     Some(
                (*(*cinfo).mem)
                    .access_virt_sarray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                (*dest).whole_image,
                
                row - 1u32,
                1u32,
                FALSE,
            ); let mut data_ptr:   JSAMPROW =  *image_ptr.offset(0); let mut col:   JDIMENSION =  (*dest).row_width;
            while col > 0u32 {
                putc(*data_ptr as c_int, outfile);
                data_ptr = data_ptr.offset(1);
                col -=  1
            }
            row -=  1
        }
        if !progress.is_null() {
            (*progress).completed_extra_passes += 1
        }
    }
    /* Make sure we wrote the output file OK */
    fflush(outfile);
    if ferror(outfile) != 0 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_FILE_WRITE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    };
}
/*
 * The module selection routine for BMP format output.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_write_bmp(
    mut cinfo: j_decompress_ptr,
    mut is_os2: boolean,
    mut use_inversion_array: boolean,
) -> super::cdjpeg::djpeg_dest_ptr {
    
      let mut row_width:  JDIMENSION =  0;
    /* Create module interface object, fill in method pointers */
     let mut dest:   bmp_dest_ptr =
     Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<bmp_dest_struct>() as c_ulong,
    ) as bmp_dest_ptr;
    (*dest).pub_0.start_output = Some(
        start_output_bmp
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
                _: super::cdjpeg::djpeg_dest_ptr,
            ) -> (),
    );
    (*dest).pub_0.finish_output = Some(
        finish_output_bmp
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
                _: super::cdjpeg::djpeg_dest_ptr,
            ) -> (),
    );
    (*dest).pub_0.calc_buffer_dimensions = ::std::mem::transmute::<
        intptr_t,
        Option<
            unsafe extern "C" fn(
                _: j_decompress_ptr,
                _: super::cdjpeg::djpeg_dest_ptr,
            ) -> (),
        >,
    >(NULL as intptr_t);
    (*dest).is_os2 = is_os2;
    if  (*cinfo).out_color_space
        ==  JCS_GRAYSCALE
    {
        (*dest).pub_0.put_pixel_rows = Some(
            put_gray_rows
                as unsafe extern "C" fn(
                    _: j_decompress_ptr,
                    _: super::cdjpeg::djpeg_dest_ptr,
                    _: JDIMENSION,
                ) -> (),
        )
    } else if  (*cinfo).out_color_space
        ==  JCS_RGB
        ||  (*cinfo).out_color_space
            >=  JCS_EXT_RGB
            &&  (*cinfo).out_color_space
                <=  JCS_EXT_ARGB
    {
        if (*cinfo).quantize_colors != 0 {
            (*dest).pub_0.put_pixel_rows = Some(
                put_gray_rows
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: super::cdjpeg::djpeg_dest_ptr,
                        _: JDIMENSION,
                    ) -> (),
            )
        } else {
            (*dest).pub_0.put_pixel_rows = Some(
                put_pixel_rows
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: super::cdjpeg::djpeg_dest_ptr,
                        _: JDIMENSION,
                    ) -> (),
            )
        }
    } else if  (*cinfo).out_color_space
        ==  JCS_RGB565
        ||  (*cinfo).out_color_space
            ==  JCS_CMYK
    {
        (*dest).pub_0.put_pixel_rows = Some(
            put_pixel_rows
                as unsafe extern "C" fn(
                    _: j_decompress_ptr,
                    _: super::cdjpeg::djpeg_dest_ptr,
                    _: JDIMENSION,
                ) -> (),
        )
    } else {
        (*(*cinfo).err).msg_code = JERR_BMP_COLORSPACE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Calculate output image dimensions so we can allocate space */
    jpeg_calc_output_dimensions(cinfo);
    /* Determine width of rows in the BMP file (padded to 4-byte boundary). */
    if  (*cinfo).out_color_space
        ==  JCS_RGB565
    {
        row_width =  (*cinfo).output_width * 2u32;
        (*dest).data_width =  (*cinfo).output_width * 3u32;
        (*dest).row_width = (*dest).data_width;
        while row_width & 3u32 != 0u32 {
            row_width +=  1
        }
    } else if (*cinfo).quantize_colors == 0
        && ((*cinfo).out_color_space
            ==  JCS_RGB
            ||  (*cinfo).out_color_space
                >=  JCS_EXT_RGB
                &&  (*cinfo).out_color_space
                    <=  JCS_EXT_ARGB
            ||  (*cinfo).out_color_space
                ==  JCS_CMYK)
    {
        row_width =  (*cinfo)
            .output_width * (*cinfo).output_components as c_uint;
        (*dest).data_width =  (*cinfo).output_width * 3u32;
        (*dest).row_width = (*dest).data_width
    } else {
        row_width =  (*cinfo)
            .output_width * (*cinfo).output_components as c_uint;
        (*dest).data_width = row_width;
        (*dest).row_width = (*dest).data_width
    }
    while (*dest).row_width & 3u32 != 0u32 {
        (*dest).row_width =  (*dest).row_width + 1
    }
    (*dest).pad_bytes = ( (*dest).row_width - (*dest).data_width) as c_int;
    if use_inversion_array != 0 {
        /* Allocate space for inversion array, prepare for write pass */
        (*dest).whole_image = Some(
            (*(*cinfo).mem)
                .request_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            FALSE,
            (*dest).row_width,
            (*cinfo).output_height,
            1u32,
        );
        (*dest).cur_output_row = 0u32;
        if !(*cinfo).progress.is_null() {
            let mut progress: super::cdjpeg::cd_progress_ptr =
                (*cinfo).progress as super::cdjpeg::cd_progress_ptr;
            (*progress).total_extra_passes += 1
            /* count file input as separate pass */
        }
    } else {
        (*dest).iobuffer = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (*dest).row_width as size_t,
        ) as *mut JSAMPLE
    }
    (*dest).use_inversion_array = use_inversion_array;
    /* Create decompressor output buffer. */
    (*dest).pub_0.buffer = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        row_width,
        1u32,
    );
    (*dest).pub_0.buffer_height = 1u32;
    return dest as super::cdjpeg::djpeg_dest_ptr;
}
/* BMP_SUPPORTED */
