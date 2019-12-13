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
#![feature(extern_types, main, register_tool)]
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
}
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
}
pub mod jconfig_h {
    pub const JPEG_LIB_VERSION: libc::c_int = 62 as libc::c_int;
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
        /* Initialization of JPEG compression objects.
         * jpeg_create_compress() and jpeg_create_decompress() are the exported
         * names that applications should call.  These expand to calls on
         * jpeg_CreateCompress and jpeg_CreateDecompress with additional information
         * passed for version mismatch checking.
         * NB: you must set up the error-manager BEFORE calling jpeg_create_xxx.
         */
        #[no_mangle]
        pub fn jpeg_CreateCompress(
            cinfo: crate::jpeglib_h::j_compress_ptr,
            version: libc::c_int,
            structsize: crate::stddef_h::size_t,
        );
        /* Destruction of JPEG compression objects */
        #[no_mangle]
        pub fn jpeg_destroy_compress(cinfo: crate::jpeglib_h::j_compress_ptr);
        /* Default parameter setup for compression */
        #[no_mangle]
        pub fn jpeg_set_defaults(cinfo: crate::jpeglib_h::j_compress_ptr);

        #[no_mangle]
        pub fn jpeg_default_colorspace(cinfo: crate::jpeglib_h::j_compress_ptr);
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

    pub type J_COLOR_SPACE = libc::c_uint;

    pub type J_DCT_METHOD = libc::c_uint;
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
    /* code for last string in addon table */
    /* Progress monitor object */

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

    pub type jvirt_barray_ptr = *mut crate::jpeglib_h::jvirt_barray_control;
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

    pub type jvirt_sarray_ptr = *mut crate::jpeglib_h::jvirt_sarray_control;
    /* "Object" declarations for JPEG modules that may be supplied or called
     * directly by the surrounding application.
     * As with all objects in the JPEG library, these structs only define the
     * publicly visible methods and state variables of a module.  Additional
     * private fields may exist after the public ones.
     */
    /* Error handler object */

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
    /* Master record for a compression instance */

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
    /* total number of passes expected */
    /* Data destination object for compression */

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
}
pub mod stdlib {
    extern "C" {
        #[no_mangle]
        pub fn _setjmp(_: *mut crate::stdlib::__jmp_buf_tag) -> libc::c_int;

        #[no_mangle]
        pub fn longjmp(_: *mut crate::stdlib::__jmp_buf_tag, _: libc::c_int) -> !;
        #[no_mangle]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct __sigset_t {
        pub __val: [libc::c_ulong; 16],
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct __jmp_buf_tag {
        pub __jmpbuf: crate::stdlib::__jmp_buf,
        pub __mask_was_saved: libc::c_int,
        pub __saved_mask: crate::stdlib::__sigset_t,
    }

    pub type jmp_buf = [crate::stdlib::__jmp_buf_tag; 1];
    pub type __jmp_buf = [libc::c_long; 8];
}
use ::mozjpeg::*;

/* Define to `unsigned int' if <sys/types.h> does not define. */

/* #undef size_t */

/* Define to empty if `const' does not conform to ANSI C. */

/* #undef const */

/* #undef __CHAR_UNSIGNED__ */

/* Define to 1 if type `char' is unsigned and you are not using gcc.  */

/* Define if your (broken) compiler shifts signed values as if they were
unsigned. */

/* #undef RIGHT_SHIFT_IS_UNSIGNED */

/* Compiler does not support pointers to undefined structures. */

/* #undef INCOMPLETE_TYPES_BROKEN */

/* Define to 1 if the system has the type `unsigned short'. */

/* Define to 1 if the system has the type `unsigned char'. */

/* Define if you have BSD-like bzero and bcopy in <strings.h> rather than
memset/memcpy in <string.h>. */

/* #undef NEED_BSD_STRINGS */

/* Define if you need to include <sys/types.h> to get size_t. */

/* Define to 1 if you have the <stdlib.h> header file. */

/* Define to 1 if you have the <stddef.h> header file. */

/* Define to 1 if you have the <locale.h> header file. */

/* use 8 or 12 */

/*
 * Define BITS_IN_JSAMPLE as either
 *   8   for 8-bit sample values (the usual setting)
 *   12  for 12-bit sample values
 * Only 8 and 12 are legal data precisions for lossy JPEG according to the
 * JPEG standard, and the IJG code does not support anything else!
 * We do not support run-time selection of data precision, sorry.
 */

/* Use accelerated SIMD routines. */

/* Support in-memory source/destination managers */

/* Support arithmetic decoding */

/* #undef D_ARITH_CODING_SUPPORTED */

/* Support arithmetic encoding */

/* #undef C_ARITH_CODING_SUPPORTED */

/* libjpeg-turbo version in integer form */

/* libjpeg-turbo version */
pub use crate::stddef_h::size_t;

pub use crate::jconfig_h::JPEG_LIB_VERSION;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_CreateCompress;
pub use crate::jpeglib_h::jpeg_c_coef_controller;
pub use crate::jpeglib_h::jpeg_c_main_controller;
pub use crate::jpeglib_h::jpeg_c_prep_controller;
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
pub use crate::jpeglib_h::jpeg_forward_dct;
pub use crate::jpeglib_h::jpeg_marker_writer;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jpeg_set_defaults;
pub use crate::jpeglib_h::jpeg_std_error;
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
pub use crate::jpeglib_h::JDCT_FLOAT;
pub use crate::jpeglib_h::JDCT_IFAST;
pub use crate::jpeglib_h::JDCT_ISLOW;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::stdlib::__jmp_buf;
pub use crate::stdlib::__jmp_buf_tag;
pub use crate::stdlib::__sigset_t;
pub use crate::stdlib::_setjmp;
pub use crate::stdlib::jmp_buf;
pub use crate::stdlib::longjmp;
use crate::stdlib::printf;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _error_mgr {
    pub pub_0: crate::jpeglib_h::jpeg_error_mgr,
    pub jb: crate::stdlib::jmp_buf,
}

pub type error_mgr = _error_mgr;
/*
 * Copyright (C)2011 D. R. Commander.  All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * - Redistributions of source code must retain the above copyright notice,
 *   this list of conditions and the following disclaimer.
 * - Redistributions in binary form must reproduce the above copyright notice,
 *   this list of conditions and the following disclaimer in the documentation
 *   and/or other materials provided with the distribution.
 * - Neither the name of the libjpeg-turbo Project nor the names of its
 *   contributors may be used to endorse or promote products derived from this
 *   software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS",
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */
/* This program demonstrates how to check for the colorspace extension
capabilities of libjpeg-turbo at both compile time and run time. */

static mut lasterror: [libc::c_char; 200] = [
    78, 111, 32, 101, 114, 114, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

unsafe extern "C" fn my_error_exit(mut cinfo: crate::jpeglib_h::j_common_ptr) {
    let mut myerr: *mut error_mgr = (*cinfo).err as *mut error_mgr;
    Some(
        (*(*cinfo).err)
            .output_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    crate::stdlib::longjmp((*myerr).jb.as_mut_ptr(), 1 as libc::c_int);
}

unsafe extern "C" fn my_output_message(mut cinfo: crate::jpeglib_h::j_common_ptr) {
    Some(
        (*(*cinfo).err)
            .format_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, lasterror.as_mut_ptr());
}

unsafe fn main_0() -> libc::c_int {
    let mut jcs_valid: libc::c_int = -(1 as libc::c_int);
    let mut jcs_alpha_valid: libc::c_int = -(1 as libc::c_int);
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
    let mut jerr: error_mgr = error_mgr {
        pub_0: crate::jpeglib_h::jpeg_error_mgr {
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
        },
        jb: [crate::stdlib::__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: crate::stdlib::__sigset_t { __val: [0; 16] },
        }; 1],
    };
    crate::stdlib::printf(
        b"libjpeg-turbo colorspace extensions:\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"  Present at compile time\n\x00" as *const u8 as *const libc::c_char);
    cinfo.err = crate::jpeglib_h::jpeg_std_error(&mut jerr.pub_0);
    jerr.pub_0.error_exit =
        Some(my_error_exit as unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ());
    jerr.pub_0.output_message =
        Some(my_output_message as unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ());
    if crate::stdlib::_setjmp(jerr.jb.as_mut_ptr()) != 0 {
        /* this will execute if libjpeg has an error */
        jcs_valid = 0 as libc::c_int
    } else {
        crate::jpeglib_h::jpeg_CreateCompress(
            &mut cinfo,
            crate::jconfig_h::JPEG_LIB_VERSION,
            ::std::mem::size_of::<crate::jpeglib_h::jpeg_compress_struct>() as libc::c_ulong,
        );
        cinfo.input_components = 3 as libc::c_int;
        crate::jpeglib_h::jpeg_set_defaults(&mut cinfo);
        cinfo.in_color_space = crate::jpeglib_h::JCS_EXT_RGB;
        crate::jpeglib_h::jpeg_default_colorspace(&mut cinfo);
        jcs_valid = 1 as libc::c_int
    }
    if jcs_valid != 0 {
        crate::stdlib::printf(b"  Working properly\n\x00" as *const u8 as *const libc::c_char);
    } else {
        crate::stdlib::printf(
            b"  Not working properly.  Error returned was:\n    %s\n\x00" as *const u8
                as *const libc::c_char,
            lasterror.as_mut_ptr(),
        );
    }
    crate::stdlib::printf(
        b"libjpeg-turbo alpha colorspace extensions:\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"  Present at compile time\n\x00" as *const u8 as *const libc::c_char);
    if crate::stdlib::_setjmp(jerr.jb.as_mut_ptr()) != 0 {
        /* this will execute if libjpeg has an error */
        jcs_alpha_valid = 0 as libc::c_int
    } else {
        cinfo.in_color_space = crate::jpeglib_h::JCS_EXT_RGBA;
        crate::jpeglib_h::jpeg_default_colorspace(&mut cinfo);
        jcs_alpha_valid = 1 as libc::c_int
    }
    if jcs_alpha_valid != 0 {
        crate::stdlib::printf(b"  Working properly\n\x00" as *const u8 as *const libc::c_char);
    } else {
        crate::stdlib::printf(
            b"  Not working properly.  Error returned was:\n    %s\n\x00" as *const u8
                as *const libc::c_char,
            lasterror.as_mut_ptr(),
        );
    }
    crate::jpeglib_h::jpeg_destroy_compress(&mut cinfo);
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
