#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_transmute)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(ptr_wrapping_offset_from)]
#![feature(main)]















































































use std::prelude::v1::*;use crate::stdlib::printf;use libc::{c_void, c_char, c_int, c_ulong};use mozjpeg::*;pub use crate::jpeglib_h::{j_common_ptr, j_compress_ptr, jpeg_CreateCompress,
                           jpeg_c_coef_controller, jpeg_c_main_controller,
                           jpeg_c_prep_controller, jpeg_color_converter,
                           jpeg_common_struct, jpeg_comp_master,
                           jpeg_component_info, jpeg_compress_struct,
                           jpeg_default_colorspace, jpeg_destination_mgr,
                           jpeg_destroy_compress, jpeg_downsampler,
                           jpeg_entropy_encoder, jpeg_error_mgr,
                           jpeg_forward_dct, jpeg_marker_writer,
                           jpeg_memory_mgr, jpeg_progress_mgr, jpeg_scan_info,
                           jpeg_set_defaults, jpeg_std_error,
                           jvirt_barray_control, jvirt_barray_ptr,
                           jvirt_sarray_control, jvirt_sarray_ptr,
                           C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY,
                           JBLOCKROW, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB,
                           JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX,
                           JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX,
                           JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB,
                           JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT,
                           JDCT_IFAST, JDCT_ISLOW, JHUFF_TBL, JQUANT_TBL,
                           JSAMPARRAY, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD};pub use crate::stdlib::{__jmp_buf, __jmp_buf_tag, __sigset_t, _setjmp,
                        jmp_buf, longjmp};pub use crate::jmorecfg_h::{boolean, JCOEF, JDIMENSION, JOCTET, JSAMPLE,
                            UINT16, UINT8};pub use crate::jconfig_h::JPEG_LIB_VERSION;pub use crate::stddef_h::size_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _error_mgr {
    pub pub_0: jpeg_error_mgr,
    pub jb: jmp_buf,
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

static mut lasterror: [c_char; 200] = [
    78, 111, 32, 101, 114, 114, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

unsafe extern "C" fn my_error_exit(mut cinfo: j_common_ptr) {
    let mut myerr: *mut error_mgr = (*cinfo).err as *mut error_mgr;
    Some(
        (*(*cinfo).err)
            .output_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    longjmp((*myerr).jb.as_mut_ptr(), 1i32);
}

unsafe extern "C" fn my_output_message(mut cinfo: j_common_ptr) {
    Some(
        (*(*cinfo).err)
            .format_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, lasterror.as_mut_ptr());
}

unsafe fn main_0() -> c_int {
    let mut jcs_valid: c_int = -1i32;
    let mut jcs_alpha_valid: c_int = -1i32;
    let mut cinfo: jpeg_compress_struct =
        jpeg_compress_struct{err:  ::std::ptr::null_mut::< jpeg_error_mgr>(),
                     mem:  ::std::ptr::null_mut::< jpeg_memory_mgr>(),
                     progress:  ::std::ptr::null_mut::< jpeg_progress_mgr>(),
                     client_data:  ::std::ptr::null_mut::< c_void>(),
                     is_decompressor:  0,
                     global_state:  0,
                     dest:  ::std::ptr::null_mut::< jpeg_destination_mgr>(),
                     image_width:  0,
                     image_height:  0,
                     input_components:  0,
                     in_color_space:  JCS_UNKNOWN,
                     input_gamma:  0.,
                     data_precision:  0,
                     num_components:  0,
                     jpeg_color_space:  JCS_UNKNOWN,
                     comp_info:  ::std::ptr::null_mut::< jpeg_component_info>(),
                     quant_tbl_ptrs:
                          [::std::ptr::null_mut::< JQUANT_TBL>(); 4],
                     dc_huff_tbl_ptrs:
                          [::std::ptr::null_mut::< JHUFF_TBL>(); 4],
                     ac_huff_tbl_ptrs:
                          [::std::ptr::null_mut::< JHUFF_TBL>(); 4],
                     arith_dc_L:  [0; 16],
                     arith_dc_U:  [0; 16],
                     arith_ac_K:  [0; 16],
                     num_scans:  0,
                     scan_info:  ::std::ptr::null::< jpeg_scan_info>(),
                     raw_data_in:  0,
                     arith_code:  0,
                     optimize_coding:  0,
                     CCIR601_sampling:  0,
                     smoothing_factor:  0,
                     dct_method:  JDCT_ISLOW,
                     restart_interval:  0,
                     restart_in_rows:  0,
                     write_JFIF_header:  0,
                     JFIF_major_version:  0,
                     JFIF_minor_version:  0,
                     density_unit:  0,
                     X_density:  0,
                     Y_density:  0,
                     write_Adobe_marker:  0,
                     next_scanline:  0,
                     progressive_mode:  0,
                     max_h_samp_factor:  0,
                     max_v_samp_factor:  0,
                     total_iMCU_rows:  0,
                     comps_in_scan:  0,
                     cur_comp_info:
                          [::std::ptr::null_mut::< jpeg_component_info>(); 4],
                     MCUs_per_row:  0,
                     MCU_rows_in_scan:  0,
                     blocks_in_MCU:  0,
                     MCU_membership:  [0; 10],
                     Ss:  0,
                     Se:  0,
                     Ah:  0,
                     Al:  0,
                     master:  ::std::ptr::null_mut::< jpeg_comp_master>(),
                     main:  ::std::ptr::null_mut::< jpeg_c_main_controller>(),
                     prep:  ::std::ptr::null_mut::< jpeg_c_prep_controller>(),
                     coef:  ::std::ptr::null_mut::< jpeg_c_coef_controller>(),
                     marker:  ::std::ptr::null_mut::< jpeg_marker_writer>(),
                     cconvert:  ::std::ptr::null_mut::< jpeg_color_converter>(),
                     downsample:  ::std::ptr::null_mut::< jpeg_downsampler>(),
                     fdct:  ::std::ptr::null_mut::< jpeg_forward_dct>(),
                     entropy:  ::std::ptr::null_mut::< jpeg_entropy_encoder>(),
                     script_space:  ::std::ptr::null_mut::< jpeg_scan_info>(),
                     script_space_size:  0,};
    let mut jerr: error_mgr = error_mgr {
        pub_0: jpeg_error_mgr{error_exit:  None,
               emit_message:  None,
               output_message:  None,
               format_message:  None,
               reset_error_mgr:  None,
               msg_code:  0,
               msg_parm:  C2RustUnnamed_2{i:  [0; 8],},
               trace_level:  0,
               num_warnings:  0,
               jpeg_message_table:  ::std::ptr::null::< *const c_char>(),
               last_jpeg_message:  0,
               addon_message_table:  ::std::ptr::null::< *const c_char>(),
               first_addon_message:  0,
               last_addon_message:  0,},
        jb: [__jmp_buf_tag{__jmpbuf:  [0; 8],
              __mask_was_saved:  0,
              __saved_mask:  __sigset_t{__val:  [0; 16],},}; 1],
    };
    printf(
        
        b"libjpeg-turbo colorspace extensions:\n\x00".as_ptr() as *const c_char,
    );
    printf(b"  Present at compile time\n\x00".as_ptr() as *const c_char);
    cinfo.err = jpeg_std_error(&mut jerr.pub_0);
    jerr.pub_0.error_exit =
        Some(my_error_exit as unsafe extern "C" fn(_: j_common_ptr) -> ());
    jerr.pub_0.output_message =
        Some(my_output_message as unsafe extern "C" fn(_: j_common_ptr) -> ());
    if _setjmp(jerr.jb.as_mut_ptr()) != 0 {
        /* this will execute if libjpeg has an error */
        jcs_valid = 0i32
    } else {
        jpeg_CreateCompress(
            &mut cinfo,
            JPEG_LIB_VERSION,
            ::std::mem::size_of::<jpeg_compress_struct>() as c_ulong,
        );
        cinfo.input_components = 3i32;
        jpeg_set_defaults(&mut cinfo);
        cinfo.in_color_space = JCS_EXT_RGB;
        jpeg_default_colorspace(&mut cinfo);
        jcs_valid = 1i32
    }
    if jcs_valid != 0 {
        printf(b"  Working properly\n\x00".as_ptr() as *const c_char);
    } else {
        printf(
            
            b"  Not working properly.  Error returned was:\n    %s\n\x00".as_ptr()
                as *const c_char,
            lasterror.as_mut_ptr(),
        );
    }
    printf(
        
        b"libjpeg-turbo alpha colorspace extensions:\n\x00".as_ptr() as *const c_char,
    );
    printf(b"  Present at compile time\n\x00".as_ptr() as *const c_char);
    if _setjmp(jerr.jb.as_mut_ptr()) != 0 {
        /* this will execute if libjpeg has an error */
        jcs_alpha_valid = 0i32
    } else {
        cinfo.in_color_space = JCS_EXT_RGBA;
        jpeg_default_colorspace(&mut cinfo);
        jcs_alpha_valid = 1i32
    }
    if jcs_alpha_valid != 0 {
        printf(b"  Working properly\n\x00".as_ptr() as *const c_char);
    } else {
        printf(
            
            b"  Not working properly.  Error returned was:\n    %s\n\x00".as_ptr()
                as *const c_char,
            lasterror.as_mut_ptr(),
        );
    }
    jpeg_destroy_compress(&mut cinfo);
    return 0i32;
}
#[main]
pub fn main() {
    unsafe { ::std::process::exit(main_0()) }
}
