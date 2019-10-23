use libc::{c_uchar, c_ulong, c_int, c_long, c_uint, c_void};extern "C" {
    #[no_mangle]
    pub fn jzero_far(target: *mut c_void, bytestozero: size_t);

    #[no_mangle]
    pub fn jround_up(a: c_long, b: c_long) -> c_long;

    #[no_mangle]
    pub fn jinit_compress_master(cinfo: j_compress_ptr);

    #[no_mangle]
    pub fn jdiv_round_up(a: c_long, b: c_long) -> c_long;

    #[no_mangle]
    pub fn jpeg_mem_dest_internal(
        cinfo: j_compress_ptr,
        outbuffer: *mut *mut c_uchar,
        outsize: *mut c_ulong,
        pool_id: c_int,
    );
    /* Constant tables in jutils.c */
    /* This table is not actually needed in v6a */
    #[no_mangle]
    pub static jpeg_natural_order: [c_int; 0];

    #[no_mangle]
    pub fn jinit_c_master_control(
        cinfo: j_compress_ptr,
        transcode_only: boolean,
    );

    #[no_mangle]
    pub fn jinit_color_converter(cinfo: j_compress_ptr);

    #[no_mangle]
    pub fn jinit_downsampler(cinfo: j_compress_ptr);

    #[no_mangle]
    pub fn jinit_c_prep_controller(
        cinfo: j_compress_ptr,
        need_full_buffer: boolean,
    );

    #[no_mangle]
    pub fn jinit_forward_dct(cinfo: j_compress_ptr);

    #[no_mangle]
    pub fn jinit_phuff_encoder(cinfo: j_compress_ptr);

    #[no_mangle]
    pub fn jinit_huff_encoder(cinfo: j_compress_ptr);

    #[no_mangle]
    pub fn jinit_c_coef_controller(
        cinfo: j_compress_ptr,
        need_full_buffer: boolean,
    );

    #[no_mangle]
    pub fn jinit_c_main_controller(
        cinfo: j_compress_ptr,
        need_full_buffer: boolean,
    );

    #[no_mangle]
    pub fn jcopy_sample_rows(
        input_array: JSAMPARRAY,
        source_row: c_int,
        output_array: JSAMPARRAY,
        dest_row: c_int,
        num_rows: c_int,
        num_cols: JDIMENSION,
    );

    #[no_mangle]
    pub fn jinit_master_decompress(cinfo: j_decompress_ptr);

    #[no_mangle]
    pub fn jinit_upsampler(cinfo: j_decompress_ptr);

    #[no_mangle]
    pub fn jinit_d_main_controller(
        cinfo: j_decompress_ptr,
        need_full_buffer: boolean,
    );

    #[no_mangle]
    pub fn jinit_d_coef_controller(
        cinfo: j_decompress_ptr,
        need_full_buffer: boolean,
    );

    #[no_mangle]
    pub fn jinit_huff_decoder(cinfo: j_decompress_ptr);

    #[no_mangle]
    pub fn jinit_phuff_decoder(cinfo: j_decompress_ptr);

    #[no_mangle]
    pub fn jinit_inverse_dct(cinfo: j_decompress_ptr);

    #[no_mangle]
    pub fn jinit_d_post_controller(
        cinfo: j_decompress_ptr,
        need_full_buffer: boolean,
    );

    #[no_mangle]
    pub fn jinit_color_deconverter(cinfo: j_decompress_ptr);

    #[no_mangle]
    pub fn jinit_merged_upsampler(cinfo: j_decompress_ptr);

    #[no_mangle]
    pub fn jinit_2pass_quantizer(cinfo: j_decompress_ptr);

    #[no_mangle]
    pub fn jinit_1pass_quantizer(cinfo: j_decompress_ptr);

    #[no_mangle]
    pub fn jcopy_block_row(
        input_row: JBLOCKROW,
        output_row: JBLOCKROW,
        num_blocks: JDIMENSION,
    );
    /* It is useful to allow each component to have a separate IDCT method. */
    /* Upsampling (note that upsampler must also call color converter) */
    /* TRUE if need rows above & below */
    /* Colorspace conversion */
    /* Color quantization or color precision reduction */
    /* Miscellaneous useful macros */
    /* We assume that right shift corresponds to signed division by 2 with
     * rounding towards minus infinity.  This is correct for typical "arithmetic
     * shift" instructions that shift in copies of the sign bit.  But some
     * C compilers implement >> with an unsigned shift.  For these machines you
     * must define RIGHT_SHIFT_IS_UNSIGNED.
     * RIGHT_SHIFT provides a proper signed right shift of a JLONG quantity.
     * It is only applied with constant shift counts.  SHIFT_TEMPS must be
     * included in the variables of any routine using RIGHT_SHIFT.
     */
    /* Compression module initialization routines */
    /* Decompression module initialization routines */
    /* Memory manager initialization */
    #[no_mangle]
    pub fn jinit_memory_mgr(cinfo: j_common_ptr);

    #[no_mangle]
    pub fn jinit_marker_writer(cinfo: j_compress_ptr);

    #[no_mangle]
    pub fn jinit_marker_reader(cinfo: j_decompress_ptr);

    #[no_mangle]
    pub fn jinit_input_controller(cinfo: j_decompress_ptr);
}
// =============== BEGIN jpegint_h ================
pub type inverse_DCT_method_ptr = Option<
    unsafe extern "C" fn(
        _: j_decompress_ptr,
        _: *mut jpeg_component_info,
        _: JCOEFPTR,
        _: JSAMPARRAY,
        _: JDIMENSION,
    ) -> (),
>;
pub const DSTATE_START: c_int = 200i32;
pub type JLONG = c_long;
pub const DSTATE_STOPPING: c_int = 210i32;
pub const DSTATE_RDCOEFS: c_int = 209i32;




use crate::jmorecfg_h::JDIMENSION;use crate::jpeglib_h::{j_decompress_ptr, jpeg_component_info, JCOEFPTR,
                       JSAMPARRAY};
pub const DSTATE_SCANNING: c_int = 205i32;
pub const DSTATE_RAW_OK: c_int = 206i32;
pub const DSTATE_PRESCAN: c_int = 204i32;
pub const DSTATE_PRELOAD: c_int = 203i32;
pub const DSTATE_BUFIMAGE: c_int = 207i32;
pub const DSTATE_READY: c_int = 202i32;
pub const DSTATE_BUFPOST: c_int = 208i32;
pub type J_BUF_MODE = c_uint;



use crate::jpeglib_h::{j_common_ptr, JBLOCKROW};
pub const JBUF_REQUANT: J_BUF_MODE = 4;
pub const JBUF_SAVE_AND_PASS: J_BUF_MODE = 3;
pub const JBUF_CRANK_DEST: J_BUF_MODE = 2;
pub const JBUF_SAVE_SOURCE: J_BUF_MODE = 1;
pub const JBUF_PASS_THRU: J_BUF_MODE = 0;
/* Values of global_state field (jdapi.c has some dependencies on ordering!) */
pub const CSTATE_START: c_int = 100i32;
/* after create_compress */
pub const CSTATE_SCANNING: c_int = 101i32;
/* start_compress done, write_scanlines OK */
pub const CSTATE_RAW_OK: c_int = 102i32;
/* start_compress done, write_raw_data OK */
pub const CSTATE_WRCOEFS: c_int = 103i32;





use crate::jmorecfg_h::boolean;use crate::stddef_h::size_t;use crate::jpeglib_h::{j_compress_ptr};
