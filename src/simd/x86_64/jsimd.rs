use libc::{c_char, c_float, c_int, c_uint, c_ulong, c_void};
extern "C" {
    #[no_mangle]
    pub fn jpeg_simd_cpu_support() -> c_uint;

    #[no_mangle]
    pub static jconst_rgb_ycc_convert_sse2: [c_int; 0];

    #[no_mangle]
    pub fn jsimd_rgb_ycc_convert_sse2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extrgb_ycc_convert_sse2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extrgbx_ycc_convert_sse2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extbgr_ycc_convert_sse2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extbgrx_ycc_convert_sse2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extxbgr_ycc_convert_sse2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extxrgb_ycc_convert_sse2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub static jconst_rgb_ycc_convert_avx2: [c_int; 0];

    #[no_mangle]
    pub fn jsimd_rgb_ycc_convert_avx2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extrgb_ycc_convert_avx2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extrgbx_ycc_convert_avx2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extbgr_ycc_convert_avx2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extbgrx_ycc_convert_avx2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extxbgr_ycc_convert_avx2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extxrgb_ycc_convert_avx2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub static jconst_rgb_gray_convert_sse2: [c_int; 0];

    #[no_mangle]
    pub fn jsimd_rgb_gray_convert_sse2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extrgb_gray_convert_sse2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extrgbx_gray_convert_sse2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extbgr_gray_convert_sse2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extbgrx_gray_convert_sse2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extxbgr_gray_convert_sse2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extxrgb_gray_convert_sse2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub static jconst_rgb_gray_convert_avx2: [c_int; 0];

    #[no_mangle]
    pub fn jsimd_rgb_gray_convert_avx2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extrgb_gray_convert_avx2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extrgbx_gray_convert_avx2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extbgr_gray_convert_avx2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extbgrx_gray_convert_avx2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extxbgr_gray_convert_avx2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_extxrgb_gray_convert_avx2(
        img_width: JDIMENSION,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: c_int,
    );

    #[no_mangle]
    pub static jconst_ycc_rgb_convert_sse2: [c_int; 0];

    #[no_mangle]
    pub fn jsimd_ycc_rgb_convert_sse2(
        out_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        input_row: JDIMENSION,
        output_buf: JSAMPARRAY,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extrgb_convert_sse2(
        out_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        input_row: JDIMENSION,
        output_buf: JSAMPARRAY,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extrgbx_convert_sse2(
        out_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        input_row: JDIMENSION,
        output_buf: JSAMPARRAY,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extbgr_convert_sse2(
        out_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        input_row: JDIMENSION,
        output_buf: JSAMPARRAY,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extbgrx_convert_sse2(
        out_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        input_row: JDIMENSION,
        output_buf: JSAMPARRAY,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extxbgr_convert_sse2(
        out_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        input_row: JDIMENSION,
        output_buf: JSAMPARRAY,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extxrgb_convert_sse2(
        out_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        input_row: JDIMENSION,
        output_buf: JSAMPARRAY,
        num_rows: c_int,
    );

    #[no_mangle]
    pub static jconst_ycc_rgb_convert_avx2: [c_int; 0];

    #[no_mangle]
    pub fn jsimd_ycc_rgb_convert_avx2(
        out_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        input_row: JDIMENSION,
        output_buf: JSAMPARRAY,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extrgb_convert_avx2(
        out_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        input_row: JDIMENSION,
        output_buf: JSAMPARRAY,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extrgbx_convert_avx2(
        out_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        input_row: JDIMENSION,
        output_buf: JSAMPARRAY,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extbgr_convert_avx2(
        out_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        input_row: JDIMENSION,
        output_buf: JSAMPARRAY,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extbgrx_convert_avx2(
        out_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        input_row: JDIMENSION,
        output_buf: JSAMPARRAY,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extxbgr_convert_avx2(
        out_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        input_row: JDIMENSION,
        output_buf: JSAMPARRAY,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extxrgb_convert_avx2(
        out_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        input_row: JDIMENSION,
        output_buf: JSAMPARRAY,
        num_rows: c_int,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_downsample_sse2(
        image_width: JDIMENSION,
        max_v_samp_factor: c_int,
        v_samp_factor: JDIMENSION,
        width_in_blocks: JDIMENSION,
        input_data: JSAMPARRAY,
        output_data: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_downsample_avx2(
        image_width: JDIMENSION,
        max_v_samp_factor: c_int,
        v_samp_factor: JDIMENSION,
        width_in_blocks: JDIMENSION,
        input_data: JSAMPARRAY,
        output_data: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_downsample_sse2(
        image_width: JDIMENSION,
        max_v_samp_factor: c_int,
        v_samp_factor: JDIMENSION,
        width_in_blocks: JDIMENSION,
        input_data: JSAMPARRAY,
        output_data: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_downsample_avx2(
        image_width: JDIMENSION,
        max_v_samp_factor: c_int,
        v_samp_factor: JDIMENSION,
        width_in_blocks: JDIMENSION,
        input_data: JSAMPARRAY,
        output_data: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_upsample_sse2(
        max_v_samp_factor: c_int,
        output_width: JDIMENSION,
        input_data: JSAMPARRAY,
        output_data_ptr: *mut JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_upsample_sse2(
        max_v_samp_factor: c_int,
        output_width: JDIMENSION,
        input_data: JSAMPARRAY,
        output_data_ptr: *mut JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_upsample_avx2(
        max_v_samp_factor: c_int,
        output_width: JDIMENSION,
        input_data: JSAMPARRAY,
        output_data_ptr: *mut JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_upsample_avx2(
        max_v_samp_factor: c_int,
        output_width: JDIMENSION,
        input_data: JSAMPARRAY,
        output_data_ptr: *mut JSAMPARRAY,
    );

    #[no_mangle]
    pub static jconst_fancy_upsample_sse2: [c_int; 0];

    #[no_mangle]
    pub fn jsimd_h2v1_fancy_upsample_sse2(
        max_v_samp_factor: c_int,
        downsampled_width: JDIMENSION,
        input_data: JSAMPARRAY,
        output_data_ptr: *mut JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_fancy_upsample_sse2(
        max_v_samp_factor: c_int,
        downsampled_width: JDIMENSION,
        input_data: JSAMPARRAY,
        output_data_ptr: *mut JSAMPARRAY,
    );

    #[no_mangle]
    pub static jconst_fancy_upsample_avx2: [c_int; 0];

    #[no_mangle]
    pub fn jsimd_h2v1_fancy_upsample_avx2(
        max_v_samp_factor: c_int,
        downsampled_width: JDIMENSION,
        input_data: JSAMPARRAY,
        output_data_ptr: *mut JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_fancy_upsample_avx2(
        max_v_samp_factor: c_int,
        downsampled_width: JDIMENSION,
        input_data: JSAMPARRAY,
        output_data_ptr: *mut JSAMPARRAY,
    );

    #[no_mangle]
    pub static jconst_merged_upsample_sse2: [c_int; 0];

    #[no_mangle]
    pub fn jsimd_h2v1_merged_upsample_sse2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extrgb_merged_upsample_sse2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extrgbx_merged_upsample_sse2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extbgr_merged_upsample_sse2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extbgrx_merged_upsample_sse2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extxbgr_merged_upsample_sse2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extxrgb_merged_upsample_sse2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_merged_upsample_sse2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extrgb_merged_upsample_sse2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extrgbx_merged_upsample_sse2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extbgr_merged_upsample_sse2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extbgrx_merged_upsample_sse2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extxbgr_merged_upsample_sse2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extxrgb_merged_upsample_sse2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub static jconst_merged_upsample_avx2: [c_int; 0];

    #[no_mangle]
    pub fn jsimd_h2v1_merged_upsample_avx2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extrgb_merged_upsample_avx2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extrgbx_merged_upsample_avx2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extbgr_merged_upsample_avx2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extbgrx_merged_upsample_avx2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extxbgr_merged_upsample_avx2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extxrgb_merged_upsample_avx2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_merged_upsample_avx2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extrgb_merged_upsample_avx2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extrgbx_merged_upsample_avx2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extbgr_merged_upsample_avx2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extbgrx_merged_upsample_avx2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extxbgr_merged_upsample_avx2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extxrgb_merged_upsample_avx2(
        output_width: JDIMENSION,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_convsamp_sse2(
        sample_data: JSAMPARRAY,
        start_col: JDIMENSION,
        workspace: *mut DCTELEM,
    );

    #[no_mangle]
    pub fn jsimd_convsamp_avx2(
        sample_data: JSAMPARRAY,
        start_col: JDIMENSION,
        workspace: *mut DCTELEM,
    );

    #[no_mangle]
    pub fn jsimd_convsamp_float_sse2(
        sample_data: JSAMPARRAY,
        start_col: JDIMENSION,
        workspace: *mut c_float,
    );

    #[no_mangle]
    pub static jconst_fdct_islow_sse2: [c_int; 0];

    #[no_mangle]
    pub fn jsimd_fdct_islow_sse2(data: *mut DCTELEM);

    #[no_mangle]
    pub static jconst_fdct_islow_avx2: [c_int; 0];

    #[no_mangle]
    pub fn jsimd_fdct_islow_avx2(data: *mut DCTELEM);

    #[no_mangle]
    pub static jconst_fdct_ifast_sse2: [c_int; 0];

    #[no_mangle]
    pub fn jsimd_fdct_ifast_sse2(data: *mut DCTELEM);

    #[no_mangle]
    pub static jconst_fdct_float_sse: [c_int; 0];

    #[no_mangle]
    pub fn jsimd_fdct_float_sse(data: *mut c_float);

    #[no_mangle]
    pub fn jsimd_quantize_sse2(
        coef_block: JCOEFPTR,
        divisors: *mut DCTELEM,
        workspace: *mut DCTELEM,
    );

    #[no_mangle]
    pub fn jsimd_quantize_avx2(
        coef_block: JCOEFPTR,
        divisors: *mut DCTELEM,
        workspace: *mut DCTELEM,
    );

    #[no_mangle]
    pub fn jsimd_quantize_float_sse2(
        coef_block: JCOEFPTR,
        divisors: *mut c_float,
        workspace: *mut c_float,
    );

    #[no_mangle]
    pub static jconst_idct_red_sse2: [c_int; 0];

    #[no_mangle]
    pub fn jsimd_idct_2x2_sse2(
        dct_table: *mut c_void,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub fn jsimd_idct_4x4_sse2(
        dct_table: *mut c_void,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub static jconst_idct_islow_sse2: [c_int; 0];

    #[no_mangle]
    pub fn jsimd_idct_islow_sse2(
        dct_table: *mut c_void,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub static jconst_idct_islow_avx2: [c_int; 0];

    #[no_mangle]
    pub fn jsimd_idct_islow_avx2(
        dct_table: *mut c_void,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub static jconst_idct_ifast_sse2: [c_int; 0];

    #[no_mangle]
    pub fn jsimd_idct_ifast_sse2(
        dct_table: *mut c_void,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );

    #[no_mangle]
    pub static jconst_idct_float_sse2: [c_int; 0];

    #[no_mangle]
    pub fn jsimd_idct_float_sse2(
        dct_table: *mut c_void,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );
    /* Huffman coding */
    #[no_mangle]
    pub static jconst_huff_encode_one_block: [c_int; 0];

    #[no_mangle]
    pub fn jsimd_huff_encode_one_block_sse2(
        state: *mut c_void,
        buffer: *mut JOCTET,
        block: JCOEFPTR,
        last_dc_val: c_int,
        dctbl: *mut c_derived_tbl,
        actbl: *mut c_derived_tbl,
    ) -> *mut JOCTET;
    /* Progressive Huffman encoding */
    #[no_mangle]
    pub fn jsimd_encode_mcu_AC_first_prepare_sse2(
        block: *const JCOEF,
        jpeg_natural_order_start: *const c_int,
        Sl: c_int,
        Al: c_int,
        values: *mut JCOEF,
        zerobits: *mut size_t,
    );

    #[no_mangle]
    pub fn jsimd_encode_mcu_AC_refine_prepare_sse2(
        block: *const JCOEF,
        jpeg_natural_order_start: *const c_int,
        Sl: c_int,
        Al: c_int,
        absvalues: *mut JCOEF,
        bits: *mut size_t,
    ) -> c_int;
}

pub use crate::jconfig_h::BITS_IN_JSAMPLE;
pub use crate::jconfigint_h::SIZEOF_SIZE_T;
pub use crate::jdct_h::{
    DCTELEM, FLOAT_MULT_TYPE, IFAST_MULT_TYPE, IFAST_SCALE_BITS, ISLOW_MULT_TYPE,
};
pub use crate::jmorecfg_h::{
    boolean, JCOEF, JDIMENSION, JOCTET, JSAMPLE, RGB_PIXELSIZE, UINT16, UINT8,
};
pub use crate::jpegint_h::{
    inverse_DCT_method_ptr, JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS,
    JBUF_SAVE_SOURCE, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_compress_ptr, j_decompress_ptr, jpeg_c_coef_controller, jpeg_c_main_controller,
    jpeg_c_prep_controller, jpeg_color_converter, jpeg_color_deconverter, jpeg_color_quantizer,
    jpeg_common_struct, jpeg_comp_master, jpeg_component_info, jpeg_compress_struct,
    jpeg_d_coef_controller, jpeg_d_main_controller, jpeg_d_post_controller, jpeg_decomp_master,
    jpeg_decompress_struct, jpeg_destination_mgr, jpeg_downsampler, jpeg_entropy_decoder,
    jpeg_entropy_encoder, jpeg_error_mgr, jpeg_forward_dct, jpeg_input_controller,
    jpeg_inverse_dct, jpeg_marker_parser_method, jpeg_marker_reader, jpeg_marker_struct,
    jpeg_marker_writer, jpeg_memory_mgr, jpeg_progress_mgr, jpeg_saved_marker_ptr, jpeg_scan_info,
    jpeg_source_mgr, jpeg_upsampler, jvirt_barray_control, jvirt_barray_ptr, jvirt_sarray_control,
    jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr, DCTSIZE, JBLOCK, JBLOCKARRAY, JBLOCKROW,
    JCOEFPTR, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX,
    JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB,
    JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JDITHER_FS,
    JDITHER_NONE, JDITHER_ORDERED, JHUFF_TBL, JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW,
    J_COLOR_SPACE, J_DCT_METHOD, J_DITHER_MODE,
};
pub use crate::src::jchuff::c_derived_tbl;
pub use crate::stddef_h::{size_t, NULL};
use crate::stdlib::{getenv, strcmp};
use libc;
// =============== BEGIN jsimd_h ================

/*
 * simd/jsimd.h
 *
 * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
 * Copyright (C) 2011, 2014-2016, 2018, D. R. Commander.
 * Copyright (C) 2013-2014, MIPS Technologies, Inc., California.
 * Copyright (C) 2014, Linaro Limited.
 * Copyright (C) 2015-2016, 2018, Matthieu Darbois.
 * Copyright (C) 2016-2017, Loongson Technology Corporation Limited, BeiJing.
 *
 * Based on the x86 SIMD extension for IJG JPEG library,
 * Copyright (C) 1999-2006, MIYASAKA Masaru.
 * For conditions of distribution and use, see copyright notice in jsimdext.inc
 *
 */

/* Bitmask for supported acceleration methods */
pub const JSIMD_SSE: c_int = 0x4i32;

pub const JSIMD_SSE2: c_int = 0x8i32;

pub const JSIMD_AVX2: c_int = 0x80i32;
/*
 * jsimd_x86_64.c
 *
 * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
 * Copyright (C) 2009-2011, 2014, 2016, 2018, D. R. Commander.
 * Copyright (C) 2015-2016, 2018, Matthieu Darbois.
 *
 * Based on the x86 SIMD extension for IJG JPEG library,
 * Copyright (C) 1999-2006, MIYASAKA Masaru.
 * For conditions of distribution and use, see copyright notice in jsimdext.inc
 *
 * This file contains the interface between the "normal" portions
 * of the library and the SIMD implementations when running on a
 * 64-bit x86 architecture.
 */
/*
 * In the PIC cases, we have no guarantee that constants will keep
 * their alignment. This macro allows us to verify it at runtime.
 */
/* 16 byte alignment */
/* 32 byte alignment */

static mut simd_support: c_uint = !0i32 as c_uint;

static mut simd_huffman: c_uint = 1i32 as c_uint;
/*
 * Check what SIMD accelerations are supported.
 *
 * FIXME: This code is racy under a multi-threaded environment.
 */

unsafe extern "C" fn init_simd() {
    let mut env: *mut c_char = NULL as *mut c_char;
    if simd_support != !0u32 {
        return;
    }
    simd_support = jpeg_simd_cpu_support();
    /* Force different settings through environment variables */
    env = getenv(b"JSIMD_FORCESSE2\x00" as *const u8 as *const c_char);
    if !env.is_null() && strcmp(env, b"1\x00" as *const u8 as *const c_char) == 0i32 {
        simd_support &= JSIMD_SSE2 as c_uint
    }
    env = getenv(b"JSIMD_FORCEAVX2\x00" as *const u8 as *const c_char);
    if !env.is_null() && strcmp(env, b"1\x00" as *const u8 as *const c_char) == 0i32 {
        simd_support &= JSIMD_AVX2 as c_uint
    }
    env = getenv(b"JSIMD_FORCENONE\x00" as *const u8 as *const c_char);
    if !env.is_null() && strcmp(env, b"1\x00" as *const u8 as *const c_char) == 0i32 {
        simd_support = 0i32 as c_uint
    }
    env = getenv(b"JSIMD_NOHUFFENC\x00" as *const u8 as *const c_char);
    if !env.is_null() && strcmp(env, b"1\x00" as *const u8 as *const c_char) == 0i32 {
        simd_huffman = 0i32 as c_uint
    };
}
/*
 * jsimd.h
 *
 * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
 * Copyright (C) 2011, 2014, D. R. Commander.
 * Copyright (C) 2015-2016, 2018, Matthieu Darbois.
 *
 * Based on the x86 SIMD extension for IJG JPEG library,
 * Copyright (C) 1999-2006, MIYASAKA Masaru.
 * For conditions of distribution and use, see copyright notice in jsimdext.inc
 *
 */
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_rgb_ycc() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JDIMENSION>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if RGB_PIXELSIZE != 3i32 && RGB_PIXELSIZE != 4i32 {
        return 0i32;
    }
    if simd_support & JSIMD_AVX2 as c_uint != 0
        && jconst_rgb_ycc_convert_avx2.as_ptr() as size_t & ((1i32 << 5i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0
        && jconst_rgb_ycc_convert_sse2.as_ptr() as size_t & ((1i32 << 4i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_rgb_gray() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JDIMENSION>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if RGB_PIXELSIZE != 3i32 && RGB_PIXELSIZE != 4i32 {
        return 0i32;
    }
    if simd_support & JSIMD_AVX2 as c_uint != 0
        && jconst_rgb_gray_convert_avx2.as_ptr() as size_t & ((1i32 << 5i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0
        && jconst_rgb_gray_convert_sse2.as_ptr() as size_t & ((1i32 << 4i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_ycc_rgb() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JDIMENSION>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if RGB_PIXELSIZE != 3i32 && RGB_PIXELSIZE != 4i32 {
        return 0i32;
    }
    if simd_support & JSIMD_AVX2 as c_uint != 0
        && jconst_ycc_rgb_convert_avx2.as_ptr() as size_t & ((1i32 << 5i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0
        && jconst_ycc_rgb_convert_sse2.as_ptr() as size_t & ((1i32 << 4i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_ycc_rgb565() -> c_int {
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_rgb_ycc_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut avx2fct: Option<
        unsafe extern "C" fn(
            _: JDIMENSION,
            _: JSAMPARRAY,
            _: JSAMPIMAGE,
            _: JDIMENSION,
            _: c_int,
        ) -> (),
    > = None;
    let mut sse2fct: Option<
        unsafe extern "C" fn(
            _: JDIMENSION,
            _: JSAMPARRAY,
            _: JSAMPIMAGE,
            _: JDIMENSION,
            _: c_int,
        ) -> (),
    > = None;
    match (*cinfo).in_color_space as c_uint {
        6 => {
            avx2fct = Some(
                jsimd_extrgb_ycc_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_extrgb_ycc_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            )
        }
        7 | 12 => {
            avx2fct = Some(
                jsimd_extrgbx_ycc_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_extrgbx_ycc_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            )
        }
        8 => {
            avx2fct = Some(
                jsimd_extbgr_ycc_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_extbgr_ycc_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            )
        }
        9 | 13 => {
            avx2fct = Some(
                jsimd_extbgrx_ycc_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_extbgrx_ycc_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            )
        }
        10 | 14 => {
            avx2fct = Some(
                jsimd_extxbgr_ycc_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_extxbgr_ycc_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            )
        }
        11 | 15 => {
            avx2fct = Some(
                jsimd_extxrgb_ycc_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_extxrgb_ycc_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            )
        }
        _ => {
            avx2fct = Some(
                jsimd_rgb_ycc_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_rgb_ycc_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            )
        }
    }
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        avx2fct.expect("non-null function pointer")(
            (*cinfo).image_width,
            input_buf,
            output_buf,
            output_row,
            num_rows,
        );
    } else {
        sse2fct.expect("non-null function pointer")(
            (*cinfo).image_width,
            input_buf,
            output_buf,
            output_row,
            num_rows,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_rgb_gray_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: c_int,
) {
    let mut avx2fct: Option<
        unsafe extern "C" fn(
            _: JDIMENSION,
            _: JSAMPARRAY,
            _: JSAMPIMAGE,
            _: JDIMENSION,
            _: c_int,
        ) -> (),
    > = None;
    let mut sse2fct: Option<
        unsafe extern "C" fn(
            _: JDIMENSION,
            _: JSAMPARRAY,
            _: JSAMPIMAGE,
            _: JDIMENSION,
            _: c_int,
        ) -> (),
    > = None;
    match (*cinfo).in_color_space as c_uint {
        6 => {
            avx2fct = Some(
                jsimd_extrgb_gray_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_extrgb_gray_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            )
        }
        7 | 12 => {
            avx2fct = Some(
                jsimd_extrgbx_gray_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_extrgbx_gray_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            )
        }
        8 => {
            avx2fct = Some(
                jsimd_extbgr_gray_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_extbgr_gray_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            )
        }
        9 | 13 => {
            avx2fct = Some(
                jsimd_extbgrx_gray_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_extbgrx_gray_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            )
        }
        10 | 14 => {
            avx2fct = Some(
                jsimd_extxbgr_gray_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_extxbgr_gray_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            )
        }
        11 | 15 => {
            avx2fct = Some(
                jsimd_extxrgb_gray_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_extxrgb_gray_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            )
        }
        _ => {
            avx2fct = Some(
                jsimd_rgb_gray_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_rgb_gray_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: c_int,
                    ) -> (),
            )
        }
    }
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        avx2fct.expect("non-null function pointer")(
            (*cinfo).image_width,
            input_buf,
            output_buf,
            output_row,
            num_rows,
        );
    } else {
        sse2fct.expect("non-null function pointer")(
            (*cinfo).image_width,
            input_buf,
            output_buf,
            output_row,
            num_rows,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_ycc_rgb_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
    let mut avx2fct: Option<
        unsafe extern "C" fn(
            _: JDIMENSION,
            _: JSAMPIMAGE,
            _: JDIMENSION,
            _: JSAMPARRAY,
            _: c_int,
        ) -> (),
    > = None;
    let mut sse2fct: Option<
        unsafe extern "C" fn(
            _: JDIMENSION,
            _: JSAMPIMAGE,
            _: JDIMENSION,
            _: JSAMPARRAY,
            _: c_int,
        ) -> (),
    > = None;
    match (*cinfo).out_color_space as c_uint {
        6 => {
            avx2fct = Some(
                jsimd_ycc_extrgb_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_ycc_extrgb_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: c_int,
                    ) -> (),
            )
        }
        7 | 12 => {
            avx2fct = Some(
                jsimd_ycc_extrgbx_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_ycc_extrgbx_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: c_int,
                    ) -> (),
            )
        }
        8 => {
            avx2fct = Some(
                jsimd_ycc_extbgr_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_ycc_extbgr_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: c_int,
                    ) -> (),
            )
        }
        9 | 13 => {
            avx2fct = Some(
                jsimd_ycc_extbgrx_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_ycc_extbgrx_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: c_int,
                    ) -> (),
            )
        }
        10 | 14 => {
            avx2fct = Some(
                jsimd_ycc_extxbgr_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_ycc_extxbgr_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: c_int,
                    ) -> (),
            )
        }
        11 | 15 => {
            avx2fct = Some(
                jsimd_ycc_extxrgb_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_ycc_extxrgb_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: c_int,
                    ) -> (),
            )
        }
        _ => {
            avx2fct = Some(
                jsimd_ycc_rgb_convert_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: c_int,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_ycc_rgb_convert_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                        _: c_int,
                    ) -> (),
            )
        }
    }
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        avx2fct.expect("non-null function pointer")(
            (*cinfo).output_width,
            input_buf,
            input_row,
            output_buf,
            num_rows,
        );
    } else {
        sse2fct.expect("non-null function pointer")(
            (*cinfo).output_width,
            input_buf,
            input_row,
            output_buf,
            num_rows,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_ycc_rgb565_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: c_int,
) {
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_h2v2_downsample() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JDIMENSION>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        return 1i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0 {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_h2v1_downsample() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JDIMENSION>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        return 1i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0 {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_h2v2_downsample(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data: JSAMPARRAY,
) {
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        jsimd_h2v2_downsample_avx2(
            (*cinfo).image_width,
            (*cinfo).max_v_samp_factor,
            (*compptr).v_samp_factor as JDIMENSION,
            (*compptr).width_in_blocks,
            input_data,
            output_data,
        );
    } else {
        jsimd_h2v2_downsample_sse2(
            (*cinfo).image_width,
            (*cinfo).max_v_samp_factor,
            (*compptr).v_samp_factor as JDIMENSION,
            (*compptr).width_in_blocks,
            input_data,
            output_data,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_h2v1_downsample(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data: JSAMPARRAY,
) {
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        jsimd_h2v1_downsample_avx2(
            (*cinfo).image_width,
            (*cinfo).max_v_samp_factor,
            (*compptr).v_samp_factor as JDIMENSION,
            (*compptr).width_in_blocks,
            input_data,
            output_data,
        );
    } else {
        jsimd_h2v1_downsample_sse2(
            (*cinfo).image_width,
            (*cinfo).max_v_samp_factor,
            (*compptr).v_samp_factor as JDIMENSION,
            (*compptr).width_in_blocks,
            input_data,
            output_data,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_h2v2_upsample() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JDIMENSION>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        return 1i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0 {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_h2v1_upsample() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JDIMENSION>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        return 1i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0 {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_h2v2_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        jsimd_h2v2_upsample_avx2(
            (*cinfo).max_v_samp_factor,
            (*cinfo).output_width,
            input_data,
            output_data_ptr,
        );
    } else {
        jsimd_h2v2_upsample_sse2(
            (*cinfo).max_v_samp_factor,
            (*cinfo).output_width,
            input_data,
            output_data_ptr,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_h2v1_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        jsimd_h2v1_upsample_avx2(
            (*cinfo).max_v_samp_factor,
            (*cinfo).output_width,
            input_data,
            output_data_ptr,
        );
    } else {
        jsimd_h2v1_upsample_sse2(
            (*cinfo).max_v_samp_factor,
            (*cinfo).output_width,
            input_data,
            output_data_ptr,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_h2v2_fancy_upsample() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JDIMENSION>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if simd_support & JSIMD_AVX2 as c_uint != 0
        && jconst_fancy_upsample_avx2.as_ptr() as size_t & ((1i32 << 5i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0
        && jconst_fancy_upsample_sse2.as_ptr() as size_t & ((1i32 << 4i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_h2v1_fancy_upsample() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JDIMENSION>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if simd_support & JSIMD_AVX2 as c_uint != 0
        && jconst_fancy_upsample_avx2.as_ptr() as size_t & ((1i32 << 5i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0
        && jconst_fancy_upsample_sse2.as_ptr() as size_t & ((1i32 << 4i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_h2v2_fancy_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        jsimd_h2v2_fancy_upsample_avx2(
            (*cinfo).max_v_samp_factor,
            (*compptr).downsampled_width,
            input_data,
            output_data_ptr,
        );
    } else {
        jsimd_h2v2_fancy_upsample_sse2(
            (*cinfo).max_v_samp_factor,
            (*compptr).downsampled_width,
            input_data,
            output_data_ptr,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_h2v1_fancy_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        jsimd_h2v1_fancy_upsample_avx2(
            (*cinfo).max_v_samp_factor,
            (*compptr).downsampled_width,
            input_data,
            output_data_ptr,
        );
    } else {
        jsimd_h2v1_fancy_upsample_sse2(
            (*cinfo).max_v_samp_factor,
            (*compptr).downsampled_width,
            input_data,
            output_data_ptr,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_h2v2_merged_upsample() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JDIMENSION>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if simd_support & JSIMD_AVX2 as c_uint != 0
        && jconst_merged_upsample_avx2.as_ptr() as size_t & ((1i32 << 5i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0
        && jconst_merged_upsample_sse2.as_ptr() as size_t & ((1i32 << 4i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_h2v1_merged_upsample() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JDIMENSION>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if simd_support & JSIMD_AVX2 as c_uint != 0
        && jconst_merged_upsample_avx2.as_ptr() as size_t & ((1i32 << 5i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0
        && jconst_merged_upsample_sse2.as_ptr() as size_t & ((1i32 << 4i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_h2v2_merged_upsample(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut avx2fct: Option<
        unsafe extern "C" fn(_: JDIMENSION, _: JSAMPIMAGE, _: JDIMENSION, _: JSAMPARRAY) -> (),
    > = None;
    let mut sse2fct: Option<
        unsafe extern "C" fn(_: JDIMENSION, _: JSAMPIMAGE, _: JDIMENSION, _: JSAMPARRAY) -> (),
    > = None;
    match (*cinfo).out_color_space as c_uint {
        6 => {
            avx2fct = Some(
                jsimd_h2v2_extrgb_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_h2v2_extrgb_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            )
        }
        7 | 12 => {
            avx2fct = Some(
                jsimd_h2v2_extrgbx_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_h2v2_extrgbx_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            )
        }
        8 => {
            avx2fct = Some(
                jsimd_h2v2_extbgr_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_h2v2_extbgr_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            )
        }
        9 | 13 => {
            avx2fct = Some(
                jsimd_h2v2_extbgrx_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_h2v2_extbgrx_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            )
        }
        10 | 14 => {
            avx2fct = Some(
                jsimd_h2v2_extxbgr_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_h2v2_extxbgr_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            )
        }
        11 | 15 => {
            avx2fct = Some(
                jsimd_h2v2_extxrgb_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_h2v2_extxrgb_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            )
        }
        _ => {
            avx2fct = Some(
                jsimd_h2v2_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_h2v2_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            )
        }
    }
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        avx2fct.expect("non-null function pointer")(
            (*cinfo).output_width,
            input_buf,
            in_row_group_ctr,
            output_buf,
        );
    } else {
        sse2fct.expect("non-null function pointer")(
            (*cinfo).output_width,
            input_buf,
            in_row_group_ctr,
            output_buf,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_h2v1_merged_upsample(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut avx2fct: Option<
        unsafe extern "C" fn(_: JDIMENSION, _: JSAMPIMAGE, _: JDIMENSION, _: JSAMPARRAY) -> (),
    > = None;
    let mut sse2fct: Option<
        unsafe extern "C" fn(_: JDIMENSION, _: JSAMPIMAGE, _: JDIMENSION, _: JSAMPARRAY) -> (),
    > = None;
    match (*cinfo).out_color_space as c_uint {
        6 => {
            avx2fct = Some(
                jsimd_h2v1_extrgb_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_h2v1_extrgb_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            )
        }
        7 | 12 => {
            avx2fct = Some(
                jsimd_h2v1_extrgbx_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_h2v1_extrgbx_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            )
        }
        8 => {
            avx2fct = Some(
                jsimd_h2v1_extbgr_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_h2v1_extbgr_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            )
        }
        9 | 13 => {
            avx2fct = Some(
                jsimd_h2v1_extbgrx_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_h2v1_extbgrx_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            )
        }
        10 | 14 => {
            avx2fct = Some(
                jsimd_h2v1_extxbgr_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_h2v1_extxbgr_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            )
        }
        11 | 15 => {
            avx2fct = Some(
                jsimd_h2v1_extxrgb_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_h2v1_extxrgb_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            )
        }
        _ => {
            avx2fct = Some(
                jsimd_h2v1_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                jsimd_h2v1_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: JDIMENSION,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: JSAMPARRAY,
                    ) -> (),
            )
        }
    }
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        avx2fct.expect("non-null function pointer")(
            (*cinfo).output_width,
            input_buf,
            in_row_group_ctr,
            output_buf,
        );
    } else {
        sse2fct.expect("non-null function pointer")(
            (*cinfo).output_width,
            input_buf,
            in_row_group_ctr,
            output_buf,
        );
    };
}
/*
 * jsimddct.h
 *
 * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
 *
 * Based on the x86 SIMD extension for IJG JPEG library,
 * Copyright (C) 1999-2006, MIYASAKA Masaru.
 * For conditions of distribution and use, see copyright notice in jsimdext.inc
 *
 */
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_convsamp() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if DCTSIZE != 8i32 {
        return 0i32;
    }
    if BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JDIMENSION>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if ::std::mem::size_of::<DCTELEM>() as c_ulong != 2i32 as c_ulong {
        return 0i32;
    }
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        return 1i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0 {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_convsamp_float() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if DCTSIZE != 8i32 {
        return 0i32;
    }
    if BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JDIMENSION>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if ::std::mem::size_of::<c_float>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0 {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_convsamp(
    mut sample_data: JSAMPARRAY,
    mut start_col: JDIMENSION,
    mut workspace: *mut DCTELEM,
) {
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        jsimd_convsamp_avx2(sample_data, start_col, workspace);
    } else {
        jsimd_convsamp_sse2(sample_data, start_col, workspace);
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_convsamp_float(
    mut sample_data: JSAMPARRAY,
    mut start_col: JDIMENSION,
    mut workspace: *mut c_float,
) {
    jsimd_convsamp_float_sse2(sample_data, start_col, workspace);
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_fdct_islow() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<DCTELEM>() as c_ulong != 2i32 as c_ulong {
        return 0i32;
    }
    if simd_support & JSIMD_AVX2 as c_uint != 0
        && jconst_fdct_islow_avx2.as_ptr() as size_t & ((1i32 << 5i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0
        && jconst_fdct_islow_sse2.as_ptr() as size_t & ((1i32 << 4i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_fdct_ifast() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<DCTELEM>() as c_ulong != 2i32 as c_ulong {
        return 0i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0
        && jconst_fdct_ifast_sse2.as_ptr() as size_t & ((1i32 << 4i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_fdct_float() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<c_float>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if simd_support & JSIMD_SSE as c_uint != 0
        && jconst_fdct_float_sse.as_ptr() as size_t & ((1i32 << 4i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_fdct_islow(mut data: *mut DCTELEM) {
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        jsimd_fdct_islow_avx2(data);
    } else {
        jsimd_fdct_islow_sse2(data);
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_fdct_ifast(mut data: *mut DCTELEM) {
    jsimd_fdct_ifast_sse2(data);
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_fdct_float(mut data: *mut c_float) {
    jsimd_fdct_float_sse(data);
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_quantize() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JCOEF>() as c_ulong != 2i32 as c_ulong {
        return 0i32;
    }
    if ::std::mem::size_of::<DCTELEM>() as c_ulong != 2i32 as c_ulong {
        return 0i32;
    }
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        return 1i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0 {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_quantize_float() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JCOEF>() as c_ulong != 2i32 as c_ulong {
        return 0i32;
    }
    if ::std::mem::size_of::<c_float>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0 {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_quantize(
    mut coef_block: JCOEFPTR,
    mut divisors: *mut DCTELEM,
    mut workspace: *mut DCTELEM,
) {
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        jsimd_quantize_avx2(coef_block, divisors, workspace);
    } else {
        jsimd_quantize_sse2(coef_block, divisors, workspace);
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_quantize_float(
    mut coef_block: JCOEFPTR,
    mut divisors: *mut c_float,
    mut workspace: *mut c_float,
) {
    jsimd_quantize_float_sse2(coef_block, divisors, workspace);
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_idct_2x2() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JCOEF>() as c_ulong != 2i32 as c_ulong {
        return 0i32;
    }
    if BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JDIMENSION>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if ::std::mem::size_of::<ISLOW_MULT_TYPE>() as c_ulong != 2i32 as c_ulong {
        return 0i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0
        && jconst_idct_red_sse2.as_ptr() as size_t & ((1i32 << 4i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_idct_4x4() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JCOEF>() as c_ulong != 2i32 as c_ulong {
        return 0i32;
    }
    if BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JDIMENSION>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if ::std::mem::size_of::<ISLOW_MULT_TYPE>() as c_ulong != 2i32 as c_ulong {
        return 0i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0
        && jconst_idct_red_sse2.as_ptr() as size_t & ((1i32 << 4i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_idct_2x2(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    jsimd_idct_2x2_sse2((*compptr).dct_table, coef_block, output_buf, output_col);
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_idct_4x4(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    jsimd_idct_4x4_sse2((*compptr).dct_table, coef_block, output_buf, output_col);
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_idct_islow() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JCOEF>() as c_ulong != 2i32 as c_ulong {
        return 0i32;
    }
    if BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JDIMENSION>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if ::std::mem::size_of::<ISLOW_MULT_TYPE>() as c_ulong != 2i32 as c_ulong {
        return 0i32;
    }
    if simd_support & JSIMD_AVX2 as c_uint != 0
        && jconst_idct_islow_avx2.as_ptr() as size_t & ((1i32 << 5i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0
        && jconst_idct_islow_sse2.as_ptr() as size_t & ((1i32 << 4i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_idct_ifast() -> c_int {
    init_simd();
    /* The code is optimised for these values only */
    if DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JCOEF>() as c_ulong != 2i32 as c_ulong {
        return 0i32;
    }
    if BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JDIMENSION>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if ::std::mem::size_of::<IFAST_MULT_TYPE>() as c_ulong != 2i32 as c_ulong {
        return 0i32;
    }
    if IFAST_SCALE_BITS != 2i32 {
        return 0i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0
        && jconst_idct_ifast_sse2.as_ptr() as size_t & ((1i32 << 4i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_idct_float() -> c_int {
    init_simd();
    if DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JCOEF>() as c_ulong != 2i32 as c_ulong {
        return 0i32;
    }
    if BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JDIMENSION>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if ::std::mem::size_of::<c_float>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if ::std::mem::size_of::<FLOAT_MULT_TYPE>() as c_ulong != 4i32 as c_ulong {
        return 0i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0
        && jconst_idct_float_sse2.as_ptr() as size_t & ((1i32 << 4i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_idct_islow(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    if simd_support & JSIMD_AVX2 as c_uint != 0 {
        jsimd_idct_islow_avx2((*compptr).dct_table, coef_block, output_buf, output_col);
    } else {
        jsimd_idct_islow_sse2((*compptr).dct_table, coef_block, output_buf, output_col);
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_idct_ifast(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    jsimd_idct_ifast_sse2((*compptr).dct_table, coef_block, output_buf, output_col);
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_idct_float(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    jsimd_idct_float_sse2((*compptr).dct_table, coef_block, output_buf, output_col);
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_huff_encode_one_block() -> c_int {
    init_simd();
    if DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JCOEF>() as c_ulong != 2i32 as c_ulong {
        return 0i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0
        && simd_huffman != 0
        && jconst_huff_encode_one_block.as_ptr() as size_t & ((1i32 << 4i32) - 1i32) as c_ulong
            == 0i32 as c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_huff_encode_one_block(
    mut state: *mut c_void,
    mut buffer: *mut JOCTET,
    mut block: JCOEFPTR,
    mut last_dc_val: c_int,
    mut dctbl: *mut c_derived_tbl,
    mut actbl: *mut c_derived_tbl,
) -> *mut JOCTET {
    return jsimd_huff_encode_one_block_sse2(state, buffer, block, last_dc_val, dctbl, actbl);
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_encode_mcu_AC_first_prepare() -> c_int {
    init_simd();
    if DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JCOEF>() as c_ulong != 2i32 as c_ulong {
        return 0i32;
    }
    if SIZEOF_SIZE_T != 8i32 {
        return 0i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0 {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_encode_mcu_AC_first_prepare(
    mut block: *const JCOEF,
    mut jpeg_natural_order_start: *const c_int,
    mut Sl: c_int,
    mut Al: c_int,
    mut values: *mut JCOEF,
    mut zerobits: *mut size_t,
) {
    jsimd_encode_mcu_AC_first_prepare_sse2(
        block,
        jpeg_natural_order_start,
        Sl,
        Al,
        values,
        zerobits,
    );
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_encode_mcu_AC_refine_prepare() -> c_int {
    init_simd();
    if DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<JCOEF>() as c_ulong != 2i32 as c_ulong {
        return 0i32;
    }
    if SIZEOF_SIZE_T != 8i32 {
        return 0i32;
    }
    if simd_support & JSIMD_SSE2 as c_uint != 0 {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_encode_mcu_AC_refine_prepare(
    mut block: *const JCOEF,
    mut jpeg_natural_order_start: *const c_int,
    mut Sl: c_int,
    mut Al: c_int,
    mut absvalues: *mut JCOEF,
    mut bits: *mut size_t,
) -> c_int {
    return jsimd_encode_mcu_AC_refine_prepare_sse2(
        block,
        jpeg_natural_order_start,
        Sl,
        Al,
        absvalues,
        bits,
    );
}
