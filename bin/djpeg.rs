#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(
    const_raw_ptr_to_usize_cast,
    const_transmute,
    extern_types,
    main,
    register_tool
)]
pub mod jconfigint_h {
    pub const BUILD: [libc::c_char; 9] =
        unsafe { *::std::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"20191212\x00") };
    /* Compiler's inline keyword */
    /* How to obtain function inlining. */
    /* Define to the full name of this package. */

    pub const PACKAGE_NAME: [libc::c_char; 8] =
        unsafe { *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"mozjpeg\x00") };
    /* Version number of package */

    pub const VERSION: [libc::c_char; 6] =
        unsafe { *::std::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"4.0.0\x00") };
}
pub mod jversion_h {
    pub const JVERSION: [libc::c_char; 16] =
        unsafe { *::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"6b  27-Mar-1998\x00") };
    /*
     * NOTE: It is our convention to place the authors in the following order:
     * - libjpeg-turbo authors (2009-) in descending order of the date of their
     *   most recent contribution to the project, then in ascending order of the
     *   date of their first contribution to the project
     * - Upstream authors in descending order of the date of the first inclusion of
     *   their code
     */

    pub const JCOPYRIGHT: [libc::c_char; 533] = unsafe {
        *::std::mem::transmute::<&[u8; 533],
                                     &[libc::c_char; 533]>(b"Copyright (C) 2009-2018 D. R. Commander\nCopyright (C) 2011-2016 Siarhei Siamashka\nCopyright (C) 2015-2016, 2018 Matthieu Darbois\nCopyright (C) 2015 Intel Corporation\nCopyright (C) 2015 Google, Inc.\nCopyright (C) 2014 Mozilla Corporation\nCopyright (C) 2013-2014 MIPS Technologies, Inc.\nCopyright (C) 2013 Linaro Limited\nCopyright (C) 2009-2011 Nokia Corporation and/or its subsidiary(-ies)\nCopyright (C) 2009 Pierre Ossman for Cendio AB\nCopyright (C) 1999-2006 MIYASAKA Masaru\nCopyright (C) 1991-2016 Thomas G. Lane, Guido Vollbeding\x00")
    };
}
pub mod jconfig_h {
    pub const JPEG_LIB_VERSION: libc::c_int = 62 as libc::c_int;
}
pub mod cdjpeg_h {
    extern "C" {
        #[no_mangle]
        pub fn jinit_write_bmp(
            cinfo: crate::jpeglib_h::j_decompress_ptr,
            is_os2: crate::jmorecfg_h::boolean,
            use_inversion_array: crate::jmorecfg_h::boolean,
        ) -> crate::cdjpeg_h::djpeg_dest_ptr;

        #[no_mangle]
        pub fn jinit_write_gif(
            cinfo: crate::jpeglib_h::j_decompress_ptr,
        ) -> crate::cdjpeg_h::djpeg_dest_ptr;

        #[no_mangle]
        pub fn jinit_write_ppm(
            cinfo: crate::jpeglib_h::j_decompress_ptr,
        ) -> crate::cdjpeg_h::djpeg_dest_ptr;

        #[no_mangle]
        pub fn jinit_write_targa(
            cinfo: crate::jpeglib_h::j_decompress_ptr,
        ) -> crate::cdjpeg_h::djpeg_dest_ptr;

        #[no_mangle]
        pub fn read_color_map(
            cinfo: crate::jpeglib_h::j_decompress_ptr,
            infile: *mut crate::stdlib::FILE,
        );

        #[no_mangle]
        pub fn keymatch(
            arg: *mut libc::c_char,
            keyword: *const libc::c_char,
            minchars: libc::c_int,
        ) -> crate::jmorecfg_h::boolean;

        #[no_mangle]
        pub fn read_stdin() -> *mut crate::stdlib::FILE;

        #[no_mangle]
        pub fn write_stdout() -> *mut crate::stdlib::FILE;
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct djpeg_dest_struct {
        pub start_output: Option<
            unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::cdjpeg_h::djpeg_dest_ptr,
            ) -> (),
        >,
        pub put_pixel_rows: Option<
            unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::cdjpeg_h::djpeg_dest_ptr,
                _: crate::jmorecfg_h::JDIMENSION,
            ) -> (),
        >,
        pub finish_output: Option<
            unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::cdjpeg_h::djpeg_dest_ptr,
            ) -> (),
        >,
        pub calc_buffer_dimensions: Option<
            unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::cdjpeg_h::djpeg_dest_ptr,
            ) -> (),
        >,
        pub output_file: *mut crate::stdlib::FILE,
        pub buffer: crate::jpeglib_h::JSAMPARRAY,
        pub buffer_height: crate::jmorecfg_h::JDIMENSION,
    }

    pub type djpeg_dest_ptr = *mut crate::cdjpeg_h::djpeg_dest_struct;
    /* miscellaneous useful macros */
    /* define mode parameters for fopen() */

    pub const READ_BINARY: [libc::c_char; 3] =
        unsafe { *::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"rb\x00") };

    pub const WRITE_BINARY: [libc::c_char; 3] =
        unsafe { *::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"wb\x00") };
    /* define exit() codes if not provided */

    pub const EXIT_WARNING: libc::c_int = 2 as libc::c_int;
}
pub mod cderror_h {
    pub const JMSG_LASTADDONCODE: crate::jerror_h::C2RustUnnamed_1 = 1046;

    pub const JERR_UNSUPPORTED_FORMAT: crate::jerror_h::C2RustUnnamed_1 = 1045;

    pub const JERR_UNKNOWN_FORMAT: crate::jerror_h::C2RustUnnamed_1 = 1044;

    pub const JERR_UNGETC_FAILED: crate::jerror_h::C2RustUnnamed_1 = 1043;

    pub const JERR_TOO_MANY_COLORS: crate::jerror_h::C2RustUnnamed_1 = 1042;
    /* TARGA_SUPPORTED */
    /* TARGA_SUPPORTED */

    pub const JERR_BAD_CMAP_FILE: crate::jerror_h::C2RustUnnamed_1 = 1041;

    pub const JTRC_TGA_MAPPED: crate::jerror_h::C2RustUnnamed_1 = 1040;

    pub const JTRC_TGA_GRAY: crate::jerror_h::C2RustUnnamed_1 = 1039;

    pub const JTRC_TGA: crate::jerror_h::C2RustUnnamed_1 = 1038;

    pub const JERR_TGA_COLORSPACE: crate::jerror_h::C2RustUnnamed_1 = 1037;

    pub const JERR_TGA_BADPARMS: crate::jerror_h::C2RustUnnamed_1 = 1036;
    /* PPM_SUPPORTED */
    /* PPM_SUPPORTED */
    /* RLE_SUPPORTED */
    /* RLE_SUPPORTED */

    pub const JERR_TGA_BADCMAP: crate::jerror_h::C2RustUnnamed_1 = 1035;

    pub const JTRC_PPM_TEXT: crate::jerror_h::C2RustUnnamed_1 = 1034;

    pub const JTRC_PPM: crate::jerror_h::C2RustUnnamed_1 = 1033;

    pub const JTRC_PGM_TEXT: crate::jerror_h::C2RustUnnamed_1 = 1032;

    pub const JTRC_PGM: crate::jerror_h::C2RustUnnamed_1 = 1031;

    pub const JERR_PPM_OUTOFRANGE: crate::jerror_h::C2RustUnnamed_1 = 1030;

    pub const JERR_PPM_NOT: crate::jerror_h::C2RustUnnamed_1 = 1029;

    pub const JERR_PPM_NONNUMERIC: crate::jerror_h::C2RustUnnamed_1 = 1028;
    /* GIF_SUPPORTED */
    /* GIF_SUPPORTED */

    pub const JERR_PPM_COLORSPACE: crate::jerror_h::C2RustUnnamed_1 = 1027;

    pub const JWRN_GIF_NOMOREDATA: crate::jerror_h::C2RustUnnamed_1 = 1026;

    pub const JWRN_GIF_ENDCODE: crate::jerror_h::C2RustUnnamed_1 = 1025;

    pub const JWRN_GIF_CHAR: crate::jerror_h::C2RustUnnamed_1 = 1024;

    pub const JWRN_GIF_BADDATA: crate::jerror_h::C2RustUnnamed_1 = 1023;

    pub const JTRC_GIF_NONSQUARE: crate::jerror_h::C2RustUnnamed_1 = 1022;

    pub const JTRC_GIF_EXTENSION: crate::jerror_h::C2RustUnnamed_1 = 1021;

    pub const JTRC_GIF_BADVERSION: crate::jerror_h::C2RustUnnamed_1 = 1020;

    pub const JTRC_GIF: crate::jerror_h::C2RustUnnamed_1 = 1019;

    pub const JERR_GIF_NOT: crate::jerror_h::C2RustUnnamed_1 = 1018;

    pub const JERR_GIF_IMAGENOTFOUND: crate::jerror_h::C2RustUnnamed_1 = 1017;

    pub const JERR_GIF_COLORSPACE: crate::jerror_h::C2RustUnnamed_1 = 1016;

    pub const JERR_GIF_CODESIZE: crate::jerror_h::C2RustUnnamed_1 = 1015;
    /* BMP_SUPPORTED */
    /* BMP_SUPPORTED */

    pub const JERR_GIF_BUG: crate::jerror_h::C2RustUnnamed_1 = 1014;

    pub const JTRC_BMP_OS2_MAPPED: crate::jerror_h::C2RustUnnamed_1 = 1013;

    pub const JTRC_BMP_OS2: crate::jerror_h::C2RustUnnamed_1 = 1012;

    pub const JTRC_BMP_MAPPED: crate::jerror_h::C2RustUnnamed_1 = 1011;

    pub const JTRC_BMP: crate::jerror_h::C2RustUnnamed_1 = 1010;

    pub const JERR_BMP_OUTOFRANGE: crate::jerror_h::C2RustUnnamed_1 = 1009;

    pub const JERR_BMP_NOT: crate::jerror_h::C2RustUnnamed_1 = 1008;

    pub const JERR_BMP_EMPTY: crate::jerror_h::C2RustUnnamed_1 = 1007;

    pub const JERR_BMP_COMPRESSED: crate::jerror_h::C2RustUnnamed_1 = 1006;

    pub const JERR_BMP_COLORSPACE: crate::jerror_h::C2RustUnnamed_1 = 1005;

    pub const JERR_BMP_BADPLANES: crate::jerror_h::C2RustUnnamed_1 = 1004;

    pub const JERR_BMP_BADHEADER: crate::jerror_h::C2RustUnnamed_1 = 1003;

    pub const JERR_BMP_BADDEPTH: crate::jerror_h::C2RustUnnamed_1 = 1002;
    /* Must be first entry! */
    /* Must be first entry! */

    pub const JERR_BMP_BADCMAP: crate::jerror_h::C2RustUnnamed_1 = 1001;
    /* JMAKE_ENUM_LIST */
    /* JMAKE_ENUM_LIST */

    pub const JMSG_FIRSTADDONCODE: crate::jerror_h::C2RustUnnamed_1 = 1000;
}
pub mod jerror_h {
    pub type C2RustUnnamed_1 = libc::c_uint;

    pub const JMSG_LASTMSGCODE: crate::jerror_h::C2RustUnnamed_1 = 129;

    pub const JWRN_BOGUS_ICC: crate::jerror_h::C2RustUnnamed_1 = 128;

    pub const JERR_UNSUPPORTED_SUSPEND: crate::jerror_h::C2RustUnnamed_1 = 127;

    pub const JERR_BAD_PARAM_VALUE: crate::jerror_h::C2RustUnnamed_1 = 126;

    pub const JERR_BAD_PARAM: crate::jerror_h::C2RustUnnamed_1 = 125;

    pub const JERR_BAD_CROP_SPEC: crate::jerror_h::C2RustUnnamed_1 = 124;

    pub const JWRN_TOO_MUCH_DATA: crate::jerror_h::C2RustUnnamed_1 = 123;

    pub const JWRN_NOT_SEQUENTIAL: crate::jerror_h::C2RustUnnamed_1 = 122;

    pub const JWRN_MUST_RESYNC: crate::jerror_h::C2RustUnnamed_1 = 121;

    pub const JWRN_JPEG_EOF: crate::jerror_h::C2RustUnnamed_1 = 120;

    pub const JWRN_JFIF_MAJOR: crate::jerror_h::C2RustUnnamed_1 = 119;

    pub const JWRN_HUFF_BAD_CODE: crate::jerror_h::C2RustUnnamed_1 = 118;

    pub const JWRN_HIT_MARKER: crate::jerror_h::C2RustUnnamed_1 = 117;

    pub const JWRN_EXTRANEOUS_DATA: crate::jerror_h::C2RustUnnamed_1 = 116;

    pub const JWRN_BOGUS_PROGRESSION: crate::jerror_h::C2RustUnnamed_1 = 115;

    pub const JWRN_ADOBE_XFORM: crate::jerror_h::C2RustUnnamed_1 = 114;

    pub const JTRC_XMS_OPEN: crate::jerror_h::C2RustUnnamed_1 = 113;

    pub const JTRC_XMS_CLOSE: crate::jerror_h::C2RustUnnamed_1 = 112;

    pub const JTRC_UNKNOWN_IDS: crate::jerror_h::C2RustUnnamed_1 = 111;

    pub const JTRC_THUMB_RGB: crate::jerror_h::C2RustUnnamed_1 = 110;

    pub const JTRC_THUMB_PALETTE: crate::jerror_h::C2RustUnnamed_1 = 109;

    pub const JTRC_THUMB_JPEG: crate::jerror_h::C2RustUnnamed_1 = 108;

    pub const JTRC_TFILE_OPEN: crate::jerror_h::C2RustUnnamed_1 = 107;

    pub const JTRC_TFILE_CLOSE: crate::jerror_h::C2RustUnnamed_1 = 106;

    pub const JTRC_SOS_PARAMS: crate::jerror_h::C2RustUnnamed_1 = 105;

    pub const JTRC_SOS_COMPONENT: crate::jerror_h::C2RustUnnamed_1 = 104;

    pub const JTRC_SOS: crate::jerror_h::C2RustUnnamed_1 = 103;

    pub const JTRC_SOI: crate::jerror_h::C2RustUnnamed_1 = 102;

    pub const JTRC_SOF_COMPONENT: crate::jerror_h::C2RustUnnamed_1 = 101;

    pub const JTRC_SOF: crate::jerror_h::C2RustUnnamed_1 = 100;

    pub const JTRC_SMOOTH_NOTIMPL: crate::jerror_h::C2RustUnnamed_1 = 99;

    pub const JTRC_RST: crate::jerror_h::C2RustUnnamed_1 = 98;

    pub const JTRC_RECOVERY_ACTION: crate::jerror_h::C2RustUnnamed_1 = 97;

    pub const JTRC_QUANT_SELECTED: crate::jerror_h::C2RustUnnamed_1 = 96;

    pub const JTRC_QUANT_NCOLORS: crate::jerror_h::C2RustUnnamed_1 = 95;

    pub const JTRC_QUANT_3_NCOLORS: crate::jerror_h::C2RustUnnamed_1 = 94;

    pub const JTRC_QUANTVALS: crate::jerror_h::C2RustUnnamed_1 = 93;

    pub const JTRC_PARMLESS_MARKER: crate::jerror_h::C2RustUnnamed_1 = 92;

    pub const JTRC_MISC_MARKER: crate::jerror_h::C2RustUnnamed_1 = 91;

    pub const JTRC_JFIF_THUMBNAIL: crate::jerror_h::C2RustUnnamed_1 = 90;

    pub const JTRC_JFIF_EXTENSION: crate::jerror_h::C2RustUnnamed_1 = 89;

    pub const JTRC_JFIF_BADTHUMBNAILSIZE: crate::jerror_h::C2RustUnnamed_1 = 88;

    pub const JTRC_JFIF: crate::jerror_h::C2RustUnnamed_1 = 87;

    pub const JTRC_HUFFBITS: crate::jerror_h::C2RustUnnamed_1 = 86;

    pub const JTRC_EOI: crate::jerror_h::C2RustUnnamed_1 = 85;

    pub const JTRC_EMS_OPEN: crate::jerror_h::C2RustUnnamed_1 = 84;

    pub const JTRC_EMS_CLOSE: crate::jerror_h::C2RustUnnamed_1 = 83;

    pub const JTRC_DRI: crate::jerror_h::C2RustUnnamed_1 = 82;

    pub const JTRC_DQT: crate::jerror_h::C2RustUnnamed_1 = 81;

    pub const JTRC_DHT: crate::jerror_h::C2RustUnnamed_1 = 80;

    pub const JTRC_DAC: crate::jerror_h::C2RustUnnamed_1 = 79;

    pub const JTRC_APP14: crate::jerror_h::C2RustUnnamed_1 = 78;

    pub const JTRC_APP0: crate::jerror_h::C2RustUnnamed_1 = 77;

    pub const JTRC_ADOBE: crate::jerror_h::C2RustUnnamed_1 = 76;

    pub const JTRC_16BIT_TABLES: crate::jerror_h::C2RustUnnamed_1 = 75;

    pub const JMSG_VERSION: crate::jerror_h::C2RustUnnamed_1 = 74;

    pub const JMSG_COPYRIGHT: crate::jerror_h::C2RustUnnamed_1 = 73;

    pub const JERR_XMS_WRITE: crate::jerror_h::C2RustUnnamed_1 = 72;

    pub const JERR_XMS_READ: crate::jerror_h::C2RustUnnamed_1 = 71;

    pub const JERR_WIDTH_OVERFLOW: crate::jerror_h::C2RustUnnamed_1 = 70;

    pub const JERR_VIRTUAL_BUG: crate::jerror_h::C2RustUnnamed_1 = 69;

    pub const JERR_UNKNOWN_MARKER: crate::jerror_h::C2RustUnnamed_1 = 68;

    pub const JERR_TOO_LITTLE_DATA: crate::jerror_h::C2RustUnnamed_1 = 67;

    pub const JERR_TFILE_WRITE: crate::jerror_h::C2RustUnnamed_1 = 66;

    pub const JERR_TFILE_SEEK: crate::jerror_h::C2RustUnnamed_1 = 65;

    pub const JERR_TFILE_READ: crate::jerror_h::C2RustUnnamed_1 = 64;

    pub const JERR_TFILE_CREATE: crate::jerror_h::C2RustUnnamed_1 = 63;

    pub const JERR_SOS_NO_SOF: crate::jerror_h::C2RustUnnamed_1 = 62;

    pub const JERR_SOI_DUPLICATE: crate::jerror_h::C2RustUnnamed_1 = 61;

    pub const JERR_SOF_UNSUPPORTED: crate::jerror_h::C2RustUnnamed_1 = 60;

    pub const JERR_SOF_NO_SOS: crate::jerror_h::C2RustUnnamed_1 = 59;

    pub const JERR_SOF_DUPLICATE: crate::jerror_h::C2RustUnnamed_1 = 58;

    pub const JERR_QUANT_MANY_COLORS: crate::jerror_h::C2RustUnnamed_1 = 57;

    pub const JERR_QUANT_FEW_COLORS: crate::jerror_h::C2RustUnnamed_1 = 56;

    pub const JERR_QUANT_COMPONENTS: crate::jerror_h::C2RustUnnamed_1 = 55;

    pub const JERR_OUT_OF_MEMORY: crate::jerror_h::C2RustUnnamed_1 = 54;

    pub const JERR_NO_SOI: crate::jerror_h::C2RustUnnamed_1 = 53;

    pub const JERR_NO_QUANT_TABLE: crate::jerror_h::C2RustUnnamed_1 = 52;

    pub const JERR_NO_IMAGE: crate::jerror_h::C2RustUnnamed_1 = 51;

    pub const JERR_NO_HUFF_TABLE: crate::jerror_h::C2RustUnnamed_1 = 50;

    pub const JERR_NO_BACKING_STORE: crate::jerror_h::C2RustUnnamed_1 = 49;

    pub const JERR_NOT_COMPILED: crate::jerror_h::C2RustUnnamed_1 = 48;

    pub const JERR_NOTIMPL: crate::jerror_h::C2RustUnnamed_1 = 47;

    pub const JERR_MODE_CHANGE: crate::jerror_h::C2RustUnnamed_1 = 46;

    pub const JERR_MISSING_DATA: crate::jerror_h::C2RustUnnamed_1 = 45;

    pub const JERR_MISMATCHED_QUANT_TABLE: crate::jerror_h::C2RustUnnamed_1 = 44;

    pub const JERR_INPUT_EOF: crate::jerror_h::C2RustUnnamed_1 = 43;

    pub const JERR_INPUT_EMPTY: crate::jerror_h::C2RustUnnamed_1 = 42;

    pub const JERR_IMAGE_TOO_BIG: crate::jerror_h::C2RustUnnamed_1 = 41;

    pub const JERR_HUFF_MISSING_CODE: crate::jerror_h::C2RustUnnamed_1 = 40;

    pub const JERR_HUFF_CLEN_OVERFLOW: crate::jerror_h::C2RustUnnamed_1 = 39;

    pub const JERR_FRACT_SAMPLE_NOTIMPL: crate::jerror_h::C2RustUnnamed_1 = 38;

    pub const JERR_FILE_WRITE: crate::jerror_h::C2RustUnnamed_1 = 37;

    pub const JERR_FILE_READ: crate::jerror_h::C2RustUnnamed_1 = 36;

    pub const JERR_EOI_EXPECTED: crate::jerror_h::C2RustUnnamed_1 = 35;

    pub const JERR_EMS_WRITE: crate::jerror_h::C2RustUnnamed_1 = 34;

    pub const JERR_EMS_READ: crate::jerror_h::C2RustUnnamed_1 = 33;

    pub const JERR_EMPTY_IMAGE: crate::jerror_h::C2RustUnnamed_1 = 32;

    pub const JERR_DQT_INDEX: crate::jerror_h::C2RustUnnamed_1 = 31;

    pub const JERR_DHT_INDEX: crate::jerror_h::C2RustUnnamed_1 = 30;

    pub const JERR_DAC_VALUE: crate::jerror_h::C2RustUnnamed_1 = 29;

    pub const JERR_DAC_INDEX: crate::jerror_h::C2RustUnnamed_1 = 28;

    pub const JERR_CONVERSION_NOTIMPL: crate::jerror_h::C2RustUnnamed_1 = 27;

    pub const JERR_COMPONENT_COUNT: crate::jerror_h::C2RustUnnamed_1 = 26;

    pub const JERR_CCIR601_NOTIMPL: crate::jerror_h::C2RustUnnamed_1 = 25;

    pub const JERR_CANT_SUSPEND: crate::jerror_h::C2RustUnnamed_1 = 24;

    pub const JERR_BUFFER_SIZE: crate::jerror_h::C2RustUnnamed_1 = 23;

    pub const JERR_BAD_VIRTUAL_ACCESS: crate::jerror_h::C2RustUnnamed_1 = 22;

    pub const JERR_BAD_STRUCT_SIZE: crate::jerror_h::C2RustUnnamed_1 = 21;

    pub const JERR_BAD_STATE: crate::jerror_h::C2RustUnnamed_1 = 20;

    pub const JERR_BAD_SCAN_SCRIPT: crate::jerror_h::C2RustUnnamed_1 = 19;

    pub const JERR_BAD_SAMPLING: crate::jerror_h::C2RustUnnamed_1 = 18;

    pub const JERR_BAD_PROG_SCRIPT: crate::jerror_h::C2RustUnnamed_1 = 17;

    pub const JERR_BAD_PROGRESSION: crate::jerror_h::C2RustUnnamed_1 = 16;

    pub const JERR_BAD_PRECISION: crate::jerror_h::C2RustUnnamed_1 = 15;

    pub const JERR_BAD_POOL_ID: crate::jerror_h::C2RustUnnamed_1 = 14;

    pub const JERR_BAD_MCU_SIZE: crate::jerror_h::C2RustUnnamed_1 = 13;

    pub const JERR_BAD_LIB_VERSION: crate::jerror_h::C2RustUnnamed_1 = 12;

    pub const JERR_BAD_LENGTH: crate::jerror_h::C2RustUnnamed_1 = 11;

    pub const JERR_BAD_J_COLORSPACE: crate::jerror_h::C2RustUnnamed_1 = 10;

    pub const JERR_BAD_IN_COLORSPACE: crate::jerror_h::C2RustUnnamed_1 = 9;

    pub const JERR_BAD_HUFF_TABLE: crate::jerror_h::C2RustUnnamed_1 = 8;

    pub const JERR_BAD_DCTSIZE: crate::jerror_h::C2RustUnnamed_1 = 7;

    pub const JERR_BAD_DCT_COEF: crate::jerror_h::C2RustUnnamed_1 = 6;

    pub const JERR_BAD_COMPONENT_ID: crate::jerror_h::C2RustUnnamed_1 = 5;

    pub const JERR_BAD_BUFFER_MODE: crate::jerror_h::C2RustUnnamed_1 = 4;

    pub const JERR_BAD_ALLOC_CHUNK: crate::jerror_h::C2RustUnnamed_1 = 3;

    pub const JERR_BAD_ALIGN_TYPE: crate::jerror_h::C2RustUnnamed_1 = 2;
    /* Must be first entry! */
    /* For maintenance convenience, list is alphabetical by message code name */

    pub const JERR_ARITH_NOTIMPL: crate::jerror_h::C2RustUnnamed_1 = 1;
    /* JMAKE_ENUM_LIST */

    pub const JMSG_NOMESSAGE: crate::jerror_h::C2RustUnnamed_1 = 0;
}
pub mod jpeglib_h {
    extern "C" {
        pub type jvirt_barray_control;

        pub type jvirt_sarray_control;

        pub type jpeg_color_quantizer;

        pub type jpeg_color_deconverter;

        pub type jpeg_upsampler;

        pub type jpeg_inverse_dct;

        pub type jpeg_entropy_decoder;

        pub type jpeg_marker_reader;

        pub type jpeg_input_controller;

        pub type jpeg_d_post_controller;

        pub type jpeg_d_coef_controller;

        pub type jpeg_d_main_controller;

        pub type jpeg_decomp_master;
        /* Originally, this macro was used as a way of defining function prototypes
         * for both modern compilers as well as older compilers that did not support
         * prototype parameters.  libjpeg-turbo has never supported these older,
         * non-ANSI compilers, but the macro is still included because there is some
         * software out there that uses it.
         */
        /* Default error-management setup */
        #[no_mangle]
        pub fn jpeg_std_error(
            err: *mut crate::jpeglib_h::jpeg_error_mgr,
        ) -> *mut crate::jpeglib_h::jpeg_error_mgr;

        #[no_mangle]
        pub fn jpeg_CreateDecompress(
            cinfo: crate::jpeglib_h::j_decompress_ptr,
            version: libc::c_int,
            structsize: crate::stddef_h::size_t,
        );

        #[no_mangle]
        pub fn jpeg_destroy_decompress(cinfo: crate::jpeglib_h::j_decompress_ptr);

        #[no_mangle]
        pub fn jpeg_stdio_src(
            cinfo: crate::jpeglib_h::j_decompress_ptr,
            infile: *mut crate::stdlib::FILE,
        );

        #[no_mangle]
        pub fn jpeg_mem_src(
            cinfo: crate::jpeglib_h::j_decompress_ptr,
            inbuffer: *const libc::c_uchar,
            insize: libc::c_ulong,
        );
        /* Decompression startup: read start of JPEG datastream to see what's there */
        #[no_mangle]
        pub fn jpeg_read_header(
            cinfo: crate::jpeglib_h::j_decompress_ptr,
            require_image: crate::jmorecfg_h::boolean,
        ) -> libc::c_int;
        /* Return value is one of: */
        /* Suspended due to lack of input data */
        /* Found valid image datastream */
        /* Found valid table-specs-only datastream */
        /* If you pass require_image = TRUE (normal case), you need not check for
         * a TABLES_ONLY return code; an abbreviated file will cause an error exit.
         * JPEG_SUSPENDED is only possible if you use a data source module that can
         * give a suspension return (the stdio source module doesn't).
         */
        /* Main entry points for decompression */
        #[no_mangle]
        pub fn jpeg_start_decompress(
            cinfo: crate::jpeglib_h::j_decompress_ptr,
        ) -> crate::jmorecfg_h::boolean;

        #[no_mangle]
        pub fn jpeg_read_scanlines(
            cinfo: crate::jpeglib_h::j_decompress_ptr,
            scanlines: crate::jpeglib_h::JSAMPARRAY,
            max_lines: crate::jmorecfg_h::JDIMENSION,
        ) -> crate::jmorecfg_h::JDIMENSION;

        #[no_mangle]
        pub fn jpeg_skip_scanlines(
            cinfo: crate::jpeglib_h::j_decompress_ptr,
            num_lines: crate::jmorecfg_h::JDIMENSION,
        ) -> crate::jmorecfg_h::JDIMENSION;

        #[no_mangle]
        pub fn jpeg_crop_scanline(
            cinfo: crate::jpeglib_h::j_decompress_ptr,
            xoffset: *mut crate::jmorecfg_h::JDIMENSION,
            width: *mut crate::jmorecfg_h::JDIMENSION,
        );

        #[no_mangle]
        pub fn jpeg_finish_decompress(
            cinfo: crate::jpeglib_h::j_decompress_ptr,
        ) -> crate::jmorecfg_h::boolean;
        /* Control saving of COM and APPn markers into marker_list. */
        #[no_mangle]
        pub fn jpeg_save_markers(
            cinfo: crate::jpeglib_h::j_decompress_ptr,
            marker_code: libc::c_int,
            length_limit: libc::c_uint,
        );
        /* Install a special processing method for COM or APPn markers. */
        #[no_mangle]
        pub fn jpeg_set_marker_processor(
            cinfo: crate::jpeglib_h::j_decompress_ptr,
            marker_code: libc::c_int,
            routine: crate::jpeglib_h::jpeg_marker_parser_method,
        );
        /* Read ICC profile.  See libjpeg.txt for usage information. */
        #[no_mangle]
        pub fn jpeg_read_icc_profile(
            cinfo: crate::jpeglib_h::j_decompress_ptr,
            icc_data_ptr: *mut *mut crate::jmorecfg_h::JOCTET,
            icc_data_len: *mut libc::c_uint,
        ) -> crate::jmorecfg_h::boolean;
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union C2RustUnnamed_0 {
        pub i: [libc::c_int; 8],
        pub s: [libc::c_char; 80],
    }
    /*
     * jpeglib.h
     *
     * This file was part of the Independent JPEG Group's software:
     * Copyright (C) 1991-1998, Thomas G. Lane.
     * Modified 2002-2009 by Guido Vollbeding.
     * libjpeg-turbo Modifications:
     * Copyright (C) 2009-2011, 2013-2014, 2016-2017, D. R. Commander.
     * Copyright (C) 2015, Google, Inc.
     * mozjpeg Modifications:
     * Copyright (C) 2014, Mozilla Corporation.
     * For conditions of distribution and use, see the accompanying README.ijg
     * file.
     *
     * This file defines the application interface for the JPEG library.
     * Most applications using the library need only include this file,
     * and perhaps jerror.h if they want to know the exact error codes.
     */
    /*
     * First we include the configuration files that record how this
     * installation of the JPEG library is set up.  jconfig.h can be
     * generated automatically for many systems.  jmorecfg.h contains
     * manual configuration options that most people need not worry about.
     */
    /* in case jinclude.h already did */
    /* Various constants determining the sizes of things.
     * All of these are specified by the JPEG standard, so don't change them
     * if you want to be compatible.
     */
    /* The basic DCT block is 8x8 samples */
    /* DCTSIZE squared; # of elements in a block */
    /* Quantization tables are numbered 0..3 */
    /* Huffman tables are numbered 0..3 */
    /* Arith-coding tables are numbered 0..15 */
    /* JPEG limit on # of components in one scan */
    /* JPEG limit on sampling factors */
    /* Unfortunately, some bozo at Adobe saw no reason to be bound by the standard;
     * the PostScript DCT filter can emit files with many more than 10 blocks/MCU.
     * If you happen to run across such a file, you can up D_MAX_BLOCKS_IN_MCU
     * to handle it.  We even let you do this from the jconfig.h file.  However,
     * we strongly discourage changing C_MAX_BLOCKS_IN_MCU; just because Adobe
     * sometimes emits noncompliant files doesn't mean you should too.
     */
    /* compressor's limit on blocks per MCU */
    /* decompressor's limit on blocks per MCU */
    /* Data structures for images (arrays of samples and of DCT coefficients).
     */

    pub type JSAMPROW = *mut crate::jmorecfg_h::JSAMPLE;
    /* ptr to one image row of pixel samples. */

    pub type JSAMPARRAY = *mut crate::jpeglib_h::JSAMPROW;
    /* a 3-D sample array: top index is color */

    pub type JBLOCK = [crate::jmorecfg_h::JCOEF; 64];
    /* one block of coefficients */

    pub type JBLOCKROW = *mut crate::jpeglib_h::JBLOCK;
    /* pointer to one row of coefficient blocks */

    pub type JBLOCKARRAY = *mut crate::jpeglib_h::JBLOCKROW;
    /* useful in a couple of places */
    /* Types for JPEG compression parameters and working tables. */
    /* DCT coefficient quantization tables. */

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct JQUANT_TBL {
        pub quantval: [crate::jmorecfg_h::UINT16; 64],
        pub sent_table: crate::jmorecfg_h::boolean,
    }
    /* Huffman coding tables. */

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct JHUFF_TBL {
        pub bits: [crate::jmorecfg_h::UINT8; 17],
        pub huffval: [crate::jmorecfg_h::UINT8; 256],
        pub sent_table: crate::jmorecfg_h::boolean,
    }
    /* Basic info about one component (color channel). */

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct jpeg_component_info {
        pub component_id: libc::c_int,
        pub component_index: libc::c_int,
        pub h_samp_factor: libc::c_int,
        pub v_samp_factor: libc::c_int,
        pub quant_tbl_no: libc::c_int,
        pub dc_tbl_no: libc::c_int,
        pub ac_tbl_no: libc::c_int,
        pub width_in_blocks: crate::jmorecfg_h::JDIMENSION,
        pub height_in_blocks: crate::jmorecfg_h::JDIMENSION,
        pub DCT_scaled_size: libc::c_int,
        pub downsampled_width: crate::jmorecfg_h::JDIMENSION,
        pub downsampled_height: crate::jmorecfg_h::JDIMENSION,
        pub component_needed: crate::jmorecfg_h::boolean,
        pub MCU_width: libc::c_int,
        pub MCU_height: libc::c_int,
        pub MCU_blocks: libc::c_int,
        pub MCU_sample_width: libc::c_int,
        pub last_col_width: libc::c_int,
        pub last_row_height: libc::c_int,
        pub quant_table: *mut crate::jpeglib_h::JQUANT_TBL,
        pub dct_table: *mut libc::c_void,
    }
    /* The decompressor can save APPn and COM markers in a list of these: */

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct jpeg_marker_struct {
        pub next: crate::jpeglib_h::jpeg_saved_marker_ptr,
        pub marker: crate::jmorecfg_h::UINT8,
        pub original_length: libc::c_uint,
        pub data_length: libc::c_uint,
        pub data: *mut crate::jmorecfg_h::JOCTET,
    }

    pub type jpeg_saved_marker_ptr = *mut crate::jpeglib_h::jpeg_marker_struct;
    /* the data contained in the marker */
    /* the marker length word is not counted in data_length or original_length */
    /* Known color spaces. */

    pub type J_COLOR_SPACE = libc::c_uint;
    /* DCT/IDCT algorithm options. */

    pub type J_DCT_METHOD = libc::c_uint;
    /* may be overridden in jconfig.h */
    /* may be overridden in jconfig.h */
    /* Dithering options for decompression. */

    pub type J_DITHER_MODE = libc::c_uint;
    /* Common fields between JPEG compression and decompression master structs. */
    /* Error handler module */
    /* Memory manager module */
    /* Progress monitor, or NULL if none */
    /* Available for use by application */
    /* So common code can tell which is which */
    /* For checking call sequence validity */
    /* Routines that are to be used by both halves of the library are declared
     * to receive a pointer to this structure.  There are no actual instances of
     * jpeg_common_struct, only of jpeg_compress_struct and jpeg_decompress_struct.
     */

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct jpeg_common_struct {
        pub err: *mut crate::jpeglib_h::jpeg_error_mgr,
        pub mem: *mut crate::jpeglib_h::jpeg_memory_mgr,
        pub progress: *mut crate::jpeglib_h::jpeg_progress_mgr,
        pub client_data: *mut libc::c_void,
        pub is_decompressor: crate::jmorecfg_h::boolean,
        pub global_state: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct jpeg_progress_mgr {
        pub progress_monitor: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ()>,
        pub pass_counter: libc::c_long,
        pub pass_limit: libc::c_long,
        pub completed_passes: libc::c_int,
        pub total_passes: libc::c_int,
    }
    /* Fields common to both master struct types */
    /* Additional fields follow in an actual jpeg_compress_struct or
     * jpeg_decompress_struct.  All three structs must agree on these
     * initial fields!  (This would be a lot cleaner in C++.)
     */

    pub type j_common_ptr = *mut crate::jpeglib_h::jpeg_common_struct;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct jpeg_memory_mgr {
        pub alloc_small: Option<
            unsafe extern "C" fn(
                _: crate::jpeglib_h::j_common_ptr,
                _: libc::c_int,
                _: crate::stddef_h::size_t,
            ) -> *mut libc::c_void,
        >,
        pub alloc_large: Option<
            unsafe extern "C" fn(
                _: crate::jpeglib_h::j_common_ptr,
                _: libc::c_int,
                _: crate::stddef_h::size_t,
            ) -> *mut libc::c_void,
        >,
        pub alloc_sarray: Option<
            unsafe extern "C" fn(
                _: crate::jpeglib_h::j_common_ptr,
                _: libc::c_int,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::JDIMENSION,
            ) -> crate::jpeglib_h::JSAMPARRAY,
        >,
        pub alloc_barray: Option<
            unsafe extern "C" fn(
                _: crate::jpeglib_h::j_common_ptr,
                _: libc::c_int,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::JDIMENSION,
            ) -> crate::jpeglib_h::JBLOCKARRAY,
        >,
        pub request_virt_sarray: Option<
            unsafe extern "C" fn(
                _: crate::jpeglib_h::j_common_ptr,
                _: libc::c_int,
                _: crate::jmorecfg_h::boolean,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::JDIMENSION,
            ) -> crate::jpeglib_h::jvirt_sarray_ptr,
        >,
        pub request_virt_barray: Option<
            unsafe extern "C" fn(
                _: crate::jpeglib_h::j_common_ptr,
                _: libc::c_int,
                _: crate::jmorecfg_h::boolean,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::JDIMENSION,
            ) -> crate::jpeglib_h::jvirt_barray_ptr,
        >,
        pub realize_virt_arrays:
            Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ()>,
        pub access_virt_sarray: Option<
            unsafe extern "C" fn(
                _: crate::jpeglib_h::j_common_ptr,
                _: crate::jpeglib_h::jvirt_sarray_ptr,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::boolean,
            ) -> crate::jpeglib_h::JSAMPARRAY,
        >,
        pub access_virt_barray: Option<
            unsafe extern "C" fn(
                _: crate::jpeglib_h::j_common_ptr,
                _: crate::jpeglib_h::jvirt_barray_ptr,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::boolean,
            ) -> crate::jpeglib_h::JBLOCKARRAY,
        >,
        pub free_pool:
            Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr, _: libc::c_int) -> ()>,
        pub self_destruct: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ()>,
        pub max_memory_to_use: libc::c_long,
        pub max_alloc_chunk: libc::c_long,
    }
    /* Master record for a compression instance */
    /* Fields shared with jpeg_decompress_struct */
    /* Destination for compressed data */
    /* Description of source image --- these fields must be filled in by
     * outer application before starting compression.  in_color_space must
     * be correct before you can even call jpeg_set_defaults().
     */
    /* input image width */
    /* input image height */
    /* # of color components in input image */
    /* colorspace of input image */
    /* image gamma of input image */
    /* Compression parameters --- these fields must be set before calling
     * jpeg_start_compress().  We recommend calling jpeg_set_defaults() to
     * initialize everything to reasonable defaults, then changing anything
     * the application specifically wants to change.  That way you won't get
     * burnt when new parameters are added.  Also note that there are several
     * helper routines to simplify changing parameters.
     */
    /* bits of precision in image data */
    /* # of color components in JPEG image */
    /* colorspace of JPEG image */
    /* comp_info[i] describes component that appears i'th in SOF */
    /* ptrs to coefficient quantization tables, or NULL if not defined,
     * and corresponding scale factors (percentage, initialized 100).
     */
    /* ptrs to Huffman coding tables, or NULL if not defined */
    /* L values for DC arith-coding tables */
    /* U values for DC arith-coding tables */
    /* Kx values for AC arith-coding tables */
    /* # of entries in scan_info array */
    /* script for multi-scan file, or NULL */
    /* The default value of scan_info is NULL, which causes a single-scan
     * sequential JPEG file to be emitted.  To create a multi-scan file,
     * set num_scans and scan_info to point to an array of scan definitions.
     */
    /* TRUE=caller supplies downsampled data */
    /* TRUE=arithmetic coding, FALSE=Huffman */
    /* TRUE=optimize entropy encoding parms */
    /* TRUE=first samples are cosited */
    /* 1..100, or 0 for no input smoothing */
    /* DCT algorithm selector */
    /* The restart interval can be specified in absolute MCUs by setting
     * restart_interval, or in MCU rows by setting restart_in_rows
     * (in which case the correct restart_interval will be figured
     * for each scan).
     */
    /* MCUs per restart, or 0 for no restart */
    /* if > 0, MCU rows per restart interval */
    /* Parameters controlling emission of special markers. */
    /* should a JFIF marker be written? */
    /* What to write for the JFIF version number */
    /* These three values are not used by the JPEG code, merely copied */
    /* into the JFIF APP0 marker.  density_unit can be 0 for unknown, */
    /* 1 for dots/inch, or 2 for dots/cm.  Note that the pixel aspect */
    /* ratio is defined by X_density/Y_density even when density_unit=0. */
    /* JFIF code for pixel size units */
    /* Horizontal pixel density */
    /* Vertical pixel density */
    /* should an Adobe marker be written? */
    /* State variable: index of next scanline to be written to
     * jpeg_write_scanlines().  Application may use this to control its
     * processing loop, e.g., "while (next_scanline < image_height)".
     */
    /* 0 .. image_height-1  */
    /* Remaining fields are known throughout compressor, but generally
     * should not be touched by a surrounding application.
     */
    /*
     * These fields are computed during compression startup
     */
    /* TRUE if scan script uses progressive mode */
    /* largest h_samp_factor */
    /* largest v_samp_factor */
    /* # of iMCU rows to be input to coef ctlr */
    /* The coefficient controller receives data in units of MCU rows as defined
     * for fully interleaved scans (whether the JPEG file is interleaved or not).
     * There are v_samp_factor * DCTSIZE sample rows of each component in an
     * "iMCU" (interleaved MCU) row.
     */
    /*
     * These fields are valid during any one scan.
     * They describe the components and MCUs actually appearing in the scan.
     */
    /* # of JPEG components in this scan */
    /* *cur_comp_info[i] describes component that appears i'th in SOS */
    /* # of MCUs across the image */
    /* # of MCU rows in the image */
    /* # of DCT blocks per MCU */
    /* MCU_membership[i] is index in cur_comp_info of component owning */
    /* i'th block in an MCU */
    /* progressive JPEG parameters for scan */
    /*
     * Links to compression subobjects (methods and private variables of modules)
     */
    /* workspace for jpeg_simple_progression */
    /* Master record for a decompression instance */
    /* Fields shared with jpeg_compress_struct */
    /* Source of compressed data */
    /* Basic description of image --- filled in by jpeg_read_header(). */
    /* Application may inspect these values to decide how to process image. */
    /* nominal image width (from SOF marker) */
    /* nominal image height */
    /* # of color components in JPEG image */
    /* colorspace of JPEG image */
    /* Decompression processing parameters --- these fields must be set before
     * calling jpeg_start_decompress().  Note that jpeg_read_header() initializes
     * them to default values.
     */
    /* colorspace for output */
    /* fraction by which to scale image */
    /* image gamma wanted in output */
    /* TRUE=multiple output passes */
    /* TRUE=downsampled data wanted */
    /* IDCT algorithm selector */
    /* TRUE=apply fancy upsampling */
    /* TRUE=apply interblock smoothing */
    /* TRUE=colormapped output wanted */
    /* the following are ignored if not quantize_colors: */
    /* type of color dithering to use */
    /* TRUE=use two-pass color quantization */
    /* max # colors to use in created colormap */
    /* these are significant only in buffered-image mode: */
    /* enable future use of 1-pass quantizer */
    /* enable future use of external colormap */
    /* enable future use of 2-pass quantizer */
    /* Description of actual output image that will be returned to application.
     * These fields are computed by jpeg_start_decompress().
     * You can also use jpeg_calc_output_dimensions() to determine these values
     * in advance of calling jpeg_start_decompress().
     */
    /* scaled image width */
    /* scaled image height */
    /* # of color components in out_color_space */
    /* # of color components returned */
    /* output_components is 1 (a colormap index) when quantizing colors;
     * otherwise it equals out_color_components.
     */
    /* min recommended height of scanline buffer */
    /* If the buffer passed to jpeg_read_scanlines() is less than this many rows
     * high, space and time will be wasted due to unnecessary data copying.
     * Usually rec_outbuf_height will be 1 or 2, at most 4.
     */
    /* When quantizing colors, the output colormap is described by these fields.
     * The application can supply a colormap by setting colormap non-NULL before
     * calling jpeg_start_decompress; otherwise a colormap is created during
     * jpeg_start_decompress or jpeg_start_output.
     * The map has out_color_components rows and actual_number_of_colors columns.
     */
    /* number of entries in use */
    /* The color map as a 2-D pixel array */
    /* State variables: these variables indicate the progress of decompression.
     * The application may examine these but must not modify them.
     */
    /* Row index of next scanline to be read from jpeg_read_scanlines().
     * Application may use this to control its processing loop, e.g.,
     * "while (output_scanline < output_height)".
     */
    /* 0 .. output_height-1  */
    /* Current input scan number and number of iMCU rows completed in scan.
     * These indicate the progress of the decompressor input side.
     */
    /* Number of SOS markers seen so far */
    /* Number of iMCU rows completed */
    /* The "output scan number" is the notional scan being displayed by the
     * output side.  The decompressor will not allow output scan/row number
     * to get ahead of input scan/row, but it can fall arbitrarily far behind.
     */
    /* Nominal scan number being displayed */
    /* Number of iMCU rows read */
    /* Current progression status.  coef_bits[c][i] indicates the precision
     * with which component c's DCT coefficient i (in zigzag order) is known.
     * It is -1 when no data has yet been received, otherwise it is the point
     * transform (shift) value for the most recent scan of the coefficient
     * (thus, 0 at completion of the progression).
     * This pointer is NULL when reading a non-progressive file.
     */
    /* -1 or current Al value for each coef */
    /* Internal JPEG parameters --- the application usually need not look at
     * these fields.  Note that the decompressor output side may not use
     * any parameters that can change between scans.
     */
    /* Quantization and Huffman tables are carried forward across input
     * datastreams when processing abbreviated JPEG datastreams.
     */
    /* ptrs to coefficient quantization tables, or NULL if not defined */
    /* ptrs to Huffman coding tables, or NULL if not defined */
    /* These parameters are never carried across datastreams, since they
     * are given in SOF/SOS markers or defined to be reset by SOI.
     */
    /* bits of precision in image data */
    /* comp_info[i] describes component that appears i'th in SOF */
    /* TRUE if SOFn specifies progressive mode */
    /* TRUE=arithmetic coding, FALSE=Huffman */
    /* L values for DC arith-coding tables */
    /* U values for DC arith-coding tables */
    /* Kx values for AC arith-coding tables */
    /* MCUs per restart interval, or 0 for no restart */
    /* These fields record data obtained from optional markers recognized by
     * the JPEG library.
     */
    /* TRUE iff a JFIF APP0 marker was found */
    /* Data copied from JFIF marker; only valid if saw_JFIF_marker is TRUE: */
    /* JFIF version number */
    /* JFIF code for pixel size units */
    /* Horizontal pixel density */
    /* Vertical pixel density */
    /* TRUE iff an Adobe APP14 marker was found */
    /* Color transform code from Adobe marker */
    /* TRUE=first samples are cosited */
    /* Aside from the specific data retained from APPn markers known to the
     * library, the uninterpreted contents of any or all APPn and COM markers
     * can be saved in a list for examination by the application.
     */
    /* Head of list of saved markers */
    /* Remaining fields are known throughout decompressor, but generally
     * should not be touched by a surrounding application.
     */
    /*
     * These fields are computed during decompression startup
     */
    /* largest h_samp_factor */
    /* largest v_samp_factor */
    /* smallest DCT_scaled_size of any component */
    /* # of iMCU rows in image */
    /* The coefficient controller's input and output progress is measured in
     * units of "iMCU" (interleaved MCU) rows.  These are the same as MCU rows
     * in fully interleaved JPEG scans, but are used whether the scan is
     * interleaved or not.  We define an iMCU row as v_samp_factor DCT block
     * rows of each component.  Therefore, the IDCT output contains
     * v_samp_factor*DCT_[v_]scaled_size sample rows of a component per iMCU row.
     */
    /* table for fast range-limiting */
    /*
     * These fields are valid during any one scan.
     * They describe the components and MCUs actually appearing in the scan.
     * Note that the decompressor output side must not use these fields.
     */
    /* # of JPEG components in this scan */
    /* *cur_comp_info[i] describes component that appears i'th in SOS */
    /* # of MCUs across the image */
    /* # of MCU rows in the image */
    /* # of DCT blocks per MCU */
    /* MCU_membership[i] is index in cur_comp_info of component owning */
    /* i'th block in an MCU */
    /* progressive JPEG parameters for scan */
    /* This field is shared between entropy decoder and marker parser.
     * It is either zero or the code of a JPEG marker that has been
     * read from the data source, but has not yet been processed.
     */
    /*
     * Links to decompression subobjects (methods, private variables of modules)
     */
    /* "Object" declarations for JPEG modules that may be supplied or called
     * directly by the surrounding application.
     * As with all objects in the JPEG library, these structs only define the
     * publicly visible methods and state variables of a module.  Additional
     * private fields may exist after the public ones.
     */
    /* Error handler object */
    /* Error exit handler: does not return to caller */
    /* Conditionally emit a trace or warning message */
    /* Routine that actually outputs a trace or error message */
    /* Format a message string for the most recent JPEG error or message */
    /* recommended size of format_message buffer */
    /* Reset error state variables at start of a new image */
    /* The message ID code and any parameters are saved here.
     * A message can have one string parameter or up to 8 int parameters.
     */
    /* Standard state variables for error facility */
    /* max msg_level that will be displayed */
    /* For recoverable corrupt-data errors, we emit a warning message,
     * but keep going unless emit_message chooses to abort.  emit_message
     * should count warnings in num_warnings.  The surrounding application
     * can check for bad data by seeing if num_warnings is nonzero at the
     * end of processing.
     */
    /* number of corrupt-data warnings */
    /* These fields point to the table(s) of error message strings.
     * An application can change the table pointer to switch to a different
     * message list (typically, to change the language in which errors are
     * reported).  Some applications may wish to add additional error codes
     * that will be handled by the JPEG library error mechanism; the second
     * table pointer is used for this purpose.
     *
     * First table includes all errors generated by JPEG library itself.
     * Error code 0 is reserved for a "no such error string" message.
     */
    /* Library errors */
    /* Table contains strings 0..last_jpeg_message */
    /* Second table can be added by application (see cjpeg/djpeg for example).
     * It contains strings numbered first_addon_message..last_addon_message.
     */
    /* Non-library errors */
    /* code for first string in addon table */
    /* code for last string in addon table */
    /* Progress monitor object */
    /* work units completed in this pass */
    /* total number of work units in this pass */
    /* passes completed so far */
    /* total number of passes expected */
    /* Data destination object for compression */
    /* => next byte to write in buffer */
    /* # of byte spaces remaining in buffer */
    /* Data source object for decompression */
    /* => next byte to read from buffer */
    /* # of bytes remaining in buffer */
    /* Memory manager object.
     * Allocates "small" objects (a few K total), "large" objects (tens of K),
     * and "really big" objects (virtual arrays with backing store if needed).
     * The memory manager does not allow individual objects to be freed; rather,
     * each created object is assigned to a pool, and whole pools can be freed
     * at once.  This is faster and more convenient than remembering exactly what
     * to free, especially where malloc()/free() are not too speedy.
     * NB: alloc routines never return NULL.  They exit to error_exit if not
     * successful.
     */
    /* lasts until master record is destroyed */
    /* lasts until done with image/datastream */

    pub type jvirt_barray_ptr = *mut crate::jpeglib_h::jvirt_barray_control;

    pub type jvirt_sarray_ptr = *mut crate::jpeglib_h::jvirt_sarray_control;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct jpeg_error_mgr {
        pub error_exit: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ()>,
        pub emit_message:
            Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr, _: libc::c_int) -> ()>,
        pub output_message: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ()>,
        pub format_message: Option<
            unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr, _: *mut libc::c_char) -> (),
        >,
        pub reset_error_mgr: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ()>,
        pub msg_code: libc::c_int,
        pub msg_parm: crate::jpeglib_h::C2RustUnnamed_0,
        pub trace_level: libc::c_int,
        pub num_warnings: libc::c_long,
        pub jpeg_message_table: *const *const libc::c_char,
        pub last_jpeg_message: libc::c_int,
        pub addon_message_table: *const *const libc::c_char,
        pub first_addon_message: libc::c_int,
        pub last_addon_message: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct jpeg_decompress_struct {
        pub err: *mut crate::jpeglib_h::jpeg_error_mgr,
        pub mem: *mut crate::jpeglib_h::jpeg_memory_mgr,
        pub progress: *mut crate::jpeglib_h::jpeg_progress_mgr,
        pub client_data: *mut libc::c_void,
        pub is_decompressor: crate::jmorecfg_h::boolean,
        pub global_state: libc::c_int,
        pub src: *mut crate::jpeglib_h::jpeg_source_mgr,
        pub image_width: crate::jmorecfg_h::JDIMENSION,
        pub image_height: crate::jmorecfg_h::JDIMENSION,
        pub num_components: libc::c_int,
        pub jpeg_color_space: crate::jpeglib_h::J_COLOR_SPACE,
        pub out_color_space: crate::jpeglib_h::J_COLOR_SPACE,
        pub scale_num: libc::c_uint,
        pub scale_denom: libc::c_uint,
        pub output_gamma: libc::c_double,
        pub buffered_image: crate::jmorecfg_h::boolean,
        pub raw_data_out: crate::jmorecfg_h::boolean,
        pub dct_method: crate::jpeglib_h::J_DCT_METHOD,
        pub do_fancy_upsampling: crate::jmorecfg_h::boolean,
        pub do_block_smoothing: crate::jmorecfg_h::boolean,
        pub quantize_colors: crate::jmorecfg_h::boolean,
        pub dither_mode: crate::jpeglib_h::J_DITHER_MODE,
        pub two_pass_quantize: crate::jmorecfg_h::boolean,
        pub desired_number_of_colors: libc::c_int,
        pub enable_1pass_quant: crate::jmorecfg_h::boolean,
        pub enable_external_quant: crate::jmorecfg_h::boolean,
        pub enable_2pass_quant: crate::jmorecfg_h::boolean,
        pub output_width: crate::jmorecfg_h::JDIMENSION,
        pub output_height: crate::jmorecfg_h::JDIMENSION,
        pub out_color_components: libc::c_int,
        pub output_components: libc::c_int,
        pub rec_outbuf_height: libc::c_int,
        pub actual_number_of_colors: libc::c_int,
        pub colormap: crate::jpeglib_h::JSAMPARRAY,
        pub output_scanline: crate::jmorecfg_h::JDIMENSION,
        pub input_scan_number: libc::c_int,
        pub input_iMCU_row: crate::jmorecfg_h::JDIMENSION,
        pub output_scan_number: libc::c_int,
        pub output_iMCU_row: crate::jmorecfg_h::JDIMENSION,
        pub coef_bits: *mut [libc::c_int; 64],
        pub quant_tbl_ptrs: [*mut crate::jpeglib_h::JQUANT_TBL; 4],
        pub dc_huff_tbl_ptrs: [*mut crate::jpeglib_h::JHUFF_TBL; 4],
        pub ac_huff_tbl_ptrs: [*mut crate::jpeglib_h::JHUFF_TBL; 4],
        pub data_precision: libc::c_int,
        pub comp_info: *mut crate::jpeglib_h::jpeg_component_info,
        pub progressive_mode: crate::jmorecfg_h::boolean,
        pub arith_code: crate::jmorecfg_h::boolean,
        pub arith_dc_L: [crate::jmorecfg_h::UINT8; 16],
        pub arith_dc_U: [crate::jmorecfg_h::UINT8; 16],
        pub arith_ac_K: [crate::jmorecfg_h::UINT8; 16],
        pub restart_interval: libc::c_uint,
        pub saw_JFIF_marker: crate::jmorecfg_h::boolean,
        pub JFIF_major_version: crate::jmorecfg_h::UINT8,
        pub JFIF_minor_version: crate::jmorecfg_h::UINT8,
        pub density_unit: crate::jmorecfg_h::UINT8,
        pub X_density: crate::jmorecfg_h::UINT16,
        pub Y_density: crate::jmorecfg_h::UINT16,
        pub saw_Adobe_marker: crate::jmorecfg_h::boolean,
        pub Adobe_transform: crate::jmorecfg_h::UINT8,
        pub CCIR601_sampling: crate::jmorecfg_h::boolean,
        pub marker_list: crate::jpeglib_h::jpeg_saved_marker_ptr,
        pub max_h_samp_factor: libc::c_int,
        pub max_v_samp_factor: libc::c_int,
        pub min_DCT_scaled_size: libc::c_int,
        pub total_iMCU_rows: crate::jmorecfg_h::JDIMENSION,
        pub sample_range_limit: *mut crate::jmorecfg_h::JSAMPLE,
        pub comps_in_scan: libc::c_int,
        pub cur_comp_info: [*mut crate::jpeglib_h::jpeg_component_info; 4],
        pub MCUs_per_row: crate::jmorecfg_h::JDIMENSION,
        pub MCU_rows_in_scan: crate::jmorecfg_h::JDIMENSION,
        pub blocks_in_MCU: libc::c_int,
        pub MCU_membership: [libc::c_int; 10],
        pub Ss: libc::c_int,
        pub Se: libc::c_int,
        pub Ah: libc::c_int,
        pub Al: libc::c_int,
        pub unread_marker: libc::c_int,
        pub master: *mut crate::jpeglib_h::jpeg_decomp_master,
        pub main: *mut crate::jpeglib_h::jpeg_d_main_controller,
        pub coef: *mut crate::jpeglib_h::jpeg_d_coef_controller,
        pub post: *mut crate::jpeglib_h::jpeg_d_post_controller,
        pub inputctl: *mut crate::jpeglib_h::jpeg_input_controller,
        pub marker: *mut crate::jpeglib_h::jpeg_marker_reader,
        pub entropy: *mut crate::jpeglib_h::jpeg_entropy_decoder,
        pub idct: *mut crate::jpeglib_h::jpeg_inverse_dct,
        pub upsample: *mut crate::jpeglib_h::jpeg_upsampler,
        pub cconvert: *mut crate::jpeglib_h::jpeg_color_deconverter,
        pub cquantize: *mut crate::jpeglib_h::jpeg_color_quantizer,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct jpeg_source_mgr {
        pub next_input_byte: *const crate::jmorecfg_h::JOCTET,
        pub bytes_in_buffer: crate::stddef_h::size_t,
        pub init_source: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
        pub fill_input_buffer: Option<
            unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
            ) -> crate::jmorecfg_h::boolean,
        >,
        pub skip_input_data: Option<
            unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr, _: libc::c_long) -> (),
        >,
        pub resync_to_restart: Option<
            unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: libc::c_int,
            ) -> crate::jmorecfg_h::boolean,
        >,
        pub term_source: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    }

    pub type j_decompress_ptr = *mut crate::jpeglib_h::jpeg_decompress_struct;
    /* Routine signature for application-supplied marker processing methods.
     * Need not pass marker code since it is stored in cinfo->unread_marker.
     */

    pub type jpeg_marker_parser_method = Option<
        unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> crate::jmorecfg_h::boolean,
    >;
    /* 5-bit red/6-bit green/5-bit blue */
    /* alpha/red/green/blue */

    pub const JCS_RGB565: crate::jpeglib_h::J_COLOR_SPACE = 16;
    /* alpha/blue/green/red */

    pub const JCS_EXT_ARGB: crate::jpeglib_h::J_COLOR_SPACE = 15;
    /* blue/green/red/alpha */

    pub const JCS_EXT_ABGR: crate::jpeglib_h::J_COLOR_SPACE = 14;
    /* red/green/blue/alpha */

    pub const JCS_EXT_BGRA: crate::jpeglib_h::J_COLOR_SPACE = 13;
    pub const JCS_EXT_RGBA: crate::jpeglib_h::J_COLOR_SPACE = 12;
    /* x/blue/green/red */

    pub const JCS_EXT_XRGB: crate::jpeglib_h::J_COLOR_SPACE = 11;
    /* blue/green/red/x */

    pub const JCS_EXT_XBGR: crate::jpeglib_h::J_COLOR_SPACE = 10;
    /* blue/green/red */

    pub const JCS_EXT_BGRX: crate::jpeglib_h::J_COLOR_SPACE = 9;
    /* red/green/blue/x */

    pub const JCS_EXT_BGR: crate::jpeglib_h::J_COLOR_SPACE = 8;
    /* red/green/blue */

    pub const JCS_EXT_RGBX: crate::jpeglib_h::J_COLOR_SPACE = 7;
    /* Y/Cb/Cr/K */

    pub const JCS_EXT_RGB: crate::jpeglib_h::J_COLOR_SPACE = 6;
    /* C/M/Y/K */

    pub const JCS_YCCK: crate::jpeglib_h::J_COLOR_SPACE = 5;
    /* Y/Cb/Cr (also known as YUV) */

    pub const JCS_CMYK: crate::jpeglib_h::J_COLOR_SPACE = 4;
    pub const JCS_YCbCr: crate::jpeglib_h::J_COLOR_SPACE = 3;
    /* monochrome */

    pub const JCS_RGB: crate::jpeglib_h::J_COLOR_SPACE = 2;
    /* error/unspecified */

    pub const JCS_GRAYSCALE: crate::jpeglib_h::J_COLOR_SPACE = 1;

    pub const JCS_UNKNOWN: crate::jpeglib_h::J_COLOR_SPACE = 0;
    /* floating-point: accurate, fast on fast HW */
    /* faster, less accurate integer method */

    pub const JDCT_FLOAT: crate::jpeglib_h::J_DCT_METHOD = 2;
    /* slow but accurate integer algorithm */

    pub const JDCT_IFAST: crate::jpeglib_h::J_DCT_METHOD = 1;

    pub const JDCT_ISLOW: crate::jpeglib_h::J_DCT_METHOD = 0;
    /* Floyd-Steinberg error diffusion dither */
    /* simple ordered dither */

    pub const JDITHER_FS: crate::jpeglib_h::J_DITHER_MODE = 2;
    /* no dithering */

    pub const JDITHER_ORDERED: crate::jpeglib_h::J_DITHER_MODE = 1;

    pub const JDITHER_NONE: crate::jpeglib_h::J_DITHER_MODE = 0;
    /* These marker codes are exported since applications and data source modules
     * are likely to want to use them.
     */
    /* RST0 marker code */
    /* EOI marker code */
    /* APP0 marker code */

    pub const JPEG_COM: libc::c_int = 0xfe as libc::c_int;

    pub const JDCT_FASTEST: libc::c_int = crate::jpeglib_h::JDCT_IFAST as libc::c_int;

    pub const JPEG_APP0: libc::c_int = 0xe0 as libc::c_int;

    pub const JDCT_DEFAULT: libc::c_int = crate::jpeglib_h::JDCT_ISLOW as libc::c_int;
}
pub mod jmorecfg_h {
    pub type JSAMPLE = libc::c_uchar;
    /* not HAVE_UNSIGNED_CHAR */
    /* HAVE_UNSIGNED_CHAR */
    /* BITS_IN_JSAMPLE == 8 */
    /* BITS_IN_JSAMPLE == 12 */
    /* Representation of a DCT frequency coefficient.
     * This should be a signed value of at least 16 bits; "short" is usually OK.
     * Again, we allocate large arrays of these, but you can change to int
     * if you have memory to burn and "short" is really slow.
     */

    pub type JCOEF = libc::c_short;
    /* Compressed datastreams are represented as arrays of JOCTET.
     * These must be EXACTLY 8 bits wide, at least once they are written to
     * external storage.  Note that when using the stdio data source/destination
     * managers, this is also the data type passed to fread/fwrite.
     */

    pub type JOCTET = libc::c_uchar;
    /* not HAVE_UNSIGNED_CHAR */
    /* HAVE_UNSIGNED_CHAR */
    /* These typedefs are used for various table entries and so forth.
     * They must be at least as wide as specified; but making them too big
     * won't cost a huge amount of memory, so we don't provide special
     * extraction code like we did for JSAMPLE.  (In other words, these
     * typedefs live at a different point on the speed/space tradeoff curve.)
     */
    /* UINT8 must hold at least the values 0..255. */

    pub type UINT8 = libc::c_uchar;
    /* not HAVE_UNSIGNED_CHAR */
    /* HAVE_UNSIGNED_CHAR */
    /* UINT16 must hold at least the values 0..65535. */

    pub type UINT16 = libc::c_ushort;
    /* Datatype used for image dimensions.  The JPEG standard only supports
     * images up to 64K*64K due to 16-bit fields in SOF markers.  Therefore
     * "unsigned int" is sufficient on all machines.  However, if you need to
     * handle larger images and you don't mind deviating from the spec, you
     * can change this datatype.  (Note that changing this datatype will
     * potentially require modifying the SIMD code.  The x86-64 SIMD extensions,
     * in particular, assume a 32-bit JDIMENSION.)
     */

    pub type JDIMENSION = libc::c_uint;
    /* a tad under 64K to prevent overflows */
    /* These macros are used in all function definitions and extern declarations.
     * You could modify them if you need to change function linkage conventions;
     * in particular, you'll need to do that to make the library a Windows DLL.
     * Another application is to make all functions global for use with debuggers
     * or code profilers that require it.
     */
    /* a function called through method pointers: */
    /* a function used only in its module: */
    /* a function referenced thru EXTERNs: */
    /* a reference to a GLOBAL function: */
    /* Originally, this macro was used as a way of defining function prototypes
     * for both modern compilers as well as older compilers that did not support
     * prototype parameters.  libjpeg-turbo has never supported these older,
     * non-ANSI compilers, but the macro is still included because there is some
     * software out there that uses it.
     */
    /* libjpeg-turbo no longer supports platforms that have far symbols (MS-DOS),
     * but again, some software relies on this macro.
     */
    /*
     * On a few systems, type boolean and/or its values FALSE, TRUE may appear
     * in standard header files.  Or you may have conflicts with application-
     * specific header files that you want to include together with these files.
     * Defining HAVE_BOOLEAN before including jpeglib.h should make it work.
     */

    pub type boolean = libc::c_int;

    pub const FALSE: libc::c_int = 0 as libc::c_int;

    pub const TRUE: libc::c_int = 1 as libc::c_int;
}
pub mod stddef_h {
    pub type size_t = libc::c_ulong;

    pub const NULL: libc::c_int = 0 as libc::c_int;
}
pub mod stdlib {
    extern "C" {
        #[no_mangle]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
        #[no_mangle]
        pub static mut stdin: *mut crate::stdlib::FILE;

        #[no_mangle]
        pub static mut stdout: *mut crate::stdlib::FILE;

        #[no_mangle]
        pub static mut stderr: *mut crate::stdlib::FILE;

        #[no_mangle]
        pub fn fclose(__stream: *mut crate::stdlib::FILE) -> libc::c_int;

        #[no_mangle]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut crate::stdlib::FILE;

        #[no_mangle]
        pub fn fprintf(_: *mut crate::stdlib::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;

        #[no_mangle]
        pub fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;

        #[no_mangle]
        pub fn putc(__c: libc::c_int, __stream: *mut crate::stdlib::FILE) -> libc::c_int;

        #[no_mangle]
        pub fn fread(
            _: *mut libc::c_void,
            _: libc::c_ulong,
            _: libc::c_ulong,
            _: *mut crate::stdlib::FILE,
        ) -> libc::c_ulong;

        #[no_mangle]
        pub fn fwrite(
            _: *const libc::c_void,
            _: libc::c_ulong,
            _: libc::c_ulong,
            _: *mut crate::stdlib::FILE,
        ) -> libc::c_ulong;

        #[no_mangle]
        pub fn ferror(__stream: *mut crate::stdlib::FILE) -> libc::c_int;
        #[no_mangle]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

        #[no_mangle]
        pub fn free(__ptr: *mut libc::c_void);

        #[no_mangle]
        pub fn exit(_: libc::c_int) -> !;
        pub type _IO_wide_data;

        pub type _IO_codecvt;

        pub type _IO_marker;
    }
    pub type FILE = crate::stdlib::_IO_FILE;
    pub const _ISalnum: crate::jerror_h::C2RustUnnamed_1 = 8;

    pub const _ISpunct: crate::jerror_h::C2RustUnnamed_1 = 4;

    pub const _IScntrl: crate::jerror_h::C2RustUnnamed_1 = 2;

    pub const _ISblank: crate::jerror_h::C2RustUnnamed_1 = 1;

    pub const _ISgraph: crate::jerror_h::C2RustUnnamed_1 = 32768;

    pub const _ISprint: crate::jerror_h::C2RustUnnamed_1 = 16384;

    pub const _ISspace: crate::jerror_h::C2RustUnnamed_1 = 8192;

    pub const _ISxdigit: crate::jerror_h::C2RustUnnamed_1 = 4096;

    pub const _ISdigit: crate::jerror_h::C2RustUnnamed_1 = 2048;

    pub const _ISalpha: crate::jerror_h::C2RustUnnamed_1 = 1024;

    pub const _ISlower: crate::jerror_h::C2RustUnnamed_1 = 512;

    pub const _ISupper: crate::jerror_h::C2RustUnnamed_1 = 256;
    pub const EXIT_SUCCESS: libc::c_int = 0 as libc::c_int;

    pub const EXIT_FAILURE: libc::c_int = 1 as libc::c_int;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut crate::stdlib::_IO_marker,
        pub _chain: *mut crate::stdlib::_IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: crate::stdlib::__off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: crate::stdlib::__off64_t,
        pub _codecvt: *mut crate::stdlib::_IO_codecvt,
        pub _wide_data: *mut crate::stdlib::_IO_wide_data,
        pub _freeres_list: *mut crate::stdlib::_IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: crate::stddef_h::size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }

    pub type _IO_lock_t = ();
    pub type __off_t = libc::c_long;

    pub type __off64_t = libc::c_long;
}
use ::mozjpeg::*;

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

pub use crate::cderror_h::JERR_BAD_CMAP_FILE;
pub use crate::cderror_h::JERR_BMP_BADCMAP;
pub use crate::cderror_h::JERR_BMP_BADDEPTH;
pub use crate::cderror_h::JERR_BMP_BADHEADER;
pub use crate::cderror_h::JERR_BMP_BADPLANES;
pub use crate::cderror_h::JERR_BMP_COLORSPACE;
pub use crate::cderror_h::JERR_BMP_COMPRESSED;
pub use crate::cderror_h::JERR_BMP_EMPTY;
pub use crate::cderror_h::JERR_BMP_NOT;
pub use crate::cderror_h::JERR_BMP_OUTOFRANGE;
pub use crate::cderror_h::JERR_GIF_BUG;
pub use crate::cderror_h::JERR_GIF_CODESIZE;
pub use crate::cderror_h::JERR_GIF_COLORSPACE;
pub use crate::cderror_h::JERR_GIF_IMAGENOTFOUND;
pub use crate::cderror_h::JERR_GIF_NOT;
pub use crate::cderror_h::JERR_PPM_COLORSPACE;
pub use crate::cderror_h::JERR_PPM_NONNUMERIC;
pub use crate::cderror_h::JERR_PPM_NOT;
pub use crate::cderror_h::JERR_PPM_OUTOFRANGE;
pub use crate::cderror_h::JERR_TGA_BADCMAP;
pub use crate::cderror_h::JERR_TGA_BADPARMS;
pub use crate::cderror_h::JERR_TGA_COLORSPACE;
pub use crate::cderror_h::JERR_TOO_MANY_COLORS;
pub use crate::cderror_h::JERR_UNGETC_FAILED;
pub use crate::cderror_h::JERR_UNKNOWN_FORMAT;
pub use crate::cderror_h::JERR_UNSUPPORTED_FORMAT;
pub use crate::cderror_h::JMSG_FIRSTADDONCODE;
pub use crate::cderror_h::JMSG_LASTADDONCODE;
pub use crate::cderror_h::JTRC_BMP;
pub use crate::cderror_h::JTRC_BMP_MAPPED;
pub use crate::cderror_h::JTRC_BMP_OS2;
pub use crate::cderror_h::JTRC_BMP_OS2_MAPPED;
pub use crate::cderror_h::JTRC_GIF;
pub use crate::cderror_h::JTRC_GIF_BADVERSION;
pub use crate::cderror_h::JTRC_GIF_EXTENSION;
pub use crate::cderror_h::JTRC_GIF_NONSQUARE;
pub use crate::cderror_h::JTRC_PGM;
pub use crate::cderror_h::JTRC_PGM_TEXT;
pub use crate::cderror_h::JTRC_PPM;
pub use crate::cderror_h::JTRC_PPM_TEXT;
pub use crate::cderror_h::JTRC_TGA;
pub use crate::cderror_h::JTRC_TGA_GRAY;
pub use crate::cderror_h::JTRC_TGA_MAPPED;
pub use crate::cderror_h::JWRN_GIF_BADDATA;
pub use crate::cderror_h::JWRN_GIF_CHAR;
pub use crate::cderror_h::JWRN_GIF_ENDCODE;
pub use crate::cderror_h::JWRN_GIF_NOMOREDATA;
pub use crate::cdjpeg_h::djpeg_dest_ptr;
pub use crate::cdjpeg_h::djpeg_dest_struct;
pub use crate::cdjpeg_h::jinit_write_bmp;
pub use crate::cdjpeg_h::jinit_write_gif;
pub use crate::cdjpeg_h::jinit_write_ppm;
pub use crate::cdjpeg_h::jinit_write_targa;
pub use crate::cdjpeg_h::keymatch;
pub use crate::cdjpeg_h::read_color_map;
pub use crate::cdjpeg_h::read_stdin;
pub use crate::cdjpeg_h::write_stdout;
pub use crate::cdjpeg_h::EXIT_WARNING;
pub use crate::cdjpeg_h::READ_BINARY;
pub use crate::cdjpeg_h::WRITE_BINARY;
pub use crate::jconfig_h::JPEG_LIB_VERSION;
pub use crate::jerror_h::C2RustUnnamed_1;
pub use crate::jerror_h::JERR_ARITH_NOTIMPL;
pub use crate::jerror_h::JERR_BAD_ALIGN_TYPE;
pub use crate::jerror_h::JERR_BAD_ALLOC_CHUNK;
pub use crate::jerror_h::JERR_BAD_BUFFER_MODE;
pub use crate::jerror_h::JERR_BAD_COMPONENT_ID;
pub use crate::jerror_h::JERR_BAD_CROP_SPEC;
pub use crate::jerror_h::JERR_BAD_DCTSIZE;
pub use crate::jerror_h::JERR_BAD_DCT_COEF;
pub use crate::jerror_h::JERR_BAD_HUFF_TABLE;
pub use crate::jerror_h::JERR_BAD_IN_COLORSPACE;
pub use crate::jerror_h::JERR_BAD_J_COLORSPACE;
pub use crate::jerror_h::JERR_BAD_LENGTH;
pub use crate::jerror_h::JERR_BAD_LIB_VERSION;
pub use crate::jerror_h::JERR_BAD_MCU_SIZE;
pub use crate::jerror_h::JERR_BAD_PARAM;
pub use crate::jerror_h::JERR_BAD_PARAM_VALUE;
pub use crate::jerror_h::JERR_BAD_POOL_ID;
pub use crate::jerror_h::JERR_BAD_PRECISION;
pub use crate::jerror_h::JERR_BAD_PROGRESSION;
pub use crate::jerror_h::JERR_BAD_PROG_SCRIPT;
pub use crate::jerror_h::JERR_BAD_SAMPLING;
pub use crate::jerror_h::JERR_BAD_SCAN_SCRIPT;
pub use crate::jerror_h::JERR_BAD_STATE;
pub use crate::jerror_h::JERR_BAD_STRUCT_SIZE;
pub use crate::jerror_h::JERR_BAD_VIRTUAL_ACCESS;
pub use crate::jerror_h::JERR_BUFFER_SIZE;
pub use crate::jerror_h::JERR_CANT_SUSPEND;
pub use crate::jerror_h::JERR_CCIR601_NOTIMPL;
pub use crate::jerror_h::JERR_COMPONENT_COUNT;
pub use crate::jerror_h::JERR_CONVERSION_NOTIMPL;
pub use crate::jerror_h::JERR_DAC_INDEX;
pub use crate::jerror_h::JERR_DAC_VALUE;
pub use crate::jerror_h::JERR_DHT_INDEX;
pub use crate::jerror_h::JERR_DQT_INDEX;
pub use crate::jerror_h::JERR_EMPTY_IMAGE;
pub use crate::jerror_h::JERR_EMS_READ;
pub use crate::jerror_h::JERR_EMS_WRITE;
pub use crate::jerror_h::JERR_EOI_EXPECTED;
pub use crate::jerror_h::JERR_FILE_READ;
pub use crate::jerror_h::JERR_FILE_WRITE;
pub use crate::jerror_h::JERR_FRACT_SAMPLE_NOTIMPL;
pub use crate::jerror_h::JERR_HUFF_CLEN_OVERFLOW;
pub use crate::jerror_h::JERR_HUFF_MISSING_CODE;
pub use crate::jerror_h::JERR_IMAGE_TOO_BIG;
pub use crate::jerror_h::JERR_INPUT_EMPTY;
pub use crate::jerror_h::JERR_INPUT_EOF;
pub use crate::jerror_h::JERR_MISMATCHED_QUANT_TABLE;
pub use crate::jerror_h::JERR_MISSING_DATA;
pub use crate::jerror_h::JERR_MODE_CHANGE;
pub use crate::jerror_h::JERR_NOTIMPL;
pub use crate::jerror_h::JERR_NOT_COMPILED;
pub use crate::jerror_h::JERR_NO_BACKING_STORE;
pub use crate::jerror_h::JERR_NO_HUFF_TABLE;
pub use crate::jerror_h::JERR_NO_IMAGE;
pub use crate::jerror_h::JERR_NO_QUANT_TABLE;
pub use crate::jerror_h::JERR_NO_SOI;
pub use crate::jerror_h::JERR_OUT_OF_MEMORY;
pub use crate::jerror_h::JERR_QUANT_COMPONENTS;
pub use crate::jerror_h::JERR_QUANT_FEW_COLORS;
pub use crate::jerror_h::JERR_QUANT_MANY_COLORS;
pub use crate::jerror_h::JERR_SOF_DUPLICATE;
pub use crate::jerror_h::JERR_SOF_NO_SOS;
pub use crate::jerror_h::JERR_SOF_UNSUPPORTED;
pub use crate::jerror_h::JERR_SOI_DUPLICATE;
pub use crate::jerror_h::JERR_SOS_NO_SOF;
pub use crate::jerror_h::JERR_TFILE_CREATE;
pub use crate::jerror_h::JERR_TFILE_READ;
pub use crate::jerror_h::JERR_TFILE_SEEK;
pub use crate::jerror_h::JERR_TFILE_WRITE;
pub use crate::jerror_h::JERR_TOO_LITTLE_DATA;
pub use crate::jerror_h::JERR_UNKNOWN_MARKER;
pub use crate::jerror_h::JERR_UNSUPPORTED_SUSPEND;
pub use crate::jerror_h::JERR_VIRTUAL_BUG;
pub use crate::jerror_h::JERR_WIDTH_OVERFLOW;
pub use crate::jerror_h::JERR_XMS_READ;
pub use crate::jerror_h::JERR_XMS_WRITE;
pub use crate::jerror_h::JMSG_COPYRIGHT;
pub use crate::jerror_h::JMSG_LASTMSGCODE;
pub use crate::jerror_h::JMSG_NOMESSAGE;
pub use crate::jerror_h::JMSG_VERSION;
pub use crate::jerror_h::JTRC_16BIT_TABLES;
pub use crate::jerror_h::JTRC_ADOBE;
pub use crate::jerror_h::JTRC_APP0;
pub use crate::jerror_h::JTRC_APP14;
pub use crate::jerror_h::JTRC_DAC;
pub use crate::jerror_h::JTRC_DHT;
pub use crate::jerror_h::JTRC_DQT;
pub use crate::jerror_h::JTRC_DRI;
pub use crate::jerror_h::JTRC_EMS_CLOSE;
pub use crate::jerror_h::JTRC_EMS_OPEN;
pub use crate::jerror_h::JTRC_EOI;
pub use crate::jerror_h::JTRC_HUFFBITS;
pub use crate::jerror_h::JTRC_JFIF;
pub use crate::jerror_h::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use crate::jerror_h::JTRC_JFIF_EXTENSION;
pub use crate::jerror_h::JTRC_JFIF_THUMBNAIL;
pub use crate::jerror_h::JTRC_MISC_MARKER;
pub use crate::jerror_h::JTRC_PARMLESS_MARKER;
pub use crate::jerror_h::JTRC_QUANTVALS;
pub use crate::jerror_h::JTRC_QUANT_3_NCOLORS;
pub use crate::jerror_h::JTRC_QUANT_NCOLORS;
pub use crate::jerror_h::JTRC_QUANT_SELECTED;
pub use crate::jerror_h::JTRC_RECOVERY_ACTION;
pub use crate::jerror_h::JTRC_RST;
pub use crate::jerror_h::JTRC_SMOOTH_NOTIMPL;
pub use crate::jerror_h::JTRC_SOF;
pub use crate::jerror_h::JTRC_SOF_COMPONENT;
pub use crate::jerror_h::JTRC_SOI;
pub use crate::jerror_h::JTRC_SOS;
pub use crate::jerror_h::JTRC_SOS_COMPONENT;
pub use crate::jerror_h::JTRC_SOS_PARAMS;
pub use crate::jerror_h::JTRC_TFILE_CLOSE;
pub use crate::jerror_h::JTRC_TFILE_OPEN;
pub use crate::jerror_h::JTRC_THUMB_JPEG;
pub use crate::jerror_h::JTRC_THUMB_PALETTE;
pub use crate::jerror_h::JTRC_THUMB_RGB;
pub use crate::jerror_h::JTRC_UNKNOWN_IDS;
pub use crate::jerror_h::JTRC_XMS_CLOSE;
pub use crate::jerror_h::JTRC_XMS_OPEN;
pub use crate::jerror_h::JWRN_ADOBE_XFORM;
pub use crate::jerror_h::JWRN_BOGUS_ICC;
pub use crate::jerror_h::JWRN_BOGUS_PROGRESSION;
pub use crate::jerror_h::JWRN_EXTRANEOUS_DATA;
pub use crate::jerror_h::JWRN_HIT_MARKER;
pub use crate::jerror_h::JWRN_HUFF_BAD_CODE;
pub use crate::jerror_h::JWRN_JFIF_MAJOR;
pub use crate::jerror_h::JWRN_JPEG_EOF;
pub use crate::jerror_h::JWRN_MUST_RESYNC;
pub use crate::jerror_h::JWRN_NOT_SEQUENTIAL;
pub use crate::jerror_h::JWRN_TOO_MUCH_DATA;
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
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_CreateDecompress;
pub use crate::jpeglib_h::jpeg_color_deconverter;
pub use crate::jpeglib_h::jpeg_color_quantizer;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_crop_scanline;
pub use crate::jpeglib_h::jpeg_d_coef_controller;
pub use crate::jpeglib_h::jpeg_d_main_controller;
pub use crate::jpeglib_h::jpeg_d_post_controller;
pub use crate::jpeglib_h::jpeg_decomp_master;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_destroy_decompress;
pub use crate::jpeglib_h::jpeg_entropy_decoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_finish_decompress;
pub use crate::jpeglib_h::jpeg_input_controller;
pub use crate::jpeglib_h::jpeg_inverse_dct;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_reader;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_mem_src;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_read_header;
pub use crate::jpeglib_h::jpeg_read_icc_profile;
pub use crate::jpeglib_h::jpeg_read_scanlines;
pub use crate::jpeglib_h::jpeg_save_markers;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_set_marker_processor;
pub use crate::jpeglib_h::jpeg_skip_scanlines;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jpeg_start_decompress;
pub use crate::jpeglib_h::jpeg_std_error;
pub use crate::jpeglib_h::jpeg_stdio_src;
pub use crate::jpeglib_h::jpeg_upsampler;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_0;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
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
pub use crate::jpeglib_h::JDCT_DEFAULT;
pub use crate::jpeglib_h::JDCT_FASTEST;
pub use crate::jpeglib_h::JDCT_FLOAT;
pub use crate::jpeglib_h::JDCT_IFAST;
pub use crate::jpeglib_h::JDCT_ISLOW;
pub use crate::jpeglib_h::JDITHER_FS;
pub use crate::jpeglib_h::JDITHER_NONE;
pub use crate::jpeglib_h::JDITHER_ORDERED;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JPEG_APP0;
pub use crate::jpeglib_h::JPEG_COM;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
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
pub use crate::stdlib::exit;
use crate::stdlib::fclose;
use crate::stdlib::ferror;
use crate::stdlib::fopen;
use crate::stdlib::fprintf;
use crate::stdlib::fread;
pub use crate::stdlib::free;
use crate::stdlib::fwrite;
use crate::stdlib::putc;
pub use crate::stdlib::realloc;
use crate::stdlib::sscanf;
use crate::stdlib::stderr;
use crate::stdlib::stdin;
use crate::stdlib::stdout;
pub use crate::stdlib::EXIT_FAILURE;
pub use crate::stdlib::EXIT_SUCCESS;

pub use crate::jconfigint_h::BUILD;
pub use crate::jconfigint_h::PACKAGE_NAME;
pub use crate::jconfigint_h::VERSION;
pub use crate::jversion_h::JCOPYRIGHT;
pub use crate::jversion_h::JVERSION;

pub type IMAGE_FORMATS = libc::c_uint;

pub const FMT_TIFF: IMAGE_FORMATS = 6;

pub const FMT_TARGA: IMAGE_FORMATS = 5;

pub const FMT_RLE: IMAGE_FORMATS = 4;

pub const FMT_PPM: IMAGE_FORMATS = 3;

pub const FMT_OS2: IMAGE_FORMATS = 2;

pub const FMT_GIF: IMAGE_FORMATS = 1;

pub const FMT_BMP: IMAGE_FORMATS = 0;
/*
 * djpeg.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * Modified 2013 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2010-2011, 2013-2017, D. R. Commander.
 * Copyright (C) 2015, Google, Inc.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains a command-line user interface for the JPEG decompressor.
 * It should work on any system with Unix- or MS-DOS-style command lines.
 *
 * Two different command line styles are permitted, depending on the
 * compile-time switch TWO_FILE_COMMANDLINE:
 *      djpeg [options]  inputfile outputfile
 *      djpeg [options]  [inputfile]
 * In the second style, output is always to standard output, which you'd
 * normally redirect to a file or pipe to some other program.  Input is
 * either from a named file or from standard input (typically redirected).
 * The second style is convenient on Unix but is unhelpful on systems that
 * don't support pipes.  Also, you MUST use the first style if your system
 * doesn't do binary I/O to stdin/stdout.
 * To simplify script writing, the "-outfile" switch is provided.  The syntax
 *      djpeg [options]  -outfile outputfile  inputfile
 * works regardless of which command line style is used.
 */
/* <stdlib.h> should declare free() */
/* to declare isprint() */
/* command-line reader for Macintosh */
/* Create the add-on message string table. */

static mut cdjpeg_message_table: [*const libc::c_char; 47] = [
    0 as *const libc::c_char,
    b"Unsupported BMP colormap format\x00" as *const u8 as *const libc::c_char,
    b"Only 8- and 24-bit BMP files are supported\x00" as *const u8 as *const libc::c_char,
    b"Invalid BMP file: bad header length\x00" as *const u8 as *const libc::c_char,
    b"Invalid BMP file: biPlanes not equal to 1\x00" as *const u8 as *const libc::c_char,
    b"BMP output must be grayscale or RGB\x00" as *const u8 as *const libc::c_char,
    b"Sorry, compressed BMPs not yet supported\x00" as *const u8 as *const libc::c_char,
    b"Empty BMP image\x00" as *const u8 as *const libc::c_char,
    b"Not a BMP file - does not start with BM\x00" as *const u8 as *const libc::c_char,
    b"Numeric value out of range in BMP file\x00" as *const u8 as *const libc::c_char,
    b"%ux%u 24-bit BMP image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u 8-bit colormapped BMP image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u 24-bit OS2 BMP image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u 8-bit colormapped OS2 BMP image\x00" as *const u8 as *const libc::c_char,
    b"GIF output got confused\x00" as *const u8 as *const libc::c_char,
    b"Bogus GIF codesize %d\x00" as *const u8 as *const libc::c_char,
    b"GIF output must be grayscale or RGB\x00" as *const u8 as *const libc::c_char,
    b"Too few images in GIF file\x00" as *const u8 as *const libc::c_char,
    b"Not a GIF file\x00" as *const u8 as *const libc::c_char,
    b"%ux%ux%d GIF image\x00" as *const u8 as *const libc::c_char,
    b"Warning: unexpected GIF version number \'%c%c%c\'\x00" as *const u8 as *const libc::c_char,
    b"Ignoring GIF extension block of type 0x%02x\x00" as *const u8 as *const libc::c_char,
    b"Caution: nonsquare pixels in input\x00" as *const u8 as *const libc::c_char,
    b"Corrupt data in GIF file\x00" as *const u8 as *const libc::c_char,
    b"Bogus char 0x%02x in GIF file, ignoring\x00" as *const u8 as *const libc::c_char,
    b"Premature end of GIF image\x00" as *const u8 as *const libc::c_char,
    b"Ran out of GIF bits\x00" as *const u8 as *const libc::c_char,
    b"PPM output must be grayscale or RGB\x00" as *const u8 as *const libc::c_char,
    b"Nonnumeric data in PPM file\x00" as *const u8 as *const libc::c_char,
    b"Not a PPM/PGM file\x00" as *const u8 as *const libc::c_char,
    b"Numeric value out of range in PPM file\x00" as *const u8 as *const libc::c_char,
    b"%ux%u PGM image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u text PGM image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u PPM image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u text PPM image\x00" as *const u8 as *const libc::c_char,
    b"Unsupported Targa colormap format\x00" as *const u8 as *const libc::c_char,
    b"Invalid or unsupported Targa file\x00" as *const u8 as *const libc::c_char,
    b"Targa output must be grayscale or RGB\x00" as *const u8 as *const libc::c_char,
    b"%ux%u RGB Targa image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u grayscale Targa image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u colormapped Targa image\x00" as *const u8 as *const libc::c_char,
    b"Color map file is invalid or of unsupported format\x00" as *const u8 as *const libc::c_char,
    b"Output file format cannot handle %d colormap entries\x00" as *const u8 as *const libc::c_char,
    b"ungetc failed\x00" as *const u8 as *const libc::c_char,
    b"MozJPEG can\'t read the image (PNG support is disabled in this build)\x00" as *const u8
        as *const libc::c_char,
    b"Unsupported output file format\x00" as *const u8 as *const libc::c_char,
    crate::stddef_h::NULL as *const libc::c_char,
];
/* so can override from CFLAGS in Makefile */

pub const DEFAULT_FMT: libc::c_int = FMT_PPM as libc::c_int;

static mut requested_fmt: IMAGE_FORMATS = FMT_BMP;
/*
 * Argument-parsing code.
 * The switch parser is designed to be useful with DOS-style command line
 * syntax, ie, intermixed switches and file names, where only the switches
 * to the left of a given file name affect processing of that file.
 * The main program in this file doesn't actually use this capability...
 */

static mut progname: *const libc::c_char = 0 as *const libc::c_char;
/* program name for error messages */

static mut icc_filename: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/* for -icc switch */

static mut outfilename: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/* for -outfile switch */
#[no_mangle]

pub static mut memsrc: crate::jmorecfg_h::boolean = 0;
/* for -memsrc switch */
#[no_mangle]

pub static mut skip: crate::jmorecfg_h::boolean = 0;
#[no_mangle]

pub static mut crop: crate::jmorecfg_h::boolean = 0;
#[no_mangle]

pub static mut skip_start: crate::jmorecfg_h::JDIMENSION = 0;
#[no_mangle]

pub static mut skip_end: crate::jmorecfg_h::JDIMENSION = 0;
#[no_mangle]

pub static mut crop_x: crate::jmorecfg_h::JDIMENSION = 0;
#[no_mangle]

pub static mut crop_y: crate::jmorecfg_h::JDIMENSION = 0;
#[no_mangle]

pub static mut crop_width: crate::jmorecfg_h::JDIMENSION = 0;
#[no_mangle]

pub static mut crop_height: crate::jmorecfg_h::JDIMENSION = 0;

pub const INPUT_BUF_SIZE: libc::c_int = 4096 as libc::c_int;

unsafe extern "C" fn usage()
/* complain about bad command line */
{
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"usage: %s [switches] \x00" as *const u8 as *const libc::c_char,
        progname,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"[inputfile]\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"Switches (names may be abbreviated):\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -colors N      Reduce image to no more than N colors\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -fast          Fast, low-quality processing\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -grayscale     Force grayscale output\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -rgb           Force RGB output\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -rgb565        Force RGB565 output\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -scale M/N     Scale output image by fraction M/N, eg, 1/8\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -bmp           Select BMP output format (Windows style)%s\n\x00" as *const u8
            as *const libc::c_char,
        if DEFAULT_FMT == FMT_BMP as libc::c_int {
            b" (default)\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -gif           Select GIF output format%s\n\x00" as *const u8 as *const libc::c_char,
        if DEFAULT_FMT == FMT_GIF as libc::c_int {
            b" (default)\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -os2           Select BMP output format (OS/2 style)%s\n\x00" as *const u8
            as *const libc::c_char,
        if DEFAULT_FMT == FMT_OS2 as libc::c_int {
            b" (default)\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -pnm           Select PBMPLUS (PPM/PGM) output format%s\n\x00" as *const u8
            as *const libc::c_char,
        if DEFAULT_FMT == FMT_PPM as libc::c_int {
            b" (default)\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -targa         Select Targa output format%s\n\x00" as *const u8 as *const libc::c_char,
        if DEFAULT_FMT == FMT_TARGA as libc::c_int {
            b" (default)\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"Switches for advanced users:\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -dct int       Use integer DCT method%s\n\x00" as *const u8 as *const libc::c_char,
        if crate::jpeglib_h::JDCT_DEFAULT == crate::jpeglib_h::JDCT_ISLOW as libc::c_int {
            b" (default)\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -dct fast      Use fast integer DCT (less accurate)%s\n\x00" as *const u8
            as *const libc::c_char,
        if crate::jpeglib_h::JDCT_DEFAULT == crate::jpeglib_h::JDCT_IFAST as libc::c_int {
            b" (default)\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -dct float     Use floating-point DCT method%s\n\x00" as *const u8
            as *const libc::c_char,
        if crate::jpeglib_h::JDCT_DEFAULT == crate::jpeglib_h::JDCT_FLOAT as libc::c_int {
            b" (default)\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -dither fs     Use F-S dithering (default)\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -dither none   Don\'t use dithering in quantization\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -dither ordered  Use ordered dither (medium speed, quality)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -icc FILE      Extract ICC profile to FILE\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -map FILE      Map to colors used in named image file\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -nosmooth      Don\'t use high-quality upsampling\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -onepass       Use 1-pass quantization (fast, low quality)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -maxmemory N   Maximum memory to use (in kbytes)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -outfile name  Specify name for output file\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -memsrc        Load input file into memory before decompressing\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -skip Y0,Y1    Decompress all rows except those between Y0 and Y1 (inclusive)\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -crop WxH+X+Y  Decompress only a rectangular subregion of the image\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"                 [requires PBMPLUS (PPM/PGM), GIF, or Targa output format]\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -verbose  or  -debug   Emit debug output\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -version       Print version information and exit\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
}

unsafe extern "C" fn parse_switches(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut last_file_arg_seen: libc::c_int,
    mut for_real: crate::jmorecfg_h::boolean,
) -> libc::c_int
/* Parse optional switches.
 * Returns argv[] index of first file-name argument (== argc if none).
 * Any file names with indexes <= last_file_arg_seen are ignored;
 * they have presumably been processed in a previous iteration.
 * (Pass 0 for last_file_arg_seen on the first or only iteration.)
 * for_real is FALSE on the first (dummy) pass; we may skip any expensive
 * processing.
 */ {
    let mut argn: libc::c_int = 0;
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Set up default JPEG parameters. */
    requested_fmt = DEFAULT_FMT as IMAGE_FORMATS; /* set default output file format */
    icc_filename = crate::stddef_h::NULL as *mut libc::c_char;
    outfilename = crate::stddef_h::NULL as *mut libc::c_char;
    memsrc = crate::jmorecfg_h::FALSE;
    skip = crate::jmorecfg_h::FALSE;
    crop = crate::jmorecfg_h::FALSE;
    (*(*cinfo).err).trace_level = 0 as libc::c_int;
    /* Scan command line options, adjust parameters */
    argn = 1 as libc::c_int;
    while argn < argc {
        arg = *argv.offset(argn as isize);
        if *arg as libc::c_int != '-' as i32 {
            /* Not a switch, must be a file name argument */
            if !(argn <= last_file_arg_seen) {
                break; /* -outfile applies to just one input file */
            }
            outfilename = crate::stddef_h::NULL as *mut libc::c_char
        /* ignore this name if previously processed */
        /* else done parsing switches */
        } else {
            arg = arg.offset(1); /* advance past switch marker character */
            if crate::cdjpeg_h::keymatch(
                arg,
                b"bmp\x00" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            ) != 0
            {
                /* BMP output format. */
                requested_fmt = FMT_BMP
            } else if crate::cdjpeg_h::keymatch(
                arg,
                b"colors\x00" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            ) != 0
                || crate::cdjpeg_h::keymatch(
                    arg,
                    b"colours\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                ) != 0
                || crate::cdjpeg_h::keymatch(
                    arg,
                    b"quantize\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                ) != 0
                || crate::cdjpeg_h::keymatch(
                    arg,
                    b"quantise\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                ) != 0
            {
                /* Do color quantization. */
                let mut val: libc::c_int = 0;
                argn += 1;
                if argn >= argc {
                    /* advance to next argument */
                    usage();
                }
                if crate::stdlib::sscanf(
                    *argv.offset(argn as isize),
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    &mut val as *mut libc::c_int,
                ) != 1 as libc::c_int
                {
                    usage();
                }
                (*cinfo).desired_number_of_colors = val;
                (*cinfo).quantize_colors = crate::jmorecfg_h::TRUE
            } else if crate::cdjpeg_h::keymatch(
                arg,
                b"dct\x00" as *const u8 as *const libc::c_char,
                2 as libc::c_int,
            ) != 0
            {
                /* Select IDCT algorithm. */
                argn += 1;
                if argn >= argc {
                    /* advance to next argument */
                    usage();
                }
                if crate::cdjpeg_h::keymatch(
                    *argv.offset(argn as isize),
                    b"int\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                ) != 0
                {
                    (*cinfo).dct_method = crate::jpeglib_h::JDCT_ISLOW
                } else if crate::cdjpeg_h::keymatch(
                    *argv.offset(argn as isize),
                    b"fast\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int,
                ) != 0
                {
                    (*cinfo).dct_method = crate::jpeglib_h::JDCT_IFAST
                } else if crate::cdjpeg_h::keymatch(
                    *argv.offset(argn as isize),
                    b"float\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int,
                ) != 0
                {
                    (*cinfo).dct_method = crate::jpeglib_h::JDCT_FLOAT
                } else {
                    usage();
                }
            } else if crate::cdjpeg_h::keymatch(
                arg,
                b"dither\x00" as *const u8 as *const libc::c_char,
                2 as libc::c_int,
            ) != 0
            {
                /* Select dithering algorithm. */
                argn += 1;
                if argn >= argc {
                    /* advance to next argument */
                    usage();
                }
                if crate::cdjpeg_h::keymatch(
                    *argv.offset(argn as isize),
                    b"fs\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int,
                ) != 0
                {
                    (*cinfo).dither_mode = crate::jpeglib_h::JDITHER_FS
                } else if crate::cdjpeg_h::keymatch(
                    *argv.offset(argn as isize),
                    b"none\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int,
                ) != 0
                {
                    (*cinfo).dither_mode = crate::jpeglib_h::JDITHER_NONE
                } else if crate::cdjpeg_h::keymatch(
                    *argv.offset(argn as isize),
                    b"ordered\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int,
                ) != 0
                {
                    (*cinfo).dither_mode = crate::jpeglib_h::JDITHER_ORDERED
                } else {
                    usage();
                }
            } else if crate::cdjpeg_h::keymatch(
                arg,
                b"debug\x00" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            ) != 0
                || crate::cdjpeg_h::keymatch(
                    arg,
                    b"verbose\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                ) != 0
            {
                /* Enable debug printouts. */
                /* On first -d, print version identification */
                static mut printed_version: crate::jmorecfg_h::boolean = crate::jmorecfg_h::FALSE;
                if printed_version == 0 {
                    crate::stdlib::fprintf(
                        crate::stdlib::stderr,
                        b"%s version %s (build %s)\n\x00" as *const u8 as *const libc::c_char,
                        crate::jconfigint_h::PACKAGE_NAME.as_ptr(),
                        crate::jconfigint_h::VERSION.as_ptr(),
                        crate::jconfigint_h::BUILD.as_ptr(),
                    );
                    crate::stdlib::fprintf(
                        crate::stdlib::stderr,
                        b"%s\n\n\x00" as *const u8 as *const libc::c_char,
                        crate::jversion_h::JCOPYRIGHT.as_ptr(),
                    );
                    crate::stdlib::fprintf(
                        crate::stdlib::stderr,
                        b"Emulating The Independent JPEG Group\'s software, version %s\n\n\x00"
                            as *const u8 as *const libc::c_char,
                        crate::jversion_h::JVERSION.as_ptr(),
                    );
                    printed_version = crate::jmorecfg_h::TRUE
                }
                (*(*cinfo).err).trace_level += 1
            } else if crate::cdjpeg_h::keymatch(
                arg,
                b"version\x00" as *const u8 as *const libc::c_char,
                4 as libc::c_int,
            ) != 0
            {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"%s version %s (build %s)\n\x00" as *const u8 as *const libc::c_char,
                    crate::jconfigint_h::PACKAGE_NAME.as_ptr(),
                    crate::jconfigint_h::VERSION.as_ptr(),
                    crate::jconfigint_h::BUILD.as_ptr(),
                );
                crate::stdlib::exit(crate::stdlib::EXIT_SUCCESS);
            } else {
                if crate::cdjpeg_h::keymatch(
                    arg,
                    b"fast\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                ) != 0
                {
                    /* Select recommended processing options for quick-and-dirty output. */
                    (*cinfo).two_pass_quantize = crate::jmorecfg_h::FALSE;
                    (*cinfo).dither_mode = crate::jpeglib_h::JDITHER_ORDERED;
                    if (*cinfo).quantize_colors == 0 {
                        /* don't override an earlier -colors */
                        (*cinfo).desired_number_of_colors = 216 as libc::c_int
                    }
                    (*cinfo).dct_method =
                        crate::jpeglib_h::JDCT_FASTEST as crate::jpeglib_h::J_DCT_METHOD;
                    (*cinfo).do_fancy_upsampling = crate::jmorecfg_h::FALSE
                } else if crate::cdjpeg_h::keymatch(
                    arg,
                    b"gif\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                ) != 0
                {
                    /* GIF output format. */
                    requested_fmt = FMT_GIF
                } else if crate::cdjpeg_h::keymatch(
                    arg,
                    b"grayscale\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int,
                ) != 0
                    || crate::cdjpeg_h::keymatch(
                        arg,
                        b"greyscale\x00" as *const u8 as *const libc::c_char,
                        2 as libc::c_int,
                    ) != 0
                {
                    /* Force monochrome output. */
                    (*cinfo).out_color_space = crate::jpeglib_h::JCS_GRAYSCALE
                } else if crate::cdjpeg_h::keymatch(
                    arg,
                    b"rgb\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int,
                ) != 0
                {
                    /* Force RGB output. */
                    (*cinfo).out_color_space = crate::jpeglib_h::JCS_RGB
                } else if crate::cdjpeg_h::keymatch(
                    arg,
                    b"rgb565\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int,
                ) != 0
                {
                    /* Force RGB565 output. */
                    (*cinfo).out_color_space = crate::jpeglib_h::JCS_RGB565
                } else if crate::cdjpeg_h::keymatch(
                    arg,
                    b"icc\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                ) != 0
                {
                    /* Set ICC filename. */
                    argn += 1;
                    if argn >= argc {
                        /* advance to next argument */
                        usage();
                    }
                    icc_filename = *argv.offset(argn as isize);
                    crate::jpeglib_h::jpeg_save_markers(
                        cinfo,
                        crate::jpeglib_h::JPEG_APP0 + 2 as libc::c_int,
                        0xffff as libc::c_int as libc::c_uint,
                    );
                } else if crate::cdjpeg_h::keymatch(
                    arg,
                    b"map\x00" as *const u8 as *const libc::c_char,
                    3 as libc::c_int,
                ) != 0
                {
                    /* Quantize to a color map taken from an input file. */
                    argn += 1;
                    if argn >= argc {
                        /* advance to next argument */
                        usage();
                    }
                    if for_real != 0 {
                        /* too expensive to do twice! */
                        /* otherwise can't quantize to supplied map */
                        let mut mapfile: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
                        mapfile = crate::stdlib::fopen(
                            *argv.offset(argn as isize),
                            crate::cdjpeg_h::READ_BINARY.as_ptr(),
                        );
                        if mapfile.is_null() {
                            crate::stdlib::fprintf(
                                crate::stdlib::stderr,
                                b"%s: can\'t open %s\n\x00" as *const u8 as *const libc::c_char,
                                progname,
                                *argv.offset(argn as isize),
                            );
                            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
                        }
                        crate::cdjpeg_h::read_color_map(cinfo, mapfile);
                        crate::stdlib::fclose(mapfile);
                        (*cinfo).quantize_colors = crate::jmorecfg_h::TRUE
                    }
                } else if crate::cdjpeg_h::keymatch(
                    arg,
                    b"maxmemory\x00" as *const u8 as *const libc::c_char,
                    3 as libc::c_int,
                ) != 0
                {
                    /* Maximum memory in Kb (or Mb with 'm'). */
                    let mut lval: libc::c_long = 0;
                    let mut ch: libc::c_char = 'x' as i32 as libc::c_char;
                    argn += 1;
                    if argn >= argc {
                        /* advance to next argument */
                        usage();
                    }
                    if crate::stdlib::sscanf(
                        *argv.offset(argn as isize),
                        b"%ld%c\x00" as *const u8 as *const libc::c_char,
                        &mut lval as *mut libc::c_long,
                        &mut ch as *mut libc::c_char,
                    ) < 1 as libc::c_int
                    {
                        usage();
                    }
                    if ch as libc::c_int == 'm' as i32 || ch as libc::c_int == 'M' as i32 {
                        lval *= 1000 as libc::c_long
                    }
                    (*(*cinfo).mem).max_memory_to_use = lval * 1000 as libc::c_long
                } else if crate::cdjpeg_h::keymatch(
                    arg,
                    b"nosmooth\x00" as *const u8 as *const libc::c_char,
                    3 as libc::c_int,
                ) != 0
                {
                    /* Suppress fancy upsampling */
                    (*cinfo).do_fancy_upsampling = crate::jmorecfg_h::FALSE
                } else if crate::cdjpeg_h::keymatch(
                    arg,
                    b"onepass\x00" as *const u8 as *const libc::c_char,
                    3 as libc::c_int,
                ) != 0
                {
                    /* Use fast one-pass quantization. */
                    (*cinfo).two_pass_quantize = crate::jmorecfg_h::FALSE
                } else if crate::cdjpeg_h::keymatch(
                    arg,
                    b"os2\x00" as *const u8 as *const libc::c_char,
                    3 as libc::c_int,
                ) != 0
                {
                    /* BMP output format (OS/2 flavor). */
                    requested_fmt = FMT_OS2
                } else if crate::cdjpeg_h::keymatch(
                    arg,
                    b"outfile\x00" as *const u8 as *const libc::c_char,
                    4 as libc::c_int,
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
                } else if crate::cdjpeg_h::keymatch(
                    arg,
                    b"memsrc\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int,
                ) != 0
                {
                    /* Use in-memory source manager */
                    memsrc = crate::jmorecfg_h::TRUE
                } else if crate::cdjpeg_h::keymatch(
                    arg,
                    b"pnm\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                ) != 0
                    || crate::cdjpeg_h::keymatch(
                        arg,
                        b"ppm\x00" as *const u8 as *const libc::c_char,
                        1 as libc::c_int,
                    ) != 0
                {
                    /* PPM/PGM output format. */
                    requested_fmt = FMT_PPM
                } else if crate::cdjpeg_h::keymatch(
                    arg,
                    b"rle\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                ) != 0
                {
                    /* RLE output format. */
                    requested_fmt = FMT_RLE
                } else if crate::cdjpeg_h::keymatch(
                    arg,
                    b"scale\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int,
                ) != 0
                {
                    /* Scale the output image by a fraction M/N. */
                    argn += 1;
                    if argn >= argc {
                        /* advance to next argument */
                        usage();
                    }
                    if crate::stdlib::sscanf(
                        *argv.offset(argn as isize),
                        b"%u/%u\x00" as *const u8 as *const libc::c_char,
                        &mut (*cinfo).scale_num as *mut libc::c_uint,
                        &mut (*cinfo).scale_denom as *mut libc::c_uint,
                    ) != 2 as libc::c_int
                    {
                        usage();
                    }
                } else if crate::cdjpeg_h::keymatch(
                    arg,
                    b"skip\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int,
                ) != 0
                {
                    argn += 1;
                    if argn >= argc {
                        usage();
                    }
                    if crate::stdlib::sscanf(
                        *argv.offset(argn as isize),
                        b"%u,%u\x00" as *const u8 as *const libc::c_char,
                        &mut skip_start as *mut crate::jmorecfg_h::JDIMENSION,
                        &mut skip_end as *mut crate::jmorecfg_h::JDIMENSION,
                    ) != 2 as libc::c_int
                        || skip_start > skip_end
                    {
                        usage();
                    }
                    skip = crate::jmorecfg_h::TRUE
                } else if crate::cdjpeg_h::keymatch(
                    arg,
                    b"crop\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int,
                ) != 0
                {
                    let mut c: libc::c_char = 0;
                    argn += 1;
                    if argn >= argc {
                        usage();
                    }
                    if crate::stdlib::sscanf(
                        *argv.offset(argn as isize),
                        b"%u%c%u+%u+%u\x00" as *const u8 as *const libc::c_char,
                        &mut crop_width as *mut crate::jmorecfg_h::JDIMENSION,
                        &mut c as *mut libc::c_char,
                        &mut crop_height as *mut crate::jmorecfg_h::JDIMENSION,
                        &mut crop_x as *mut crate::jmorecfg_h::JDIMENSION,
                        &mut crop_y as *mut crate::jmorecfg_h::JDIMENSION,
                    ) != 5 as libc::c_int
                        || c as libc::c_int != 'X' as i32 && c as libc::c_int != 'x' as i32
                        || crop_width < 1 as libc::c_int as libc::c_uint
                        || crop_height < 1 as libc::c_int as libc::c_uint
                    {
                        usage();
                    }
                    crop = crate::jmorecfg_h::TRUE
                } else if crate::cdjpeg_h::keymatch(
                    arg,
                    b"targa\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                ) != 0
                {
                    /* Targa output format. */
                    requested_fmt = FMT_TARGA
                } else {
                    usage();
                    /* bogus switch */
                }
            }
        }
        argn += 1
    }
    return argn;
    /* return index of next arg (file name) */
}
/*
 * Marker processor for COM and interesting APPn markers.
 * This replaces the library's built-in processor, which just skips the marker.
 * We want to print out the marker as text, to the extent possible.
 * Note this code relies on a non-suspending data source.
 */

unsafe extern "C" fn jpeg_getc(mut cinfo: crate::jpeglib_h::j_decompress_ptr) -> libc::c_uint
/* Read next byte */ {
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src; /* discount the length word itself */
    if (*datasrc).bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            (*(*cinfo).err).msg_code = crate::jerror_h::JERR_CANT_SUSPEND as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
    }
    (*datasrc).bytes_in_buffer = (*datasrc).bytes_in_buffer.wrapping_sub(1);
    let fresh0 = (*datasrc).next_input_byte;
    (*datasrc).next_input_byte = (*datasrc).next_input_byte.offset(1);
    return *fresh0 as libc::c_uint;
}

unsafe extern "C" fn print_text_marker(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean {
    let mut traceit: crate::jmorecfg_h::boolean =
        ((*(*cinfo).err).trace_level >= 1 as libc::c_int) as libc::c_int;
    let mut length: libc::c_long = 0;
    let mut ch: libc::c_uint = 0;
    let mut lastch: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    length = (jpeg_getc(cinfo) << 8 as libc::c_int) as libc::c_long;
    length += jpeg_getc(cinfo) as libc::c_long;
    length -= 2 as libc::c_int as libc::c_long;
    if traceit != 0 {
        if (*cinfo).unread_marker == crate::jpeglib_h::JPEG_COM {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"Comment, length %ld:\n\x00" as *const u8 as *const libc::c_char,
                length,
            );
        } else {
            /* assume it is an APPn otherwise */
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"APP%d, length %ld:\n\x00" as *const u8 as *const libc::c_char,
                (*cinfo).unread_marker - crate::jpeglib_h::JPEG_APP0,
                length,
            );
        }
    }
    loop {
        length -= 1;
        if !(length >= 0 as libc::c_int as libc::c_long) {
            break;
        }
        ch = jpeg_getc(cinfo);
        if traceit != 0 {
            /* Emit the character in a readable form.
             * Nonprintables are converted to \nnn form,
             * while \ is converted to \\.
             * Newlines in CR, CR/LF, or LF form will be printed as one newline.
             */
            if ch == '\r' as i32 as libc::c_uint {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"\n\x00" as *const u8 as *const libc::c_char,
                );
            } else if ch == '\n' as i32 as libc::c_uint {
                if lastch != '\r' as i32 as libc::c_uint {
                    crate::stdlib::fprintf(
                        crate::stdlib::stderr,
                        b"\n\x00" as *const u8 as *const libc::c_char,
                    );
                }
            } else if ch == '\\' as i32 as libc::c_uint {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"\\\\\x00" as *const u8 as *const libc::c_char,
                );
            } else if *(*crate::stdlib::__ctype_b_loc()).offset(ch as libc::c_int as isize)
                as libc::c_int
                & crate::stdlib::_ISprint as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                crate::stdlib::putc(ch as libc::c_int, crate::stdlib::stderr);
            } else {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"\\%03o\x00" as *const u8 as *const libc::c_char,
                    ch,
                );
            }
            lastch = ch
        }
    }
    if traceit != 0 {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    return crate::jmorecfg_h::TRUE;
}
/*
 * The main program.
 */

unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut cinfo: crate::jpeglib_h::jpeg_decompress_struct =
        crate::jpeglib_h::jpeg_decompress_struct {
            err: 0 as *mut crate::jpeglib_h::jpeg_error_mgr,
            mem: 0 as *mut crate::jpeglib_h::jpeg_memory_mgr,
            progress: 0 as *mut crate::jpeglib_h::jpeg_progress_mgr,
            client_data: 0 as *mut libc::c_void,
            is_decompressor: 0,
            global_state: 0,
            src: 0 as *mut crate::jpeglib_h::jpeg_source_mgr,
            image_width: 0,
            image_height: 0,
            num_components: 0,
            jpeg_color_space: crate::jpeglib_h::JCS_UNKNOWN,
            out_color_space: crate::jpeglib_h::JCS_UNKNOWN,
            scale_num: 0,
            scale_denom: 0,
            output_gamma: 0.,
            buffered_image: 0,
            raw_data_out: 0,
            dct_method: crate::jpeglib_h::JDCT_ISLOW,
            do_fancy_upsampling: 0,
            do_block_smoothing: 0,
            quantize_colors: 0,
            dither_mode: crate::jpeglib_h::JDITHER_NONE,
            two_pass_quantize: 0,
            desired_number_of_colors: 0,
            enable_1pass_quant: 0,
            enable_external_quant: 0,
            enable_2pass_quant: 0,
            output_width: 0,
            output_height: 0,
            out_color_components: 0,
            output_components: 0,
            rec_outbuf_height: 0,
            actual_number_of_colors: 0,
            colormap: 0 as *mut crate::jpeglib_h::JSAMPROW,
            output_scanline: 0,
            input_scan_number: 0,
            input_iMCU_row: 0,
            output_scan_number: 0,
            output_iMCU_row: 0,
            coef_bits: 0 as *mut [libc::c_int; 64],
            quant_tbl_ptrs: [0 as *mut crate::jpeglib_h::JQUANT_TBL; 4],
            dc_huff_tbl_ptrs: [0 as *mut crate::jpeglib_h::JHUFF_TBL; 4],
            ac_huff_tbl_ptrs: [0 as *mut crate::jpeglib_h::JHUFF_TBL; 4],
            data_precision: 0,
            comp_info: 0 as *mut crate::jpeglib_h::jpeg_component_info,
            progressive_mode: 0,
            arith_code: 0,
            arith_dc_L: [0; 16],
            arith_dc_U: [0; 16],
            arith_ac_K: [0; 16],
            restart_interval: 0,
            saw_JFIF_marker: 0,
            JFIF_major_version: 0,
            JFIF_minor_version: 0,
            density_unit: 0,
            X_density: 0,
            Y_density: 0,
            saw_Adobe_marker: 0,
            Adobe_transform: 0,
            CCIR601_sampling: 0,
            marker_list: 0 as *mut crate::jpeglib_h::jpeg_marker_struct,
            max_h_samp_factor: 0,
            max_v_samp_factor: 0,
            min_DCT_scaled_size: 0,
            total_iMCU_rows: 0,
            sample_range_limit: 0 as *mut crate::jmorecfg_h::JSAMPLE,
            comps_in_scan: 0,
            cur_comp_info: [0 as *mut crate::jpeglib_h::jpeg_component_info; 4],
            MCUs_per_row: 0,
            MCU_rows_in_scan: 0,
            blocks_in_MCU: 0,
            MCU_membership: [0; 10],
            Ss: 0,
            Se: 0,
            Ah: 0,
            Al: 0,
            unread_marker: 0,
            master: 0 as *mut crate::jpeglib_h::jpeg_decomp_master,
            main: 0 as *mut crate::jpeglib_h::jpeg_d_main_controller,
            coef: 0 as *mut crate::jpeglib_h::jpeg_d_coef_controller,
            post: 0 as *mut crate::jpeglib_h::jpeg_d_post_controller,
            inputctl: 0 as *mut crate::jpeglib_h::jpeg_input_controller,
            marker: 0 as *mut crate::jpeglib_h::jpeg_marker_reader,
            entropy: 0 as *mut crate::jpeglib_h::jpeg_entropy_decoder,
            idct: 0 as *mut crate::jpeglib_h::jpeg_inverse_dct,
            upsample: 0 as *mut crate::jpeglib_h::jpeg_upsampler,
            cconvert: 0 as *mut crate::jpeglib_h::jpeg_color_deconverter,
            cquantize: 0 as *mut crate::jpeglib_h::jpeg_color_quantizer,
        };
    let mut jerr: crate::jpeglib_h::jpeg_error_mgr = crate::jpeglib_h::jpeg_error_mgr {
        error_exit: None,
        emit_message: None,
        output_message: None,
        format_message: None,
        reset_error_mgr: None,
        msg_code: 0,
        msg_parm: crate::jpeglib_h::C2RustUnnamed_0 { i: [0; 8] },
        trace_level: 0,
        num_warnings: 0,
        jpeg_message_table: 0 as *const *const libc::c_char,
        last_jpeg_message: 0,
        addon_message_table: 0 as *const *const libc::c_char,
        first_addon_message: 0,
        last_addon_message: 0,
    };
    let mut file_index: libc::c_int = 0;
    let mut dest_mgr: crate::cdjpeg_h::djpeg_dest_ptr =
        crate::stddef_h::NULL as crate::cdjpeg_h::djpeg_dest_ptr;
    let mut input_file: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    let mut output_file: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    let mut inbuffer: *mut libc::c_uchar = crate::stddef_h::NULL as *mut libc::c_uchar;
    let mut insize: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut num_scanlines: crate::jmorecfg_h::JDIMENSION = 0;
    /* On Mac, fetch a command line. */
    progname = *argv.offset(0 as libc::c_int as isize); /* in case C library doesn't provide it */
    if progname.is_null()
        || *progname.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
    {
        progname = b"djpeg\x00" as *const u8 as *const libc::c_char
    }
    /* Initialize the JPEG decompression object with default error handling. */
    cinfo.err = crate::jpeglib_h::jpeg_std_error(&mut jerr);
    crate::jpeglib_h::jpeg_CreateDecompress(
        &mut cinfo,
        crate::jconfig_h::JPEG_LIB_VERSION,
        ::std::mem::size_of::<crate::jpeglib_h::jpeg_decompress_struct>() as libc::c_ulong,
    );
    /* Add some application-specific error messages (from cderror.h) */
    jerr.addon_message_table = cdjpeg_message_table.as_ptr();
    jerr.first_addon_message = crate::cderror_h::JMSG_FIRSTADDONCODE as libc::c_int;
    jerr.last_addon_message = crate::cderror_h::JMSG_LASTADDONCODE as libc::c_int;
    /* Insert custom marker processor for COM and APP12.
     * APP12 is used by some digital camera makers for textual info,
     * so we provide the ability to display it as text.
     * If you like, additional APPn marker types can be selected for display,
     * but don't try to override APP0 or APP14 this way (see libjpeg.txt).
     */
    crate::jpeglib_h::jpeg_set_marker_processor(
        &mut cinfo,
        crate::jpeglib_h::JPEG_COM,
        Some(
            print_text_marker
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                ) -> crate::jmorecfg_h::boolean,
        ),
    );
    crate::jpeglib_h::jpeg_set_marker_processor(
        &mut cinfo,
        crate::jpeglib_h::JPEG_APP0 + 12 as libc::c_int,
        Some(
            print_text_marker
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                ) -> crate::jmorecfg_h::boolean,
        ),
    );
    /* Scan command line to find file names. */
    /* It is convenient to use just one switch-parsing routine, but the switch
     * values read here are ignored; we will rescan the switches after opening
     * the input file.
     * (Exception: tracing level set here controls verbosity for COM markers
     * found during jpeg_read_header...)
     */
    file_index = parse_switches(
        &mut cinfo,
        argc,
        argv,
        0 as libc::c_int,
        crate::jmorecfg_h::FALSE,
    );
    /* Unix style: expect zero or one file name */
    if file_index < argc - 1 as libc::c_int {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s: only one input file\n\x00" as *const u8 as *const libc::c_char,
            progname,
        );
        usage();
    }
    /* TWO_FILE_COMMANDLINE */
    /* Open the input file. */
    if file_index < argc {
        input_file = crate::stdlib::fopen(
            *argv.offset(file_index as isize),
            crate::cdjpeg_h::READ_BINARY.as_ptr(),
        );
        if input_file.is_null() {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: can\'t open %s\n\x00" as *const u8 as *const libc::c_char,
                progname,
                *argv.offset(file_index as isize),
            );
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
    } else {
        /* default input file is stdin */
        input_file = crate::cdjpeg_h::read_stdin()
    }
    /* Open the output file. */
    if !outfilename.is_null() {
        output_file = crate::stdlib::fopen(outfilename, crate::cdjpeg_h::WRITE_BINARY.as_ptr());
        if output_file.is_null() {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: can\'t open %s\n\x00" as *const u8 as *const libc::c_char,
                progname,
                outfilename,
            );
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
    } else {
        /* default output file is stdout */
        output_file = crate::cdjpeg_h::write_stdout()
    }
    /* Specify data source for decompression */
    if memsrc != 0 {
        let mut nbytes: crate::stddef_h::size_t = 0;
        loop {
            inbuffer = crate::stdlib::realloc(
                inbuffer as *mut libc::c_void,
                insize.wrapping_add(INPUT_BUF_SIZE as libc::c_ulong),
            ) as *mut libc::c_uchar;
            if inbuffer.is_null() {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"%s: memory allocation failure\n\x00" as *const u8 as *const libc::c_char,
                    progname,
                );
                crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
            }
            nbytes = crate::stdlib::fread(
                &mut *inbuffer.offset(insize as isize) as *mut libc::c_uchar as *mut libc::c_void,
                1 as libc::c_int as crate::stddef_h::size_t,
                4096 as libc::c_int as crate::stddef_h::size_t,
                input_file,
            );
            if nbytes < INPUT_BUF_SIZE as libc::c_ulong && crate::stdlib::ferror(input_file) != 0 {
                if file_index < argc {
                    crate::stdlib::fprintf(
                        crate::stdlib::stderr,
                        b"%s: can\'t read from %s\n\x00" as *const u8 as *const libc::c_char,
                        progname,
                        *argv.offset(file_index as isize),
                    );
                } else {
                    crate::stdlib::fprintf(
                        crate::stdlib::stderr,
                        b"%s: can\'t read from stdin\n\x00" as *const u8 as *const libc::c_char,
                        progname,
                    );
                }
            }
            insize = insize.wrapping_add(nbytes);
            if !(nbytes == INPUT_BUF_SIZE as libc::c_ulong) {
                break;
            }
        }
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"Compressed size:  %lu bytes\n\x00" as *const u8 as *const libc::c_char,
            insize,
        );
        crate::jpeglib_h::jpeg_mem_src(&mut cinfo, inbuffer, insize);
    } else {
        crate::jpeglib_h::jpeg_stdio_src(&mut cinfo, input_file);
    }
    /* Read file header, set default decompression parameters */
    crate::jpeglib_h::jpeg_read_header(&mut cinfo, crate::jmorecfg_h::TRUE);
    /* Adjust default decompression parameters by re-parsing the options */
    file_index = parse_switches(
        &mut cinfo,
        argc,
        argv,
        0 as libc::c_int,
        crate::jmorecfg_h::TRUE,
    );
    /* Initialize the output module now to let it override any crucial
     * option settings (for instance, GIF wants to force color quantization).
     */
    match requested_fmt as libc::c_uint {
        0 => {
            dest_mgr = crate::cdjpeg_h::jinit_write_bmp(
                &mut cinfo,
                crate::jmorecfg_h::FALSE,
                crate::jmorecfg_h::TRUE,
            )
        }
        2 => {
            dest_mgr = crate::cdjpeg_h::jinit_write_bmp(
                &mut cinfo,
                crate::jmorecfg_h::TRUE,
                crate::jmorecfg_h::TRUE,
            )
        }
        1 => dest_mgr = crate::cdjpeg_h::jinit_write_gif(&mut cinfo),
        3 => dest_mgr = crate::cdjpeg_h::jinit_write_ppm(&mut cinfo),
        5 => dest_mgr = crate::cdjpeg_h::jinit_write_targa(&mut cinfo),
        _ => {
            (*cinfo.err).msg_code = crate::cderror_h::JERR_UNSUPPORTED_FORMAT as libc::c_int;
            Some((*cinfo.err).error_exit.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                &mut cinfo as *mut crate::jpeglib_h::jpeg_decompress_struct
                    as crate::jpeglib_h::j_common_ptr,
            );
        }
    }
    (*dest_mgr).output_file = output_file;
    /* Start decompressor */
    crate::jpeglib_h::jpeg_start_decompress(&mut cinfo);
    /* Skip rows */
    if skip != 0 {
        let mut tmp: crate::jmorecfg_h::JDIMENSION = 0;
        /* Decompress a subregion */
        if skip_end
            > cinfo
                .output_height
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
        {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: skip region exceeds image height %d\n\x00" as *const u8
                    as *const libc::c_char,
                progname,
                cinfo.output_height,
            );
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
        tmp = cinfo.output_height;
        cinfo.output_height = (cinfo.output_height as libc::c_uint).wrapping_sub(
            skip_end
                .wrapping_sub(skip_start)
                .wrapping_add(1 as libc::c_int as libc::c_uint),
        ) as crate::jmorecfg_h::JDIMENSION
            as crate::jmorecfg_h::JDIMENSION;
        Some((*dest_mgr).start_output.expect("non-null function pointer"))
            .expect("non-null function pointer")(&mut cinfo, dest_mgr);
        cinfo.output_height = tmp;
        while cinfo.output_scanline < skip_start
        /* Check for valid skip_end.  We cannot check this value until after
         * jpeg_start_decompress() is called.  Note that we have already verified
         * that skip_start <= skip_end.
         */
        /* Write output file header.  This is a hack to ensure that the destination
         * manager creates an output image of the proper size.
         */
        /* Process data */
        {
            num_scanlines = crate::jpeglib_h::jpeg_read_scanlines(
                &mut cinfo,
                (*dest_mgr).buffer,
                (*dest_mgr).buffer_height,
            );
            Some(
                (*dest_mgr)
                    .put_pixel_rows
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(&mut cinfo, dest_mgr, num_scanlines);
        }
        crate::jpeglib_h::jpeg_skip_scanlines(
            &mut cinfo,
            skip_end
                .wrapping_sub(skip_start)
                .wrapping_add(1 as libc::c_int as libc::c_uint),
        );
        while cinfo.output_scanline < cinfo.output_height {
            num_scanlines = crate::jpeglib_h::jpeg_read_scanlines(
                &mut cinfo,
                (*dest_mgr).buffer,
                (*dest_mgr).buffer_height,
            );
            Some(
                (*dest_mgr)
                    .put_pixel_rows
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(&mut cinfo, dest_mgr, num_scanlines);
        }
    } else if crop != 0 {
        let mut tmp_0: crate::jmorecfg_h::JDIMENSION = 0;
        /* Normal full-image decompress */
        if crop_x.wrapping_add(crop_width) > cinfo.output_width
            || crop_y.wrapping_add(crop_height) > cinfo.output_height
        {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: crop dimensions exceed image dimensions %d x %d\n\x00" as *const u8
                    as *const libc::c_char,
                progname,
                cinfo.output_width,
                cinfo.output_height,
            );
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
        crate::jpeglib_h::jpeg_crop_scanline(&mut cinfo, &mut crop_x, &mut crop_width);
        if (*dest_mgr).calc_buffer_dimensions.is_some() {
            Some(
                (*dest_mgr)
                    .calc_buffer_dimensions
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(&mut cinfo, dest_mgr);
        } else {
            (*cinfo.err).msg_code = crate::cderror_h::JERR_UNSUPPORTED_FORMAT as libc::c_int;
            Some((*cinfo.err).error_exit.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                &mut cinfo as *mut crate::jpeglib_h::jpeg_decompress_struct
                    as crate::jpeglib_h::j_common_ptr,
            );
        }
        tmp_0 = cinfo.output_height;
        cinfo.output_height = crop_height;
        Some((*dest_mgr).start_output.expect("non-null function pointer"))
            .expect("non-null function pointer")(&mut cinfo, dest_mgr);
        cinfo.output_height = tmp_0;
        crate::jpeglib_h::jpeg_skip_scanlines(&mut cinfo, crop_y);
        while cinfo.output_scanline < crop_y.wrapping_add(crop_height) {
            num_scanlines = crate::jpeglib_h::jpeg_read_scanlines(
                &mut cinfo,
                (*dest_mgr).buffer,
                (*dest_mgr).buffer_height,
            );
            Some(
                (*dest_mgr)
                    .put_pixel_rows
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(&mut cinfo, dest_mgr, num_scanlines);
        }
        crate::jpeglib_h::jpeg_skip_scanlines(
            &mut cinfo,
            cinfo
                .output_height
                .wrapping_sub(crop_y)
                .wrapping_sub(crop_height),
        );
    } else {
        /* Check for valid crop dimensions.  We cannot check these values until
         * after jpeg_start_decompress() is called.
         */
        /* Write output file header.  This is a hack to ensure that the destination
         * manager creates an output image of the proper size.
         */
        /* Process data */
        /* Write output file header */
        Some((*dest_mgr).start_output.expect("non-null function pointer"))
            .expect("non-null function pointer")(&mut cinfo, dest_mgr);
        /* Process data */
        while cinfo.output_scanline < cinfo.output_height {
            num_scanlines = crate::jpeglib_h::jpeg_read_scanlines(
                &mut cinfo,
                (*dest_mgr).buffer,
                (*dest_mgr).buffer_height,
            );
            Some(
                (*dest_mgr)
                    .put_pixel_rows
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(&mut cinfo, dest_mgr, num_scanlines);
        }
    }
    if !icc_filename.is_null() {
        let mut icc_file: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
        let mut icc_profile: *mut crate::jmorecfg_h::JOCTET = 0 as *mut crate::jmorecfg_h::JOCTET;
        let mut icc_len: libc::c_uint = 0;
        icc_file = crate::stdlib::fopen(icc_filename, crate::cdjpeg_h::WRITE_BINARY.as_ptr());
        if icc_file.is_null() {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: can\'t open %s\n\x00" as *const u8 as *const libc::c_char,
                progname,
                icc_filename,
            );
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
        if crate::jpeglib_h::jpeg_read_icc_profile(&mut cinfo, &mut icc_profile, &mut icc_len) != 0
        {
            if crate::stdlib::fwrite(
                icc_profile as *const libc::c_void,
                icc_len as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                icc_file,
            ) < 1 as libc::c_int as libc::c_ulong
            {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"%s: can\'t read ICC profile from %s\n\x00" as *const u8
                        as *const libc::c_char,
                    progname,
                    icc_filename,
                );
                crate::stdlib::free(icc_profile as *mut libc::c_void);
                crate::stdlib::fclose(icc_file);
                crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
            }
            crate::stdlib::free(icc_profile as *mut libc::c_void);
            crate::stdlib::fclose(icc_file);
        } else if (*cinfo.err).msg_code != crate::jerror_h::JWRN_BOGUS_ICC as libc::c_int {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: no ICC profile data in JPEG file\n\x00" as *const u8 as *const libc::c_char,
                progname,
            );
        }
    }
    /* Finish decompression and release memory.
     * I must do it in this order because output module has allocated memory
     * of lifespan JPOOL_IMAGE; it needs to finish before releasing memory.
     */
    Some(
        (*dest_mgr)
            .finish_output
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(&mut cinfo, dest_mgr);
    crate::jpeglib_h::jpeg_finish_decompress(&mut cinfo);
    crate::jpeglib_h::jpeg_destroy_decompress(&mut cinfo);
    /* Close files, if we opened them */
    if input_file != crate::stdlib::stdin {
        crate::stdlib::fclose(input_file);
    }
    if output_file != crate::stdlib::stdout {
        crate::stdlib::fclose(output_file);
    }
    if memsrc != 0 && !inbuffer.is_null() {
        crate::stdlib::free(inbuffer as *mut libc::c_void);
    }
    /* All done. */
    crate::stdlib::exit(if jerr.num_warnings != 0 {
        crate::cdjpeg_h::EXIT_WARNING
    } else {
        crate::stdlib::EXIT_SUCCESS
    });
    /* suppress no-return-value warnings */
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
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
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
