pub use crate::jerror::C2RustUnnamed_4;
pub use crate::jpeglib_h::C2RustUnnamed_3;
use libc::c_char;
use libc::c_int;
use libc::c_long;
use libc::c_uint;
use libc::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d_derived_tbl {
    pub maxcode: [JLONG; 18],
    pub valoffset: [JLONG; 18],
    pub pub_0: *mut JHUFF_TBL,
    pub lookup: [c_int; 256],
}
/* If long is > 32 bits on your machine, and shifting/masking longs is
 * reasonably fast, making bit_buf_type be long and setting BIT_BUF_SIZE
 * appropriately should be a win.  Unfortunately we can't define the size
 * with something like  #define BIT_BUF_SIZE (sizeof(bit_buf_type)*8)
 * because not all machines measure sizeof in 8-bit bytes.
 */
/* Bitreading state saved across MCUs */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bitread_perm_state {
    pub get_buffer: bit_buf_type,
    pub bits_left: c_int,
}
/*
 * Fetching the next N bits from the input stream is a time-critical operation
 * for the Huffman decoders.  We implement it with a combination of inline
 * macros and out-of-line subroutines.  Note that N (the number of bits
 * demanded at one time) never exceeds 15 for JPEG use.
 *
 * We read source bytes into get_buffer and dole out bits as needed.
 * If get_buffer already contains enough bits, they are fetched in-line
 * by the macros CHECK_BIT_BUFFER and GET_BITS.  When there aren't enough
 * bits, jpeg_fill_bit_buffer is called; it will attempt to fill get_buffer
 * as full as possible (not just to the number of bits needed; this
 * prefetching reduces the overhead cost of calling jpeg_fill_bit_buffer).
 * Note that jpeg_fill_bit_buffer may return FALSE to indicate suspension.
 * On TRUE return, jpeg_fill_bit_buffer guarantees that get_buffer contains
 * at least the requested number of bits --- dummy zeroes are inserted if
 * necessary.
 */
/* type of bit-extraction buffer */
pub type bit_buf_type = size_t;
/* Bitreading working state within an MCU */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bitread_working_state {
    pub next_input_byte: *const JOCTET,
    pub bytes_in_buffer: size_t,
    pub get_buffer: bit_buf_type,
    pub bits_left: c_int,
    pub cinfo: j_decompress_ptr,
}
pub use crate::jpegint_h::JLONG;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::JHUFF_TBL;
use libc;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jpeg_c_coef_controller;
pub use crate::jpegint_h::jpeg_c_main_controller;
pub use crate::jpegint_h::jpeg_c_prep_controller;
pub use crate::jpegint_h::jpeg_color_converter;
pub use crate::jpegint_h::jpeg_color_deconverter;
pub use crate::jpegint_h::jpeg_color_quantizer;
pub use crate::jpegint_h::jpeg_comp_master;
pub use crate::jpegint_h::jpeg_d_coef_controller;
pub use crate::jpegint_h::jpeg_d_main_controller;
pub use crate::jpegint_h::jpeg_d_post_controller;
pub use crate::jpegint_h::jpeg_decomp_master;
pub use crate::jpegint_h::jpeg_downsampler;
pub use crate::jpegint_h::jpeg_entropy_decoder;
pub use crate::jpegint_h::jpeg_entropy_encoder;
pub use crate::jpegint_h::jpeg_forward_dct;
pub use crate::jpegint_h::jpeg_input_controller;
pub use crate::jpegint_h::jpeg_inverse_dct;
pub use crate::jpegint_h::jpeg_marker_reader;
pub use crate::jpegint_h::jpeg_marker_writer;
pub use crate::jpegint_h::jpeg_upsampler;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JCOEFPTR;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::stddef_h::size_t;
pub type huff_entropy_ptr = *mut huff_entropy_decoder;
/* This macro is to work around compilers with missing or broken
 * structure assignment.  You'll need to fix this code if you have
 * such a compiler and you change MAX_COMPS_IN_SCAN.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct huff_entropy_decoder {
    pub pub_0: jpeg_entropy_decoder,
    pub bitstate: bitread_perm_state,
    pub saved: savable_state,
    pub restarts_to_go: c_uint,
    pub dc_derived_tbls: [*mut d_derived_tbl; 4],
    pub ac_derived_tbls: [*mut d_derived_tbl; 4],
    pub dc_cur_tbls: [*mut d_derived_tbl; 10],
    pub ac_cur_tbls: [*mut d_derived_tbl; 10],
    pub dc_needed: [boolean; 10],
    pub ac_needed: [boolean; 10],
}
/*
 * jdhuff.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009-2011, 2016, 2018, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains Huffman entropy decoding routines.
 *
 * Much of the complexity here has to do with supporting input suspension.
 * If the data source module demands suspension, we want to be able to back
 * up to the start of the current MCU.  To do this, we copy state variables
 * into local working storage, and update them back to the permanent
 * storage only upon successful completion of an MCU.
 *
 * NOTE: All referenced figures are from
 * Recommendation ITU-T T.81 (1992) | ISO/IEC 10918-1:1994.
 */
/*
 * Expanded entropy decoder object for Huffman decoding.
 *
 * The savable_state subrecord contains fields that change within an MCU,
 * but must not be updated permanently until we complete the MCU.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct savable_state {
    pub last_dc_val: [c_int; 4],
}
/*
 * jdhuff.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2010-2011, 2015-2016, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains declarations for Huffman entropy decoding routines
 * that are shared between the sequential decoder (jdhuff.c) and the
 * progressive decoder (jdphuff.c).  No other modules need to see these.
 */
/* Derived data constructed for each Huffman table */
/* # of bits of lookahead */
pub const HUFF_LOOKAHEAD: c_int = 8i32;
/* size of buffer in bits */
pub const BIT_BUF_SIZE: c_int = 64i32;
/*
 * Compute the derived values for a Huffman table.
 * This routine also performs some validation checks on the table.
 *
 * Note this is also used by jdphuff.c.
 */
/* Expand a Huffman table definition into the derived format */
#[no_mangle]
pub unsafe extern "C" fn jpeg_make_d_derived_tbl(
    mut cinfo: j_decompress_ptr,
    mut isDC: boolean,
    mut tblno: c_int,
    mut pdtbl: *mut *mut d_derived_tbl,
) {
    let mut htbl: *mut JHUFF_TBL = 0 as *mut JHUFF_TBL;
    let mut dtbl: *mut d_derived_tbl = 0 as *mut d_derived_tbl;
    let mut p: c_int = 0;
    let mut i: c_int = 0;
    let mut l: c_int = 0;
    let mut si: c_int = 0;
    let mut numsymbols: c_int = 0;
    let mut lookbits: c_int = 0;
    let mut ctr: c_int = 0;
    let mut huffsize: [c_char; 257] = [0; 257];
    let mut huffcode: [c_uint; 257] = [0; 257];
    let mut code: c_uint = 0;
    if tblno < 0i32 || tblno >= NUM_HUFF_TBLS {
        (*(*cinfo).err).msg_code = JERR_NO_HUFF_TABLE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = tblno;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    htbl = if 0 != isDC {
        (*cinfo).dc_huff_tbl_ptrs[tblno as usize]
    } else {
        (*cinfo).ac_huff_tbl_ptrs[tblno as usize]
    };
    if htbl.is_null() {
        (*(*cinfo).err).msg_code = JERR_NO_HUFF_TABLE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = tblno;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*pdtbl).is_null() {
        *pdtbl = (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            ::std::mem::size_of::<d_derived_tbl>() as c_ulong,
        ) as *mut d_derived_tbl
    }
    dtbl = *pdtbl;
    (*dtbl).pub_0 = htbl;
    p = 0i32;
    l = 1i32;
    while l <= 16i32 {
        i = (*htbl).bits[l as usize] as c_int;
        if i < 0i32 || p + i > 256i32 {
            (*(*cinfo).err).msg_code = JERR_BAD_HUFF_TABLE as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        loop {
            let fresh0 = i;
            i = i - 1;
            if !(0 != fresh0) {
                break;
            }
            let fresh1 = p;
            p = p + 1;
            huffsize[fresh1 as usize] = l as c_char
        }
        l += 1
    }
    huffsize[p as usize] = 0i32 as c_char;
    numsymbols = p;
    code = 0i32 as c_uint;
    si = huffsize[0usize] as c_int;
    p = 0i32;
    while 0 != huffsize[p as usize] {
        while huffsize[p as usize] as c_int == si {
            let fresh2 = p;
            p = p + 1;
            huffcode[fresh2 as usize] = code;
            code = code.wrapping_add(1)
        }
        if code as JLONG >= (1i32 as JLONG) << si {
            (*(*cinfo).err).msg_code = JERR_BAD_HUFF_TABLE as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        code <<= 1i32;
        si += 1
    }
    p = 0i32;
    l = 1i32;
    while l <= 16i32 {
        if 0 != (*htbl).bits[l as usize] {
            (*dtbl).valoffset[l as usize] = p as JLONG - huffcode[p as usize] as JLONG;
            p += (*htbl).bits[l as usize] as c_int;
            (*dtbl).maxcode[l as usize] = huffcode[(p - 1i32) as usize] as JLONG
        } else {
            (*dtbl).maxcode[l as usize] = -1i32 as JLONG
        }
        l += 1
    }
    (*dtbl).valoffset[17usize] = 0i32 as JLONG;
    (*dtbl).maxcode[17usize] = 0xfffffi64;
    i = 0i32;
    while i < 1i32 << HUFF_LOOKAHEAD {
        (*dtbl).lookup[i as usize] = HUFF_LOOKAHEAD + 1i32 << HUFF_LOOKAHEAD;
        i += 1
    }
    p = 0i32;
    l = 1i32;
    while l <= HUFF_LOOKAHEAD {
        i = 1i32;
        while i <= (*htbl).bits[l as usize] as c_int {
            lookbits = (huffcode[p as usize] << HUFF_LOOKAHEAD - l) as c_int;
            ctr = 1i32 << HUFF_LOOKAHEAD - l;
            while ctr > 0i32 {
                (*dtbl).lookup[lookbits as usize] =
                    l << HUFF_LOOKAHEAD | (*htbl).huffval[p as usize] as c_int;
                lookbits += 1;
                ctr -= 1
            }
            i += 1;
            p += 1
        }
        l += 1
    }
    if 0 != isDC {
        i = 0i32;
        while i < numsymbols {
            let mut sym: c_int = (*htbl).huffval[i as usize] as c_int;
            if sym < 0i32 || sym > 15i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_HUFF_TABLE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            i += 1
        }
    };
}
/* Macros to declare and load/save bitread local variables. */
/*
 * These macros provide the in-line portion of bit fetching.
 * Use CHECK_BIT_BUFFER to ensure there are N bits in get_buffer
 * before using GET_BITS, PEEK_BITS, or DROP_BITS.
 * The variables get_buffer and bits_left are assumed to be locals,
 * but the state struct might not be (jpeg_huff_decode needs this).
 *      CHECK_BIT_BUFFER(state, n, action);
 *              Ensure there are N bits in get_buffer; if suspend, take action.
 *      val = GET_BITS(n);
 *              Fetch next N bits.
 *      val = PEEK_BITS(n);
 *              Fetch next N bits without removing them from the buffer.
 *      DROP_BITS(n);
 *              Discard next N bits.
 * The value N should be a simple variable, not an expression, because it
 * is evaluated multiple times.
 */
/* Load up the bit buffer to a depth of at least nbits */
#[no_mangle]
pub unsafe extern "C" fn jpeg_fill_bit_buffer(
    mut state: *mut bitread_working_state,
    mut get_buffer: bit_buf_type,
    mut bits_left: c_int,
    mut nbits: c_int,
) -> boolean {
    /* Copy heavily used state fields into locals (hopefully registers) */
    let mut next_input_byte: *const JOCTET = (*state).next_input_byte;
    let mut bytes_in_buffer: size_t = (*state).bytes_in_buffer;
    let mut cinfo: j_decompress_ptr = (*state).cinfo;
    let mut current_block_30: u64;
    if (*cinfo).unread_marker == 0i32 {
        /* cannot advance past a marker */
        loop {
            if !(bits_left < MIN_GET_BITS) {
                current_block_30 = 6417057564578538666;
                break;
            }
            let mut c: c_int = 0;
            if bytes_in_buffer == 0i32 as c_ulong {
                if 0 == (*(*cinfo).src)
                    .fill_input_buffer
                    .expect("non-null function pointer")(cinfo)
                {
                    return FALSE;
                }
                next_input_byte = (*(*cinfo).src).next_input_byte;
                bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer
            }
            bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
            let fresh3 = next_input_byte;
            next_input_byte = next_input_byte.offset(1);
            c = *fresh3 as c_int;
            /* If it's 0xFF, check and discard stuffed zero byte */
            if c == 0xffi32 {
                loop {
                    if bytes_in_buffer == 0i32 as c_ulong {
                        if 0 == (*(*cinfo).src)
                            .fill_input_buffer
                            .expect("non-null function pointer")(
                            cinfo
                        ) {
                            return FALSE;
                        }
                        next_input_byte = (*(*cinfo).src).next_input_byte;
                        bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer
                    }
                    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
                    let fresh4 = next_input_byte;
                    next_input_byte = next_input_byte.offset(1);
                    c = *fresh4 as c_int;
                    if !(c == 0xffi32) {
                        break;
                    }
                }
                if c == 0i32 {
                    c = 0xffi32
                } else {
                    (*cinfo).unread_marker = c;
                    /* See if we need to insert some fake zero bits. */
                    current_block_30 = 7022714159392939963;
                    break;
                }
            }
            get_buffer = get_buffer << 8i32 | c as c_ulong;
            bits_left += 8i32
        }
    } else {
        /* end while */
        current_block_30 = 7022714159392939963;
    }
    match current_block_30 {
        7022714159392939963 => {
            if nbits > bits_left {
                if 0 == (*(*cinfo).entropy).insufficient_data {
                    (*(*cinfo).err).msg_code = JWRN_HIT_MARKER as c_int;
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer")(
                        cinfo as j_common_ptr, -1i32
                    );
                    (*(*cinfo).entropy).insufficient_data = TRUE
                }
                get_buffer <<= MIN_GET_BITS - bits_left;
                bits_left = MIN_GET_BITS
            }
        }
        _ => {}
    }
    (*state).next_input_byte = next_input_byte;
    (*state).bytes_in_buffer = bytes_in_buffer;
    (*state).get_buffer = get_buffer;
    (*state).bits_left = bits_left;
    return TRUE;
}
/*
 * Code for extracting next Huffman-coded symbol from input bit stream.
 * Again, this is time-critical and we make the main paths be macros.
 *
 * We use a lookahead table to process codes of up to HUFF_LOOKAHEAD bits
 * without looping.  Usually, more than 95% of the Huffman codes will be 8
 * or fewer bits long.  The few overlength codes are handled with a loop,
 * which need not be inline code.
 *
 * Notes about the HUFF_DECODE macro:
 * 1. Near the end of the data segment, we may fail to get enough bits
 *    for a lookahead.  In that case, we do it the hard way.
 * 2. If the lookahead table contains no entry, the next code must be
 *    more than HUFF_LOOKAHEAD bits long.
 * 3. jpeg_huff_decode returns -1 if forced to suspend.
 */
/* Pre-execute the common case of nb <= HUFF_LOOKAHEAD */
/* Equivalent of jpeg_huff_decode() */
/* Don't use GET_BITS() here because we don't want to modify bits_left */
/* Out-of-line case for Huffman code fetching */
/* Macro version of the above, which performs much better but does not
handle markers.  We have to hand off any blocks with markers to the
slower routines. */
/* Pre-execute most common case */
/* Pre-execute case of FF/00, which represents an FF data byte */
/* Oops, it's actually a marker indicating end of compressed data. */
/* Back out pre-execution and fill the buffer with zero bits */
/* Pre-fetch 48 bytes, because the holding register is 64-bit */
/*
 * Out-of-line code for Huffman code decoding.
 * See jdhuff.h for info about usage.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_huff_decode(
    mut state: *mut bitread_working_state,
    mut get_buffer: bit_buf_type,
    mut bits_left: c_int,
    mut htbl: *mut d_derived_tbl,
    mut min_bits: c_int,
) -> c_int {
    let mut l: c_int = min_bits;
    let mut code: JLONG = 0;
    if bits_left < l {
        if 0 == jpeg_fill_bit_buffer(state, get_buffer, bits_left, l) {
            return -1i32;
        }
        get_buffer = (*state).get_buffer;
        bits_left = (*state).bits_left
    }
    bits_left -= l;
    code = ((get_buffer >> bits_left) as c_int & (1i32 << l) - 1i32) as JLONG;
    while code > (*htbl).maxcode[l as usize] {
        code <<= 1i32;
        if bits_left < 1i32 {
            if 0 == jpeg_fill_bit_buffer(state, get_buffer, bits_left, 1i32) {
                return -1i32;
            }
            get_buffer = (*state).get_buffer;
            bits_left = (*state).bits_left
        }
        bits_left -= 1i32;
        code |= ((get_buffer >> bits_left) as c_int & (1i32 << 1i32) - 1i32) as c_long;
        l += 1
    }
    (*state).get_buffer = get_buffer;
    (*state).bits_left = bits_left;
    if l > 16i32 {
        (*(*(*state).cinfo).err).msg_code = JWRN_HUFF_BAD_CODE as c_int;
        (*(*(*state).cinfo).err)
            .emit_message
            .expect("non-null function pointer")((*state).cinfo as j_common_ptr, -1i32);
        return 0i32;
    }
    return (*(*htbl).pub_0).huffval[(code + (*htbl).valoffset[l as usize]) as c_int as usize]
        as c_int;
}
pub use crate::jerror::JERR_ARITH_NOTIMPL;
pub use crate::jerror::JERR_BAD_ALIGN_TYPE;
pub use crate::jerror::JERR_BAD_ALLOC_CHUNK;
pub use crate::jerror::JERR_BAD_BUFFER_MODE;
pub use crate::jerror::JERR_BAD_COMPONENT_ID;
pub use crate::jerror::JERR_BAD_CROP_SPEC;
pub use crate::jerror::JERR_BAD_DCTSIZE;
pub use crate::jerror::JERR_BAD_DCT_COEF;
pub use crate::jerror::JERR_BAD_HUFF_TABLE;
pub use crate::jerror::JERR_BAD_IN_COLORSPACE;
pub use crate::jerror::JERR_BAD_J_COLORSPACE;
pub use crate::jerror::JERR_BAD_LENGTH;
pub use crate::jerror::JERR_BAD_LIB_VERSION;
pub use crate::jerror::JERR_BAD_MCU_SIZE;
pub use crate::jerror::JERR_BAD_PARAM;
pub use crate::jerror::JERR_BAD_PARAM_VALUE;
pub use crate::jerror::JERR_BAD_POOL_ID;
pub use crate::jerror::JERR_BAD_PRECISION;
pub use crate::jerror::JERR_BAD_PROGRESSION;
pub use crate::jerror::JERR_BAD_PROG_SCRIPT;
pub use crate::jerror::JERR_BAD_SAMPLING;
pub use crate::jerror::JERR_BAD_SCAN_SCRIPT;
pub use crate::jerror::JERR_BAD_STATE;
pub use crate::jerror::JERR_BAD_STRUCT_SIZE;
pub use crate::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use crate::jerror::JERR_BUFFER_SIZE;
pub use crate::jerror::JERR_CANT_SUSPEND;
pub use crate::jerror::JERR_CCIR601_NOTIMPL;
pub use crate::jerror::JERR_COMPONENT_COUNT;
pub use crate::jerror::JERR_CONVERSION_NOTIMPL;
pub use crate::jerror::JERR_DAC_INDEX;
pub use crate::jerror::JERR_DAC_VALUE;
pub use crate::jerror::JERR_DHT_INDEX;
pub use crate::jerror::JERR_DQT_INDEX;
pub use crate::jerror::JERR_EMPTY_IMAGE;
pub use crate::jerror::JERR_EMS_READ;
pub use crate::jerror::JERR_EMS_WRITE;
pub use crate::jerror::JERR_EOI_EXPECTED;
pub use crate::jerror::JERR_FILE_READ;
pub use crate::jerror::JERR_FILE_WRITE;
pub use crate::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use crate::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use crate::jerror::JERR_HUFF_MISSING_CODE;
pub use crate::jerror::JERR_IMAGE_TOO_BIG;
pub use crate::jerror::JERR_INPUT_EMPTY;
pub use crate::jerror::JERR_INPUT_EOF;
pub use crate::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use crate::jerror::JERR_MISSING_DATA;
pub use crate::jerror::JERR_MODE_CHANGE;
pub use crate::jerror::JERR_NOTIMPL;
pub use crate::jerror::JERR_NOT_COMPILED;
pub use crate::jerror::JERR_NO_BACKING_STORE;
pub use crate::jerror::JERR_NO_HUFF_TABLE;
pub use crate::jerror::JERR_NO_IMAGE;
pub use crate::jerror::JERR_NO_QUANT_TABLE;
pub use crate::jerror::JERR_NO_SOI;
pub use crate::jerror::JERR_OUT_OF_MEMORY;
pub use crate::jerror::JERR_QUANT_COMPONENTS;
pub use crate::jerror::JERR_QUANT_FEW_COLORS;
pub use crate::jerror::JERR_QUANT_MANY_COLORS;
pub use crate::jerror::JERR_SOF_DUPLICATE;
pub use crate::jerror::JERR_SOF_NO_SOS;
pub use crate::jerror::JERR_SOF_UNSUPPORTED;
pub use crate::jerror::JERR_SOI_DUPLICATE;
pub use crate::jerror::JERR_SOS_NO_SOF;
pub use crate::jerror::JERR_TFILE_CREATE;
pub use crate::jerror::JERR_TFILE_READ;
pub use crate::jerror::JERR_TFILE_SEEK;
pub use crate::jerror::JERR_TFILE_WRITE;
pub use crate::jerror::JERR_TOO_LITTLE_DATA;
pub use crate::jerror::JERR_UNKNOWN_MARKER;
pub use crate::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use crate::jerror::JERR_VIRTUAL_BUG;
pub use crate::jerror::JERR_WIDTH_OVERFLOW;
pub use crate::jerror::JERR_XMS_READ;
pub use crate::jerror::JERR_XMS_WRITE;
pub use crate::jerror::JMSG_COPYRIGHT;
pub use crate::jerror::JMSG_LASTMSGCODE;
pub use crate::jerror::JMSG_NOMESSAGE;
pub use crate::jerror::JMSG_VERSION;
pub use crate::jerror::JTRC_16BIT_TABLES;
pub use crate::jerror::JTRC_ADOBE;
pub use crate::jerror::JTRC_APP0;
pub use crate::jerror::JTRC_APP14;
pub use crate::jerror::JTRC_DAC;
pub use crate::jerror::JTRC_DHT;
pub use crate::jerror::JTRC_DQT;
pub use crate::jerror::JTRC_DRI;
pub use crate::jerror::JTRC_EMS_CLOSE;
pub use crate::jerror::JTRC_EMS_OPEN;
pub use crate::jerror::JTRC_EOI;
pub use crate::jerror::JTRC_HUFFBITS;
pub use crate::jerror::JTRC_JFIF;
pub use crate::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use crate::jerror::JTRC_JFIF_EXTENSION;
pub use crate::jerror::JTRC_JFIF_THUMBNAIL;
pub use crate::jerror::JTRC_MISC_MARKER;
pub use crate::jerror::JTRC_PARMLESS_MARKER;
pub use crate::jerror::JTRC_QUANTVALS;
pub use crate::jerror::JTRC_QUANT_3_NCOLORS;
pub use crate::jerror::JTRC_QUANT_NCOLORS;
pub use crate::jerror::JTRC_QUANT_SELECTED;
pub use crate::jerror::JTRC_RECOVERY_ACTION;
pub use crate::jerror::JTRC_RST;
pub use crate::jerror::JTRC_SMOOTH_NOTIMPL;
pub use crate::jerror::JTRC_SOF;
pub use crate::jerror::JTRC_SOF_COMPONENT;
pub use crate::jerror::JTRC_SOI;
pub use crate::jerror::JTRC_SOS;
pub use crate::jerror::JTRC_SOS_COMPONENT;
pub use crate::jerror::JTRC_SOS_PARAMS;
pub use crate::jerror::JTRC_TFILE_CLOSE;
pub use crate::jerror::JTRC_TFILE_OPEN;
pub use crate::jerror::JTRC_THUMB_JPEG;
pub use crate::jerror::JTRC_THUMB_PALETTE;
pub use crate::jerror::JTRC_THUMB_RGB;
pub use crate::jerror::JTRC_UNKNOWN_IDS;
pub use crate::jerror::JTRC_XMS_CLOSE;
pub use crate::jerror::JTRC_XMS_OPEN;
pub use crate::jerror::JWRN_ADOBE_XFORM;
pub use crate::jerror::JWRN_BOGUS_ICC;
pub use crate::jerror::JWRN_BOGUS_PROGRESSION;
pub use crate::jerror::JWRN_EXTRANEOUS_DATA;
pub use crate::jerror::JWRN_HIT_MARKER;
pub use crate::jerror::JWRN_HUFF_BAD_CODE;
pub use crate::jerror::JWRN_JFIF_MAJOR;
pub use crate::jerror::JWRN_JPEG_EOF;
pub use crate::jerror::JWRN_MUST_RESYNC;
pub use crate::jerror::JWRN_NOT_SEQUENTIAL;
pub use crate::jerror::JWRN_TOO_MUCH_DATA;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jpegint_h::jpeg_natural_order;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpeglib_h::jpeg_alloc_huff_table;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::DCTSIZE2;
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
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::NUM_HUFF_TBLS;
pub use crate::jstdhuff_c::add_huff_table;
pub use crate::jstdhuff_c::std_huff_tables;
pub use crate::stddef_h::NULL;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
/*
 * Initialize for a Huffman-compressed scan.
 */
unsafe extern "C" fn start_pass_huff_decoder(mut cinfo: j_decompress_ptr) {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut ci: c_int = 0;
    let mut blkn: c_int = 0;
    let mut dctbl: c_int = 0;
    let mut actbl: c_int = 0;
    let mut pdtbl: *mut *mut d_derived_tbl = 0 as *mut *mut d_derived_tbl;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    if (*cinfo).Ss != 0i32
        || (*cinfo).Se != DCTSIZE2 - 1i32
        || (*cinfo).Ah != 0i32
        || (*cinfo).Al != 0i32
    {
        (*(*cinfo).err).msg_code = JWRN_NOT_SEQUENTIAL as c_int;
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer")(cinfo as j_common_ptr, -1i32);
    }
    ci = 0i32;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        dctbl = (*compptr).dc_tbl_no;
        actbl = (*compptr).ac_tbl_no;
        pdtbl = (*entropy)
            .dc_derived_tbls
            .as_mut_ptr()
            .offset(dctbl as isize);
        jpeg_make_d_derived_tbl(cinfo, TRUE, dctbl, pdtbl);
        pdtbl = (*entropy)
            .ac_derived_tbls
            .as_mut_ptr()
            .offset(actbl as isize);
        jpeg_make_d_derived_tbl(cinfo, FALSE, actbl, pdtbl);
        (*entropy).saved.last_dc_val[ci as usize] = 0i32;
        ci += 1
    }
    blkn = 0i32;
    while blkn < (*cinfo).blocks_in_MCU {
        ci = (*cinfo).MCU_membership[blkn as usize];
        compptr = (*cinfo).cur_comp_info[ci as usize];
        (*entropy).dc_cur_tbls[blkn as usize] =
            (*entropy).dc_derived_tbls[(*compptr).dc_tbl_no as usize];
        (*entropy).ac_cur_tbls[blkn as usize] =
            (*entropy).ac_derived_tbls[(*compptr).ac_tbl_no as usize];
        if 0 != (*compptr).component_needed {
            (*entropy).dc_needed[blkn as usize] = TRUE;
            (*entropy).ac_needed[blkn as usize] = ((*compptr).DCT_scaled_size > 1i32) as c_int
        } else {
            (*entropy).ac_needed[blkn as usize] = FALSE;
            (*entropy).dc_needed[blkn as usize] = (*entropy).ac_needed[blkn as usize]
        }
        blkn += 1
    }
    (*entropy).bitstate.bits_left = 0i32;
    (*entropy).bitstate.get_buffer = 0i32 as bit_buf_type;
    (*entropy).pub_0.insufficient_data = FALSE;
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
}
/*
 * Out-of-line code for bit fetching (shared with jdphuff.c).
 * See jdhuff.h for info about usage.
 * Note: current values of get_buffer and bits_left are passed as parameters,
 * but are returned in the corresponding fields of the state struct.
 *
 * On most machines MIN_GET_BITS should be 25 to allow the full 32-bit width
 * of get_buffer to be used.  (On machines with wider words, an even larger
 * buffer could be used.)  However, on some machines 32-bit shifts are
 * quite slow and take time proportional to the number of places shifted.
 * (This is true with most PC compilers, for instance.)  In this case it may
 * be a win to set MIN_GET_BITS to the minimum value of 15.  This reduces the
 * average shift distance at the cost of more calls to jpeg_fill_bit_buffer.
 */
pub const MIN_GET_BITS: c_int = BIT_BUF_SIZE - 7i32;
/* AVOID_TABLES */
/*
 * Check for a restart marker & resynchronize decoder.
 * Returns FALSE if must suspend.
 */
unsafe extern "C" fn process_restart(mut cinfo: j_decompress_ptr) -> boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut ci: c_int = 0;
    (*(*cinfo).marker).discarded_bytes = (*(*cinfo).marker)
        .discarded_bytes
        .wrapping_add(((*entropy).bitstate.bits_left / 8i32) as c_uint);
    (*entropy).bitstate.bits_left = 0i32;
    if 0 == (*(*cinfo).marker)
        .read_restart_marker
        .expect("non-null function pointer")(cinfo)
    {
        return FALSE;
    }
    ci = 0i32;
    while ci < (*cinfo).comps_in_scan {
        (*entropy).saved.last_dc_val[ci as usize] = 0i32;
        ci += 1
    }
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
    if (*cinfo).unread_marker == 0i32 {
        (*entropy).pub_0.insufficient_data = FALSE
    }
    return TRUE;
}
unsafe extern "C" fn decode_mcu_slow(
    mut cinfo: j_decompress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut get_buffer: bit_buf_type = 0;
    let mut bits_left: c_int = 0;
    let mut br_state: bitread_working_state = bitread_working_state {
        next_input_byte: 0 as *const JOCTET,
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: 0 as *mut jpeg_decompress_struct,
    };
    let mut blkn: c_int = 0;
    let mut state: savable_state = savable_state {
        last_dc_val: [0; 4],
    };
    br_state.cinfo = cinfo;
    br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
    br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
    get_buffer = (*entropy).bitstate.get_buffer;
    bits_left = (*entropy).bitstate.bits_left;
    state = (*entropy).saved;
    blkn = 0i32;
    while blkn < (*cinfo).blocks_in_MCU {
        let mut block: JBLOCKROW = if !MCU_data.is_null() {
            *MCU_data.offset(blkn as isize)
        } else {
            NULL as JBLOCKROW
        };
        let mut dctbl: *mut d_derived_tbl = (*entropy).dc_cur_tbls[blkn as usize];
        let mut actbl: *mut d_derived_tbl = (*entropy).ac_cur_tbls[blkn as usize];
        let mut s: c_int = 0;
        let mut k: c_int = 0;
        let mut r: c_int = 0;
        let mut current_block_22: u64;
        let mut nb: c_int = 0;
        let mut look: c_int = 0;
        if bits_left < HUFF_LOOKAHEAD {
            if 0 == jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 0i32) {
                return 0i32;
            }
            get_buffer = br_state.get_buffer;
            bits_left = br_state.bits_left;
            if bits_left < HUFF_LOOKAHEAD {
                nb = 1i32;
                current_block_22 = 6603671518751921130;
            } else {
                current_block_22 = 14576567515993809846;
            }
        } else {
            current_block_22 = 14576567515993809846;
        }
        match current_block_22 {
            14576567515993809846 => {
                look = (get_buffer >> bits_left - 8i32) as c_int & (1i32 << 8i32) - 1i32;
                nb = (*dctbl).lookup[look as usize] >> HUFF_LOOKAHEAD;
                if nb <= HUFF_LOOKAHEAD {
                    bits_left -= nb;
                    s = (*dctbl).lookup[look as usize] & (1i32 << HUFF_LOOKAHEAD) - 1i32;
                    current_block_22 = 652864300344834934;
                } else {
                    current_block_22 = 6603671518751921130;
                }
            }
            _ => {}
        }
        match current_block_22 {
            6603671518751921130 => {
                s = jpeg_huff_decode(&mut br_state, get_buffer, bits_left, dctbl, nb);
                if s < 0i32 {
                    return 0i32;
                }
                get_buffer = br_state.get_buffer;
                bits_left = br_state.bits_left
            }
            _ => {}
        }
        if 0 != s {
            if bits_left < s {
                if 0 == jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, s) {
                    return 0i32;
                }
                get_buffer = br_state.get_buffer;
                bits_left = br_state.bits_left
            }
            bits_left -= s;
            r = (get_buffer >> bits_left) as c_int & (1i32 << s) - 1i32;
            s = (r as c_uint).wrapping_add(
                (r - (1i32 << s - 1i32) >> 31i32) as c_uint
                    & ((-1i32 as c_uint) << s).wrapping_add(1i32 as c_uint),
            ) as c_int
        }
        if 0 != (*entropy).dc_needed[blkn as usize] {
            let mut ci: c_int = (*cinfo).MCU_membership[blkn as usize];
            s += state.last_dc_val[ci as usize];
            state.last_dc_val[ci as usize] = s;
            if !block.is_null() {
                (*block)[0usize] = s as JCOEF
            }
        }
        if 0 != (*entropy).ac_needed[blkn as usize] && !block.is_null() {
            k = 1i32;
            while k < DCTSIZE2 {
                let mut current_block_60: u64;
                let mut nb_0: c_int = 0;
                let mut look_0: c_int = 0;
                if bits_left < HUFF_LOOKAHEAD {
                    if 0 == jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 0i32) {
                        return 0i32;
                    }
                    get_buffer = br_state.get_buffer;
                    bits_left = br_state.bits_left;
                    if bits_left < HUFF_LOOKAHEAD {
                        nb_0 = 1i32;
                        current_block_60 = 276222993270550982;
                    } else {
                        current_block_60 = 3580086814630675314;
                    }
                } else {
                    current_block_60 = 3580086814630675314;
                }
                match current_block_60 {
                    3580086814630675314 => {
                        look_0 = (get_buffer >> bits_left - 8i32) as c_int & (1i32 << 8i32) - 1i32;
                        nb_0 = (*actbl).lookup[look_0 as usize] >> HUFF_LOOKAHEAD;
                        if nb_0 <= HUFF_LOOKAHEAD {
                            bits_left -= nb_0;
                            s = (*actbl).lookup[look_0 as usize] & (1i32 << HUFF_LOOKAHEAD) - 1i32;
                            current_block_60 = 7385833325316299293;
                        } else {
                            current_block_60 = 276222993270550982;
                        }
                    }
                    _ => {}
                }
                match current_block_60 {
                    276222993270550982 => {
                        s = jpeg_huff_decode(&mut br_state, get_buffer, bits_left, actbl, nb_0);
                        if s < 0i32 {
                            return 0i32;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left
                    }
                    _ => {}
                }
                r = s >> 4i32;
                s &= 15i32;
                if 0 != s {
                    k += r;
                    if bits_left < s {
                        if 0 == jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, s) {
                            return 0i32;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left
                    }
                    bits_left -= s;
                    r = (get_buffer >> bits_left) as c_int & (1i32 << s) - 1i32;
                    s = (r as c_uint).wrapping_add(
                        (r - (1i32 << s - 1i32) >> 31i32) as c_uint
                            & ((-1i32 as c_uint) << s).wrapping_add(1i32 as c_uint),
                    ) as c_int;
                    (*block)[*jpeg_natural_order.as_ptr().offset(k as isize) as usize] = s as JCOEF
                } else {
                    if r != 15i32 {
                        break;
                    }
                    k += 15i32
                }
                k += 1
            }
        } else {
            k = 1i32;
            while k < DCTSIZE2 {
                let mut current_block_97: u64;
                let mut nb_1: c_int = 0;
                let mut look_1: c_int = 0;
                if bits_left < HUFF_LOOKAHEAD {
                    if 0 == jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 0i32) {
                        return 0i32;
                    }
                    get_buffer = br_state.get_buffer;
                    bits_left = br_state.bits_left;
                    if bits_left < HUFF_LOOKAHEAD {
                        nb_1 = 1i32;
                        current_block_97 = 6072411194766323756;
                    } else {
                        current_block_97 = 9521147444787763968;
                    }
                } else {
                    current_block_97 = 9521147444787763968;
                }
                match current_block_97 {
                    9521147444787763968 => {
                        look_1 = (get_buffer >> bits_left - 8i32) as c_int & (1i32 << 8i32) - 1i32;
                        nb_1 = (*actbl).lookup[look_1 as usize] >> HUFF_LOOKAHEAD;
                        if nb_1 <= HUFF_LOOKAHEAD {
                            bits_left -= nb_1;
                            s = (*actbl).lookup[look_1 as usize] & (1i32 << HUFF_LOOKAHEAD) - 1i32;
                            current_block_97 = 16375338222180917333;
                        } else {
                            current_block_97 = 6072411194766323756;
                        }
                    }
                    _ => {}
                }
                match current_block_97 {
                    6072411194766323756 => {
                        s = jpeg_huff_decode(&mut br_state, get_buffer, bits_left, actbl, nb_1);
                        if s < 0i32 {
                            return 0i32;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left
                    }
                    _ => {}
                }
                r = s >> 4i32;
                s &= 15i32;
                if 0 != s {
                    k += r;
                    if bits_left < s {
                        if 0 == jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, s) {
                            return 0i32;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left
                    }
                    bits_left -= s
                } else {
                    if r != 15i32 {
                        break;
                    }
                    k += 15i32
                }
                k += 1
            }
        }
        blkn += 1
    }
    (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
    (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
    (*entropy).bitstate.get_buffer = get_buffer;
    (*entropy).bitstate.bits_left = bits_left;
    (*entropy).saved = state;
    return TRUE;
}
unsafe extern "C" fn decode_mcu_fast(
    mut cinfo: j_decompress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut get_buffer: bit_buf_type = 0;
    let mut bits_left: c_int = 0;
    let mut br_state: bitread_working_state = bitread_working_state {
        next_input_byte: 0 as *const JOCTET,
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: 0 as *mut jpeg_decompress_struct,
    };
    let mut buffer: *mut JOCTET = 0 as *mut JOCTET;
    let mut blkn: c_int = 0;
    let mut state: savable_state = savable_state {
        last_dc_val: [0; 4],
    };
    br_state.cinfo = cinfo;
    br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
    br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
    get_buffer = (*entropy).bitstate.get_buffer;
    bits_left = (*entropy).bitstate.bits_left;
    buffer = br_state.next_input_byte as *mut JOCTET;
    state = (*entropy).saved;
    blkn = 0i32;
    while blkn < (*cinfo).blocks_in_MCU {
        let mut block: JBLOCKROW = if !MCU_data.is_null() {
            *MCU_data.offset(blkn as isize)
        } else {
            NULL as JBLOCKROW
        };
        let mut dctbl: *mut d_derived_tbl = (*entropy).dc_cur_tbls[blkn as usize];
        let mut actbl: *mut d_derived_tbl = (*entropy).ac_cur_tbls[blkn as usize];
        let mut s: c_int = 0;
        let mut k: c_int = 0;
        let mut r: c_int = 0;
        let mut l: c_int = 0;
        if bits_left <= 16i32 {
            let mut c0: c_int = 0;
            let mut c1: c_int = 0;
            let fresh5 = buffer;
            buffer = buffer.offset(1);
            c0 = *fresh5 as c_int;
            c1 = *buffer as c_int;
            get_buffer = get_buffer << 8i32 | c0 as c_ulong;
            bits_left += 8i32;
            if c0 == 0xffi32 {
                buffer = buffer.offset(1isize);
                if c1 != 0i32 {
                    (*cinfo).unread_marker = c1;
                    buffer = buffer.offset(-2isize);
                    get_buffer &= !0xffi32 as c_ulong
                }
            }
            let mut c0_0: c_int = 0;
            let mut c1_0: c_int = 0;
            let fresh6 = buffer;
            buffer = buffer.offset(1);
            c0_0 = *fresh6 as c_int;
            c1_0 = *buffer as c_int;
            get_buffer = get_buffer << 8i32 | c0_0 as c_ulong;
            bits_left += 8i32;
            if c0_0 == 0xffi32 {
                buffer = buffer.offset(1isize);
                if c1_0 != 0i32 {
                    (*cinfo).unread_marker = c1_0;
                    buffer = buffer.offset(-2isize);
                    get_buffer &= !0xffi32 as c_ulong
                }
            }
            let mut c0_1: c_int = 0;
            let mut c1_1: c_int = 0;
            let fresh7 = buffer;
            buffer = buffer.offset(1);
            c0_1 = *fresh7 as c_int;
            c1_1 = *buffer as c_int;
            get_buffer = get_buffer << 8i32 | c0_1 as c_ulong;
            bits_left += 8i32;
            if c0_1 == 0xffi32 {
                buffer = buffer.offset(1isize);
                if c1_1 != 0i32 {
                    (*cinfo).unread_marker = c1_1;
                    buffer = buffer.offset(-2isize);
                    get_buffer &= !0xffi32 as c_ulong
                }
            }
            let mut c0_2: c_int = 0;
            let mut c1_2: c_int = 0;
            let fresh8 = buffer;
            buffer = buffer.offset(1);
            c0_2 = *fresh8 as c_int;
            c1_2 = *buffer as c_int;
            get_buffer = get_buffer << 8i32 | c0_2 as c_ulong;
            bits_left += 8i32;
            if c0_2 == 0xffi32 {
                buffer = buffer.offset(1isize);
                if c1_2 != 0i32 {
                    (*cinfo).unread_marker = c1_2;
                    buffer = buffer.offset(-2isize);
                    get_buffer &= !0xffi32 as c_ulong
                }
            }
            let mut c0_3: c_int = 0;
            let mut c1_3: c_int = 0;
            let fresh9 = buffer;
            buffer = buffer.offset(1);
            c0_3 = *fresh9 as c_int;
            c1_3 = *buffer as c_int;
            get_buffer = get_buffer << 8i32 | c0_3 as c_ulong;
            bits_left += 8i32;
            if c0_3 == 0xffi32 {
                buffer = buffer.offset(1isize);
                if c1_3 != 0i32 {
                    (*cinfo).unread_marker = c1_3;
                    buffer = buffer.offset(-2isize);
                    get_buffer &= !0xffi32 as c_ulong
                }
            }
            let mut c0_4: c_int = 0;
            let mut c1_4: c_int = 0;
            let fresh10 = buffer;
            buffer = buffer.offset(1);
            c0_4 = *fresh10 as c_int;
            c1_4 = *buffer as c_int;
            get_buffer = get_buffer << 8i32 | c0_4 as c_ulong;
            bits_left += 8i32;
            if c0_4 == 0xffi32 {
                buffer = buffer.offset(1isize);
                if c1_4 != 0i32 {
                    (*cinfo).unread_marker = c1_4;
                    buffer = buffer.offset(-2isize);
                    get_buffer &= !0xffi32 as c_ulong
                }
            }
        }
        s = (get_buffer >> bits_left - 8i32) as c_int & (1i32 << 8i32) - 1i32;
        s = (*dctbl).lookup[s as usize];
        l = s >> HUFF_LOOKAHEAD;
        bits_left -= l;
        s = s & (1i32 << HUFF_LOOKAHEAD) - 1i32;
        if l > HUFF_LOOKAHEAD {
            s = (get_buffer >> bits_left & ((1i32 << l) - 1i32) as c_ulong) as c_int;
            while s as c_long > (*dctbl).maxcode[l as usize] {
                s <<= 1i32;
                bits_left -= 1i32;
                s |= (get_buffer >> bits_left) as c_int & (1i32 << 1i32) - 1i32;
                l += 1
            }
            s = (*(*dctbl).pub_0).huffval
                [((s as c_long + (*dctbl).valoffset[l as usize]) as c_int & 0xffi32) as usize]
                as c_int
        }
        if 0 != s {
            if bits_left <= 16i32 {
                let mut c0_5: c_int = 0;
                let mut c1_5: c_int = 0;
                let fresh11 = buffer;
                buffer = buffer.offset(1);
                c0_5 = *fresh11 as c_int;
                c1_5 = *buffer as c_int;
                get_buffer = get_buffer << 8i32 | c0_5 as c_ulong;
                bits_left += 8i32;
                if c0_5 == 0xffi32 {
                    buffer = buffer.offset(1isize);
                    if c1_5 != 0i32 {
                        (*cinfo).unread_marker = c1_5;
                        buffer = buffer.offset(-2isize);
                        get_buffer &= !0xffi32 as c_ulong
                    }
                }
                let mut c0_6: c_int = 0;
                let mut c1_6: c_int = 0;
                let fresh12 = buffer;
                buffer = buffer.offset(1);
                c0_6 = *fresh12 as c_int;
                c1_6 = *buffer as c_int;
                get_buffer = get_buffer << 8i32 | c0_6 as c_ulong;
                bits_left += 8i32;
                if c0_6 == 0xffi32 {
                    buffer = buffer.offset(1isize);
                    if c1_6 != 0i32 {
                        (*cinfo).unread_marker = c1_6;
                        buffer = buffer.offset(-2isize);
                        get_buffer &= !0xffi32 as c_ulong
                    }
                }
                let mut c0_7: c_int = 0;
                let mut c1_7: c_int = 0;
                let fresh13 = buffer;
                buffer = buffer.offset(1);
                c0_7 = *fresh13 as c_int;
                c1_7 = *buffer as c_int;
                get_buffer = get_buffer << 8i32 | c0_7 as c_ulong;
                bits_left += 8i32;
                if c0_7 == 0xffi32 {
                    buffer = buffer.offset(1isize);
                    if c1_7 != 0i32 {
                        (*cinfo).unread_marker = c1_7;
                        buffer = buffer.offset(-2isize);
                        get_buffer &= !0xffi32 as c_ulong
                    }
                }
                let mut c0_8: c_int = 0;
                let mut c1_8: c_int = 0;
                let fresh14 = buffer;
                buffer = buffer.offset(1);
                c0_8 = *fresh14 as c_int;
                c1_8 = *buffer as c_int;
                get_buffer = get_buffer << 8i32 | c0_8 as c_ulong;
                bits_left += 8i32;
                if c0_8 == 0xffi32 {
                    buffer = buffer.offset(1isize);
                    if c1_8 != 0i32 {
                        (*cinfo).unread_marker = c1_8;
                        buffer = buffer.offset(-2isize);
                        get_buffer &= !0xffi32 as c_ulong
                    }
                }
                let mut c0_9: c_int = 0;
                let mut c1_9: c_int = 0;
                let fresh15 = buffer;
                buffer = buffer.offset(1);
                c0_9 = *fresh15 as c_int;
                c1_9 = *buffer as c_int;
                get_buffer = get_buffer << 8i32 | c0_9 as c_ulong;
                bits_left += 8i32;
                if c0_9 == 0xffi32 {
                    buffer = buffer.offset(1isize);
                    if c1_9 != 0i32 {
                        (*cinfo).unread_marker = c1_9;
                        buffer = buffer.offset(-2isize);
                        get_buffer &= !0xffi32 as c_ulong
                    }
                }
                let mut c0_10: c_int = 0;
                let mut c1_10: c_int = 0;
                let fresh16 = buffer;
                buffer = buffer.offset(1);
                c0_10 = *fresh16 as c_int;
                c1_10 = *buffer as c_int;
                get_buffer = get_buffer << 8i32 | c0_10 as c_ulong;
                bits_left += 8i32;
                if c0_10 == 0xffi32 {
                    buffer = buffer.offset(1isize);
                    if c1_10 != 0i32 {
                        (*cinfo).unread_marker = c1_10;
                        buffer = buffer.offset(-2isize);
                        get_buffer &= !0xffi32 as c_ulong
                    }
                }
            }
            bits_left -= s;
            r = (get_buffer >> bits_left) as c_int & (1i32 << s) - 1i32;
            s = (r as c_uint).wrapping_add(
                (r - (1i32 << s - 1i32) >> 31i32) as c_uint
                    & ((-1i32 as c_uint) << s).wrapping_add(1i32 as c_uint),
            ) as c_int
        }
        if 0 != (*entropy).dc_needed[blkn as usize] {
            let mut ci: c_int = (*cinfo).MCU_membership[blkn as usize];
            s += state.last_dc_val[ci as usize];
            state.last_dc_val[ci as usize] = s;
            if !block.is_null() {
                (*block)[0usize] = s as JCOEF
            }
        }
        if 0 != (*entropy).ac_needed[blkn as usize] && !block.is_null() {
            k = 1i32;
            while k < DCTSIZE2 {
                if bits_left <= 16i32 {
                    let mut c0_11: c_int = 0;
                    let mut c1_11: c_int = 0;
                    let fresh17 = buffer;
                    buffer = buffer.offset(1);
                    c0_11 = *fresh17 as c_int;
                    c1_11 = *buffer as c_int;
                    get_buffer = get_buffer << 8i32 | c0_11 as c_ulong;
                    bits_left += 8i32;
                    if c0_11 == 0xffi32 {
                        buffer = buffer.offset(1isize);
                        if c1_11 != 0i32 {
                            (*cinfo).unread_marker = c1_11;
                            buffer = buffer.offset(-2isize);
                            get_buffer &= !0xffi32 as c_ulong
                        }
                    }
                    let mut c0_12: c_int = 0;
                    let mut c1_12: c_int = 0;
                    let fresh18 = buffer;
                    buffer = buffer.offset(1);
                    c0_12 = *fresh18 as c_int;
                    c1_12 = *buffer as c_int;
                    get_buffer = get_buffer << 8i32 | c0_12 as c_ulong;
                    bits_left += 8i32;
                    if c0_12 == 0xffi32 {
                        buffer = buffer.offset(1isize);
                        if c1_12 != 0i32 {
                            (*cinfo).unread_marker = c1_12;
                            buffer = buffer.offset(-2isize);
                            get_buffer &= !0xffi32 as c_ulong
                        }
                    }
                    let mut c0_13: c_int = 0;
                    let mut c1_13: c_int = 0;
                    let fresh19 = buffer;
                    buffer = buffer.offset(1);
                    c0_13 = *fresh19 as c_int;
                    c1_13 = *buffer as c_int;
                    get_buffer = get_buffer << 8i32 | c0_13 as c_ulong;
                    bits_left += 8i32;
                    if c0_13 == 0xffi32 {
                        buffer = buffer.offset(1isize);
                        if c1_13 != 0i32 {
                            (*cinfo).unread_marker = c1_13;
                            buffer = buffer.offset(-2isize);
                            get_buffer &= !0xffi32 as c_ulong
                        }
                    }
                    let mut c0_14: c_int = 0;
                    let mut c1_14: c_int = 0;
                    let fresh20 = buffer;
                    buffer = buffer.offset(1);
                    c0_14 = *fresh20 as c_int;
                    c1_14 = *buffer as c_int;
                    get_buffer = get_buffer << 8i32 | c0_14 as c_ulong;
                    bits_left += 8i32;
                    if c0_14 == 0xffi32 {
                        buffer = buffer.offset(1isize);
                        if c1_14 != 0i32 {
                            (*cinfo).unread_marker = c1_14;
                            buffer = buffer.offset(-2isize);
                            get_buffer &= !0xffi32 as c_ulong
                        }
                    }
                    let mut c0_15: c_int = 0;
                    let mut c1_15: c_int = 0;
                    let fresh21 = buffer;
                    buffer = buffer.offset(1);
                    c0_15 = *fresh21 as c_int;
                    c1_15 = *buffer as c_int;
                    get_buffer = get_buffer << 8i32 | c0_15 as c_ulong;
                    bits_left += 8i32;
                    if c0_15 == 0xffi32 {
                        buffer = buffer.offset(1isize);
                        if c1_15 != 0i32 {
                            (*cinfo).unread_marker = c1_15;
                            buffer = buffer.offset(-2isize);
                            get_buffer &= !0xffi32 as c_ulong
                        }
                    }
                    let mut c0_16: c_int = 0;
                    let mut c1_16: c_int = 0;
                    let fresh22 = buffer;
                    buffer = buffer.offset(1);
                    c0_16 = *fresh22 as c_int;
                    c1_16 = *buffer as c_int;
                    get_buffer = get_buffer << 8i32 | c0_16 as c_ulong;
                    bits_left += 8i32;
                    if c0_16 == 0xffi32 {
                        buffer = buffer.offset(1isize);
                        if c1_16 != 0i32 {
                            (*cinfo).unread_marker = c1_16;
                            buffer = buffer.offset(-2isize);
                            get_buffer &= !0xffi32 as c_ulong
                        }
                    }
                }
                s = (get_buffer >> bits_left - 8i32) as c_int & (1i32 << 8i32) - 1i32;
                s = (*actbl).lookup[s as usize];
                l = s >> HUFF_LOOKAHEAD;
                bits_left -= l;
                s = s & (1i32 << HUFF_LOOKAHEAD) - 1i32;
                if l > HUFF_LOOKAHEAD {
                    s = (get_buffer >> bits_left & ((1i32 << l) - 1i32) as c_ulong) as c_int;
                    while s as c_long > (*actbl).maxcode[l as usize] {
                        s <<= 1i32;
                        bits_left -= 1i32;
                        s |= (get_buffer >> bits_left) as c_int & (1i32 << 1i32) - 1i32;
                        l += 1
                    }
                    s = (*(*actbl).pub_0).huffval[((s as c_long + (*actbl).valoffset[l as usize])
                        as c_int
                        & 0xffi32) as usize] as c_int
                }
                r = s >> 4i32;
                s &= 15i32;
                if 0 != s {
                    k += r;
                    if bits_left <= 16i32 {
                        let mut c0_17: c_int = 0;
                        let mut c1_17: c_int = 0;
                        let fresh23 = buffer;
                        buffer = buffer.offset(1);
                        c0_17 = *fresh23 as c_int;
                        c1_17 = *buffer as c_int;
                        get_buffer = get_buffer << 8i32 | c0_17 as c_ulong;
                        bits_left += 8i32;
                        if c0_17 == 0xffi32 {
                            buffer = buffer.offset(1isize);
                            if c1_17 != 0i32 {
                                (*cinfo).unread_marker = c1_17;
                                buffer = buffer.offset(-2isize);
                                get_buffer &= !0xffi32 as c_ulong
                            }
                        }
                        let mut c0_18: c_int = 0;
                        let mut c1_18: c_int = 0;
                        let fresh24 = buffer;
                        buffer = buffer.offset(1);
                        c0_18 = *fresh24 as c_int;
                        c1_18 = *buffer as c_int;
                        get_buffer = get_buffer << 8i32 | c0_18 as c_ulong;
                        bits_left += 8i32;
                        if c0_18 == 0xffi32 {
                            buffer = buffer.offset(1isize);
                            if c1_18 != 0i32 {
                                (*cinfo).unread_marker = c1_18;
                                buffer = buffer.offset(-2isize);
                                get_buffer &= !0xffi32 as c_ulong
                            }
                        }
                        let mut c0_19: c_int = 0;
                        let mut c1_19: c_int = 0;
                        let fresh25 = buffer;
                        buffer = buffer.offset(1);
                        c0_19 = *fresh25 as c_int;
                        c1_19 = *buffer as c_int;
                        get_buffer = get_buffer << 8i32 | c0_19 as c_ulong;
                        bits_left += 8i32;
                        if c0_19 == 0xffi32 {
                            buffer = buffer.offset(1isize);
                            if c1_19 != 0i32 {
                                (*cinfo).unread_marker = c1_19;
                                buffer = buffer.offset(-2isize);
                                get_buffer &= !0xffi32 as c_ulong
                            }
                        }
                        let mut c0_20: c_int = 0;
                        let mut c1_20: c_int = 0;
                        let fresh26 = buffer;
                        buffer = buffer.offset(1);
                        c0_20 = *fresh26 as c_int;
                        c1_20 = *buffer as c_int;
                        get_buffer = get_buffer << 8i32 | c0_20 as c_ulong;
                        bits_left += 8i32;
                        if c0_20 == 0xffi32 {
                            buffer = buffer.offset(1isize);
                            if c1_20 != 0i32 {
                                (*cinfo).unread_marker = c1_20;
                                buffer = buffer.offset(-2isize);
                                get_buffer &= !0xffi32 as c_ulong
                            }
                        }
                        let mut c0_21: c_int = 0;
                        let mut c1_21: c_int = 0;
                        let fresh27 = buffer;
                        buffer = buffer.offset(1);
                        c0_21 = *fresh27 as c_int;
                        c1_21 = *buffer as c_int;
                        get_buffer = get_buffer << 8i32 | c0_21 as c_ulong;
                        bits_left += 8i32;
                        if c0_21 == 0xffi32 {
                            buffer = buffer.offset(1isize);
                            if c1_21 != 0i32 {
                                (*cinfo).unread_marker = c1_21;
                                buffer = buffer.offset(-2isize);
                                get_buffer &= !0xffi32 as c_ulong
                            }
                        }
                        let mut c0_22: c_int = 0;
                        let mut c1_22: c_int = 0;
                        let fresh28 = buffer;
                        buffer = buffer.offset(1);
                        c0_22 = *fresh28 as c_int;
                        c1_22 = *buffer as c_int;
                        get_buffer = get_buffer << 8i32 | c0_22 as c_ulong;
                        bits_left += 8i32;
                        if c0_22 == 0xffi32 {
                            buffer = buffer.offset(1isize);
                            if c1_22 != 0i32 {
                                (*cinfo).unread_marker = c1_22;
                                buffer = buffer.offset(-2isize);
                                get_buffer &= !0xffi32 as c_ulong
                            }
                        }
                    }
                    bits_left -= s;
                    r = (get_buffer >> bits_left) as c_int & (1i32 << s) - 1i32;
                    s = (r as c_uint).wrapping_add(
                        (r - (1i32 << s - 1i32) >> 31i32) as c_uint
                            & ((-1i32 as c_uint) << s).wrapping_add(1i32 as c_uint),
                    ) as c_int;
                    (*block)[*jpeg_natural_order.as_ptr().offset(k as isize) as usize] = s as JCOEF
                } else {
                    if r != 15i32 {
                        break;
                    }
                    k += 15i32
                }
                k += 1
            }
        } else {
            k = 1i32;
            while k < DCTSIZE2 {
                if bits_left <= 16i32 {
                    let mut c0_23: c_int = 0;
                    let mut c1_23: c_int = 0;
                    let fresh29 = buffer;
                    buffer = buffer.offset(1);
                    c0_23 = *fresh29 as c_int;
                    c1_23 = *buffer as c_int;
                    get_buffer = get_buffer << 8i32 | c0_23 as c_ulong;
                    bits_left += 8i32;
                    if c0_23 == 0xffi32 {
                        buffer = buffer.offset(1isize);
                        if c1_23 != 0i32 {
                            (*cinfo).unread_marker = c1_23;
                            buffer = buffer.offset(-2isize);
                            get_buffer &= !0xffi32 as c_ulong
                        }
                    }
                    let mut c0_24: c_int = 0;
                    let mut c1_24: c_int = 0;
                    let fresh30 = buffer;
                    buffer = buffer.offset(1);
                    c0_24 = *fresh30 as c_int;
                    c1_24 = *buffer as c_int;
                    get_buffer = get_buffer << 8i32 | c0_24 as c_ulong;
                    bits_left += 8i32;
                    if c0_24 == 0xffi32 {
                        buffer = buffer.offset(1isize);
                        if c1_24 != 0i32 {
                            (*cinfo).unread_marker = c1_24;
                            buffer = buffer.offset(-2isize);
                            get_buffer &= !0xffi32 as c_ulong
                        }
                    }
                    let mut c0_25: c_int = 0;
                    let mut c1_25: c_int = 0;
                    let fresh31 = buffer;
                    buffer = buffer.offset(1);
                    c0_25 = *fresh31 as c_int;
                    c1_25 = *buffer as c_int;
                    get_buffer = get_buffer << 8i32 | c0_25 as c_ulong;
                    bits_left += 8i32;
                    if c0_25 == 0xffi32 {
                        buffer = buffer.offset(1isize);
                        if c1_25 != 0i32 {
                            (*cinfo).unread_marker = c1_25;
                            buffer = buffer.offset(-2isize);
                            get_buffer &= !0xffi32 as c_ulong
                        }
                    }
                    let mut c0_26: c_int = 0;
                    let mut c1_26: c_int = 0;
                    let fresh32 = buffer;
                    buffer = buffer.offset(1);
                    c0_26 = *fresh32 as c_int;
                    c1_26 = *buffer as c_int;
                    get_buffer = get_buffer << 8i32 | c0_26 as c_ulong;
                    bits_left += 8i32;
                    if c0_26 == 0xffi32 {
                        buffer = buffer.offset(1isize);
                        if c1_26 != 0i32 {
                            (*cinfo).unread_marker = c1_26;
                            buffer = buffer.offset(-2isize);
                            get_buffer &= !0xffi32 as c_ulong
                        }
                    }
                    let mut c0_27: c_int = 0;
                    let mut c1_27: c_int = 0;
                    let fresh33 = buffer;
                    buffer = buffer.offset(1);
                    c0_27 = *fresh33 as c_int;
                    c1_27 = *buffer as c_int;
                    get_buffer = get_buffer << 8i32 | c0_27 as c_ulong;
                    bits_left += 8i32;
                    if c0_27 == 0xffi32 {
                        buffer = buffer.offset(1isize);
                        if c1_27 != 0i32 {
                            (*cinfo).unread_marker = c1_27;
                            buffer = buffer.offset(-2isize);
                            get_buffer &= !0xffi32 as c_ulong
                        }
                    }
                    let mut c0_28: c_int = 0;
                    let mut c1_28: c_int = 0;
                    let fresh34 = buffer;
                    buffer = buffer.offset(1);
                    c0_28 = *fresh34 as c_int;
                    c1_28 = *buffer as c_int;
                    get_buffer = get_buffer << 8i32 | c0_28 as c_ulong;
                    bits_left += 8i32;
                    if c0_28 == 0xffi32 {
                        buffer = buffer.offset(1isize);
                        if c1_28 != 0i32 {
                            (*cinfo).unread_marker = c1_28;
                            buffer = buffer.offset(-2isize);
                            get_buffer &= !0xffi32 as c_ulong
                        }
                    }
                }
                s = (get_buffer >> bits_left - 8i32) as c_int & (1i32 << 8i32) - 1i32;
                s = (*actbl).lookup[s as usize];
                l = s >> HUFF_LOOKAHEAD;
                bits_left -= l;
                s = s & (1i32 << HUFF_LOOKAHEAD) - 1i32;
                if l > HUFF_LOOKAHEAD {
                    s = (get_buffer >> bits_left & ((1i32 << l) - 1i32) as c_ulong) as c_int;
                    while s as c_long > (*actbl).maxcode[l as usize] {
                        s <<= 1i32;
                        bits_left -= 1i32;
                        s |= (get_buffer >> bits_left) as c_int & (1i32 << 1i32) - 1i32;
                        l += 1
                    }
                    s = (*(*actbl).pub_0).huffval[((s as c_long + (*actbl).valoffset[l as usize])
                        as c_int
                        & 0xffi32) as usize] as c_int
                }
                r = s >> 4i32;
                s &= 15i32;
                if 0 != s {
                    k += r;
                    if bits_left <= 16i32 {
                        let mut c0_29: c_int = 0;
                        let mut c1_29: c_int = 0;
                        let fresh35 = buffer;
                        buffer = buffer.offset(1);
                        c0_29 = *fresh35 as c_int;
                        c1_29 = *buffer as c_int;
                        get_buffer = get_buffer << 8i32 | c0_29 as c_ulong;
                        bits_left += 8i32;
                        if c0_29 == 0xffi32 {
                            buffer = buffer.offset(1isize);
                            if c1_29 != 0i32 {
                                (*cinfo).unread_marker = c1_29;
                                buffer = buffer.offset(-2isize);
                                get_buffer &= !0xffi32 as c_ulong
                            }
                        }
                        let mut c0_30: c_int = 0;
                        let mut c1_30: c_int = 0;
                        let fresh36 = buffer;
                        buffer = buffer.offset(1);
                        c0_30 = *fresh36 as c_int;
                        c1_30 = *buffer as c_int;
                        get_buffer = get_buffer << 8i32 | c0_30 as c_ulong;
                        bits_left += 8i32;
                        if c0_30 == 0xffi32 {
                            buffer = buffer.offset(1isize);
                            if c1_30 != 0i32 {
                                (*cinfo).unread_marker = c1_30;
                                buffer = buffer.offset(-2isize);
                                get_buffer &= !0xffi32 as c_ulong
                            }
                        }
                        let mut c0_31: c_int = 0;
                        let mut c1_31: c_int = 0;
                        let fresh37 = buffer;
                        buffer = buffer.offset(1);
                        c0_31 = *fresh37 as c_int;
                        c1_31 = *buffer as c_int;
                        get_buffer = get_buffer << 8i32 | c0_31 as c_ulong;
                        bits_left += 8i32;
                        if c0_31 == 0xffi32 {
                            buffer = buffer.offset(1isize);
                            if c1_31 != 0i32 {
                                (*cinfo).unread_marker = c1_31;
                                buffer = buffer.offset(-2isize);
                                get_buffer &= !0xffi32 as c_ulong
                            }
                        }
                        let mut c0_32: c_int = 0;
                        let mut c1_32: c_int = 0;
                        let fresh38 = buffer;
                        buffer = buffer.offset(1);
                        c0_32 = *fresh38 as c_int;
                        c1_32 = *buffer as c_int;
                        get_buffer = get_buffer << 8i32 | c0_32 as c_ulong;
                        bits_left += 8i32;
                        if c0_32 == 0xffi32 {
                            buffer = buffer.offset(1isize);
                            if c1_32 != 0i32 {
                                (*cinfo).unread_marker = c1_32;
                                buffer = buffer.offset(-2isize);
                                get_buffer &= !0xffi32 as c_ulong
                            }
                        }
                        let mut c0_33: c_int = 0;
                        let mut c1_33: c_int = 0;
                        let fresh39 = buffer;
                        buffer = buffer.offset(1);
                        c0_33 = *fresh39 as c_int;
                        c1_33 = *buffer as c_int;
                        get_buffer = get_buffer << 8i32 | c0_33 as c_ulong;
                        bits_left += 8i32;
                        if c0_33 == 0xffi32 {
                            buffer = buffer.offset(1isize);
                            if c1_33 != 0i32 {
                                (*cinfo).unread_marker = c1_33;
                                buffer = buffer.offset(-2isize);
                                get_buffer &= !0xffi32 as c_ulong
                            }
                        }
                        let mut c0_34: c_int = 0;
                        let mut c1_34: c_int = 0;
                        let fresh40 = buffer;
                        buffer = buffer.offset(1);
                        c0_34 = *fresh40 as c_int;
                        c1_34 = *buffer as c_int;
                        get_buffer = get_buffer << 8i32 | c0_34 as c_ulong;
                        bits_left += 8i32;
                        if c0_34 == 0xffi32 {
                            buffer = buffer.offset(1isize);
                            if c1_34 != 0i32 {
                                (*cinfo).unread_marker = c1_34;
                                buffer = buffer.offset(-2isize);
                                get_buffer &= !0xffi32 as c_ulong
                            }
                        }
                    }
                    bits_left -= s
                } else {
                    if r != 15i32 {
                        break;
                    }
                    k += 15i32
                }
                k += 1
            }
        }
        blkn += 1
    }
    if (*cinfo).unread_marker != 0i32 {
        (*cinfo).unread_marker = 0i32;
        return FALSE;
    }
    br_state.bytes_in_buffer = (br_state.bytes_in_buffer as c_ulong)
        .wrapping_sub(buffer.wrapping_offset_from(br_state.next_input_byte) as c_long as c_ulong)
        as size_t as size_t;
    br_state.next_input_byte = buffer;
    (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
    (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
    (*entropy).bitstate.get_buffer = get_buffer;
    (*entropy).bitstate.bits_left = bits_left;
    (*entropy).saved = state;
    return TRUE;
}
/*
 * Decode and return one MCU's worth of Huffman-compressed coefficients.
 * The coefficients are reordered from zigzag order into natural array order,
 * but are not dequantized.
 *
 * The i'th block of the MCU is stored into the block pointed to by
 * MCU_data[i].  WE ASSUME THIS AREA HAS BEEN ZEROED BY THE CALLER.
 * (Wholesale zeroing is usually a little faster than retail...)
 *
 * Returns FALSE if data source requested suspension.  In that case no
 * changes have been made to permanent state.  (Exception: some output
 * coefficients may already have been assigned.  This is harmless for
 * this module, since we'll just re-assign them on the next call.)
 */
pub const BUFSIZE: c_int = DCTSIZE2 * 8i32;
unsafe extern "C" fn decode_mcu(
    mut cinfo: j_decompress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut usefast: c_int = 1i32;
    if 0 != (*cinfo).restart_interval {
        if (*entropy).restarts_to_go == 0i32 as c_uint {
            if 0 == process_restart(cinfo) {
                return FALSE;
            }
        }
        usefast = 0i32
    }
    if (*(*cinfo).src).bytes_in_buffer
        < (BUFSIZE as c_ulong).wrapping_mul((*cinfo).blocks_in_MCU as size_t)
        || (*cinfo).unread_marker != 0i32
    {
        usefast = 0i32
    }
    if 0 == (*entropy).pub_0.insufficient_data {
        let mut current_block_9: u64;
        if 0 != usefast {
            if 0 == decode_mcu_fast(cinfo, MCU_data) {
                current_block_9 = 4519966451224754431;
            } else {
                current_block_9 = 12349973810996921269;
            }
        } else {
            current_block_9 = 4519966451224754431;
        }
        match current_block_9 {
            4519966451224754431 => {
                if 0 == decode_mcu_slow(cinfo, MCU_data) {
                    return FALSE;
                }
            }
            _ => {}
        }
    }
    (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    return TRUE;
}
/*
 * Module initialization routine for Huffman entropy decoding.
 */
#[no_mangle]
pub unsafe extern "C" fn jinit_huff_decoder(mut cinfo: j_decompress_ptr) {
    let mut entropy: huff_entropy_ptr = 0 as *mut huff_entropy_decoder;
    let mut i: c_int = 0;
    std_huff_tables(cinfo as j_common_ptr);
    entropy = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<huff_entropy_decoder>() as c_ulong,
    ) as huff_entropy_ptr;
    (*cinfo).entropy = entropy as *mut jpeg_entropy_decoder;
    (*entropy).pub_0.start_pass =
        Some(start_pass_huff_decoder as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*entropy).pub_0.decode_mcu =
        Some(decode_mcu as unsafe extern "C" fn(_: j_decompress_ptr, _: *mut JBLOCKROW) -> boolean);
    i = 0i32;
    while i < NUM_HUFF_TBLS {
        (*entropy).ac_derived_tbls[i as usize] = NULL as *mut d_derived_tbl;
        (*entropy).dc_derived_tbls[i as usize] = (*entropy).ac_derived_tbls[i as usize];
        i += 1
    }
}
