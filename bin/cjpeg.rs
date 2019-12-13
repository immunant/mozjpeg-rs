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
        pub fn write_stdout() -> *mut crate::stdlib::FILE;

        #[no_mangle]
        pub fn read_stdin() -> *mut crate::stdlib::FILE;

        #[no_mangle]
        pub fn keymatch(
            arg: *mut libc::c_char,
            keyword: *const libc::c_char,
            minchars: libc::c_int,
        ) -> crate::jmorecfg_h::boolean;

        #[no_mangle]
        pub fn set_sample_factors(
            cinfo: crate::jpeglib_h::j_compress_ptr,
            arg: *mut libc::c_char,
        ) -> crate::jmorecfg_h::boolean;

        #[no_mangle]
        pub fn set_quant_slots(
            cinfo: crate::jpeglib_h::j_compress_ptr,
            arg: *mut libc::c_char,
        ) -> crate::jmorecfg_h::boolean;

        #[no_mangle]
        pub fn set_quality_ratings(
            cinfo: crate::jpeglib_h::j_compress_ptr,
            arg: *mut libc::c_char,
            force_baseline: crate::jmorecfg_h::boolean,
        ) -> crate::jmorecfg_h::boolean;

        #[no_mangle]
        pub fn read_scan_script(
            cinfo: crate::jpeglib_h::j_compress_ptr,
            filename: *mut libc::c_char,
        ) -> crate::jmorecfg_h::boolean;

        #[no_mangle]
        pub fn read_quant_tables(
            cinfo: crate::jpeglib_h::j_compress_ptr,
            filename: *mut libc::c_char,
            force_baseline: crate::jmorecfg_h::boolean,
        ) -> crate::jmorecfg_h::boolean;

        #[no_mangle]
        pub fn jinit_read_targa(
            cinfo: crate::jpeglib_h::j_compress_ptr,
        ) -> crate::cdjpeg_h::cjpeg_source_ptr;

        #[no_mangle]
        pub fn jinit_read_jpeg(
            cinfo: crate::jpeglib_h::j_compress_ptr,
        ) -> crate::cdjpeg_h::cjpeg_source_ptr;

        #[no_mangle]
        pub fn jinit_read_bmp(
            cinfo: crate::jpeglib_h::j_compress_ptr,
            use_inversion_array: crate::jmorecfg_h::boolean,
        ) -> crate::cdjpeg_h::cjpeg_source_ptr;

        #[no_mangle]
        pub fn jinit_read_gif(
            cinfo: crate::jpeglib_h::j_compress_ptr,
        ) -> crate::cdjpeg_h::cjpeg_source_ptr;

        #[no_mangle]
        pub fn jinit_read_ppm(
            cinfo: crate::jpeglib_h::j_compress_ptr,
        ) -> crate::cdjpeg_h::cjpeg_source_ptr;
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cjpeg_source_struct {
        pub start_input: Option<
            unsafe extern "C" fn(
                _: crate::jpeglib_h::j_compress_ptr,
                _: crate::cdjpeg_h::cjpeg_source_ptr,
            ) -> (),
        >,
        pub get_pixel_rows: Option<
            unsafe extern "C" fn(
                _: crate::jpeglib_h::j_compress_ptr,
                _: crate::cdjpeg_h::cjpeg_source_ptr,
            ) -> crate::jmorecfg_h::JDIMENSION,
        >,
        pub finish_input: Option<
            unsafe extern "C" fn(
                _: crate::jpeglib_h::j_compress_ptr,
                _: crate::cdjpeg_h::cjpeg_source_ptr,
            ) -> (),
        >,
        pub input_file: *mut crate::stdlib::FILE,
        pub buffer: crate::jpeglib_h::JSAMPARRAY,
        pub buffer_height: crate::jmorecfg_h::JDIMENSION,
        pub marker_list: crate::jpeglib_h::jpeg_saved_marker_ptr,
    }

    pub type cjpeg_source_ptr = *mut crate::cdjpeg_h::cjpeg_source_struct;
    pub const WRITE_BINARY: [libc::c_char; 3] =
        unsafe { *::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"wb\x00") };

    pub const READ_BINARY: [libc::c_char; 3] =
        unsafe { *::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"rb\x00") };
    /* define exit() codes if not provided */

    pub const EXIT_WARNING: libc::c_int = 2 as libc::c_int;
}
pub mod cderror_h {
    pub const JMSG_LASTADDONCODE: crate::jpeglib_h::C2RustUnnamed_0 = 1046;

    pub const JERR_UNSUPPORTED_FORMAT: crate::jpeglib_h::C2RustUnnamed_0 = 1045;

    pub const JERR_UNKNOWN_FORMAT: crate::jpeglib_h::C2RustUnnamed_0 = 1044;

    pub const JERR_UNGETC_FAILED: crate::jpeglib_h::C2RustUnnamed_0 = 1043;

    pub const JERR_TOO_MANY_COLORS: crate::jpeglib_h::C2RustUnnamed_0 = 1042;
    /* TARGA_SUPPORTED */
    /* TARGA_SUPPORTED */

    pub const JERR_BAD_CMAP_FILE: crate::jpeglib_h::C2RustUnnamed_0 = 1041;

    pub const JTRC_TGA_MAPPED: crate::jpeglib_h::C2RustUnnamed_0 = 1040;

    pub const JTRC_TGA_GRAY: crate::jpeglib_h::C2RustUnnamed_0 = 1039;

    pub const JTRC_TGA: crate::jpeglib_h::C2RustUnnamed_0 = 1038;

    pub const JERR_TGA_COLORSPACE: crate::jpeglib_h::C2RustUnnamed_0 = 1037;

    pub const JERR_TGA_BADPARMS: crate::jpeglib_h::C2RustUnnamed_0 = 1036;
    /* PPM_SUPPORTED */
    /* PPM_SUPPORTED */
    /* RLE_SUPPORTED */
    /* RLE_SUPPORTED */

    pub const JERR_TGA_BADCMAP: crate::jpeglib_h::C2RustUnnamed_0 = 1035;

    pub const JTRC_PPM_TEXT: crate::jpeglib_h::C2RustUnnamed_0 = 1034;

    pub const JTRC_PPM: crate::jpeglib_h::C2RustUnnamed_0 = 1033;

    pub const JTRC_PGM_TEXT: crate::jpeglib_h::C2RustUnnamed_0 = 1032;

    pub const JTRC_PGM: crate::jpeglib_h::C2RustUnnamed_0 = 1031;

    pub const JERR_PPM_OUTOFRANGE: crate::jpeglib_h::C2RustUnnamed_0 = 1030;

    pub const JERR_PPM_NOT: crate::jpeglib_h::C2RustUnnamed_0 = 1029;

    pub const JERR_PPM_NONNUMERIC: crate::jpeglib_h::C2RustUnnamed_0 = 1028;
    /* GIF_SUPPORTED */
    /* GIF_SUPPORTED */

    pub const JERR_PPM_COLORSPACE: crate::jpeglib_h::C2RustUnnamed_0 = 1027;

    pub const JWRN_GIF_NOMOREDATA: crate::jpeglib_h::C2RustUnnamed_0 = 1026;

    pub const JWRN_GIF_ENDCODE: crate::jpeglib_h::C2RustUnnamed_0 = 1025;

    pub const JWRN_GIF_CHAR: crate::jpeglib_h::C2RustUnnamed_0 = 1024;

    pub const JWRN_GIF_BADDATA: crate::jpeglib_h::C2RustUnnamed_0 = 1023;

    pub const JTRC_GIF_NONSQUARE: crate::jpeglib_h::C2RustUnnamed_0 = 1022;

    pub const JTRC_GIF_EXTENSION: crate::jpeglib_h::C2RustUnnamed_0 = 1021;

    pub const JTRC_GIF_BADVERSION: crate::jpeglib_h::C2RustUnnamed_0 = 1020;

    pub const JTRC_GIF: crate::jpeglib_h::C2RustUnnamed_0 = 1019;

    pub const JERR_GIF_NOT: crate::jpeglib_h::C2RustUnnamed_0 = 1018;

    pub const JERR_GIF_IMAGENOTFOUND: crate::jpeglib_h::C2RustUnnamed_0 = 1017;

    pub const JERR_GIF_COLORSPACE: crate::jpeglib_h::C2RustUnnamed_0 = 1016;

    pub const JERR_GIF_CODESIZE: crate::jpeglib_h::C2RustUnnamed_0 = 1015;
    /* BMP_SUPPORTED */
    /* BMP_SUPPORTED */

    pub const JERR_GIF_BUG: crate::jpeglib_h::C2RustUnnamed_0 = 1014;

    pub const JTRC_BMP_OS2_MAPPED: crate::jpeglib_h::C2RustUnnamed_0 = 1013;

    pub const JTRC_BMP_OS2: crate::jpeglib_h::C2RustUnnamed_0 = 1012;

    pub const JTRC_BMP_MAPPED: crate::jpeglib_h::C2RustUnnamed_0 = 1011;

    pub const JTRC_BMP: crate::jpeglib_h::C2RustUnnamed_0 = 1010;

    pub const JERR_BMP_OUTOFRANGE: crate::jpeglib_h::C2RustUnnamed_0 = 1009;

    pub const JERR_BMP_NOT: crate::jpeglib_h::C2RustUnnamed_0 = 1008;

    pub const JERR_BMP_EMPTY: crate::jpeglib_h::C2RustUnnamed_0 = 1007;

    pub const JERR_BMP_COMPRESSED: crate::jpeglib_h::C2RustUnnamed_0 = 1006;

    pub const JERR_BMP_COLORSPACE: crate::jpeglib_h::C2RustUnnamed_0 = 1005;

    pub const JERR_BMP_BADPLANES: crate::jpeglib_h::C2RustUnnamed_0 = 1004;

    pub const JERR_BMP_BADHEADER: crate::jpeglib_h::C2RustUnnamed_0 = 1003;

    pub const JERR_BMP_BADDEPTH: crate::jpeglib_h::C2RustUnnamed_0 = 1002;
    /* Must be first entry! */
    /* Must be first entry! */

    pub const JERR_BMP_BADCMAP: crate::jpeglib_h::C2RustUnnamed_0 = 1001;
    /* JMAKE_ENUM_LIST */
    /* JMAKE_ENUM_LIST */

    pub const JMSG_FIRSTADDONCODE: crate::jpeglib_h::C2RustUnnamed_0 = 1000;
}
pub mod jerror_h {
    pub const JMSG_LASTMSGCODE: crate::jpeglib_h::C2RustUnnamed_0 = 129;

    pub const JWRN_BOGUS_ICC: crate::jpeglib_h::C2RustUnnamed_0 = 128;

    pub const JERR_UNSUPPORTED_SUSPEND: crate::jpeglib_h::C2RustUnnamed_0 = 127;

    pub const JERR_BAD_PARAM_VALUE: crate::jpeglib_h::C2RustUnnamed_0 = 126;

    pub const JERR_BAD_PARAM: crate::jpeglib_h::C2RustUnnamed_0 = 125;

    pub const JERR_BAD_CROP_SPEC: crate::jpeglib_h::C2RustUnnamed_0 = 124;

    pub const JWRN_TOO_MUCH_DATA: crate::jpeglib_h::C2RustUnnamed_0 = 123;

    pub const JWRN_NOT_SEQUENTIAL: crate::jpeglib_h::C2RustUnnamed_0 = 122;

    pub const JWRN_MUST_RESYNC: crate::jpeglib_h::C2RustUnnamed_0 = 121;

    pub const JWRN_JPEG_EOF: crate::jpeglib_h::C2RustUnnamed_0 = 120;

    pub const JWRN_JFIF_MAJOR: crate::jpeglib_h::C2RustUnnamed_0 = 119;

    pub const JWRN_HUFF_BAD_CODE: crate::jpeglib_h::C2RustUnnamed_0 = 118;

    pub const JWRN_HIT_MARKER: crate::jpeglib_h::C2RustUnnamed_0 = 117;

    pub const JWRN_EXTRANEOUS_DATA: crate::jpeglib_h::C2RustUnnamed_0 = 116;

    pub const JWRN_BOGUS_PROGRESSION: crate::jpeglib_h::C2RustUnnamed_0 = 115;

    pub const JWRN_ADOBE_XFORM: crate::jpeglib_h::C2RustUnnamed_0 = 114;

    pub const JTRC_XMS_OPEN: crate::jpeglib_h::C2RustUnnamed_0 = 113;

    pub const JTRC_XMS_CLOSE: crate::jpeglib_h::C2RustUnnamed_0 = 112;

    pub const JTRC_UNKNOWN_IDS: crate::jpeglib_h::C2RustUnnamed_0 = 111;

    pub const JTRC_THUMB_RGB: crate::jpeglib_h::C2RustUnnamed_0 = 110;

    pub const JTRC_THUMB_PALETTE: crate::jpeglib_h::C2RustUnnamed_0 = 109;

    pub const JTRC_THUMB_JPEG: crate::jpeglib_h::C2RustUnnamed_0 = 108;

    pub const JTRC_TFILE_OPEN: crate::jpeglib_h::C2RustUnnamed_0 = 107;

    pub const JTRC_TFILE_CLOSE: crate::jpeglib_h::C2RustUnnamed_0 = 106;

    pub const JTRC_SOS_PARAMS: crate::jpeglib_h::C2RustUnnamed_0 = 105;

    pub const JTRC_SOS_COMPONENT: crate::jpeglib_h::C2RustUnnamed_0 = 104;

    pub const JTRC_SOS: crate::jpeglib_h::C2RustUnnamed_0 = 103;

    pub const JTRC_SOI: crate::jpeglib_h::C2RustUnnamed_0 = 102;

    pub const JTRC_SOF_COMPONENT: crate::jpeglib_h::C2RustUnnamed_0 = 101;

    pub const JTRC_SOF: crate::jpeglib_h::C2RustUnnamed_0 = 100;

    pub const JTRC_SMOOTH_NOTIMPL: crate::jpeglib_h::C2RustUnnamed_0 = 99;

    pub const JTRC_RST: crate::jpeglib_h::C2RustUnnamed_0 = 98;

    pub const JTRC_RECOVERY_ACTION: crate::jpeglib_h::C2RustUnnamed_0 = 97;

    pub const JTRC_QUANT_SELECTED: crate::jpeglib_h::C2RustUnnamed_0 = 96;

    pub const JTRC_QUANT_NCOLORS: crate::jpeglib_h::C2RustUnnamed_0 = 95;

    pub const JTRC_QUANT_3_NCOLORS: crate::jpeglib_h::C2RustUnnamed_0 = 94;

    pub const JTRC_QUANTVALS: crate::jpeglib_h::C2RustUnnamed_0 = 93;

    pub const JTRC_PARMLESS_MARKER: crate::jpeglib_h::C2RustUnnamed_0 = 92;

    pub const JTRC_MISC_MARKER: crate::jpeglib_h::C2RustUnnamed_0 = 91;

    pub const JTRC_JFIF_THUMBNAIL: crate::jpeglib_h::C2RustUnnamed_0 = 90;

    pub const JTRC_JFIF_EXTENSION: crate::jpeglib_h::C2RustUnnamed_0 = 89;

    pub const JTRC_JFIF_BADTHUMBNAILSIZE: crate::jpeglib_h::C2RustUnnamed_0 = 88;

    pub const JTRC_JFIF: crate::jpeglib_h::C2RustUnnamed_0 = 87;

    pub const JTRC_HUFFBITS: crate::jpeglib_h::C2RustUnnamed_0 = 86;

    pub const JTRC_EOI: crate::jpeglib_h::C2RustUnnamed_0 = 85;

    pub const JTRC_EMS_OPEN: crate::jpeglib_h::C2RustUnnamed_0 = 84;

    pub const JTRC_EMS_CLOSE: crate::jpeglib_h::C2RustUnnamed_0 = 83;

    pub const JTRC_DRI: crate::jpeglib_h::C2RustUnnamed_0 = 82;

    pub const JTRC_DQT: crate::jpeglib_h::C2RustUnnamed_0 = 81;

    pub const JTRC_DHT: crate::jpeglib_h::C2RustUnnamed_0 = 80;

    pub const JTRC_DAC: crate::jpeglib_h::C2RustUnnamed_0 = 79;

    pub const JTRC_APP14: crate::jpeglib_h::C2RustUnnamed_0 = 78;

    pub const JTRC_APP0: crate::jpeglib_h::C2RustUnnamed_0 = 77;

    pub const JTRC_ADOBE: crate::jpeglib_h::C2RustUnnamed_0 = 76;

    pub const JTRC_16BIT_TABLES: crate::jpeglib_h::C2RustUnnamed_0 = 75;

    pub const JMSG_VERSION: crate::jpeglib_h::C2RustUnnamed_0 = 74;

    pub const JMSG_COPYRIGHT: crate::jpeglib_h::C2RustUnnamed_0 = 73;

    pub const JERR_XMS_WRITE: crate::jpeglib_h::C2RustUnnamed_0 = 72;

    pub const JERR_XMS_READ: crate::jpeglib_h::C2RustUnnamed_0 = 71;

    pub const JERR_WIDTH_OVERFLOW: crate::jpeglib_h::C2RustUnnamed_0 = 70;

    pub const JERR_VIRTUAL_BUG: crate::jpeglib_h::C2RustUnnamed_0 = 69;

    pub const JERR_UNKNOWN_MARKER: crate::jpeglib_h::C2RustUnnamed_0 = 68;

    pub const JERR_TOO_LITTLE_DATA: crate::jpeglib_h::C2RustUnnamed_0 = 67;

    pub const JERR_TFILE_WRITE: crate::jpeglib_h::C2RustUnnamed_0 = 66;

    pub const JERR_TFILE_SEEK: crate::jpeglib_h::C2RustUnnamed_0 = 65;

    pub const JERR_TFILE_READ: crate::jpeglib_h::C2RustUnnamed_0 = 64;

    pub const JERR_TFILE_CREATE: crate::jpeglib_h::C2RustUnnamed_0 = 63;

    pub const JERR_SOS_NO_SOF: crate::jpeglib_h::C2RustUnnamed_0 = 62;

    pub const JERR_SOI_DUPLICATE: crate::jpeglib_h::C2RustUnnamed_0 = 61;

    pub const JERR_SOF_UNSUPPORTED: crate::jpeglib_h::C2RustUnnamed_0 = 60;

    pub const JERR_SOF_NO_SOS: crate::jpeglib_h::C2RustUnnamed_0 = 59;

    pub const JERR_SOF_DUPLICATE: crate::jpeglib_h::C2RustUnnamed_0 = 58;

    pub const JERR_QUANT_MANY_COLORS: crate::jpeglib_h::C2RustUnnamed_0 = 57;

    pub const JERR_QUANT_FEW_COLORS: crate::jpeglib_h::C2RustUnnamed_0 = 56;

    pub const JERR_QUANT_COMPONENTS: crate::jpeglib_h::C2RustUnnamed_0 = 55;

    pub const JERR_OUT_OF_MEMORY: crate::jpeglib_h::C2RustUnnamed_0 = 54;

    pub const JERR_NO_SOI: crate::jpeglib_h::C2RustUnnamed_0 = 53;

    pub const JERR_NO_QUANT_TABLE: crate::jpeglib_h::C2RustUnnamed_0 = 52;

    pub const JERR_NO_IMAGE: crate::jpeglib_h::C2RustUnnamed_0 = 51;

    pub const JERR_NO_HUFF_TABLE: crate::jpeglib_h::C2RustUnnamed_0 = 50;

    pub const JERR_NO_BACKING_STORE: crate::jpeglib_h::C2RustUnnamed_0 = 49;

    pub const JERR_NOT_COMPILED: crate::jpeglib_h::C2RustUnnamed_0 = 48;

    pub const JERR_NOTIMPL: crate::jpeglib_h::C2RustUnnamed_0 = 47;

    pub const JERR_MODE_CHANGE: crate::jpeglib_h::C2RustUnnamed_0 = 46;

    pub const JERR_MISSING_DATA: crate::jpeglib_h::C2RustUnnamed_0 = 45;

    pub const JERR_MISMATCHED_QUANT_TABLE: crate::jpeglib_h::C2RustUnnamed_0 = 44;

    pub const JERR_INPUT_EOF: crate::jpeglib_h::C2RustUnnamed_0 = 43;

    pub const JERR_INPUT_EMPTY: crate::jpeglib_h::C2RustUnnamed_0 = 42;

    pub const JERR_IMAGE_TOO_BIG: crate::jpeglib_h::C2RustUnnamed_0 = 41;

    pub const JERR_HUFF_MISSING_CODE: crate::jpeglib_h::C2RustUnnamed_0 = 40;

    pub const JERR_HUFF_CLEN_OVERFLOW: crate::jpeglib_h::C2RustUnnamed_0 = 39;

    pub const JERR_FRACT_SAMPLE_NOTIMPL: crate::jpeglib_h::C2RustUnnamed_0 = 38;

    pub const JERR_FILE_WRITE: crate::jpeglib_h::C2RustUnnamed_0 = 37;

    pub const JERR_FILE_READ: crate::jpeglib_h::C2RustUnnamed_0 = 36;

    pub const JERR_EOI_EXPECTED: crate::jpeglib_h::C2RustUnnamed_0 = 35;

    pub const JERR_EMS_WRITE: crate::jpeglib_h::C2RustUnnamed_0 = 34;

    pub const JERR_EMS_READ: crate::jpeglib_h::C2RustUnnamed_0 = 33;

    pub const JERR_EMPTY_IMAGE: crate::jpeglib_h::C2RustUnnamed_0 = 32;

    pub const JERR_DQT_INDEX: crate::jpeglib_h::C2RustUnnamed_0 = 31;

    pub const JERR_DHT_INDEX: crate::jpeglib_h::C2RustUnnamed_0 = 30;

    pub const JERR_DAC_VALUE: crate::jpeglib_h::C2RustUnnamed_0 = 29;

    pub const JERR_DAC_INDEX: crate::jpeglib_h::C2RustUnnamed_0 = 28;

    pub const JERR_CONVERSION_NOTIMPL: crate::jpeglib_h::C2RustUnnamed_0 = 27;

    pub const JERR_COMPONENT_COUNT: crate::jpeglib_h::C2RustUnnamed_0 = 26;

    pub const JERR_CCIR601_NOTIMPL: crate::jpeglib_h::C2RustUnnamed_0 = 25;

    pub const JERR_CANT_SUSPEND: crate::jpeglib_h::C2RustUnnamed_0 = 24;

    pub const JERR_BUFFER_SIZE: crate::jpeglib_h::C2RustUnnamed_0 = 23;

    pub const JERR_BAD_VIRTUAL_ACCESS: crate::jpeglib_h::C2RustUnnamed_0 = 22;

    pub const JERR_BAD_STRUCT_SIZE: crate::jpeglib_h::C2RustUnnamed_0 = 21;

    pub const JERR_BAD_STATE: crate::jpeglib_h::C2RustUnnamed_0 = 20;

    pub const JERR_BAD_SCAN_SCRIPT: crate::jpeglib_h::C2RustUnnamed_0 = 19;

    pub const JERR_BAD_SAMPLING: crate::jpeglib_h::C2RustUnnamed_0 = 18;

    pub const JERR_BAD_PROG_SCRIPT: crate::jpeglib_h::C2RustUnnamed_0 = 17;

    pub const JERR_BAD_PROGRESSION: crate::jpeglib_h::C2RustUnnamed_0 = 16;

    pub const JERR_BAD_PRECISION: crate::jpeglib_h::C2RustUnnamed_0 = 15;

    pub const JERR_BAD_POOL_ID: crate::jpeglib_h::C2RustUnnamed_0 = 14;

    pub const JERR_BAD_MCU_SIZE: crate::jpeglib_h::C2RustUnnamed_0 = 13;

    pub const JERR_BAD_LIB_VERSION: crate::jpeglib_h::C2RustUnnamed_0 = 12;

    pub const JERR_BAD_LENGTH: crate::jpeglib_h::C2RustUnnamed_0 = 11;

    pub const JERR_BAD_J_COLORSPACE: crate::jpeglib_h::C2RustUnnamed_0 = 10;

    pub const JERR_BAD_IN_COLORSPACE: crate::jpeglib_h::C2RustUnnamed_0 = 9;

    pub const JERR_BAD_HUFF_TABLE: crate::jpeglib_h::C2RustUnnamed_0 = 8;

    pub const JERR_BAD_DCTSIZE: crate::jpeglib_h::C2RustUnnamed_0 = 7;

    pub const JERR_BAD_DCT_COEF: crate::jpeglib_h::C2RustUnnamed_0 = 6;

    pub const JERR_BAD_COMPONENT_ID: crate::jpeglib_h::C2RustUnnamed_0 = 5;

    pub const JERR_BAD_BUFFER_MODE: crate::jpeglib_h::C2RustUnnamed_0 = 4;

    pub const JERR_BAD_ALLOC_CHUNK: crate::jpeglib_h::C2RustUnnamed_0 = 3;

    pub const JERR_BAD_ALIGN_TYPE: crate::jpeglib_h::C2RustUnnamed_0 = 2;
    /* Must be first entry! */
    /* For maintenance convenience, list is alphabetical by message code name */

    pub const JERR_ARITH_NOTIMPL: crate::jpeglib_h::C2RustUnnamed_0 = 1;
    /* JMAKE_ENUM_LIST */

    pub const JMSG_NOMESSAGE: crate::jpeglib_h::C2RustUnnamed_0 = 0;
}
pub mod jpeglib_h {
    extern "C" {
        pub type jvirt_barray_control;

        pub type jvirt_sarray_control;

        pub type jpeg_entropy_encoder;

        pub type jpeg_forward_dct;

        pub type jpeg_downsampler;

        pub type jpeg_color_converter;

        pub type jpeg_marker_writer;

        pub type jpeg_c_coef_controller;

        pub type jpeg_c_prep_controller;

        pub type jpeg_c_main_controller;

        pub type jpeg_comp_master;

        #[no_mangle]
        pub fn jpeg_std_error(
            err: *mut crate::jpeglib_h::jpeg_error_mgr,
        ) -> *mut crate::jpeglib_h::jpeg_error_mgr;

        #[no_mangle]
        pub fn jpeg_CreateCompress(
            cinfo: crate::jpeglib_h::j_compress_ptr,
            version: libc::c_int,
            structsize: crate::stddef_h::size_t,
        );

        #[no_mangle]
        pub fn jpeg_destroy_compress(cinfo: crate::jpeglib_h::j_compress_ptr);

        #[no_mangle]
        pub fn jpeg_stdio_dest(
            cinfo: crate::jpeglib_h::j_compress_ptr,
            outfile: *mut crate::stdlib::FILE,
        );

        #[no_mangle]
        pub fn jpeg_mem_dest(
            cinfo: crate::jpeglib_h::j_compress_ptr,
            outbuffer: *mut *mut libc::c_uchar,
            outsize: *mut libc::c_ulong,
        );

        #[no_mangle]
        pub fn jpeg_set_defaults(cinfo: crate::jpeglib_h::j_compress_ptr);

        #[no_mangle]
        pub fn jpeg_set_colorspace(
            cinfo: crate::jpeglib_h::j_compress_ptr,
            colorspace: crate::jpeglib_h::J_COLOR_SPACE,
        );

        #[no_mangle]
        pub fn jpeg_default_colorspace(cinfo: crate::jpeglib_h::j_compress_ptr);

        #[no_mangle]
        pub fn jpeg_set_quality(
            cinfo: crate::jpeglib_h::j_compress_ptr,
            quality: libc::c_int,
            force_baseline: crate::jmorecfg_h::boolean,
        );

        #[no_mangle]
        pub fn jpeg_simple_progression(cinfo: crate::jpeglib_h::j_compress_ptr);

        #[no_mangle]
        pub fn jpeg_start_compress(
            cinfo: crate::jpeglib_h::j_compress_ptr,
            write_all_tables: crate::jmorecfg_h::boolean,
        );

        #[no_mangle]
        pub fn jpeg_write_scanlines(
            cinfo: crate::jpeglib_h::j_compress_ptr,
            scanlines: crate::jpeglib_h::JSAMPARRAY,
            num_lines: crate::jmorecfg_h::JDIMENSION,
        ) -> crate::jmorecfg_h::JDIMENSION;

        #[no_mangle]
        pub fn jpeg_finish_compress(cinfo: crate::jpeglib_h::j_compress_ptr);

        #[no_mangle]
        pub fn jpeg_write_marker(
            cinfo: crate::jpeglib_h::j_compress_ptr,
            marker: libc::c_int,
            dataptr: *const crate::jmorecfg_h::JOCTET,
            datalen: libc::c_uint,
        );

        #[no_mangle]
        pub fn jpeg_write_icc_profile(
            cinfo: crate::jpeglib_h::j_compress_ptr,
            icc_data_ptr: *const crate::jmorecfg_h::JOCTET,
            icc_data_len: libc::c_uint,
        );

        #[no_mangle]
        pub fn jpeg_c_set_bool_param(
            cinfo: crate::jpeglib_h::j_compress_ptr,
            param: crate::jpeglib_h::J_BOOLEAN_PARAM,
            value: crate::jmorecfg_h::boolean,
        );

        #[no_mangle]
        pub fn jpeg_c_set_float_param(
            cinfo: crate::jpeglib_h::j_compress_ptr,
            param: crate::jpeglib_h::J_FLOAT_PARAM,
            value: libc::c_float,
        );

        #[no_mangle]
        pub fn jpeg_c_set_int_param(
            cinfo: crate::jpeglib_h::j_compress_ptr,
            param: crate::jpeglib_h::J_INT_PARAM,
            value: libc::c_int,
        );

        #[no_mangle]
        pub fn jpeg_c_get_int_param(
            cinfo: crate::jpeglib_h::j_compress_ptr,
            param: crate::jpeglib_h::J_INT_PARAM,
        ) -> libc::c_int;
    }
    pub type C2RustUnnamed_0 = libc::c_uint;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union C2RustUnnamed_1 {
        pub i: [libc::c_int; 8],
        pub s: [libc::c_char; 80],
    }

    pub type JSAMPROW = *mut crate::jmorecfg_h::JSAMPLE;

    pub type JSAMPARRAY = *mut crate::jpeglib_h::JSAMPROW;

    pub type JBLOCK = [crate::jmorecfg_h::JCOEF; 64];

    pub type JBLOCKROW = *mut crate::jpeglib_h::JBLOCK;

    pub type JBLOCKARRAY = *mut crate::jpeglib_h::JBLOCKROW;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct JQUANT_TBL {
        pub quantval: [crate::jmorecfg_h::UINT16; 64],
        pub sent_table: crate::jmorecfg_h::boolean,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct JHUFF_TBL {
        pub bits: [crate::jmorecfg_h::UINT8; 17],
        pub huffval: [crate::jmorecfg_h::UINT8; 256],
        pub sent_table: crate::jmorecfg_h::boolean,
    }

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

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct jpeg_scan_info {
        pub comps_in_scan: libc::c_int,
        pub component_index: [libc::c_int; 4],
        pub Ss: libc::c_int,
        pub Se: libc::c_int,
        pub Ah: libc::c_int,
        pub Al: libc::c_int,
    }

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

    pub type J_COLOR_SPACE = libc::c_uint;

    pub type J_DCT_METHOD = libc::c_uint;

    pub type J_BOOLEAN_PARAM = libc::c_uint;

    pub type J_FLOAT_PARAM = libc::c_uint;

    pub type J_INT_PARAM = libc::c_uint;

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
        pub msg_parm: crate::jpeglib_h::C2RustUnnamed_1,
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
    pub struct jpeg_compress_struct {
        pub err: *mut crate::jpeglib_h::jpeg_error_mgr,
        pub mem: *mut crate::jpeglib_h::jpeg_memory_mgr,
        pub progress: *mut crate::jpeglib_h::jpeg_progress_mgr,
        pub client_data: *mut libc::c_void,
        pub is_decompressor: crate::jmorecfg_h::boolean,
        pub global_state: libc::c_int,
        pub dest: *mut crate::jpeglib_h::jpeg_destination_mgr,
        pub image_width: crate::jmorecfg_h::JDIMENSION,
        pub image_height: crate::jmorecfg_h::JDIMENSION,
        pub input_components: libc::c_int,
        pub in_color_space: crate::jpeglib_h::J_COLOR_SPACE,
        pub input_gamma: libc::c_double,
        pub data_precision: libc::c_int,
        pub num_components: libc::c_int,
        pub jpeg_color_space: crate::jpeglib_h::J_COLOR_SPACE,
        pub comp_info: *mut crate::jpeglib_h::jpeg_component_info,
        pub quant_tbl_ptrs: [*mut crate::jpeglib_h::JQUANT_TBL; 4],
        pub dc_huff_tbl_ptrs: [*mut crate::jpeglib_h::JHUFF_TBL; 4],
        pub ac_huff_tbl_ptrs: [*mut crate::jpeglib_h::JHUFF_TBL; 4],
        pub arith_dc_L: [crate::jmorecfg_h::UINT8; 16],
        pub arith_dc_U: [crate::jmorecfg_h::UINT8; 16],
        pub arith_ac_K: [crate::jmorecfg_h::UINT8; 16],
        pub num_scans: libc::c_int,
        pub scan_info: *const crate::jpeglib_h::jpeg_scan_info,
        pub raw_data_in: crate::jmorecfg_h::boolean,
        pub arith_code: crate::jmorecfg_h::boolean,
        pub optimize_coding: crate::jmorecfg_h::boolean,
        pub CCIR601_sampling: crate::jmorecfg_h::boolean,
        pub smoothing_factor: libc::c_int,
        pub dct_method: crate::jpeglib_h::J_DCT_METHOD,
        pub restart_interval: libc::c_uint,
        pub restart_in_rows: libc::c_int,
        pub write_JFIF_header: crate::jmorecfg_h::boolean,
        pub JFIF_major_version: crate::jmorecfg_h::UINT8,
        pub JFIF_minor_version: crate::jmorecfg_h::UINT8,
        pub density_unit: crate::jmorecfg_h::UINT8,
        pub X_density: crate::jmorecfg_h::UINT16,
        pub Y_density: crate::jmorecfg_h::UINT16,
        pub write_Adobe_marker: crate::jmorecfg_h::boolean,
        pub next_scanline: crate::jmorecfg_h::JDIMENSION,
        pub progressive_mode: crate::jmorecfg_h::boolean,
        pub max_h_samp_factor: libc::c_int,
        pub max_v_samp_factor: libc::c_int,
        pub total_iMCU_rows: crate::jmorecfg_h::JDIMENSION,
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
        pub master: *mut crate::jpeglib_h::jpeg_comp_master,
        pub main: *mut crate::jpeglib_h::jpeg_c_main_controller,
        pub prep: *mut crate::jpeglib_h::jpeg_c_prep_controller,
        pub coef: *mut crate::jpeglib_h::jpeg_c_coef_controller,
        pub marker: *mut crate::jpeglib_h::jpeg_marker_writer,
        pub cconvert: *mut crate::jpeglib_h::jpeg_color_converter,
        pub downsample: *mut crate::jpeglib_h::jpeg_downsampler,
        pub fdct: *mut crate::jpeglib_h::jpeg_forward_dct,
        pub entropy: *mut crate::jpeglib_h::jpeg_entropy_encoder,
        pub script_space: *mut crate::jpeglib_h::jpeg_scan_info,
        pub script_space_size: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct jpeg_destination_mgr {
        pub next_output_byte: *mut crate::jmorecfg_h::JOCTET,
        pub free_in_buffer: crate::stddef_h::size_t,
        pub init_destination:
            Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
        pub empty_output_buffer: Option<
            unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> crate::jmorecfg_h::boolean,
        >,
        pub term_destination:
            Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    }

    pub type j_compress_ptr = *mut crate::jpeglib_h::jpeg_compress_struct;

    pub const JCS_RGB565: crate::jpeglib_h::J_COLOR_SPACE = 16;

    pub const JCS_EXT_ARGB: crate::jpeglib_h::J_COLOR_SPACE = 15;

    pub const JCS_EXT_ABGR: crate::jpeglib_h::J_COLOR_SPACE = 14;

    pub const JCS_EXT_BGRA: crate::jpeglib_h::J_COLOR_SPACE = 13;

    pub const JCS_EXT_RGBA: crate::jpeglib_h::J_COLOR_SPACE = 12;

    pub const JCS_EXT_XRGB: crate::jpeglib_h::J_COLOR_SPACE = 11;

    pub const JCS_EXT_XBGR: crate::jpeglib_h::J_COLOR_SPACE = 10;

    pub const JCS_EXT_BGRX: crate::jpeglib_h::J_COLOR_SPACE = 9;

    pub const JCS_EXT_BGR: crate::jpeglib_h::J_COLOR_SPACE = 8;

    pub const JCS_EXT_RGBX: crate::jpeglib_h::J_COLOR_SPACE = 7;

    pub const JCS_EXT_RGB: crate::jpeglib_h::J_COLOR_SPACE = 6;

    pub const JCS_YCCK: crate::jpeglib_h::J_COLOR_SPACE = 5;

    pub const JCS_CMYK: crate::jpeglib_h::J_COLOR_SPACE = 4;

    pub const JCS_YCbCr: crate::jpeglib_h::J_COLOR_SPACE = 3;

    pub const JCS_RGB: crate::jpeglib_h::J_COLOR_SPACE = 2;

    pub const JCS_GRAYSCALE: crate::jpeglib_h::J_COLOR_SPACE = 1;

    pub const JCS_UNKNOWN: crate::jpeglib_h::J_COLOR_SPACE = 0;

    pub const JDCT_FLOAT: crate::jpeglib_h::J_DCT_METHOD = 2;

    pub const JDCT_IFAST: crate::jpeglib_h::J_DCT_METHOD = 1;

    pub const JDCT_ISLOW: crate::jpeglib_h::J_DCT_METHOD = 0;

    pub const JBOOLEAN_OVERSHOOT_DERINGING: crate::jpeglib_h::J_BOOLEAN_PARAM = 1061927929;

    pub const JBOOLEAN_TRELLIS_Q_OPT: crate::jpeglib_h::J_BOOLEAN_PARAM = 3777684073;

    pub const JBOOLEAN_USE_SCANS_IN_TRELLIS: crate::jpeglib_h::J_BOOLEAN_PARAM = 4253291573;

    pub const JBOOLEAN_USE_LAMBDA_WEIGHT_TBL: crate::jpeglib_h::J_BOOLEAN_PARAM = 865973855;

    pub const JBOOLEAN_TRELLIS_EOB_OPT: crate::jpeglib_h::J_BOOLEAN_PARAM = 3623303040;

    pub const JBOOLEAN_TRELLIS_QUANT_DC: crate::jpeglib_h::J_BOOLEAN_PARAM = 865946636;

    pub const JBOOLEAN_TRELLIS_QUANT: crate::jpeglib_h::J_BOOLEAN_PARAM = 3306299443;

    pub const JBOOLEAN_OPTIMIZE_SCANS: crate::jpeglib_h::J_BOOLEAN_PARAM = 1745618462;

    pub const JFLOAT_TRELLIS_DELTA_DC_WEIGHT: crate::jpeglib_h::J_FLOAT_PARAM = 326587475;

    pub const JFLOAT_LAMBDA_LOG_SCALE2: crate::jpeglib_h::J_FLOAT_PARAM = 3116084739;

    pub const JFLOAT_LAMBDA_LOG_SCALE1: crate::jpeglib_h::J_FLOAT_PARAM = 1533126041;

    pub const JINT_DC_SCAN_OPT_MODE: crate::jpeglib_h::J_INT_PARAM = 199732540;

    pub const JINT_BASE_QUANT_TBL_IDX: crate::jpeglib_h::J_INT_PARAM = 1145645745;

    pub const JINT_TRELLIS_NUM_LOOPS: crate::jpeglib_h::J_INT_PARAM = 3057565497;

    pub const JINT_TRELLIS_FREQ_SPLIT: crate::jpeglib_h::J_INT_PARAM = 1873801511;

    pub const JINT_COMPRESS_PROFILE: crate::jpeglib_h::J_INT_PARAM = 3918628389;

    pub const JCP_FASTEST: crate::jpeglib_h::C2RustUnnamed_0 = 720002228;

    pub const JCP_MAX_COMPRESSION: crate::jpeglib_h::C2RustUnnamed_0 = 1560820397;
    pub const JPEG_APP0: libc::c_int = 0xe0 as libc::c_int;

    pub const JDCT_DEFAULT: libc::c_int = crate::jpeglib_h::JDCT_ISLOW as libc::c_int;
}
pub mod jmorecfg_h {
    pub type JSAMPLE = libc::c_uchar;

    pub type JCOEF = libc::c_short;

    pub type JOCTET = libc::c_uchar;

    pub type UINT8 = libc::c_uchar;

    pub type UINT16 = libc::c_ushort;

    pub type JDIMENSION = libc::c_uint;

    pub type boolean = libc::c_int;
    /*
     * jmorecfg.h
     *
     * This file was part of the Independent JPEG Group's software:
     * Copyright (C) 1991-1997, Thomas G. Lane.
     * Modified 1997-2009 by Guido Vollbeding.
     * libjpeg-turbo Modifications:
     * Copyright (C) 2009, 2011, 2014-2015, 2018, D. R. Commander.
     * For conditions of distribution and use, see the accompanying README.ijg
     * file.
     *
     * This file contains additional configuration options that customize the
     * JPEG software for special applications or support machine-dependent
     * optimizations.  Most users will not need to touch this file.
     */
    /*
     * Maximum number of components (color channels) allowed in JPEG image.
     * To meet the letter of Rec. ITU-T T.81 | ISO/IEC 10918-1, set this to 255.
     * However, darn few applications need more than 4 channels (maybe 5 for CMYK +
     * alpha mask).  We recommend 10 as a reasonable compromise; use 4 if you are
     * really short on memory.  (Each allowed component costs a hundred or so
     * bytes of storage, whether actually used in an image or not.)
     */
    /* maximum number of image components */
    /*
     * Basic data types.
     * You may need to change these if you have a machine with unusual data
     * type sizes; for example, "char" not 8 bits, "short" not 16 bits,
     * or "long" not 32 bits.  We don't care whether "int" is 16 or 32 bits,
     * but it had better be at least 16.
     */
    /* Representation of a single sample (pixel element value).
     * We frequently allocate large arrays of these, so it's important to keep
     * them small.  But if you have memory to burn and access to char or short
     * arrays is very slow on your hardware, you might want to change these.
     */
    /* JSAMPLE should be the smallest type that will hold the values 0..255.
     * You can use a signed char by having GETJSAMPLE mask it with 0xFF.
     */
    /* not HAVE_UNSIGNED_CHAR */
    /* HAVE_UNSIGNED_CHAR */
    /* BITS_IN_JSAMPLE == 8 */
    /* BITS_IN_JSAMPLE == 12 */
    /* Representation of a DCT frequency coefficient.
     * This should be a signed value of at least 16 bits; "short" is usually OK.
     * Again, we allocate large arrays of these, but you can change to int
     * if you have memory to burn and "short" is really slow.
     */
    /* Compressed datastreams are represented as arrays of JOCTET.
     * These must be EXACTLY 8 bits wide, at least once they are written to
     * external storage.  Note that when using the stdio data source/destination
     * managers, this is also the data type passed to fread/fwrite.
     */
    /* not HAVE_UNSIGNED_CHAR */
    /* HAVE_UNSIGNED_CHAR */
    /* These typedefs are used for various table entries and so forth.
     * They must be at least as wide as specified; but making them too big
     * won't cost a huge amount of memory, so we don't provide special
     * extraction code like we did for JSAMPLE.  (In other words, these
     * typedefs live at a different point on the speed/space tradeoff curve.)
     */
    /* UINT8 must hold at least the values 0..255. */
    /* not HAVE_UNSIGNED_CHAR */
    /* HAVE_UNSIGNED_CHAR */
    /* UINT16 must hold at least the values 0..65535. */
    /* not HAVE_UNSIGNED_SHORT */
    /* HAVE_UNSIGNED_SHORT */
    /* INT16 must hold at least the values -32768..32767. */
    /* X11/xmd.h correctly defines INT16 */
    /* INT32 must hold at least signed 32-bit values.
     *
     * NOTE: The INT32 typedef dates back to libjpeg v5 (1994.)  Integers were
     * sometimes 16-bit back then (MS-DOS), which is why INT32 is typedef'd to
     * long.  It also wasn't common (or at least as common) in 1994 for INT32 to be
     * defined by platform headers.  Since then, however, INT32 is defined in
     * several other common places:
     *
     * Xmd.h (X11 header) typedefs INT32 to int on 64-bit platforms and long on
     * 32-bit platforms (i.e always a 32-bit signed type.)
     *
     * basetsd.h (Win32 header) typedefs INT32 to int (always a 32-bit signed type
     * on modern platforms.)
     *
     * qglobal.h (Qt header) typedefs INT32 to int (always a 32-bit signed type on
     * modern platforms.)
     *
     * This is a recipe for conflict, since "long" and "int" aren't always
     * compatible types.  Since the definition of INT32 has technically been part
     * of the libjpeg API for more than 20 years, we can't remove it, but we do not
     * use it internally any longer.  We instead define a separate type (JLONG)
     * for internal use, which ensures that internal behavior will always be the
     * same regardless of any external headers that may be included.
     */
    /* X11/xmd.h correctly defines INT32 */
    /* Microsoft defines it in basetsd.h */
    /* MinGW is slightly different */
    /* Qt defines it in qglobal.h */
    /* Datatype used for image dimensions.  The JPEG standard only supports
     * images up to 64K*64K due to 16-bit fields in SOF markers.  Therefore
     * "unsigned int" is sufficient on all machines.  However, if you need to
     * handle larger images and you don't mind deviating from the spec, you
     * can change this datatype.  (Note that changing this datatype will
     * potentially require modifying the SIMD code.  The x86-64 SIMD extensions,
     * in particular, assume a 32-bit JDIMENSION.)
     */
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
    /* in case these macros already exist */

    pub const FALSE: libc::c_int = 0 as libc::c_int;
    /* values of boolean */

    pub const TRUE: libc::c_int = 1 as libc::c_int;
}
pub mod stddef_h {
    pub type size_t = libc::c_ulong;

    pub const NULL: libc::c_int = 0 as libc::c_int;

    pub const NULL_0: libc::c_int = 0 as libc::c_int;
}
pub mod stdlib {
    extern "C" {
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
        pub fn getc(__stream: *mut crate::stdlib::FILE) -> libc::c_int;

        #[no_mangle]
        pub fn ungetc(__c: libc::c_int, __stream: *mut crate::stdlib::FILE) -> libc::c_int;

        #[no_mangle]
        pub fn fread(
            _: *mut libc::c_void,
            _: libc::c_ulong,
            _: libc::c_ulong,
            _: *mut crate::stdlib::FILE,
        ) -> libc::c_ulong;

        #[no_mangle]
        pub fn fseek(
            __stream: *mut crate::stdlib::FILE,
            __off: libc::c_long,
            __whence: libc::c_int,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn ftell(__stream: *mut crate::stdlib::FILE) -> libc::c_long;
        #[no_mangle]
        pub fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;

        #[no_mangle]
        pub fn strtol(
            _: *const libc::c_char,
            _: *mut *mut libc::c_char,
            _: libc::c_int,
        ) -> libc::c_long;

        #[no_mangle]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;

        #[no_mangle]
        pub fn free(__ptr: *mut libc::c_void);

        #[no_mangle]
        pub fn exit(_: libc::c_int) -> !;
        pub type _IO_wide_data;

        pub type _IO_codecvt;

        pub type _IO_marker;
    }
    pub type FILE = crate::stdlib::_IO_FILE;
    pub const SEEK_END: libc::c_int = 2 as libc::c_int;

    pub const SEEK_SET: libc::c_int = 0 as libc::c_int;

    pub const EOF: libc::c_int = -(1 as libc::c_int);
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

#[c2rust::header_src = "/usr/include/bits/stdlib-float.h:31"]
pub mod stdlib_float_h {
    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
        return crate::stdlib::strtod(
            __nptr,
            crate::stddef_h::NULL as *mut libc::c_void as *mut *mut libc::c_char,
        );
    }
    use crate::stddef_h::NULL;
    use crate::stdlib::strtod;
}
#[c2rust::header_src = "/usr/include/stdlib.h:31"]
pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
        return crate::stdlib::strtol(
            __nptr,
            crate::stddef_h::NULL as *mut libc::c_void as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as libc::c_int;
    }

    use crate::stddef_h::NULL;
}

pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
pub use crate::stddef_h::NULL_0;
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
pub use crate::cdjpeg_h::cjpeg_source_ptr;
pub use crate::cdjpeg_h::cjpeg_source_struct;
pub use crate::cdjpeg_h::jinit_read_bmp;
pub use crate::cdjpeg_h::jinit_read_gif;
pub use crate::cdjpeg_h::jinit_read_jpeg;
pub use crate::cdjpeg_h::jinit_read_ppm;
pub use crate::cdjpeg_h::jinit_read_targa;
pub use crate::cdjpeg_h::keymatch;
pub use crate::cdjpeg_h::read_quant_tables;
pub use crate::cdjpeg_h::read_scan_script;
pub use crate::cdjpeg_h::read_stdin;
pub use crate::cdjpeg_h::set_quality_ratings;
pub use crate::cdjpeg_h::set_quant_slots;
pub use crate::cdjpeg_h::set_sample_factors;
pub use crate::cdjpeg_h::write_stdout;
pub use crate::cdjpeg_h::EXIT_WARNING;
pub use crate::cdjpeg_h::READ_BINARY;
pub use crate::cdjpeg_h::WRITE_BINARY;
pub use crate::jconfig_h::JPEG_LIB_VERSION;
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
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_CreateCompress;
pub use crate::jpeglib_h::jpeg_c_coef_controller;
pub use crate::jpeglib_h::jpeg_c_get_int_param;
pub use crate::jpeglib_h::jpeg_c_main_controller;
pub use crate::jpeglib_h::jpeg_c_prep_controller;
pub use crate::jpeglib_h::jpeg_c_set_bool_param;
pub use crate::jpeglib_h::jpeg_c_set_float_param;
pub use crate::jpeglib_h::jpeg_c_set_int_param;
pub use crate::jpeglib_h::jpeg_color_converter;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_comp_master;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_default_colorspace;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_destroy_compress;
pub use crate::jpeglib_h::jpeg_downsampler;
pub use crate::jpeglib_h::jpeg_entropy_encoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_finish_compress;
pub use crate::jpeglib_h::jpeg_forward_dct;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_marker_writer;
pub use crate::jpeglib_h::jpeg_mem_dest;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jpeg_set_colorspace;
pub use crate::jpeglib_h::jpeg_set_defaults;
pub use crate::jpeglib_h::jpeg_set_quality;
pub use crate::jpeglib_h::jpeg_simple_progression;
pub use crate::jpeglib_h::jpeg_start_compress;
pub use crate::jpeglib_h::jpeg_std_error;
pub use crate::jpeglib_h::jpeg_stdio_dest;
pub use crate::jpeglib_h::jpeg_write_icc_profile;
pub use crate::jpeglib_h::jpeg_write_marker;
pub use crate::jpeglib_h::jpeg_write_scanlines;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_0;
pub use crate::jpeglib_h::C2RustUnnamed_1;
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
pub use crate::jpeglib_h::JDCT_DEFAULT;
pub use crate::jpeglib_h::JDCT_FLOAT;
pub use crate::jpeglib_h::JDCT_IFAST;
pub use crate::jpeglib_h::JDCT_ISLOW;
pub use crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE1;
pub use crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE2;
pub use crate::jpeglib_h::JFLOAT_TRELLIS_DELTA_DC_WEIGHT;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JINT_BASE_QUANT_TBL_IDX;
pub use crate::jpeglib_h::JINT_COMPRESS_PROFILE;
pub use crate::jpeglib_h::JINT_DC_SCAN_OPT_MODE;
pub use crate::jpeglib_h::JINT_TRELLIS_FREQ_SPLIT;
pub use crate::jpeglib_h::JINT_TRELLIS_NUM_LOOPS;
pub use crate::jpeglib_h::JPEG_APP0;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_BOOLEAN_PARAM;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_FLOAT_PARAM;
pub use crate::jpeglib_h::J_INT_PARAM;
pub use crate::stdlib::exit;
pub use crate::stdlib::free;
pub use crate::stdlib::malloc;
pub use crate::stdlib::strtod;
pub use crate::stdlib::strtol;
pub use crate::stdlib::EXIT_FAILURE;
pub use crate::stdlib::EXIT_SUCCESS;
pub use crate::stdlib_float_h::atof;
pub use crate::stdlib_h::atoi;

pub use crate::jconfigint_h::BUILD;
pub use crate::jconfigint_h::PACKAGE_NAME;
pub use crate::jconfigint_h::VERSION;
pub use crate::jversion_h::JCOPYRIGHT;
pub use crate::jversion_h::JVERSION;
pub use crate::stdlib::fclose;
pub use crate::stdlib::fopen;
pub use crate::stdlib::fprintf;
pub use crate::stdlib::fread;
pub use crate::stdlib::fseek;
pub use crate::stdlib::ftell;
pub use crate::stdlib::getc;
pub use crate::stdlib::sscanf;
pub use crate::stdlib::stderr;
pub use crate::stdlib::stdin;
pub use crate::stdlib::stdout;
pub use crate::stdlib::ungetc;
pub use crate::stdlib::EOF;
pub use crate::stdlib::SEEK_END;
pub use crate::stdlib::SEEK_SET;
/*
 * cjpeg.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1998, Thomas G. Lane.
 * Modified 2003-2011 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2010, 2013-2014, 2017, D. R. Commander.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains a command-line user interface for the JPEG compressor.
 * It should work on any system with Unix- or MS-DOS-style command lines.
 *
 * Two different command line styles are permitted, depending on the
 * compile-time switch TWO_FILE_COMMANDLINE:
 *      cjpeg [options]  inputfile outputfile
 *      cjpeg [options]  [inputfile]
 * In the second style, output is always to standard output, which you'd
 * normally redirect to a file or pipe to some other program.  Input is
 * either from a named file or from standard input (typically redirected).
 * The second style is convenient on Unix but is unhelpful on systems that
 * don't support pipes.  Also, you MUST use the first style if your system
 * doesn't do binary I/O to stdin/stdout.
 * To simplify script writing, the "-outfile" switch is provided.  The syntax
 *      cjpeg [options]  -outfile outputfile  inputfile
 * works regardless of which command line style is used.
 */
/* <stdlib.h> should declare malloc(),free() */
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
    crate::stddef_h::NULL_0 as *const libc::c_char,
];
/*
 * This routine determines what format the input file is,
 * and selects the appropriate input-reading module.
 *
 * To determine which family of input formats the file belongs to,
 * we may look only at the first byte of the file, since C does not
 * guarantee that more than one character can be pushed back with ungetc.
 * Looking at additional bytes would require one of these approaches:
 *     1) assume we can fseek() the input file (fails for piped input);
 *     2) assume we can push back more than one character (works in
 *        some C implementations, but unportable);
 *     3) provide our own buffering (breaks input readers that want to use
 *        stdio directly, such as the RLE library);
 * or  4) don't put back the data, and modify the input_init methods to assume
 *        they start reading after the start of file (also breaks RLE library).
 * #1 is attractive for MS-DOS but is untenable on Unix.
 *
 * The most portable solution for file types that can't be identified by their
 * first byte is to make the user tell us what they are.  This is also the
 * only approach for "raw" file types that contain only arbitrary values.
 * We presently apply this method for Targa files.  Most of the time Targa
 * files start with 0x00, so we recognize that case.  Potentially, however,
 * a Targa file could start with any byte value (byte 0 is the length of the
 * seldom-used ID field), so we provide a switch to force Targa input mode.
 */

static mut is_targa: crate::jmorecfg_h::boolean = 0;
/* records user -targa switch */

static mut is_jpeg: crate::jmorecfg_h::boolean = 0;

static mut copy_markers: crate::jmorecfg_h::boolean = 0;

unsafe extern "C" fn select_file_type(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut infile: *mut crate::stdlib::FILE,
) -> crate::cdjpeg_h::cjpeg_source_ptr {
    let mut c: libc::c_int = 0;
    if is_targa != 0 {
        return crate::cdjpeg_h::jinit_read_targa(cinfo);
    }
    c = crate::stdlib::getc(infile);
    if c == crate::stdlib::EOF {
        (*(*cinfo).err).msg_code = crate::jerror_h::JERR_INPUT_EMPTY as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if crate::stdlib::ungetc(c, infile) == crate::stdlib::EOF {
        (*(*cinfo).err).msg_code = crate::cderror_h::JERR_UNGETC_FAILED as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    match c {
        66 => return crate::cdjpeg_h::jinit_read_bmp(cinfo, crate::jmorecfg_h::TRUE),
        71 => return crate::cdjpeg_h::jinit_read_gif(cinfo),
        80 => return crate::cdjpeg_h::jinit_read_ppm(cinfo),
        0 => return crate::cdjpeg_h::jinit_read_targa(cinfo),
        255 => {
            is_jpeg = crate::jmorecfg_h::TRUE;
            copy_markers = crate::jmorecfg_h::TRUE;
            return crate::cdjpeg_h::jinit_read_jpeg(cinfo);
        }
        _ => {
            (*(*cinfo).err).msg_code = crate::cderror_h::JERR_UNKNOWN_FORMAT as libc::c_int;
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
    return crate::stddef_h::NULL_0 as crate::cdjpeg_h::cjpeg_source_ptr;
    /* suppress compiler warnings */
}
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

pub static mut memdst: crate::jmorecfg_h::boolean = 0;
/* for -memdst switch */

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
        b"  -quality N[,...]   Compression quality (0..100; 5-95 is most useful range,\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"                     default is 75)\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -grayscale     Create monochrome JPEG file\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -rgb           Create RGB JPEG file\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(crate::stdlib::stderr,
            b"  -optimize      Optimize Huffman table (smaller file, but slow compression, enabled by default)\n\x00"
                as *const u8 as *const libc::c_char);
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -progressive   Create progressive JPEG file (enabled by default)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -baseline      Create baseline JPEG file (disable progressive coding)\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -targa         Input file is Targa format (usually not needed)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -revert        Revert to standard defaults (instead of mozjpeg defaults)\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -fastcrush     Disable progressive scan optimization\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -dc-scan-opt   DC scan optimization mode\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"                 - 0 One scan for all components\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"                 - 1 One scan per component (default)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(crate::stdlib::stderr,
            b"                 - 2 Optimize between one scan for all components and one scan for 1st component\n\x00"
                as *const u8 as *const libc::c_char);
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"                     plus one scan for remaining components\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -notrellis     Disable trellis optimization\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -trellis-dc    Enable trellis optimization of DC coefficients (default)\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -notrellis-dc  Disable trellis optimization of DC coefficients\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -tune-psnr     Tune trellis optimization for PSNR\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -tune-hvs-psnr Tune trellis optimization for PSNR-HVS (default)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -tune-ssim     Tune trellis optimization for SSIM\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -tune-ms-ssim  Tune trellis optimization for MS-SSIM\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"Switches for advanced users:\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -noovershoot   Disable black-on-white deringing via overshoot\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(crate::stdlib::stderr,
            b"  -nojfif        Do not write JFIF. Reduce size in 18 bytes but break standar. No know problems in web use.\n\x00"
                as *const u8 as *const libc::c_char);
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
    crate::stdlib::fprintf(crate::stdlib::stderr,
            b"  -quant-baseline Use 8-bit quantization table entries for baseline JPEG compatibility\n\x00"
                as *const u8 as *const libc::c_char);
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -quant-table N Use predefined quantization table N:\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"                 - 0 JPEG Annex K\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"                 - 1 Flat\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"                 - 2 Custom, tuned for MS-SSIM\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"                 - 3 ImageMagick table by N. Robidoux\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"                 - 4 Custom, tuned for PSNR-HVS\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"                 - 5 Table from paper by Klein, Silverstein and Carney\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -icc FILE      Embed ICC profile contained in FILE\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -restart N     Set restart interval in rows, or in blocks with B\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -smooth N      Smooth dithered input (N=1..100 is strength)\n\x00" as *const u8
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
        b"  -memdst        Compress to memory instead of file (useful for benchmarking)\n\x00"
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
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"Switches for wizards:\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -qtables FILE  Use quantization tables given in FILE\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -qslots N[,...]    Set component quantization tables\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -sample HxV[,...]  Set component sampling factors\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -scans FILE    Create multi-scan JPEG per script FILE\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
}

unsafe extern "C" fn parse_switches(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
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
    let mut argn: libc::c_int = 0; /* saves -quality parm if any */
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char; /* saves -qtables filename if any */
    let mut force_baseline: crate::jmorecfg_h::boolean = 0; /* saves -qslots parm if any */
    let mut simple_progressive: crate::jmorecfg_h::boolean = 0; /* saves -sample parm if any */
    let mut qualityarg: *mut libc::c_char = crate::stddef_h::NULL_0 as *mut libc::c_char; /* saves -scans parm if any */
    let mut qtablefile: *mut libc::c_char = crate::stddef_h::NULL_0 as *mut libc::c_char;
    let mut qslotsarg: *mut libc::c_char = crate::stddef_h::NULL_0 as *mut libc::c_char;
    let mut samplearg: *mut libc::c_char = crate::stddef_h::NULL_0 as *mut libc::c_char;
    let mut scansarg: *mut libc::c_char = crate::stddef_h::NULL_0 as *mut libc::c_char;
    /* Set up default JPEG parameters. */
    force_baseline = crate::jmorecfg_h::FALSE; /* by default, allow 16-bit quantizers */
    simple_progressive = if (*cinfo).num_scans == 0 as libc::c_int {
        crate::jmorecfg_h::FALSE
    } else {
        crate::jmorecfg_h::TRUE
    };
    is_targa = crate::jmorecfg_h::FALSE;
    icc_filename = crate::stddef_h::NULL_0 as *mut libc::c_char;
    outfilename = crate::stddef_h::NULL_0 as *mut libc::c_char;
    memdst = crate::jmorecfg_h::FALSE;
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
            outfilename = crate::stddef_h::NULL_0 as *mut libc::c_char
        /* ignore this name if previously processed */
        /* else done parsing switches */
        } else {
            arg = arg.offset(1); /* advance past switch marker character */
            if crate::cdjpeg_h::keymatch(
                arg,
                b"arithmetic\x00" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            ) != 0
            {
                /* Use arithmetic coding. */
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"%s: sorry, arithmetic coding not supported\n\x00" as *const u8
                        as *const libc::c_char,
                    progname,
                );
                crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
            } else {
                if crate::cdjpeg_h::keymatch(
                    arg,
                    b"baseline\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                ) != 0
                {
                    /* Force baseline-compatible output (8-bit quantizer values). */
                    force_baseline = crate::jmorecfg_h::TRUE;
                    /* Disable multiple scans */
                    simple_progressive = crate::jmorecfg_h::FALSE;
                    (*cinfo).num_scans = 0 as libc::c_int;
                    (*cinfo).scan_info =
                        crate::stddef_h::NULL_0 as *const crate::jpeglib_h::jpeg_scan_info
                } else if crate::cdjpeg_h::keymatch(
                    arg,
                    b"dct\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int,
                ) != 0
                {
                    /* Select DCT algorithm. */
                    argn += 1;
                    if argn >= argc {
                        /* advance to next argument */
                        crate::stdlib::fprintf(
                            crate::stdlib::stderr,
                            b"%s: missing argument for dct\n\x00" as *const u8
                                as *const libc::c_char,
                            progname,
                        );
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
                        crate::stdlib::fprintf(
                            crate::stdlib::stderr,
                            b"%s: invalid argument for dct\n\x00" as *const u8
                                as *const libc::c_char,
                            progname,
                        );
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
                    static mut printed_version: crate::jmorecfg_h::boolean =
                        crate::jmorecfg_h::FALSE;
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
                        b"fastcrush\x00" as *const u8 as *const libc::c_char,
                        4 as libc::c_int,
                    ) != 0
                    {
                        crate::jpeglib_h::jpeg_c_set_bool_param(
                            cinfo,
                            crate::jpeglib_h::JBOOLEAN_OPTIMIZE_SCANS,
                            crate::jmorecfg_h::FALSE,
                        );
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
                        /* Force a monochrome JPEG file to be generated. */
                        crate::jpeglib_h::jpeg_set_colorspace(
                            cinfo,
                            crate::jpeglib_h::JCS_GRAYSCALE,
                        );
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"rgb\x00" as *const u8 as *const libc::c_char,
                        3 as libc::c_int,
                    ) != 0
                    {
                        /* Force an RGB JPEG file to be generated. */
                        crate::jpeglib_h::jpeg_set_colorspace(cinfo, crate::jpeglib_h::JCS_RGB);
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"lambda1\x00" as *const u8 as *const libc::c_char,
                        7 as libc::c_int,
                    ) != 0
                    {
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE1,
                            atof(*argv.offset(argn as isize)) as libc::c_float,
                        );
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"lambda2\x00" as *const u8 as *const libc::c_char,
                        7 as libc::c_int,
                    ) != 0
                    {
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE2,
                            atof(*argv.offset(argn as isize)) as libc::c_float,
                        );
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
                        icc_filename = *argv.offset(argn as isize)
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
                        b"dc-scan-opt\x00" as *const u8 as *const libc::c_char,
                        3 as libc::c_int,
                    ) != 0
                    {
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            crate::stdlib::fprintf(
                                crate::stdlib::stderr,
                                b"%s: missing argument for dc-scan-opt\n\x00" as *const u8
                                    as *const libc::c_char,
                                progname,
                            );
                            usage();
                        }
                        crate::jpeglib_h::jpeg_c_set_int_param(
                            cinfo,
                            crate::jpeglib_h::JINT_DC_SCAN_OPT_MODE,
                            atoi(*argv.offset(argn as isize)),
                        );
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"optimize\x00" as *const u8 as *const libc::c_char,
                        1 as libc::c_int,
                    ) != 0
                        || crate::cdjpeg_h::keymatch(
                            arg,
                            b"optimise\x00" as *const u8 as *const libc::c_char,
                            1 as libc::c_int,
                        ) != 0
                    {
                        /* Enable entropy parm optimization. */
                        (*cinfo).optimize_coding = crate::jmorecfg_h::TRUE
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
                            crate::stdlib::fprintf(
                                crate::stdlib::stderr,
                                b"%s: missing argument for outfile\n\x00" as *const u8
                                    as *const libc::c_char,
                                progname,
                            );
                            usage();
                        }
                        outfilename = *argv.offset(argn as isize)
                    /* save it away for later use */
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"progressive\x00" as *const u8 as *const libc::c_char,
                        1 as libc::c_int,
                    ) != 0
                    {
                        /* Select simple progressive mode. */
                        simple_progressive = crate::jmorecfg_h::TRUE
                    /* We must postpone execution until num_components is known. */
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"memdst\x00" as *const u8 as *const libc::c_char,
                        2 as libc::c_int,
                    ) != 0
                    {
                        /* Use in-memory destination manager */
                        memdst = crate::jmorecfg_h::TRUE
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"quality\x00" as *const u8 as *const libc::c_char,
                        1 as libc::c_int,
                    ) != 0
                    {
                        /* Quality ratings (quantization table scaling factors). */
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            crate::stdlib::fprintf(
                                crate::stdlib::stderr,
                                b"%s: missing argument for quality\n\x00" as *const u8
                                    as *const libc::c_char,
                                progname,
                            );
                            usage();
                        }
                        qualityarg = *argv.offset(argn as isize)
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"qslots\x00" as *const u8 as *const libc::c_char,
                        2 as libc::c_int,
                    ) != 0
                    {
                        /* Quantization table slot numbers. */
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        qslotsarg = *argv.offset(argn as isize)
                    /* Must delay setting qslots until after we have processed any
                     * colorspace-determining switches, since jpeg_set_colorspace sets
                     * default quant table numbers.
                     */
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"qtables\x00" as *const u8 as *const libc::c_char,
                        2 as libc::c_int,
                    ) != 0
                    {
                        /* Quantization tables fetched from file. */
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        qtablefile = *argv.offset(argn as isize)
                    /* We postpone actually reading the file in case -quality comes later. */
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"quant-table\x00" as *const u8 as *const libc::c_char,
                        7 as libc::c_int,
                    ) != 0
                    {
                        let mut val: libc::c_int = 0;
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        val = atoi(*argv.offset(argn as isize));
                        crate::jpeglib_h::jpeg_c_set_int_param(
                            cinfo,
                            crate::jpeglib_h::JINT_BASE_QUANT_TBL_IDX,
                            val,
                        );
                        if crate::jpeglib_h::jpeg_c_get_int_param(
                            cinfo,
                            crate::jpeglib_h::JINT_BASE_QUANT_TBL_IDX,
                        ) != val
                        {
                            crate::stdlib::fprintf(
                                crate::stdlib::stderr,
                                b"%s: %d is invalid argument for quant-table\n\x00" as *const u8
                                    as *const libc::c_char,
                                progname,
                                val,
                            );
                            usage();
                        }
                        crate::jpeglib_h::jpeg_set_quality(
                            cinfo,
                            75 as libc::c_int,
                            crate::jmorecfg_h::TRUE,
                        );
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"quant-baseline\x00" as *const u8 as *const libc::c_char,
                        7 as libc::c_int,
                    ) != 0
                    {
                        /* Force quantization table to meet baseline requirements */
                        force_baseline = crate::jmorecfg_h::TRUE
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"restart\x00" as *const u8 as *const libc::c_char,
                        1 as libc::c_int,
                    ) != 0
                    {
                        /* Restart interval in MCU rows (or in MCUs with 'b'). */
                        let mut lval_0: libc::c_long = 0;
                        let mut ch_0: libc::c_char = 'x' as i32 as libc::c_char;
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        if crate::stdlib::sscanf(
                            *argv.offset(argn as isize),
                            b"%ld%c\x00" as *const u8 as *const libc::c_char,
                            &mut lval_0 as *mut libc::c_long,
                            &mut ch_0 as *mut libc::c_char,
                        ) < 1 as libc::c_int
                        {
                            usage();
                        }
                        if lval_0 < 0 as libc::c_int as libc::c_long
                            || lval_0 > 65535 as libc::c_long
                        {
                            usage();
                        }
                        if ch_0 as libc::c_int == 'b' as i32 || ch_0 as libc::c_int == 'B' as i32 {
                            (*cinfo).restart_interval = lval_0 as libc::c_uint;
                            (*cinfo).restart_in_rows = 0 as libc::c_int
                        /* else prior '-restart n' overrides me */
                        } else {
                            (*cinfo).restart_in_rows = lval_0 as libc::c_int
                            /* restart_interval will be computed during startup */
                        }
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"revert\x00" as *const u8 as *const libc::c_char,
                        3 as libc::c_int,
                    ) != 0
                    {
                        /* revert to old JPEG default */
                        crate::jpeglib_h::jpeg_c_set_int_param(
                            cinfo,
                            crate::jpeglib_h::JINT_COMPRESS_PROFILE,
                            crate::jpeglib_h::JCP_FASTEST as libc::c_int,
                        );
                        crate::jpeglib_h::jpeg_set_defaults(cinfo);
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"sample\x00" as *const u8 as *const libc::c_char,
                        2 as libc::c_int,
                    ) != 0
                    {
                        /* Set sampling factors. */
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        samplearg = *argv.offset(argn as isize)
                    /* Must delay setting sample factors until after we have processed any
                     * colorspace-determining switches, since jpeg_set_colorspace sets
                     * default sampling factors.
                     */
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"scans\x00" as *const u8 as *const libc::c_char,
                        4 as libc::c_int,
                    ) != 0
                    {
                        /* Set scan script. */
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        scansarg = *argv.offset(argn as isize)
                    /* We must postpone reading the file in case -progressive appears. */
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"smooth\x00" as *const u8 as *const libc::c_char,
                        2 as libc::c_int,
                    ) != 0
                    {
                        /* Set input smoothing factor. */
                        let mut val_0: libc::c_int = 0;
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            usage();
                        }
                        if crate::stdlib::sscanf(
                            *argv.offset(argn as isize),
                            b"%d\x00" as *const u8 as *const libc::c_char,
                            &mut val_0 as *mut libc::c_int,
                        ) != 1 as libc::c_int
                        {
                            usage();
                        }
                        if val_0 < 0 as libc::c_int || val_0 > 100 as libc::c_int {
                            usage();
                        }
                        (*cinfo).smoothing_factor = val_0
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"targa\x00" as *const u8 as *const libc::c_char,
                        1 as libc::c_int,
                    ) != 0
                    {
                        /* Input file is Targa format. */
                        is_targa = crate::jmorecfg_h::TRUE
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"notrellis-dc\x00" as *const u8 as *const libc::c_char,
                        11 as libc::c_int,
                    ) != 0
                    {
                        /* disable trellis quantization */
                        crate::jpeglib_h::jpeg_c_set_bool_param(
                            cinfo,
                            crate::jpeglib_h::JBOOLEAN_TRELLIS_QUANT_DC,
                            crate::jmorecfg_h::FALSE,
                        );
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"notrellis\x00" as *const u8 as *const libc::c_char,
                        1 as libc::c_int,
                    ) != 0
                    {
                        /* disable trellis quantization */
                        crate::jpeglib_h::jpeg_c_set_bool_param(
                            cinfo,
                            crate::jpeglib_h::JBOOLEAN_TRELLIS_QUANT,
                            crate::jmorecfg_h::FALSE,
                        );
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"trellis-dc-ver-weight\x00" as *const u8 as *const libc::c_char,
                        12 as libc::c_int,
                    ) != 0
                    {
                        argn += 1;
                        if argn >= argc {
                            /* advance to next argument */
                            crate::stdlib::fprintf(
                                crate::stdlib::stderr,
                                b"%s: missing argument for trellis-dc-ver-weight\n\x00" as *const u8
                                    as *const libc::c_char,
                                progname,
                            );
                            usage();
                        }
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_TRELLIS_DELTA_DC_WEIGHT,
                            atof(*argv.offset(argn as isize)) as libc::c_float,
                        );
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"trellis-dc\x00" as *const u8 as *const libc::c_char,
                        9 as libc::c_int,
                    ) != 0
                    {
                        /* enable DC trellis quantization */
                        crate::jpeglib_h::jpeg_c_set_bool_param(
                            cinfo,
                            crate::jpeglib_h::JBOOLEAN_TRELLIS_QUANT_DC,
                            crate::jmorecfg_h::TRUE,
                        );
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"tune-psnr\x00" as *const u8 as *const libc::c_char,
                        6 as libc::c_int,
                    ) != 0
                    {
                        crate::jpeglib_h::jpeg_c_set_int_param(
                            cinfo,
                            crate::jpeglib_h::JINT_BASE_QUANT_TBL_IDX,
                            1 as libc::c_int,
                        );
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE1,
                            9.0f64 as libc::c_float,
                        );
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE2,
                            0.0f64 as libc::c_float,
                        );
                        crate::jpeglib_h::jpeg_c_set_bool_param(
                            cinfo,
                            crate::jpeglib_h::JBOOLEAN_USE_LAMBDA_WEIGHT_TBL,
                            crate::jmorecfg_h::FALSE,
                        );
                        crate::jpeglib_h::jpeg_set_quality(
                            cinfo,
                            75 as libc::c_int,
                            crate::jmorecfg_h::TRUE,
                        );
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"tune-ssim\x00" as *const u8 as *const libc::c_char,
                        6 as libc::c_int,
                    ) != 0
                    {
                        crate::jpeglib_h::jpeg_c_set_int_param(
                            cinfo,
                            crate::jpeglib_h::JINT_BASE_QUANT_TBL_IDX,
                            1 as libc::c_int,
                        );
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE1,
                            11.5f64 as libc::c_float,
                        );
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE2,
                            12.75f64 as libc::c_float,
                        );
                        crate::jpeglib_h::jpeg_c_set_bool_param(
                            cinfo,
                            crate::jpeglib_h::JBOOLEAN_USE_LAMBDA_WEIGHT_TBL,
                            crate::jmorecfg_h::FALSE,
                        );
                        crate::jpeglib_h::jpeg_set_quality(
                            cinfo,
                            75 as libc::c_int,
                            crate::jmorecfg_h::TRUE,
                        );
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"tune-ms-ssim\x00" as *const u8 as *const libc::c_char,
                        6 as libc::c_int,
                    ) != 0
                    {
                        crate::jpeglib_h::jpeg_c_set_int_param(
                            cinfo,
                            crate::jpeglib_h::JINT_BASE_QUANT_TBL_IDX,
                            3 as libc::c_int,
                        );
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE1,
                            12.0f64 as libc::c_float,
                        );
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE2,
                            13.0f64 as libc::c_float,
                        );
                        crate::jpeglib_h::jpeg_c_set_bool_param(
                            cinfo,
                            crate::jpeglib_h::JBOOLEAN_USE_LAMBDA_WEIGHT_TBL,
                            crate::jmorecfg_h::TRUE,
                        );
                        crate::jpeglib_h::jpeg_set_quality(
                            cinfo,
                            75 as libc::c_int,
                            crate::jmorecfg_h::TRUE,
                        );
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"tune-hvs-psnr\x00" as *const u8 as *const libc::c_char,
                        6 as libc::c_int,
                    ) != 0
                    {
                        crate::jpeglib_h::jpeg_c_set_int_param(
                            cinfo,
                            crate::jpeglib_h::JINT_BASE_QUANT_TBL_IDX,
                            3 as libc::c_int,
                        );
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE1,
                            14.75f64 as libc::c_float,
                        );
                        crate::jpeglib_h::jpeg_c_set_float_param(
                            cinfo,
                            crate::jpeglib_h::JFLOAT_LAMBDA_LOG_SCALE2,
                            16.5f64 as libc::c_float,
                        );
                        crate::jpeglib_h::jpeg_c_set_bool_param(
                            cinfo,
                            crate::jpeglib_h::JBOOLEAN_USE_LAMBDA_WEIGHT_TBL,
                            crate::jmorecfg_h::TRUE,
                        );
                        crate::jpeglib_h::jpeg_set_quality(
                            cinfo,
                            75 as libc::c_int,
                            crate::jmorecfg_h::TRUE,
                        );
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"noovershoot\x00" as *const u8 as *const libc::c_char,
                        11 as libc::c_int,
                    ) != 0
                    {
                        crate::jpeglib_h::jpeg_c_set_bool_param(
                            cinfo,
                            crate::jpeglib_h::JBOOLEAN_OVERSHOOT_DERINGING,
                            crate::jmorecfg_h::FALSE,
                        );
                    } else if crate::cdjpeg_h::keymatch(
                        arg,
                        b"nojfif\x00" as *const u8 as *const libc::c_char,
                        6 as libc::c_int,
                    ) != 0
                    {
                        (*cinfo).write_JFIF_header = 0 as libc::c_int
                    } else {
                        crate::stdlib::fprintf(
                            crate::stdlib::stderr,
                            b"%s: unknown option \'%s\'\n\x00" as *const u8 as *const libc::c_char,
                            progname,
                            arg,
                        );
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
        /* Set quantization tables for selected quality. */
        /* Some or all may be overridden if -qtables is present. */
        if !qualityarg.is_null() {
            /* process -quality if it was present */
            if crate::cdjpeg_h::set_quality_ratings(cinfo, qualityarg, force_baseline) == 0 {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"%s: can\'t set quality ratings\n\x00" as *const u8 as *const libc::c_char,
                    progname,
                );
                usage();
            }
        }
        if !qtablefile.is_null() {
            /* process -qtables if it was present */
            if crate::cdjpeg_h::read_quant_tables(cinfo, qtablefile, force_baseline) == 0 {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"%s: can\'t read qtable file\n\x00" as *const u8 as *const libc::c_char,
                    progname,
                );
                usage();
            }
        }
        if !qslotsarg.is_null() {
            /* process -qslots if it was present */
            if crate::cdjpeg_h::set_quant_slots(cinfo, qslotsarg) == 0 {
                usage();
            }
        }
        /* set_quality_ratings sets default subsampling, so the explicit
        subsampling must be set after it */
        if !samplearg.is_null() {
            /* process -sample if it was present */
            if crate::cdjpeg_h::set_sample_factors(cinfo, samplearg) == 0 {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"%s: can\'t set sample factors\n\x00" as *const u8 as *const libc::c_char,
                    progname,
                );
                usage();
            }
        }
        if simple_progressive != 0 {
            /* process -progressive; -scans can override */
            crate::jpeglib_h::jpeg_simple_progression(cinfo);
        }
        if !scansarg.is_null() {
            /* process -scans if it was present */
            if crate::cdjpeg_h::read_scan_script(cinfo, scansarg) == 0 {
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

unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut cinfo: crate::jpeglib_h::jpeg_compress_struct =
        crate::jpeglib_h::jpeg_compress_struct {
            err: 0 as *mut crate::jpeglib_h::jpeg_error_mgr,
            mem: 0 as *mut crate::jpeglib_h::jpeg_memory_mgr,
            progress: 0 as *mut crate::jpeglib_h::jpeg_progress_mgr,
            client_data: 0 as *mut libc::c_void,
            is_decompressor: 0,
            global_state: 0,
            dest: 0 as *mut crate::jpeglib_h::jpeg_destination_mgr,
            image_width: 0,
            image_height: 0,
            input_components: 0,
            in_color_space: crate::jpeglib_h::JCS_UNKNOWN,
            input_gamma: 0.,
            data_precision: 0,
            num_components: 0,
            jpeg_color_space: crate::jpeglib_h::JCS_UNKNOWN,
            comp_info: 0 as *mut crate::jpeglib_h::jpeg_component_info,
            quant_tbl_ptrs: [0 as *mut crate::jpeglib_h::JQUANT_TBL; 4],
            dc_huff_tbl_ptrs: [0 as *mut crate::jpeglib_h::JHUFF_TBL; 4],
            ac_huff_tbl_ptrs: [0 as *mut crate::jpeglib_h::JHUFF_TBL; 4],
            arith_dc_L: [0; 16],
            arith_dc_U: [0; 16],
            arith_ac_K: [0; 16],
            num_scans: 0,
            scan_info: 0 as *const crate::jpeglib_h::jpeg_scan_info,
            raw_data_in: 0,
            arith_code: 0,
            optimize_coding: 0,
            CCIR601_sampling: 0,
            smoothing_factor: 0,
            dct_method: crate::jpeglib_h::JDCT_ISLOW,
            restart_interval: 0,
            restart_in_rows: 0,
            write_JFIF_header: 0,
            JFIF_major_version: 0,
            JFIF_minor_version: 0,
            density_unit: 0,
            X_density: 0,
            Y_density: 0,
            write_Adobe_marker: 0,
            next_scanline: 0,
            progressive_mode: 0,
            max_h_samp_factor: 0,
            max_v_samp_factor: 0,
            total_iMCU_rows: 0,
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
            master: 0 as *mut crate::jpeglib_h::jpeg_comp_master,
            main: 0 as *mut crate::jpeglib_h::jpeg_c_main_controller,
            prep: 0 as *mut crate::jpeglib_h::jpeg_c_prep_controller,
            coef: 0 as *mut crate::jpeglib_h::jpeg_c_coef_controller,
            marker: 0 as *mut crate::jpeglib_h::jpeg_marker_writer,
            cconvert: 0 as *mut crate::jpeglib_h::jpeg_color_converter,
            downsample: 0 as *mut crate::jpeglib_h::jpeg_downsampler,
            fdct: 0 as *mut crate::jpeglib_h::jpeg_forward_dct,
            entropy: 0 as *mut crate::jpeglib_h::jpeg_entropy_encoder,
            script_space: 0 as *mut crate::jpeglib_h::jpeg_scan_info,
            script_space_size: 0,
        };
    let mut jerr: crate::jpeglib_h::jpeg_error_mgr = crate::jpeglib_h::jpeg_error_mgr {
        error_exit: None,
        emit_message: None,
        output_message: None,
        format_message: None,
        reset_error_mgr: None,
        msg_code: 0,
        msg_parm: crate::jpeglib_h::C2RustUnnamed_1 { i: [0; 8] },
        trace_level: 0,
        num_warnings: 0,
        jpeg_message_table: 0 as *const *const libc::c_char,
        last_jpeg_message: 0,
        addon_message_table: 0 as *const *const libc::c_char,
        first_addon_message: 0,
        last_addon_message: 0,
    };
    let mut file_index: libc::c_int = 0;
    let mut src_mgr: crate::cdjpeg_h::cjpeg_source_ptr =
        0 as *mut crate::cdjpeg_h::cjpeg_source_struct;
    let mut input_file: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    let mut icc_file: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    let mut icc_profile: *mut crate::jmorecfg_h::JOCTET =
        crate::stddef_h::NULL_0 as *mut crate::jmorecfg_h::JOCTET;
    let mut icc_len: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut output_file: *mut crate::stdlib::FILE =
        crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE;
    let mut outbuffer: *mut libc::c_uchar = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
    let mut outsize: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut num_scanlines: crate::jmorecfg_h::JDIMENSION = 0;
    /* On Mac, fetch a command line. */
    progname = *argv.offset(0 as libc::c_int as isize); /* in case C library doesn't provide it */
    if progname.is_null()
        || *progname.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
    {
        progname = b"cjpeg\x00" as *const u8 as *const libc::c_char
    }
    /* Initialize the JPEG compression object with default error handling. */
    cinfo.err = crate::jpeglib_h::jpeg_std_error(&mut jerr);
    crate::jpeglib_h::jpeg_CreateCompress(
        &mut cinfo,
        crate::jconfig_h::JPEG_LIB_VERSION,
        ::std::mem::size_of::<crate::jpeglib_h::jpeg_compress_struct>() as libc::c_ulong,
    );
    /* Add some application-specific error messages (from cderror.h) */
    jerr.addon_message_table = cdjpeg_message_table.as_ptr();
    jerr.first_addon_message = crate::cderror_h::JMSG_FIRSTADDONCODE as libc::c_int;
    jerr.last_addon_message = crate::cderror_h::JMSG_LASTADDONCODE as libc::c_int;
    /* Initialize JPEG parameters.
     * Much of this may be overridden later.
     * In particular, we don't yet know the input file's color space,
     * but we need to provide some value for jpeg_set_defaults() to work.
     */
    cinfo.in_color_space = crate::jpeglib_h::JCS_RGB; /* arbitrary guess */
    crate::jpeglib_h::jpeg_set_defaults(&mut cinfo);
    /* Scan command line to find file names.
     * It is convenient to use just one switch-parsing routine, but the switch
     * values read here are ignored; we will rescan the switches after opening
     * the input file.
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
    } else if memdst == 0 {
        /* default output file is stdout */
        output_file = crate::cdjpeg_h::write_stdout()
    }
    if !icc_filename.is_null() {
        icc_file = crate::stdlib::fopen(icc_filename, crate::cdjpeg_h::READ_BINARY.as_ptr());
        if icc_file.is_null() {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: can\'t open %s\n\x00" as *const u8 as *const libc::c_char,
                progname,
                icc_filename,
            );
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
        if crate::stdlib::fseek(
            icc_file,
            0 as libc::c_int as libc::c_long,
            crate::stdlib::SEEK_END,
        ) < 0 as libc::c_int
            || {
                icc_len = crate::stdlib::ftell(icc_file);
                (icc_len) < 1 as libc::c_int as libc::c_long
            }
            || crate::stdlib::fseek(
                icc_file,
                0 as libc::c_int as libc::c_long,
                crate::stdlib::SEEK_SET,
            ) < 0 as libc::c_int
        {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: can\'t determine size of %s\n\x00" as *const u8 as *const libc::c_char,
                progname,
                icc_filename,
            );
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
        icc_profile =
            crate::stdlib::malloc(icc_len as libc::c_ulong) as *mut crate::jmorecfg_h::JOCTET;
        if icc_profile.is_null() {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: can\'t allocate memory for ICC profile\n\x00" as *const u8
                    as *const libc::c_char,
                progname,
            );
            crate::stdlib::fclose(icc_file);
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
        if crate::stdlib::fread(
            icc_profile as *mut libc::c_void,
            icc_len as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            icc_file,
        ) < 1 as libc::c_int as libc::c_ulong
        {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: can\'t read ICC profile from %s\n\x00" as *const u8 as *const libc::c_char,
                progname,
                icc_filename,
            );
            crate::stdlib::free(icc_profile as *mut libc::c_void);
            crate::stdlib::fclose(icc_file);
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
        crate::stdlib::fclose(icc_file);
    }
    /* Figure out the input file format, and set up to read it. */
    src_mgr = select_file_type(&mut cinfo, input_file);
    (*src_mgr).input_file = input_file;
    /* Read the input file header to obtain file size & colorspace. */
    Some((*src_mgr).start_input.expect("non-null function pointer"))
        .expect("non-null function pointer")(&mut cinfo, src_mgr);
    /* Now that we know input colorspace, fix colorspace-dependent defaults */
    crate::jpeglib_h::jpeg_default_colorspace(&mut cinfo);
    /* Adjust default compression parameters by re-parsing the options */
    file_index = parse_switches(
        &mut cinfo,
        argc,
        argv,
        0 as libc::c_int,
        crate::jmorecfg_h::TRUE,
    );
    /* Specify data destination for compression */
    if memdst != 0 {
        crate::jpeglib_h::jpeg_mem_dest(&mut cinfo, &mut outbuffer, &mut outsize);
    } else {
        crate::jpeglib_h::jpeg_stdio_dest(&mut cinfo, output_file);
    }
    /* Start compressor */
    crate::jpeglib_h::jpeg_start_compress(&mut cinfo, crate::jmorecfg_h::TRUE);
    /* Copy metadata */
    if copy_markers != 0 {
        let mut marker: crate::jpeglib_h::jpeg_saved_marker_ptr =
            0 as *mut crate::jpeglib_h::jpeg_marker_struct;
        /* In the current implementation, we don't actually need to examine the
         * option flag here; we just copy everything that got saved.
         * But to avoid confusion, we do not output JFIF and Adobe APP14 markers
         * if the encoder library already wrote one.
         */
        marker = (*src_mgr).marker_list; /* reject duplicate JFIF */
        while !marker.is_null() {
            if !(cinfo.write_JFIF_header != 0
                && (*marker).marker as libc::c_int == crate::jpeglib_h::JPEG_APP0
                && (*marker).data_length >= 5 as libc::c_int as libc::c_uint
                && *(*marker).data.offset(0 as libc::c_int as isize) as libc::c_int
                    == 0x4a as libc::c_int
                && *(*marker).data.offset(1 as libc::c_int as isize) as libc::c_int
                    == 0x46 as libc::c_int
                && *(*marker).data.offset(2 as libc::c_int as isize) as libc::c_int
                    == 0x49 as libc::c_int
                && *(*marker).data.offset(3 as libc::c_int as isize) as libc::c_int
                    == 0x46 as libc::c_int
                && *(*marker).data.offset(4 as libc::c_int as isize) as libc::c_int
                    == 0 as libc::c_int)
            {
                if !(cinfo.write_Adobe_marker != 0
                    && (*marker).marker as libc::c_int
                        == crate::jpeglib_h::JPEG_APP0 + 14 as libc::c_int
                    && (*marker).data_length >= 5 as libc::c_int as libc::c_uint
                    && *(*marker).data.offset(0 as libc::c_int as isize) as libc::c_int
                        == 0x41 as libc::c_int
                    && *(*marker).data.offset(1 as libc::c_int as isize) as libc::c_int
                        == 0x64 as libc::c_int
                    && *(*marker).data.offset(2 as libc::c_int as isize) as libc::c_int
                        == 0x6f as libc::c_int
                    && *(*marker).data.offset(3 as libc::c_int as isize) as libc::c_int
                        == 0x62 as libc::c_int
                    && *(*marker).data.offset(4 as libc::c_int as isize) as libc::c_int
                        == 0x65 as libc::c_int)
                {
                    crate::jpeglib_h::jpeg_write_marker(
                        &mut cinfo,
                        (*marker).marker as libc::c_int,
                        (*marker).data,
                        (*marker).data_length,
                    ); /* reject duplicate Adobe */
                }
            }
            marker = (*marker).next
        }
    }
    if !icc_profile.is_null() {
        crate::jpeglib_h::jpeg_write_icc_profile(&mut cinfo, icc_profile, icc_len as libc::c_uint);
    }
    /* Process data */
    while cinfo.next_scanline < cinfo.image_height {
        num_scanlines = Some(
            (*src_mgr)
                .get_pixel_rows
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(&mut cinfo, src_mgr);
        crate::jpeglib_h::jpeg_write_scanlines(&mut cinfo, (*src_mgr).buffer, num_scanlines);
    }
    /* Finish compression and release memory */
    Some((*src_mgr).finish_input.expect("non-null function pointer"))
        .expect("non-null function pointer")(&mut cinfo, src_mgr);
    crate::jpeglib_h::jpeg_finish_compress(&mut cinfo);
    crate::jpeglib_h::jpeg_destroy_compress(&mut cinfo);
    /* Close files, if we opened them */
    if input_file != crate::stdlib::stdin {
        crate::stdlib::fclose(input_file);
    }
    if output_file != crate::stdlib::stdout && !output_file.is_null() {
        crate::stdlib::fclose(output_file);
    }
    if memdst != 0 {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"Compressed size:  %lu bytes\n\x00" as *const u8 as *const libc::c_char,
            outsize,
        );
        if !outbuffer.is_null() {
            crate::stdlib::free(outbuffer as *mut libc::c_void);
        }
    }
    if !icc_profile.is_null() {
        crate::stdlib::free(icc_profile as *mut libc::c_void);
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
