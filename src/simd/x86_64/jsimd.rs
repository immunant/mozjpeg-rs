extern "C" {
    #[no_mangle]
    pub fn jpeg_simd_cpu_support() -> libc::c_uint;

    #[no_mangle]
    pub static jconst_rgb_ycc_convert_sse2: [libc::c_int; 0];

    #[no_mangle]
    pub fn jsimd_rgb_ycc_convert_sse2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extrgb_ycc_convert_sse2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extrgbx_ycc_convert_sse2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extbgr_ycc_convert_sse2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extbgrx_ycc_convert_sse2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extxbgr_ycc_convert_sse2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extxrgb_ycc_convert_sse2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub static jconst_rgb_ycc_convert_avx2: [libc::c_int; 0];

    #[no_mangle]
    pub fn jsimd_rgb_ycc_convert_avx2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extrgb_ycc_convert_avx2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extrgbx_ycc_convert_avx2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extbgr_ycc_convert_avx2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extbgrx_ycc_convert_avx2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extxbgr_ycc_convert_avx2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extxrgb_ycc_convert_avx2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub static jconst_rgb_gray_convert_sse2: [libc::c_int; 0];

    #[no_mangle]
    pub fn jsimd_rgb_gray_convert_sse2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extrgb_gray_convert_sse2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extrgbx_gray_convert_sse2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extbgr_gray_convert_sse2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extbgrx_gray_convert_sse2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extxbgr_gray_convert_sse2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extxrgb_gray_convert_sse2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub static jconst_rgb_gray_convert_avx2: [libc::c_int; 0];

    #[no_mangle]
    pub fn jsimd_rgb_gray_convert_avx2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extrgb_gray_convert_avx2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extrgbx_gray_convert_avx2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extbgr_gray_convert_avx2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extbgrx_gray_convert_avx2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extxbgr_gray_convert_avx2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_extxrgb_gray_convert_avx2(
        img_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPARRAY,
        output_buf: crate::jpeglib_h::JSAMPIMAGE,
        output_row: crate::jmorecfg_h::JDIMENSION,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub static jconst_ycc_rgb_convert_sse2: [libc::c_int; 0];

    #[no_mangle]
    pub fn jsimd_ycc_rgb_convert_sse2(
        out_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        input_row: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extrgb_convert_sse2(
        out_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        input_row: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extrgbx_convert_sse2(
        out_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        input_row: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extbgr_convert_sse2(
        out_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        input_row: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extbgrx_convert_sse2(
        out_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        input_row: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extxbgr_convert_sse2(
        out_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        input_row: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extxrgb_convert_sse2(
        out_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        input_row: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub static jconst_ycc_rgb_convert_avx2: [libc::c_int; 0];

    #[no_mangle]
    pub fn jsimd_ycc_rgb_convert_avx2(
        out_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        input_row: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extrgb_convert_avx2(
        out_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        input_row: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extrgbx_convert_avx2(
        out_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        input_row: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extbgr_convert_avx2(
        out_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        input_row: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extbgrx_convert_avx2(
        out_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        input_row: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extxbgr_convert_avx2(
        out_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        input_row: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_ycc_extxrgb_convert_avx2(
        out_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        input_row: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
        num_rows: libc::c_int,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_downsample_sse2(
        image_width: crate::jmorecfg_h::JDIMENSION,
        max_v_samp_factor: libc::c_int,
        v_samp_factor: crate::jmorecfg_h::JDIMENSION,
        width_in_blocks: crate::jmorecfg_h::JDIMENSION,
        input_data: crate::jpeglib_h::JSAMPARRAY,
        output_data: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_downsample_avx2(
        image_width: crate::jmorecfg_h::JDIMENSION,
        max_v_samp_factor: libc::c_int,
        v_samp_factor: crate::jmorecfg_h::JDIMENSION,
        width_in_blocks: crate::jmorecfg_h::JDIMENSION,
        input_data: crate::jpeglib_h::JSAMPARRAY,
        output_data: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_downsample_sse2(
        image_width: crate::jmorecfg_h::JDIMENSION,
        max_v_samp_factor: libc::c_int,
        v_samp_factor: crate::jmorecfg_h::JDIMENSION,
        width_in_blocks: crate::jmorecfg_h::JDIMENSION,
        input_data: crate::jpeglib_h::JSAMPARRAY,
        output_data: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_downsample_avx2(
        image_width: crate::jmorecfg_h::JDIMENSION,
        max_v_samp_factor: libc::c_int,
        v_samp_factor: crate::jmorecfg_h::JDIMENSION,
        width_in_blocks: crate::jmorecfg_h::JDIMENSION,
        input_data: crate::jpeglib_h::JSAMPARRAY,
        output_data: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_upsample_sse2(
        max_v_samp_factor: libc::c_int,
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_data: crate::jpeglib_h::JSAMPARRAY,
        output_data_ptr: *mut crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_upsample_sse2(
        max_v_samp_factor: libc::c_int,
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_data: crate::jpeglib_h::JSAMPARRAY,
        output_data_ptr: *mut crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_upsample_avx2(
        max_v_samp_factor: libc::c_int,
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_data: crate::jpeglib_h::JSAMPARRAY,
        output_data_ptr: *mut crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_upsample_avx2(
        max_v_samp_factor: libc::c_int,
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_data: crate::jpeglib_h::JSAMPARRAY,
        output_data_ptr: *mut crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub static jconst_fancy_upsample_sse2: [libc::c_int; 0];

    #[no_mangle]
    pub fn jsimd_h2v1_fancy_upsample_sse2(
        max_v_samp_factor: libc::c_int,
        downsampled_width: crate::jmorecfg_h::JDIMENSION,
        input_data: crate::jpeglib_h::JSAMPARRAY,
        output_data_ptr: *mut crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_fancy_upsample_sse2(
        max_v_samp_factor: libc::c_int,
        downsampled_width: crate::jmorecfg_h::JDIMENSION,
        input_data: crate::jpeglib_h::JSAMPARRAY,
        output_data_ptr: *mut crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub static jconst_fancy_upsample_avx2: [libc::c_int; 0];

    #[no_mangle]
    pub fn jsimd_h2v1_fancy_upsample_avx2(
        max_v_samp_factor: libc::c_int,
        downsampled_width: crate::jmorecfg_h::JDIMENSION,
        input_data: crate::jpeglib_h::JSAMPARRAY,
        output_data_ptr: *mut crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_fancy_upsample_avx2(
        max_v_samp_factor: libc::c_int,
        downsampled_width: crate::jmorecfg_h::JDIMENSION,
        input_data: crate::jpeglib_h::JSAMPARRAY,
        output_data_ptr: *mut crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub static jconst_merged_upsample_sse2: [libc::c_int; 0];

    #[no_mangle]
    pub fn jsimd_h2v1_merged_upsample_sse2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extrgb_merged_upsample_sse2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extrgbx_merged_upsample_sse2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extbgr_merged_upsample_sse2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extbgrx_merged_upsample_sse2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extxbgr_merged_upsample_sse2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extxrgb_merged_upsample_sse2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_merged_upsample_sse2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extrgb_merged_upsample_sse2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extrgbx_merged_upsample_sse2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extbgr_merged_upsample_sse2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extbgrx_merged_upsample_sse2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extxbgr_merged_upsample_sse2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extxrgb_merged_upsample_sse2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub static jconst_merged_upsample_avx2: [libc::c_int; 0];

    #[no_mangle]
    pub fn jsimd_h2v1_merged_upsample_avx2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extrgb_merged_upsample_avx2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extrgbx_merged_upsample_avx2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extbgr_merged_upsample_avx2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extbgrx_merged_upsample_avx2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extxbgr_merged_upsample_avx2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v1_extxrgb_merged_upsample_avx2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_merged_upsample_avx2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extrgb_merged_upsample_avx2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extrgbx_merged_upsample_avx2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extbgr_merged_upsample_avx2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extbgrx_merged_upsample_avx2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extxbgr_merged_upsample_avx2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_h2v2_extxrgb_merged_upsample_avx2(
        output_width: crate::jmorecfg_h::JDIMENSION,
        input_buf: crate::jpeglib_h::JSAMPIMAGE,
        in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
    );

    #[no_mangle]
    pub fn jsimd_convsamp_sse2(
        sample_data: crate::jpeglib_h::JSAMPARRAY,
        start_col: crate::jmorecfg_h::JDIMENSION,
        workspace: *mut crate::jdct_h::DCTELEM,
    );

    #[no_mangle]
    pub fn jsimd_convsamp_avx2(
        sample_data: crate::jpeglib_h::JSAMPARRAY,
        start_col: crate::jmorecfg_h::JDIMENSION,
        workspace: *mut crate::jdct_h::DCTELEM,
    );

    #[no_mangle]
    pub fn jsimd_convsamp_float_sse2(
        sample_data: crate::jpeglib_h::JSAMPARRAY,
        start_col: crate::jmorecfg_h::JDIMENSION,
        workspace: *mut libc::c_float,
    );

    #[no_mangle]
    pub static jconst_fdct_islow_sse2: [libc::c_int; 0];

    #[no_mangle]
    pub fn jsimd_fdct_islow_sse2(data: *mut crate::jdct_h::DCTELEM);

    #[no_mangle]
    pub static jconst_fdct_islow_avx2: [libc::c_int; 0];

    #[no_mangle]
    pub fn jsimd_fdct_islow_avx2(data: *mut crate::jdct_h::DCTELEM);

    #[no_mangle]
    pub static jconst_fdct_ifast_sse2: [libc::c_int; 0];

    #[no_mangle]
    pub fn jsimd_fdct_ifast_sse2(data: *mut crate::jdct_h::DCTELEM);

    #[no_mangle]
    pub static jconst_fdct_float_sse: [libc::c_int; 0];

    #[no_mangle]
    pub fn jsimd_fdct_float_sse(data: *mut libc::c_float);

    #[no_mangle]
    pub fn jsimd_quantize_sse2(
        coef_block: crate::jpeglib_h::JCOEFPTR,
        divisors: *mut crate::jdct_h::DCTELEM,
        workspace: *mut crate::jdct_h::DCTELEM,
    );

    #[no_mangle]
    pub fn jsimd_quantize_avx2(
        coef_block: crate::jpeglib_h::JCOEFPTR,
        divisors: *mut crate::jdct_h::DCTELEM,
        workspace: *mut crate::jdct_h::DCTELEM,
    );

    #[no_mangle]
    pub fn jsimd_quantize_float_sse2(
        coef_block: crate::jpeglib_h::JCOEFPTR,
        divisors: *mut libc::c_float,
        workspace: *mut libc::c_float,
    );

    #[no_mangle]
    pub static jconst_idct_red_sse2: [libc::c_int; 0];

    #[no_mangle]
    pub fn jsimd_idct_2x2_sse2(
        dct_table: *mut libc::c_void,
        coef_block: crate::jpeglib_h::JCOEFPTR,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
        output_col: crate::jmorecfg_h::JDIMENSION,
    );

    #[no_mangle]
    pub fn jsimd_idct_4x4_sse2(
        dct_table: *mut libc::c_void,
        coef_block: crate::jpeglib_h::JCOEFPTR,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
        output_col: crate::jmorecfg_h::JDIMENSION,
    );

    #[no_mangle]
    pub static jconst_idct_islow_sse2: [libc::c_int; 0];

    #[no_mangle]
    pub fn jsimd_idct_islow_sse2(
        dct_table: *mut libc::c_void,
        coef_block: crate::jpeglib_h::JCOEFPTR,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
        output_col: crate::jmorecfg_h::JDIMENSION,
    );

    #[no_mangle]
    pub static jconst_idct_islow_avx2: [libc::c_int; 0];

    #[no_mangle]
    pub fn jsimd_idct_islow_avx2(
        dct_table: *mut libc::c_void,
        coef_block: crate::jpeglib_h::JCOEFPTR,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
        output_col: crate::jmorecfg_h::JDIMENSION,
    );

    #[no_mangle]
    pub static jconst_idct_ifast_sse2: [libc::c_int; 0];

    #[no_mangle]
    pub fn jsimd_idct_ifast_sse2(
        dct_table: *mut libc::c_void,
        coef_block: crate::jpeglib_h::JCOEFPTR,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
        output_col: crate::jmorecfg_h::JDIMENSION,
    );

    #[no_mangle]
    pub static jconst_idct_float_sse2: [libc::c_int; 0];

    #[no_mangle]
    pub fn jsimd_idct_float_sse2(
        dct_table: *mut libc::c_void,
        coef_block: crate::jpeglib_h::JCOEFPTR,
        output_buf: crate::jpeglib_h::JSAMPARRAY,
        output_col: crate::jmorecfg_h::JDIMENSION,
    );
    /* Huffman coding */
    #[no_mangle]
    pub static jconst_huff_encode_one_block: [libc::c_int; 0];

    #[no_mangle]
    pub fn jsimd_huff_encode_one_block_sse2(
        state: *mut libc::c_void,
        buffer: *mut crate::jmorecfg_h::JOCTET,
        block: crate::jpeglib_h::JCOEFPTR,
        last_dc_val: libc::c_int,
        dctbl: *mut crate::src::jchuff::c_derived_tbl,
        actbl: *mut crate::src::jchuff::c_derived_tbl,
    ) -> *mut crate::jmorecfg_h::JOCTET;
    /* Progressive Huffman encoding */
    #[no_mangle]
    pub fn jsimd_encode_mcu_AC_first_prepare_sse2(
        block: *const crate::jmorecfg_h::JCOEF,
        jpeg_natural_order_start: *const libc::c_int,
        Sl: libc::c_int,
        Al: libc::c_int,
        values: *mut crate::jmorecfg_h::JCOEF,
        zerobits: *mut crate::stddef_h::size_t,
    );

    #[no_mangle]
    pub fn jsimd_encode_mcu_AC_refine_prepare_sse2(
        block: *const crate::jmorecfg_h::JCOEF,
        jpeg_natural_order_start: *const libc::c_int,
        Sl: libc::c_int,
        Al: libc::c_int,
        absvalues: *mut crate::jmorecfg_h::JCOEF,
        bits: *mut crate::stddef_h::size_t,
    ) -> libc::c_int;
}
pub use crate::jdct_h::DCTELEM;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JCOEFPTR;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::src::jchuff::c_derived_tbl;
pub use crate::stddef_h::size_t;
/* TARGA_SUPPORTED */
pub use crate::jconfig_h::BITS_IN_JSAMPLE;
pub use crate::jdct_h::FLOAT_MULT_TYPE;
pub use crate::jdct_h::IFAST_MULT_TYPE;
pub use crate::jdct_h::IFAST_SCALE_BITS;
pub use crate::jdct_h::ISLOW_MULT_TYPE;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::RGB_PIXELSIZE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::jpeg_c_coef_controller;
pub use crate::jpeglib_h::jpeg_c_main_controller;
pub use crate::jpeglib_h::jpeg_c_prep_controller;
pub use crate::jpeglib_h::jpeg_color_converter;
pub use crate::jpeglib_h::jpeg_color_deconverter;
pub use crate::jpeglib_h::jpeg_color_quantizer;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_comp_master;
pub use crate::jpeglib_h::jpeg_d_coef_controller;
pub use crate::jpeglib_h::jpeg_d_main_controller;
pub use crate::jpeglib_h::jpeg_d_post_controller;
pub use crate::jpeglib_h::jpeg_decomp_master;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_downsampler;
pub use crate::jpeglib_h::jpeg_entropy_decoder;
pub use crate::jpeglib_h::jpeg_entropy_encoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_forward_dct;
pub use crate::jpeglib_h::jpeg_input_controller;
pub use crate::jpeglib_h::jpeg_inverse_dct;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_reader;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_marker_writer;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jpeg_upsampler;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::DCTSIZE;
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
pub use crate::jpeglib_h::JDITHER_FS;
pub use crate::jpeglib_h::JDITHER_NONE;
pub use crate::jpeglib_h::JDITHER_ORDERED;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::stddef_h::NULL;
use crate::stdlib::getenv;
use crate::stdlib::strcmp;
use libc;

pub use crate::jconfigint_h::SIZEOF_SIZE_T;
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
pub const JSIMD_SSE: libc::c_int = 0x4i32;

pub const JSIMD_SSE2: libc::c_int = 0x8i32;

pub const JSIMD_AVX2: libc::c_int = 0x80i32;
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

static mut simd_support: libc::c_uint = !0i32 as libc::c_uint;

static mut simd_huffman: libc::c_uint = 1i32 as libc::c_uint;
/*
 * Check what SIMD accelerations are supported.
 *
 * FIXME: This code is racy under a multi-threaded environment.
 */

unsafe extern "C" fn init_simd() {
    let mut env: *mut libc::c_char = crate::stddef_h::NULL as *mut libc::c_char;
    if simd_support != !0u32 {
        return;
    }
    simd_support = crate::src::simd::x86_64::jsimd::jpeg_simd_cpu_support();
    /* Force different settings through environment variables */
    env = crate::stdlib::getenv(b"JSIMD_FORCESSE2\x00" as *const u8 as *const libc::c_char);
    if !env.is_null()
        && crate::stdlib::strcmp(env, b"1\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        simd_support &= crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint
    }
    env = crate::stdlib::getenv(b"JSIMD_FORCEAVX2\x00" as *const u8 as *const libc::c_char);
    if !env.is_null()
        && crate::stdlib::strcmp(env, b"1\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        simd_support &= crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint
    }
    env = crate::stdlib::getenv(b"JSIMD_FORCENONE\x00" as *const u8 as *const libc::c_char);
    if !env.is_null()
        && crate::stdlib::strcmp(env, b"1\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        simd_support = 0i32 as libc::c_uint
    }
    env = crate::stdlib::getenv(b"JSIMD_NOHUFFENC\x00" as *const u8 as *const libc::c_char);
    if !env.is_null()
        && crate::stdlib::strcmp(env, b"1\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        simd_huffman = 0i32 as libc::c_uint
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

pub unsafe extern "C" fn jsimd_can_rgb_ycc() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jconfig_h::BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JDIMENSION>() as libc::c_ulong
        != 4i32 as libc::c_ulong
    {
        return 0i32;
    }
    if crate::jmorecfg_h::RGB_PIXELSIZE != 3i32 && crate::jmorecfg_h::RGB_PIXELSIZE != 4i32 {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_rgb_ycc_convert_avx2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 5i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_rgb_ycc_convert_sse2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 4i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_rgb_gray() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jconfig_h::BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JDIMENSION>() as libc::c_ulong
        != 4i32 as libc::c_ulong
    {
        return 0i32;
    }
    if crate::jmorecfg_h::RGB_PIXELSIZE != 3i32 && crate::jmorecfg_h::RGB_PIXELSIZE != 4i32 {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_rgb_gray_convert_avx2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 5i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_rgb_gray_convert_sse2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 4i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_ycc_rgb() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jconfig_h::BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JDIMENSION>() as libc::c_ulong
        != 4i32 as libc::c_ulong
    {
        return 0i32;
    }
    if crate::jmorecfg_h::RGB_PIXELSIZE != 3i32 && crate::jmorecfg_h::RGB_PIXELSIZE != 4i32 {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_ycc_rgb_convert_avx2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 5i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_ycc_rgb_convert_sse2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 4i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_ycc_rgb565() -> libc::c_int {
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_rgb_ycc_convert(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut avx2fct: Option<
        unsafe extern "C" fn(
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
            _: libc::c_int,
        ) -> (),
    > = None;
    let mut sse2fct: Option<
        unsafe extern "C" fn(
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
            _: libc::c_int,
        ) -> (),
    > = None;
    match (*cinfo).in_color_space as libc::c_uint {
        6 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extrgb_ycc_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extrgb_ycc_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            )
        }
        7 | 12 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extrgbx_ycc_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extrgbx_ycc_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            )
        }
        8 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extbgr_ycc_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extbgr_ycc_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            )
        }
        9 | 13 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extbgrx_ycc_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extbgrx_ycc_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            )
        }
        10 | 14 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extxbgr_ycc_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extxbgr_ycc_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            )
        }
        11 | 15 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extxrgb_ycc_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extxrgb_ycc_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            )
        }
        _ => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_rgb_ycc_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_rgb_ycc_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            )
        }
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
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
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut avx2fct: Option<
        unsafe extern "C" fn(
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
            _: libc::c_int,
        ) -> (),
    > = None;
    let mut sse2fct: Option<
        unsafe extern "C" fn(
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
            _: libc::c_int,
        ) -> (),
    > = None;
    match (*cinfo).in_color_space as libc::c_uint {
        6 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extrgb_gray_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extrgb_gray_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            )
        }
        7 | 12 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extrgbx_gray_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extrgbx_gray_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            )
        }
        8 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extbgr_gray_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extbgr_gray_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            )
        }
        9 | 13 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extbgrx_gray_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extbgrx_gray_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            )
        }
        10 | 14 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extxbgr_gray_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extxbgr_gray_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            )
        }
        11 | 15 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extxrgb_gray_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_extxrgb_gray_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            )
        }
        _ => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_rgb_gray_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_rgb_gray_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            )
        }
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
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
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
    let mut avx2fct: Option<
        unsafe extern "C" fn(
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: libc::c_int,
        ) -> (),
    > = None;
    let mut sse2fct: Option<
        unsafe extern "C" fn(
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: libc::c_int,
        ) -> (),
    > = None;
    match (*cinfo).out_color_space as libc::c_uint {
        6 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_ycc_extrgb_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_ycc_extrgb_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: libc::c_int,
                    ) -> (),
            )
        }
        7 | 12 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_ycc_extrgbx_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_ycc_extrgbx_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: libc::c_int,
                    ) -> (),
            )
        }
        8 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_ycc_extbgr_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_ycc_extbgr_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: libc::c_int,
                    ) -> (),
            )
        }
        9 | 13 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_ycc_extbgrx_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_ycc_extbgrx_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: libc::c_int,
                    ) -> (),
            )
        }
        10 | 14 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_ycc_extxbgr_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_ycc_extxbgr_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: libc::c_int,
                    ) -> (),
            )
        }
        11 | 15 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_ycc_extxrgb_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_ycc_extxrgb_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: libc::c_int,
                    ) -> (),
            )
        }
        _ => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_ycc_rgb_convert_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: libc::c_int,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_ycc_rgb_convert_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: libc::c_int,
                    ) -> (),
            )
        }
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
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
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
) {
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_h2v2_downsample() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jconfig_h::BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JDIMENSION>() as libc::c_ulong
        != 4i32 as libc::c_ulong
    {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
        return 1i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0 {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_h2v1_downsample() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jconfig_h::BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JDIMENSION>() as libc::c_ulong
        != 4i32 as libc::c_ulong
    {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
        return 1i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0 {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_h2v2_downsample(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut input_data: crate::jpeglib_h::JSAMPARRAY,
    mut output_data: crate::jpeglib_h::JSAMPARRAY,
) {
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
        crate::src::simd::x86_64::jsimd::jsimd_h2v2_downsample_avx2(
            (*cinfo).image_width,
            (*cinfo).max_v_samp_factor,
            (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
            (*compptr).width_in_blocks,
            input_data,
            output_data,
        );
    } else {
        crate::src::simd::x86_64::jsimd::jsimd_h2v2_downsample_sse2(
            (*cinfo).image_width,
            (*cinfo).max_v_samp_factor,
            (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
            (*compptr).width_in_blocks,
            input_data,
            output_data,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_h2v1_downsample(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut input_data: crate::jpeglib_h::JSAMPARRAY,
    mut output_data: crate::jpeglib_h::JSAMPARRAY,
) {
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
        crate::src::simd::x86_64::jsimd::jsimd_h2v1_downsample_avx2(
            (*cinfo).image_width,
            (*cinfo).max_v_samp_factor,
            (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
            (*compptr).width_in_blocks,
            input_data,
            output_data,
        );
    } else {
        crate::src::simd::x86_64::jsimd::jsimd_h2v1_downsample_sse2(
            (*cinfo).image_width,
            (*cinfo).max_v_samp_factor,
            (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
            (*compptr).width_in_blocks,
            input_data,
            output_data,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_h2v2_upsample() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jconfig_h::BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JDIMENSION>() as libc::c_ulong
        != 4i32 as libc::c_ulong
    {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
        return 1i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0 {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_h2v1_upsample() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jconfig_h::BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JDIMENSION>() as libc::c_ulong
        != 4i32 as libc::c_ulong
    {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
        return 1i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0 {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_h2v2_upsample(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut input_data: crate::jpeglib_h::JSAMPARRAY,
    mut output_data_ptr: *mut crate::jpeglib_h::JSAMPARRAY,
) {
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
        crate::src::simd::x86_64::jsimd::jsimd_h2v2_upsample_avx2(
            (*cinfo).max_v_samp_factor,
            (*cinfo).output_width,
            input_data,
            output_data_ptr,
        );
    } else {
        crate::src::simd::x86_64::jsimd::jsimd_h2v2_upsample_sse2(
            (*cinfo).max_v_samp_factor,
            (*cinfo).output_width,
            input_data,
            output_data_ptr,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_h2v1_upsample(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut input_data: crate::jpeglib_h::JSAMPARRAY,
    mut output_data_ptr: *mut crate::jpeglib_h::JSAMPARRAY,
) {
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
        crate::src::simd::x86_64::jsimd::jsimd_h2v1_upsample_avx2(
            (*cinfo).max_v_samp_factor,
            (*cinfo).output_width,
            input_data,
            output_data_ptr,
        );
    } else {
        crate::src::simd::x86_64::jsimd::jsimd_h2v1_upsample_sse2(
            (*cinfo).max_v_samp_factor,
            (*cinfo).output_width,
            input_data,
            output_data_ptr,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_h2v2_fancy_upsample() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jconfig_h::BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JDIMENSION>() as libc::c_ulong
        != 4i32 as libc::c_ulong
    {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_fancy_upsample_avx2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 5i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_fancy_upsample_sse2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 4i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_h2v1_fancy_upsample() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jconfig_h::BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JDIMENSION>() as libc::c_ulong
        != 4i32 as libc::c_ulong
    {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_fancy_upsample_avx2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 5i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_fancy_upsample_sse2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 4i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_h2v2_fancy_upsample(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut input_data: crate::jpeglib_h::JSAMPARRAY,
    mut output_data_ptr: *mut crate::jpeglib_h::JSAMPARRAY,
) {
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
        crate::src::simd::x86_64::jsimd::jsimd_h2v2_fancy_upsample_avx2(
            (*cinfo).max_v_samp_factor,
            (*compptr).downsampled_width,
            input_data,
            output_data_ptr,
        );
    } else {
        crate::src::simd::x86_64::jsimd::jsimd_h2v2_fancy_upsample_sse2(
            (*cinfo).max_v_samp_factor,
            (*compptr).downsampled_width,
            input_data,
            output_data_ptr,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_h2v1_fancy_upsample(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut input_data: crate::jpeglib_h::JSAMPARRAY,
    mut output_data_ptr: *mut crate::jpeglib_h::JSAMPARRAY,
) {
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
        crate::src::simd::x86_64::jsimd::jsimd_h2v1_fancy_upsample_avx2(
            (*cinfo).max_v_samp_factor,
            (*compptr).downsampled_width,
            input_data,
            output_data_ptr,
        );
    } else {
        crate::src::simd::x86_64::jsimd::jsimd_h2v1_fancy_upsample_sse2(
            (*cinfo).max_v_samp_factor,
            (*compptr).downsampled_width,
            input_data,
            output_data_ptr,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_h2v2_merged_upsample() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jconfig_h::BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JDIMENSION>() as libc::c_ulong
        != 4i32 as libc::c_ulong
    {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_merged_upsample_avx2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 5i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_merged_upsample_sse2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 4i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_h2v1_merged_upsample() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jconfig_h::BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JDIMENSION>() as libc::c_ulong
        != 4i32 as libc::c_ulong
    {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_merged_upsample_avx2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 5i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_merged_upsample_sse2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 4i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_h2v2_merged_upsample(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    let mut avx2fct: Option<
        unsafe extern "C" fn(
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPARRAY,
        ) -> (),
    > = None;
    let mut sse2fct: Option<
        unsafe extern "C" fn(
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPARRAY,
        ) -> (),
    > = None;
    match (*cinfo).out_color_space as libc::c_uint {
        6 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v2_extrgb_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v2_extrgb_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            )
        }
        7 | 12 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v2_extrgbx_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v2_extrgbx_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            )
        }
        8 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v2_extbgr_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v2_extbgr_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            )
        }
        9 | 13 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v2_extbgrx_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v2_extbgrx_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            )
        }
        10 | 14 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v2_extxbgr_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v2_extxbgr_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            )
        }
        11 | 15 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v2_extxrgb_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v2_extxrgb_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            )
        }
        _ => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v2_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v2_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            )
        }
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
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
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    let mut avx2fct: Option<
        unsafe extern "C" fn(
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPARRAY,
        ) -> (),
    > = None;
    let mut sse2fct: Option<
        unsafe extern "C" fn(
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPARRAY,
        ) -> (),
    > = None;
    match (*cinfo).out_color_space as libc::c_uint {
        6 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v1_extrgb_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v1_extrgb_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            )
        }
        7 | 12 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v1_extrgbx_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v1_extrgbx_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            )
        }
        8 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v1_extbgr_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v1_extbgr_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            )
        }
        9 | 13 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v1_extbgrx_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v1_extbgrx_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            )
        }
        10 | 14 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v1_extxbgr_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v1_extxbgr_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            )
        }
        11 | 15 => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v1_extxrgb_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v1_extxrgb_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            )
        }
        _ => {
            avx2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v1_merged_upsample_avx2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            );
            sse2fct = Some(
                crate::src::simd::x86_64::jsimd::jsimd_h2v1_merged_upsample_sse2
                    as unsafe extern "C" fn(
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            )
        }
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
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

pub unsafe extern "C" fn jsimd_can_convsamp() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jpeglib_h::DCTSIZE != 8i32 {
        return 0i32;
    }
    if crate::jconfig_h::BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JDIMENSION>() as libc::c_ulong
        != 4i32 as libc::c_ulong
    {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong != 2i32 as libc::c_ulong {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
        return 1i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0 {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_convsamp_float() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jpeglib_h::DCTSIZE != 8i32 {
        return 0i32;
    }
    if crate::jconfig_h::BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JDIMENSION>() as libc::c_ulong
        != 4i32 as libc::c_ulong
    {
        return 0i32;
    }
    if ::std::mem::size_of::<libc::c_float>() as libc::c_ulong != 4i32 as libc::c_ulong {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0 {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_convsamp(
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
    mut workspace: *mut crate::jdct_h::DCTELEM,
) {
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
        crate::src::simd::x86_64::jsimd::jsimd_convsamp_avx2(sample_data, start_col, workspace);
    } else {
        crate::src::simd::x86_64::jsimd::jsimd_convsamp_sse2(sample_data, start_col, workspace);
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_convsamp_float(
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
    mut workspace: *mut libc::c_float,
) {
    crate::src::simd::x86_64::jsimd::jsimd_convsamp_float_sse2(sample_data, start_col, workspace);
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_fdct_islow() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jpeglib_h::DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong != 2i32 as libc::c_ulong {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_fdct_islow_avx2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 5i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_fdct_islow_sse2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 4i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_fdct_ifast() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jpeglib_h::DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong != 2i32 as libc::c_ulong {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_fdct_ifast_sse2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 4i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_fdct_float() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jpeglib_h::DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<libc::c_float>() as libc::c_ulong != 4i32 as libc::c_ulong {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_fdct_float_sse.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 4i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_fdct_islow(mut data: *mut crate::jdct_h::DCTELEM) {
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
        crate::src::simd::x86_64::jsimd::jsimd_fdct_islow_avx2(data);
    } else {
        crate::src::simd::x86_64::jsimd::jsimd_fdct_islow_sse2(data);
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_fdct_ifast(mut data: *mut crate::jdct_h::DCTELEM) {
    crate::src::simd::x86_64::jsimd::jsimd_fdct_ifast_sse2(data);
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_fdct_float(mut data: *mut libc::c_float) {
    crate::src::simd::x86_64::jsimd::jsimd_fdct_float_sse(data);
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_quantize() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jpeglib_h::DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JCOEF>() as libc::c_ulong != 2i32 as libc::c_ulong {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong != 2i32 as libc::c_ulong {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
        return 1i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0 {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_quantize_float() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jpeglib_h::DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JCOEF>() as libc::c_ulong != 2i32 as libc::c_ulong {
        return 0i32;
    }
    if ::std::mem::size_of::<libc::c_float>() as libc::c_ulong != 4i32 as libc::c_ulong {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0 {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_quantize(
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut divisors: *mut crate::jdct_h::DCTELEM,
    mut workspace: *mut crate::jdct_h::DCTELEM,
) {
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
        crate::src::simd::x86_64::jsimd::jsimd_quantize_avx2(coef_block, divisors, workspace);
    } else {
        crate::src::simd::x86_64::jsimd::jsimd_quantize_sse2(coef_block, divisors, workspace);
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_quantize_float(
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut divisors: *mut libc::c_float,
    mut workspace: *mut libc::c_float,
) {
    crate::src::simd::x86_64::jsimd::jsimd_quantize_float_sse2(coef_block, divisors, workspace);
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_idct_2x2() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jpeglib_h::DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JCOEF>() as libc::c_ulong != 2i32 as libc::c_ulong {
        return 0i32;
    }
    if crate::jconfig_h::BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JDIMENSION>() as libc::c_ulong
        != 4i32 as libc::c_ulong
    {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jdct_h::ISLOW_MULT_TYPE>() as libc::c_ulong
        != 2i32 as libc::c_ulong
    {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_idct_red_sse2.as_ptr() as crate::stddef_h::size_t
            & ((1i32 << 4i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_idct_4x4() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jpeglib_h::DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JCOEF>() as libc::c_ulong != 2i32 as libc::c_ulong {
        return 0i32;
    }
    if crate::jconfig_h::BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JDIMENSION>() as libc::c_ulong
        != 4i32 as libc::c_ulong
    {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jdct_h::ISLOW_MULT_TYPE>() as libc::c_ulong
        != 2i32 as libc::c_ulong
    {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_idct_red_sse2.as_ptr() as crate::stddef_h::size_t
            & ((1i32 << 4i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_idct_2x2(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    crate::src::simd::x86_64::jsimd::jsimd_idct_2x2_sse2(
        (*compptr).dct_table,
        coef_block,
        output_buf,
        output_col,
    );
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_idct_4x4(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    crate::src::simd::x86_64::jsimd::jsimd_idct_4x4_sse2(
        (*compptr).dct_table,
        coef_block,
        output_buf,
        output_col,
    );
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_idct_islow() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jpeglib_h::DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JCOEF>() as libc::c_ulong != 2i32 as libc::c_ulong {
        return 0i32;
    }
    if crate::jconfig_h::BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JDIMENSION>() as libc::c_ulong
        != 4i32 as libc::c_ulong
    {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jdct_h::ISLOW_MULT_TYPE>() as libc::c_ulong
        != 2i32 as libc::c_ulong
    {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_idct_islow_avx2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 5i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_idct_islow_sse2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 4i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_idct_ifast() -> libc::c_int {
    init_simd();
    /* The code is optimised for these values only */
    if crate::jpeglib_h::DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JCOEF>() as libc::c_ulong != 2i32 as libc::c_ulong {
        return 0i32;
    }
    if crate::jconfig_h::BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JDIMENSION>() as libc::c_ulong
        != 4i32 as libc::c_ulong
    {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jdct_h::IFAST_MULT_TYPE>() as libc::c_ulong
        != 2i32 as libc::c_ulong
    {
        return 0i32;
    }
    if crate::jdct_h::IFAST_SCALE_BITS != 2i32 {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_idct_ifast_sse2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 4i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_idct_float() -> libc::c_int {
    init_simd();
    if crate::jpeglib_h::DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JCOEF>() as libc::c_ulong != 2i32 as libc::c_ulong {
        return 0i32;
    }
    if crate::jconfig_h::BITS_IN_JSAMPLE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JDIMENSION>() as libc::c_ulong
        != 4i32 as libc::c_ulong
    {
        return 0i32;
    }
    if ::std::mem::size_of::<libc::c_float>() as libc::c_ulong != 4i32 as libc::c_ulong {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jdct_h::FLOAT_MULT_TYPE>() as libc::c_ulong
        != 4i32 as libc::c_ulong
    {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0
        && crate::src::simd::x86_64::jsimd::jconst_idct_float_sse2.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 4i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_idct_islow(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_AVX2 as libc::c_uint != 0 {
        crate::src::simd::x86_64::jsimd::jsimd_idct_islow_avx2(
            (*compptr).dct_table,
            coef_block,
            output_buf,
            output_col,
        );
    } else {
        crate::src::simd::x86_64::jsimd::jsimd_idct_islow_sse2(
            (*compptr).dct_table,
            coef_block,
            output_buf,
            output_col,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_idct_ifast(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    crate::src::simd::x86_64::jsimd::jsimd_idct_ifast_sse2(
        (*compptr).dct_table,
        coef_block,
        output_buf,
        output_col,
    );
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_idct_float(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    crate::src::simd::x86_64::jsimd::jsimd_idct_float_sse2(
        (*compptr).dct_table,
        coef_block,
        output_buf,
        output_col,
    );
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_huff_encode_one_block() -> libc::c_int {
    init_simd();
    if crate::jpeglib_h::DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JCOEF>() as libc::c_ulong != 2i32 as libc::c_ulong {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0
        && simd_huffman != 0
        && crate::src::simd::x86_64::jsimd::jconst_huff_encode_one_block.as_ptr()
            as crate::stddef_h::size_t
            & ((1i32 << 4i32) - 1i32) as libc::c_ulong
            == 0i32 as libc::c_ulong
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_huff_encode_one_block(
    mut state: *mut libc::c_void,
    mut buffer: *mut crate::jmorecfg_h::JOCTET,
    mut block: crate::jpeglib_h::JCOEFPTR,
    mut last_dc_val: libc::c_int,
    mut dctbl: *mut crate::src::jchuff::c_derived_tbl,
    mut actbl: *mut crate::src::jchuff::c_derived_tbl,
) -> *mut crate::jmorecfg_h::JOCTET {
    return crate::src::simd::x86_64::jsimd::jsimd_huff_encode_one_block_sse2(
        state,
        buffer,
        block,
        last_dc_val,
        dctbl,
        actbl,
    );
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_encode_mcu_AC_first_prepare() -> libc::c_int {
    init_simd();
    if crate::jpeglib_h::DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JCOEF>() as libc::c_ulong != 2i32 as libc::c_ulong {
        return 0i32;
    }
    if crate::jconfigint_h::SIZEOF_SIZE_T != 8i32 {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0 {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_encode_mcu_AC_first_prepare(
    mut block: *const crate::jmorecfg_h::JCOEF,
    mut jpeg_natural_order_start: *const libc::c_int,
    mut Sl: libc::c_int,
    mut Al: libc::c_int,
    mut values: *mut crate::jmorecfg_h::JCOEF,
    mut zerobits: *mut crate::stddef_h::size_t,
) {
    crate::src::simd::x86_64::jsimd::jsimd_encode_mcu_AC_first_prepare_sse2(
        block,
        jpeg_natural_order_start,
        Sl,
        Al,
        values,
        zerobits,
    );
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_can_encode_mcu_AC_refine_prepare() -> libc::c_int {
    init_simd();
    if crate::jpeglib_h::DCTSIZE != 8i32 {
        return 0i32;
    }
    if ::std::mem::size_of::<crate::jmorecfg_h::JCOEF>() as libc::c_ulong != 2i32 as libc::c_ulong {
        return 0i32;
    }
    if crate::jconfigint_h::SIZEOF_SIZE_T != 8i32 {
        return 0i32;
    }
    if simd_support & crate::src::simd::x86_64::jsimd::JSIMD_SSE2 as libc::c_uint != 0 {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn jsimd_encode_mcu_AC_refine_prepare(
    mut block: *const crate::jmorecfg_h::JCOEF,
    mut jpeg_natural_order_start: *const libc::c_int,
    mut Sl: libc::c_int,
    mut Al: libc::c_int,
    mut absvalues: *mut crate::jmorecfg_h::JCOEF,
    mut bits: *mut crate::stddef_h::size_t,
) -> libc::c_int {
    return crate::src::simd::x86_64::jsimd::jsimd_encode_mcu_AC_refine_prepare_sse2(
        block,
        jpeg_natural_order_start,
        Sl,
        Al,
        absvalues,
        bits,
    );
}
