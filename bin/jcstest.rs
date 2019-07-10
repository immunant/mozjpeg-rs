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
pub use crate::jpeglib_h::C2RustUnnamed_2;
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
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__jmp_buf;
pub use crate::stdlib::__jmp_buf_tag;
pub use crate::stdlib::__sigset_t;
pub use crate::stdlib::_setjmp;
pub use crate::stdlib::jmp_buf;
pub use crate::stdlib::longjmp;
use crate::stdlib::printf;
extern crate libc;
use mozjpeg::*;

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
    (*(*cinfo).err)
        .output_message
        .expect("non-null function pointer")(cinfo);
    crate::stdlib::longjmp((*myerr).jb.as_mut_ptr(), 1i32);
}
unsafe extern "C" fn my_output_message(mut cinfo: crate::jpeglib_h::j_common_ptr) {
    (*(*cinfo).err)
        .format_message
        .expect("non-null function pointer")(cinfo, lasterror.as_mut_ptr());
}
unsafe fn main_0() -> libc::c_int {
    let mut jcs_valid: libc::c_int = -1i32;
    let mut jcs_alpha_valid: libc::c_int = -1i32;
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
            msg_parm: crate::jpeglib_h::C2RustUnnamed_2 { i: [0; 8] },
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
    if 0 != crate::stdlib::_setjmp(jerr.jb.as_mut_ptr()) {
        jcs_valid = 0i32
    } else {
        crate::jpeglib_h::jpeg_CreateCompress(
            &mut cinfo,
            crate::jconfig_h::JPEG_LIB_VERSION,
            ::std::mem::size_of::<crate::jpeglib_h::jpeg_compress_struct>() as libc::c_ulong,
        );
        cinfo.input_components = 3i32;
        crate::jpeglib_h::jpeg_set_defaults(&mut cinfo);
        cinfo.in_color_space = crate::jpeglib_h::JCS_EXT_RGB;
        crate::jpeglib_h::jpeg_default_colorspace(&mut cinfo);
        jcs_valid = 1i32
    }
    if 0 != jcs_valid {
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
    if 0 != crate::stdlib::_setjmp(jerr.jb.as_mut_ptr()) {
        jcs_alpha_valid = 0i32
    } else {
        cinfo.in_color_space = crate::jpeglib_h::JCS_EXT_RGBA;
        crate::jpeglib_h::jpeg_default_colorspace(&mut cinfo);
        jcs_alpha_valid = 1i32
    }
    if 0 != jcs_alpha_valid {
        crate::stdlib::printf(b"  Working properly\n\x00" as *const u8 as *const libc::c_char);
    } else {
        crate::stdlib::printf(
            b"  Not working properly.  Error returned was:\n    %s\n\x00" as *const u8
                as *const libc::c_char,
            lasterror.as_mut_ptr(),
        );
    }
    crate::jpeglib_h::jpeg_destroy_compress(&mut cinfo);
    return 0i32;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
