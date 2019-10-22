extern "C" {
    #[no_mangle]
    pub fn jzero_far(target: *mut libc::c_void, bytestozero: crate::stddef_h::size_t);

    #[no_mangle]
    pub fn jround_up(a: libc::c_long, b: libc::c_long) -> libc::c_long;

    #[no_mangle]
    pub fn jinit_compress_master(cinfo: crate::jpeglib_h::j_compress_ptr);

    #[no_mangle]
    pub fn jdiv_round_up(a: libc::c_long, b: libc::c_long) -> libc::c_long;

    #[no_mangle]
    pub fn jpeg_mem_dest_internal(
        cinfo: crate::jpeglib_h::j_compress_ptr,
        outbuffer: *mut *mut libc::c_uchar,
        outsize: *mut libc::c_ulong,
        pool_id: libc::c_int,
    );
    /* Constant tables in jutils.c */
    /* This table is not actually needed in v6a */
    #[no_mangle]
    pub static jpeg_natural_order: [libc::c_int; 0];

    #[no_mangle]
    pub fn jinit_c_master_control(
        cinfo: crate::jpeglib_h::j_compress_ptr,
        transcode_only: crate::jmorecfg_h::boolean,
    );

    #[no_mangle]
    pub fn jinit_color_converter(cinfo: crate::jpeglib_h::j_compress_ptr);

    #[no_mangle]
    pub fn jinit_downsampler(cinfo: crate::jpeglib_h::j_compress_ptr);

    #[no_mangle]
    pub fn jinit_c_prep_controller(
        cinfo: crate::jpeglib_h::j_compress_ptr,
        need_full_buffer: crate::jmorecfg_h::boolean,
    );

    #[no_mangle]
    pub fn jinit_forward_dct(cinfo: crate::jpeglib_h::j_compress_ptr);

    #[no_mangle]
    pub fn jinit_phuff_encoder(cinfo: crate::jpeglib_h::j_compress_ptr);

    #[no_mangle]
    pub fn jinit_huff_encoder(cinfo: crate::jpeglib_h::j_compress_ptr);

    #[no_mangle]
    pub fn jinit_c_coef_controller(
        cinfo: crate::jpeglib_h::j_compress_ptr,
        need_full_buffer: crate::jmorecfg_h::boolean,
    );

    #[no_mangle]
    pub fn jinit_c_main_controller(
        cinfo: crate::jpeglib_h::j_compress_ptr,
        need_full_buffer: crate::jmorecfg_h::boolean,
    );

    #[no_mangle]
    pub fn jcopy_sample_rows(
        input_array: crate::jpeglib_h::JSAMPARRAY,
        source_row: libc::c_int,
        output_array: crate::jpeglib_h::JSAMPARRAY,
        dest_row: libc::c_int,
        num_rows: libc::c_int,
        num_cols: crate::jmorecfg_h::JDIMENSION,
    );

    #[no_mangle]
    pub fn jinit_master_decompress(cinfo: crate::jpeglib_h::j_decompress_ptr);

    #[no_mangle]
    pub fn jinit_upsampler(cinfo: crate::jpeglib_h::j_decompress_ptr);

    #[no_mangle]
    pub fn jinit_d_main_controller(
        cinfo: crate::jpeglib_h::j_decompress_ptr,
        need_full_buffer: crate::jmorecfg_h::boolean,
    );

    #[no_mangle]
    pub fn jinit_d_coef_controller(
        cinfo: crate::jpeglib_h::j_decompress_ptr,
        need_full_buffer: crate::jmorecfg_h::boolean,
    );

    #[no_mangle]
    pub fn jinit_huff_decoder(cinfo: crate::jpeglib_h::j_decompress_ptr);

    #[no_mangle]
    pub fn jinit_phuff_decoder(cinfo: crate::jpeglib_h::j_decompress_ptr);

    #[no_mangle]
    pub fn jinit_inverse_dct(cinfo: crate::jpeglib_h::j_decompress_ptr);

    #[no_mangle]
    pub fn jinit_d_post_controller(
        cinfo: crate::jpeglib_h::j_decompress_ptr,
        need_full_buffer: crate::jmorecfg_h::boolean,
    );

    #[no_mangle]
    pub fn jinit_color_deconverter(cinfo: crate::jpeglib_h::j_decompress_ptr);

    #[no_mangle]
    pub fn jinit_merged_upsampler(cinfo: crate::jpeglib_h::j_decompress_ptr);

    #[no_mangle]
    pub fn jinit_2pass_quantizer(cinfo: crate::jpeglib_h::j_decompress_ptr);

    #[no_mangle]
    pub fn jinit_1pass_quantizer(cinfo: crate::jpeglib_h::j_decompress_ptr);

    #[no_mangle]
    pub fn jcopy_block_row(
        input_row: crate::jpeglib_h::JBLOCKROW,
        output_row: crate::jpeglib_h::JBLOCKROW,
        num_blocks: crate::jmorecfg_h::JDIMENSION,
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
    pub fn jinit_memory_mgr(cinfo: crate::jpeglib_h::j_common_ptr);

    #[no_mangle]
    pub fn jinit_marker_writer(cinfo: crate::jpeglib_h::j_compress_ptr);

    #[no_mangle]
    pub fn jinit_marker_reader(cinfo: crate::jpeglib_h::j_decompress_ptr);

    #[no_mangle]
    pub fn jinit_input_controller(cinfo: crate::jpeglib_h::j_decompress_ptr);
}
// =============== BEGIN jpegint_h ================
pub type inverse_DCT_method_ptr = Option<
    unsafe extern "C" fn(
        _: crate::jpeglib_h::j_decompress_ptr,
        _: *mut crate::jpeglib_h::jpeg_component_info,
        _: crate::jpeglib_h::JCOEFPTR,
        _: crate::jpeglib_h::JSAMPARRAY,
        _: crate::jmorecfg_h::JDIMENSION,
    ) -> (),
>;
pub const DSTATE_START: libc::c_int = 200i32;
pub type JLONG = libc::c_long;
pub const DSTATE_STOPPING: libc::c_int = 210i32;
pub const DSTATE_RDCOEFS: libc::c_int = 209i32;
use crate::jmorecfg_h::JDIMENSION;
use crate::jpeglib_h::j_decompress_ptr;
use crate::jpeglib_h::jpeg_component_info;
use crate::jpeglib_h::JCOEFPTR;
use crate::jpeglib_h::JSAMPARRAY;
pub const DSTATE_SCANNING: libc::c_int = 205i32;
pub const DSTATE_RAW_OK: libc::c_int = 206i32;
pub const DSTATE_PRESCAN: libc::c_int = 204i32;
pub const DSTATE_PRELOAD: libc::c_int = 203i32;
pub const DSTATE_BUFIMAGE: libc::c_int = 207i32;
pub const DSTATE_READY: libc::c_int = 202i32;
pub const DSTATE_BUFPOST: libc::c_int = 208i32;
pub type J_BUF_MODE = libc::c_uint;
use crate::jpeglib_h::j_common_ptr;
use crate::jpeglib_h::jpeg_common_struct;
use crate::jpeglib_h::JBLOCK;
use crate::jpeglib_h::JBLOCKROW;
pub const JBUF_REQUANT: crate::jpegint_h::J_BUF_MODE = 4;
pub const JBUF_SAVE_AND_PASS: crate::jpegint_h::J_BUF_MODE = 3;
pub const JBUF_CRANK_DEST: crate::jpegint_h::J_BUF_MODE = 2;
pub const JBUF_SAVE_SOURCE: crate::jpegint_h::J_BUF_MODE = 1;
pub const JBUF_PASS_THRU: crate::jpegint_h::J_BUF_MODE = 0;
/* Values of global_state field (jdapi.c has some dependencies on ordering!) */
pub const CSTATE_START: libc::c_int = 100i32;
/* after create_compress */
pub const CSTATE_SCANNING: libc::c_int = 101i32;
/* start_compress done, write_scanlines OK */
pub const CSTATE_RAW_OK: libc::c_int = 102i32;
/* start_compress done, write_raw_data OK */
pub const CSTATE_WRCOEFS: libc::c_int = 103i32;
use crate::jmorecfg_h::boolean;
use crate::jpeglib_h::j_compress_ptr;
use crate::jpeglib_h::jpeg_compress_struct;
use crate::jpeglib_h::jpeg_decompress_struct;
use crate::jpeglib_h::JSAMPROW;
use crate::stddef_h::size_t;
