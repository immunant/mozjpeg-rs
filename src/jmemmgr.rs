use libc;
use libc::c_char;
use libc::c_int;
use libc::c_long;
use libc::c_uint;
use libc::c_ulong;
use libc::c_void;

pub use crate::jerror::C2RustUnnamed_4;
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
pub use crate::jmemsys_h::backing_store_info;
pub use crate::jmemsys_h::backing_store_ptr;
pub use crate::jmemsys_h::backing_store_struct;
pub use crate::jmemsys_h::jpeg_free_large;
pub use crate::jmemsys_h::jpeg_free_small;
pub use crate::jmemsys_h::jpeg_get_large;
pub use crate::jmemsys_h::jpeg_get_small;
pub use crate::jmemsys_h::jpeg_mem_available;
pub use crate::jmemsys_h::jpeg_mem_init;
pub use crate::jmemsys_h::jpeg_mem_term;
pub use crate::jmemsys_h::jpeg_open_backing_store;
pub use crate::jmemsys_h::MAX_ALLOC_CHUNK;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::TRUE;
use crate::jpegint_h::jzero_far;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_3;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JPOOL_NUMPOOLS;
pub use crate::jpeglib_h::JPOOL_PERMANENT;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
use crate::stdlib::getenv;
use crate::stdlib::sscanf;
pub use crate::stdlib::FILE;
pub use crate::stdlib::SIZE_MAX;
pub use crate::stdlib::_IO_FILE;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvirt_barray_control {
    pub mem_buffer: JBLOCKARRAY,
    pub rows_in_array: JDIMENSION,
    pub blocksperrow: JDIMENSION,
    pub maxaccess: JDIMENSION,
    pub rows_in_mem: JDIMENSION,
    pub rowsperchunk: JDIMENSION,
    pub cur_start_row: JDIMENSION,
    pub first_undef_row: JDIMENSION,
    pub pre_zero: boolean,
    pub dirty: boolean,
    pub b_s_open: boolean,
    pub next: jvirt_barray_ptr,
    pub b_s_info: backing_store_info,
}
/*
 * The control blocks for virtual arrays.
 * Note that these blocks are allocated in the "small" pool area.
 * System-dependent info for the associated backing store (if any) is hidden
 * inside the backing_store_info struct.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvirt_sarray_control {
    pub mem_buffer: JSAMPARRAY,
    pub rows_in_array: JDIMENSION,
    pub samplesperrow: JDIMENSION,
    pub maxaccess: JDIMENSION,
    pub rows_in_mem: JDIMENSION,
    pub rowsperchunk: JDIMENSION,
    pub cur_start_row: JDIMENSION,
    pub first_undef_row: JDIMENSION,
    pub pre_zero: boolean,
    pub dirty: boolean,
    pub b_s_open: boolean,
    pub next: jvirt_sarray_ptr,
    pub b_s_info: backing_store_info,
}
pub type my_mem_ptr = *mut my_memory_mgr;
/*
 * Here is the full definition of a memory manager object.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_memory_mgr {
    pub pub_0: jpeg_memory_mgr,
    pub small_list: [small_pool_ptr; 2],
    pub large_list: [large_pool_ptr; 2],
    pub virt_sarray_list: jvirt_sarray_ptr,
    pub virt_barray_list: jvirt_barray_ptr,
    pub total_space_allocated: size_t,
    pub last_rowsperchunk: JDIMENSION,
}
pub type large_pool_ptr = *mut large_pool_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct large_pool_struct {
    pub next: large_pool_ptr,
    pub bytes_used: size_t,
    pub bytes_left: size_t,
}
/*
 * We allocate objects from "pools", where each pool is gotten with a single
 * request to jpeg_get_small() or jpeg_get_large().  There is no per-object
 * overhead within a pool, except for alignment padding.  Each pool has a
 * header with a link to the next pool of the same class.
 * Small and large pool headers are identical.
 */
pub type small_pool_ptr = *mut small_pool_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct small_pool_struct {
    pub next: small_pool_ptr,
    pub bytes_used: size_t,
    pub bytes_left: size_t,
}
pub type small_pool_hdr = small_pool_struct;
pub type large_pool_hdr = large_pool_struct;
/*
 * jmemmgr.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2016, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains the JPEG system-independent memory management
 * routines.  This code is usable across a wide variety of machines; most
 * of the system dependencies have been isolated in a separate file.
 * The major functions provided here are:
 *   * pool-based allocation and freeing of memory;
 *   * policy decisions about how to divide available memory among the
 *     virtual arrays;
 *   * control logic for swapping virtual arrays between main memory and
 *     backing storage.
 * The separate system-dependent file provides the actual backing-storage
 * access code, and it contains the policy decision about how much total
 * main memory to use.
 * This file is system-dependent in the sense that some of its functions
 * are unnecessary in some systems.  For example, if there is enough virtual
 * memory so that backing storage will never be used, much of the virtual
 * array control logic could be removed.  (Of course, if you have that much
 * memory then you shouldn't care about a little bit of unused code...)
 */
/* we define jvirt_Xarray_control structs */
/* <stdlib.h> should declare getenv() */
unsafe extern "C" fn round_up_pow2(mut a: size_t, mut b: size_t) -> size_t {
    return a.wrapping_add(b).wrapping_sub(1i32 as c_ulong) & !b.wrapping_sub(1i32 as c_ulong);
}
/*
 * Some important notes:
 *   The allocation routines provided here must never return NULL.
 *   They should exit to error_exit if unsuccessful.
 *
 *   It's not a good idea to try to merge the sarray and barray routines,
 *   even though they are textually almost the same, because samples are
 *   usually stored as bytes while coefficients are shorts or ints.  Thus,
 *   in machines where byte pointers have a different representation from
 *   word pointers, the resulting machine code could not be the same.
 */
/*
 * Many machines require storage alignment: longs must start on 4-byte
 * boundaries, doubles on 8-byte boundaries, etc.  On such machines, malloc()
 * always returns pointers that are multiples of the worst-case alignment
 * requirement, and we had better do so too.
 * There isn't any really portable way to determine the worst-case alignment
 * requirement.  This module assumes that the alignment requirement is
 * multiples of ALIGN_SIZE.
 * By default, we define ALIGN_SIZE as sizeof(double).  This is necessary on
 * some workstations (where doubles really do need 8-byte alignment) and will
 * work fine on nearly everything.  If your machine has lesser alignment needs,
 * you can save a few bytes by making ALIGN_SIZE smaller.
 * The only place I know of where this will NOT work is certain Macintosh
 * 680x0 compilers that define double as a 10-byte IEEE extended float.
 * Doing 10-byte alignment is counterproductive because longwords won't be
 * aligned well.  Put "#define ALIGN_SIZE 4" in jconfig.h if you have
 * such a compiler.
 */
/* so can override from jconfig.h */
/* Most of the SIMD instructions we support require
16-byte (128-bit) alignment, but AVX2 requires
32-byte alignment. */
pub const ALIGN_SIZE: c_int = 32i32;
/* optional extra stuff for statistics */
/* MEM_STATS */
unsafe extern "C" fn out_of_memory(mut cinfo: j_common_ptr, mut which: c_int) {
    (*(*cinfo).err).msg_code = JERR_OUT_OF_MEMORY as c_int;
    (*(*cinfo).err).msg_parm.i[0usize] = which;
    (*(*cinfo).err)
        .error_exit
        .expect("non-null function pointer")(cinfo);
}
/*
 * Allocation of "small" objects.
 *
 * For these, we use pooled storage.  When a new pool must be created,
 * we try to get enough space for the current request plus a "slop" factor,
 * where the slop will be the amount of leftover space in the new pool.
 * The speed vs. space tradeoff is largely determined by the slop values.
 * A different slop value is provided for each pool class (lifetime),
 * and we also distinguish the first pool of a class from later ones.
 * NOTE: the values given work fairly well on both 16- and 32-bit-int
 * machines, but may be too small if longs are 64 bits or more.
 *
 * Since we do not know what alignment malloc() gives us, we have to
 * allocate ALIGN_SIZE-1 extra space per pool to have room for alignment
 * adjustment.
 */
static mut first_pool_slop: [size_t; 2] = [1600i32 as size_t, 16000i32 as size_t];
/* first PERMANENT pool */
/* first IMAGE pool */
static mut extra_pool_slop: [size_t; 2] = [0i32 as size_t, 5000i32 as size_t];
/* additional PERMANENT pools */
/* additional IMAGE pools */
/* greater than 0 to avoid futile looping */
pub const MIN_SLOP: c_int = 50i32;
unsafe extern "C" fn alloc_small(
    mut cinfo: j_common_ptr,
    mut pool_id: c_int,
    mut sizeofobject: size_t,
) -> *mut c_void {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut hdr_ptr: small_pool_ptr = 0 as *mut small_pool_struct;
    let mut prev_hdr_ptr: small_pool_ptr = 0 as *mut small_pool_struct;
    let mut data_ptr: *mut c_char = 0 as *mut c_char;
    let mut min_request: size_t = 0;
    let mut slop: size_t = 0;
    if sizeofobject > MAX_ALLOC_CHUNK as c_ulong {
        out_of_memory(cinfo, 7i32);
    }
    sizeofobject = round_up_pow2(sizeofobject, ALIGN_SIZE as size_t);
    if (::std::mem::size_of::<small_pool_hdr>() as c_ulong)
        .wrapping_add(sizeofobject)
        .wrapping_add(ALIGN_SIZE as c_ulong)
        .wrapping_sub(1i32 as c_ulong)
        > MAX_ALLOC_CHUNK as c_ulong
    {
        out_of_memory(cinfo, 1i32);
    }
    if pool_id < 0i32 || pool_id >= JPOOL_NUMPOOLS {
        (*(*cinfo).err).msg_code = JERR_BAD_POOL_ID as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = pool_id;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo);
    }
    prev_hdr_ptr = NULL as small_pool_ptr;
    hdr_ptr = (*mem).small_list[pool_id as usize];
    while !hdr_ptr.is_null() {
        if (*hdr_ptr).bytes_left >= sizeofobject {
            /* found pool with enough space */
            break;
        } else {
            prev_hdr_ptr = hdr_ptr;
            hdr_ptr = (*hdr_ptr).next
        }
    }
    if hdr_ptr.is_null() {
        min_request = (::std::mem::size_of::<small_pool_hdr>() as c_ulong)
            .wrapping_add(sizeofobject)
            .wrapping_add(ALIGN_SIZE as c_ulong)
            .wrapping_sub(1i32 as c_ulong);
        if prev_hdr_ptr.is_null() {
            slop = first_pool_slop[pool_id as usize]
        } else {
            slop = extra_pool_slop[pool_id as usize]
        }
        if slop > (MAX_ALLOC_CHUNK as c_ulong).wrapping_sub(min_request) {
            slop = (MAX_ALLOC_CHUNK as c_ulong).wrapping_sub(min_request)
        }
        loop {
            hdr_ptr = jpeg_get_small(cinfo, min_request.wrapping_add(slop)) as small_pool_ptr;
            if !hdr_ptr.is_null() {
                break;
            }
            slop = (slop as c_ulong).wrapping_div(2i32 as c_ulong) as size_t as size_t;
            if slop < MIN_SLOP as c_ulong {
                out_of_memory(cinfo, 2i32);
            }
        }
        (*mem).total_space_allocated = ((*mem).total_space_allocated as c_ulong)
            .wrapping_add(min_request.wrapping_add(slop))
            as size_t as size_t;
        (*hdr_ptr).next = NULL as small_pool_ptr;
        (*hdr_ptr).bytes_used = 0i32 as size_t;
        (*hdr_ptr).bytes_left = sizeofobject.wrapping_add(slop);
        if prev_hdr_ptr.is_null() {
            (*mem).small_list[pool_id as usize] = hdr_ptr
        } else {
            (*prev_hdr_ptr).next = hdr_ptr
        }
    }
    data_ptr = hdr_ptr as *mut c_char;
    data_ptr = data_ptr.offset(::std::mem::size_of::<small_pool_hdr>() as c_ulong as isize);
    if 0 != (data_ptr as size_t).wrapping_rem(ALIGN_SIZE as c_ulong) {
        data_ptr = data_ptr.offset(
            (ALIGN_SIZE as c_ulong)
                .wrapping_sub((data_ptr as size_t).wrapping_rem(ALIGN_SIZE as c_ulong))
                as isize,
        )
    }
    data_ptr = data_ptr.offset((*hdr_ptr).bytes_used as isize);
    (*hdr_ptr).bytes_used =
        ((*hdr_ptr).bytes_used as c_ulong).wrapping_add(sizeofobject) as size_t as size_t;
    (*hdr_ptr).bytes_left =
        ((*hdr_ptr).bytes_left as c_ulong).wrapping_sub(sizeofobject) as size_t as size_t;
    return data_ptr as *mut c_void;
}
/*
 * Allocation of "large" objects.
 *
 * The external semantics of these are the same as "small" objects.  However,
 * the pool management heuristics are quite different.  We assume that each
 * request is large enough that it may as well be passed directly to
 * jpeg_get_large; the pool management just links everything together
 * so that we can free it all on demand.
 * Note: the major use of "large" objects is in JSAMPARRAY and JBLOCKARRAY
 * structures.  The routines that create these structures (see below)
 * deliberately bunch rows together to ensure a large request size.
 */
unsafe extern "C" fn alloc_large(
    mut cinfo: j_common_ptr,
    mut pool_id: c_int,
    mut sizeofobject: size_t,
) -> *mut c_void {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut hdr_ptr: large_pool_ptr = 0 as *mut large_pool_struct;
    let mut data_ptr: *mut c_char = 0 as *mut c_char;
    if sizeofobject > MAX_ALLOC_CHUNK as c_ulong {
        out_of_memory(cinfo, 8i32);
    }
    sizeofobject = round_up_pow2(sizeofobject, ALIGN_SIZE as size_t);
    if (::std::mem::size_of::<large_pool_hdr>() as c_ulong)
        .wrapping_add(sizeofobject)
        .wrapping_add(ALIGN_SIZE as c_ulong)
        .wrapping_sub(1i32 as c_ulong)
        > MAX_ALLOC_CHUNK as c_ulong
    {
        out_of_memory(cinfo, 3i32);
    }
    if pool_id < 0i32 || pool_id >= JPOOL_NUMPOOLS {
        (*(*cinfo).err).msg_code = JERR_BAD_POOL_ID as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = pool_id;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo);
    }
    hdr_ptr = jpeg_get_large(
        cinfo,
        sizeofobject
            .wrapping_add(::std::mem::size_of::<large_pool_hdr>() as c_ulong)
            .wrapping_add(ALIGN_SIZE as c_ulong)
            .wrapping_sub(1i32 as c_ulong),
    ) as large_pool_ptr;
    if hdr_ptr.is_null() {
        out_of_memory(cinfo, 4i32);
    }
    (*mem).total_space_allocated = ((*mem).total_space_allocated as c_ulong).wrapping_add(
        sizeofobject
            .wrapping_add(::std::mem::size_of::<large_pool_hdr>() as c_ulong)
            .wrapping_add(ALIGN_SIZE as c_ulong)
            .wrapping_sub(1i32 as c_ulong),
    ) as size_t as size_t;
    (*hdr_ptr).next = (*mem).large_list[pool_id as usize];
    (*hdr_ptr).bytes_used = sizeofobject;
    (*hdr_ptr).bytes_left = 0i32 as size_t;
    (*mem).large_list[pool_id as usize] = hdr_ptr;
    data_ptr = hdr_ptr as *mut c_char;
    data_ptr = data_ptr.offset(::std::mem::size_of::<small_pool_hdr>() as c_ulong as isize);
    if 0 != (data_ptr as size_t).wrapping_rem(ALIGN_SIZE as c_ulong) {
        data_ptr = data_ptr.offset(
            (ALIGN_SIZE as c_ulong)
                .wrapping_sub((data_ptr as size_t).wrapping_rem(ALIGN_SIZE as c_ulong))
                as isize,
        )
    }
    return data_ptr as *mut c_void;
}
/*
 * Creation of 2-D sample arrays.
 *
 * To minimize allocation overhead and to allow I/O of large contiguous
 * blocks, we allocate the sample rows in groups of as many rows as possible
 * without exceeding MAX_ALLOC_CHUNK total bytes per allocation request.
 * NB: the virtual array control routines, later in this file, know about
 * this chunking of rows.  The rowsperchunk value is left in the mem manager
 * object so that it can be saved away if this sarray is the workspace for
 * a virtual array.
 *
 * Since we are often upsampling with a factor 2, we align the size (not
 * the start) to 2 * ALIGN_SIZE so that the upsampling routines don't have
 * to be as careful about size.
 */
unsafe extern "C" fn alloc_sarray(
    mut cinfo: j_common_ptr,
    mut pool_id: c_int,
    mut samplesperrow: JDIMENSION,
    mut numrows: JDIMENSION,
) -> JSAMPARRAY {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut result: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut workspace: JSAMPROW = 0 as *mut JSAMPLE;
    let mut rowsperchunk: JDIMENSION = 0;
    let mut currow: JDIMENSION = 0;
    let mut i: JDIMENSION = 0;
    let mut ltemp: c_long = 0;
    if (ALIGN_SIZE as c_ulong).wrapping_rem(::std::mem::size_of::<JSAMPLE>() as c_ulong)
        != 0i32 as c_ulong
    {
        out_of_memory(cinfo, 5i32);
    }
    if samplesperrow as c_long > MAX_ALLOC_CHUNK {
        out_of_memory(cinfo, 9i32);
    }
    samplesperrow = round_up_pow2(
        samplesperrow as size_t,
        ((2i32 * ALIGN_SIZE) as c_ulong).wrapping_div(::std::mem::size_of::<JSAMPLE>() as c_ulong),
    ) as JDIMENSION;
    ltemp = (MAX_ALLOC_CHUNK as c_ulong)
        .wrapping_sub(::std::mem::size_of::<large_pool_hdr>() as c_ulong)
        .wrapping_div(
            (samplesperrow as c_long as c_ulong)
                .wrapping_mul(::std::mem::size_of::<JSAMPLE>() as c_ulong),
        ) as c_long;
    if ltemp <= 0i32 as c_long {
        (*(*cinfo).err).msg_code = JERR_WIDTH_OVERFLOW as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo);
    }
    if ltemp < numrows as c_long {
        rowsperchunk = ltemp as JDIMENSION
    } else {
        rowsperchunk = numrows
    }
    (*mem).last_rowsperchunk = rowsperchunk;
    result = alloc_small(
        cinfo,
        pool_id,
        (numrows as c_ulong).wrapping_mul(::std::mem::size_of::<JSAMPROW>() as c_ulong),
    ) as JSAMPARRAY;
    currow = 0i32 as JDIMENSION;
    while currow < numrows {
        rowsperchunk = if rowsperchunk < numrows.wrapping_sub(currow) {
            rowsperchunk
        } else {
            numrows.wrapping_sub(currow)
        };
        workspace = alloc_large(
            cinfo,
            pool_id,
            (rowsperchunk as size_t)
                .wrapping_mul(samplesperrow as size_t)
                .wrapping_mul(::std::mem::size_of::<JSAMPLE>() as c_ulong),
        ) as JSAMPROW;
        i = rowsperchunk;
        while i > 0i32 as c_uint {
            let fresh0 = currow;
            currow = currow.wrapping_add(1);
            let ref mut fresh1 = *result.offset(fresh0 as isize);
            *fresh1 = workspace;
            workspace = workspace.offset(samplesperrow as isize);
            i = i.wrapping_sub(1)
        }
    }
    return result;
}
/*
 * Creation of 2-D coefficient-block arrays.
 * This is essentially the same as the code for sample arrays, above.
 */
unsafe extern "C" fn alloc_barray(
    mut cinfo: j_common_ptr,
    mut pool_id: c_int,
    mut blocksperrow: JDIMENSION,
    mut numrows: JDIMENSION,
) -> JBLOCKARRAY {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut result: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut workspace: JBLOCKROW = 0 as *mut JBLOCK;
    let mut rowsperchunk: JDIMENSION = 0;
    let mut currow: JDIMENSION = 0;
    let mut i: JDIMENSION = 0;
    let mut ltemp: c_long = 0;
    if (::std::mem::size_of::<JBLOCK>() as c_ulong).wrapping_rem(ALIGN_SIZE as c_ulong)
        != 0i32 as c_ulong
    {
        out_of_memory(cinfo, 6i32);
    }
    ltemp = (MAX_ALLOC_CHUNK as c_ulong)
        .wrapping_sub(::std::mem::size_of::<large_pool_hdr>() as c_ulong)
        .wrapping_div(
            (blocksperrow as c_long as c_ulong)
                .wrapping_mul(::std::mem::size_of::<JBLOCK>() as c_ulong),
        ) as c_long;
    if ltemp <= 0i32 as c_long {
        (*(*cinfo).err).msg_code = JERR_WIDTH_OVERFLOW as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo);
    }
    if ltemp < numrows as c_long {
        rowsperchunk = ltemp as JDIMENSION
    } else {
        rowsperchunk = numrows
    }
    (*mem).last_rowsperchunk = rowsperchunk;
    result = alloc_small(
        cinfo,
        pool_id,
        (numrows as c_ulong).wrapping_mul(::std::mem::size_of::<JBLOCKROW>() as c_ulong),
    ) as JBLOCKARRAY;
    currow = 0i32 as JDIMENSION;
    while currow < numrows {
        rowsperchunk = if rowsperchunk < numrows.wrapping_sub(currow) {
            rowsperchunk
        } else {
            numrows.wrapping_sub(currow)
        };
        workspace = alloc_large(
            cinfo,
            pool_id,
            (rowsperchunk as size_t)
                .wrapping_mul(blocksperrow as size_t)
                .wrapping_mul(::std::mem::size_of::<JBLOCK>() as c_ulong),
        ) as JBLOCKROW;
        i = rowsperchunk;
        while i > 0i32 as c_uint {
            let fresh2 = currow;
            currow = currow.wrapping_add(1);
            let ref mut fresh3 = *result.offset(fresh2 as isize);
            *fresh3 = workspace;
            workspace = workspace.offset(blocksperrow as isize);
            i = i.wrapping_sub(1)
        }
    }
    return result;
}
/*
 * About virtual array management:
 *
 * The above "normal" array routines are only used to allocate strip buffers
 * (as wide as the image, but just a few rows high).  Full-image-sized buffers
 * are handled as "virtual" arrays.  The array is still accessed a strip at a
 * time, but the memory manager must save the whole array for repeated
 * accesses.  The intended implementation is that there is a strip buffer in
 * memory (as high as is possible given the desired memory limit), plus a
 * backing file that holds the rest of the array.
 *
 * The request_virt_array routines are told the total size of the image and
 * the maximum number of rows that will be accessed at once.  The in-memory
 * buffer must be at least as large as the maxaccess value.
 *
 * The request routines create control blocks but not the in-memory buffers.
 * That is postponed until realize_virt_arrays is called.  At that time the
 * total amount of space needed is known (approximately, anyway), so free
 * memory can be divided up fairly.
 *
 * The access_virt_array routines are responsible for making a specific strip
 * area accessible (after reading or writing the backing file, if necessary).
 * Note that the access routines are told whether the caller intends to modify
 * the accessed strip; during a read-only pass this saves having to rewrite
 * data to disk.  The access routines are also responsible for pre-zeroing
 * any newly accessed rows, if pre-zeroing was requested.
 *
 * In current usage, the access requests are usually for nonoverlapping
 * strips; that is, successive access start_row numbers differ by exactly
 * num_rows = maxaccess.  This means we can get good performance with simple
 * buffer dump/reload logic, by making the in-memory buffer be a multiple
 * of the access height; then there will never be accesses across bufferload
 * boundaries.  The code will still work with overlapping access requests,
 * but it doesn't handle bufferload overlaps very efficiently.
 */
unsafe extern "C" fn request_virt_sarray(
    mut cinfo: j_common_ptr,
    mut pool_id: c_int,
    mut pre_zero: boolean,
    mut samplesperrow: JDIMENSION,
    mut numrows: JDIMENSION,
    mut maxaccess: JDIMENSION,
) -> jvirt_sarray_ptr {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut result: jvirt_sarray_ptr = 0 as *mut jvirt_sarray_control;
    if pool_id != JPOOL_IMAGE {
        (*(*cinfo).err).msg_code = JERR_BAD_POOL_ID as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = pool_id;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo);
    }
    result = alloc_small(
        cinfo,
        pool_id,
        ::std::mem::size_of::<jvirt_sarray_control>() as c_ulong,
    ) as jvirt_sarray_ptr;
    (*result).mem_buffer = NULL as JSAMPARRAY;
    (*result).rows_in_array = numrows;
    (*result).samplesperrow = samplesperrow;
    (*result).maxaccess = maxaccess;
    (*result).pre_zero = pre_zero;
    (*result).b_s_open = FALSE;
    (*result).next = (*mem).virt_sarray_list;
    (*mem).virt_sarray_list = result;
    return result;
}
unsafe extern "C" fn request_virt_barray(
    mut cinfo: j_common_ptr,
    mut pool_id: c_int,
    mut pre_zero: boolean,
    mut blocksperrow: JDIMENSION,
    mut numrows: JDIMENSION,
    mut maxaccess: JDIMENSION,
) -> jvirt_barray_ptr {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut result: jvirt_barray_ptr = 0 as *mut jvirt_barray_control;
    if pool_id != JPOOL_IMAGE {
        (*(*cinfo).err).msg_code = JERR_BAD_POOL_ID as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = pool_id;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo);
    }
    result = alloc_small(
        cinfo,
        pool_id,
        ::std::mem::size_of::<jvirt_barray_control>() as c_ulong,
    ) as jvirt_barray_ptr;
    (*result).mem_buffer = NULL as JBLOCKARRAY;
    (*result).rows_in_array = numrows;
    (*result).blocksperrow = blocksperrow;
    (*result).maxaccess = maxaccess;
    (*result).pre_zero = pre_zero;
    (*result).b_s_open = FALSE;
    (*result).next = (*mem).virt_barray_list;
    (*mem).virt_barray_list = result;
    return result;
}
unsafe extern "C" fn realize_virt_arrays(mut cinfo: j_common_ptr) {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut space_per_minheight: size_t = 0;
    let mut maximum_space: size_t = 0;
    let mut avail_mem: size_t = 0;
    let mut minheights: size_t = 0;
    let mut max_minheights: size_t = 0;
    let mut sptr: jvirt_sarray_ptr = 0 as *mut jvirt_sarray_control;
    let mut bptr: jvirt_barray_ptr = 0 as *mut jvirt_barray_control;
    space_per_minheight = 0i32 as size_t;
    maximum_space = 0i32 as size_t;
    sptr = (*mem).virt_sarray_list;
    while !sptr.is_null() {
        if (*sptr).mem_buffer.is_null() {
            let mut new_space: size_t =
                (((*sptr).rows_in_array as c_long * (*sptr).samplesperrow as c_long) as c_ulong)
                    .wrapping_mul(::std::mem::size_of::<JSAMPLE>() as c_ulong);
            space_per_minheight = (space_per_minheight as c_ulong).wrapping_add(
                (((*sptr).maxaccess as c_long * (*sptr).samplesperrow as c_long) as c_ulong)
                    .wrapping_mul(::std::mem::size_of::<JSAMPLE>() as c_ulong),
            ) as size_t as size_t;
            if SIZE_MAX.wrapping_sub(maximum_space) < new_space {
                out_of_memory(cinfo, 10i32);
            }
            maximum_space = (maximum_space as c_ulong).wrapping_add(new_space) as size_t as size_t
        }
        sptr = (*sptr).next
    }
    bptr = (*mem).virt_barray_list;
    while !bptr.is_null() {
        if (*bptr).mem_buffer.is_null() {
            let mut new_space_0: size_t =
                (((*bptr).rows_in_array as c_long * (*bptr).blocksperrow as c_long) as c_ulong)
                    .wrapping_mul(::std::mem::size_of::<JBLOCK>() as c_ulong);
            space_per_minheight = (space_per_minheight as c_ulong).wrapping_add(
                (((*bptr).maxaccess as c_long * (*bptr).blocksperrow as c_long) as c_ulong)
                    .wrapping_mul(::std::mem::size_of::<JBLOCK>() as c_ulong),
            ) as size_t as size_t;
            if SIZE_MAX.wrapping_sub(maximum_space) < new_space_0 {
                out_of_memory(cinfo, 11i32);
            }
            maximum_space = (maximum_space as c_ulong).wrapping_add(new_space_0) as size_t as size_t
        }
        bptr = (*bptr).next
    }
    if space_per_minheight <= 0i32 as c_ulong {
        return;
    }
    avail_mem = jpeg_mem_available(
        cinfo,
        space_per_minheight,
        maximum_space,
        (*mem).total_space_allocated,
    );
    if avail_mem >= maximum_space {
        max_minheights = 1000000000i64 as size_t
    } else {
        max_minheights = avail_mem.wrapping_div(space_per_minheight);
        if max_minheights <= 0i32 as c_ulong {
            max_minheights = 1i32 as size_t
        }
    }
    sptr = (*mem).virt_sarray_list;
    while !sptr.is_null() {
        if (*sptr).mem_buffer.is_null() {
            minheights = (((*sptr).rows_in_array as c_long - 1i64) / (*sptr).maxaccess as c_long
                + 1i64) as size_t;
            if minheights <= max_minheights {
                (*sptr).rows_in_mem = (*sptr).rows_in_array
            } else {
                (*sptr).rows_in_mem =
                    max_minheights.wrapping_mul((*sptr).maxaccess as c_ulong) as JDIMENSION;
                jpeg_open_backing_store(
                    cinfo,
                    &mut (*sptr).b_s_info,
                    (*sptr).rows_in_array as c_long
                        * (*sptr).samplesperrow as c_long
                        * ::std::mem::size_of::<JSAMPLE>() as c_ulong as c_long,
                );
                (*sptr).b_s_open = TRUE
            }
            (*sptr).mem_buffer = alloc_sarray(
                cinfo,
                JPOOL_IMAGE,
                (*sptr).samplesperrow,
                (*sptr).rows_in_mem,
            );
            (*sptr).rowsperchunk = (*mem).last_rowsperchunk;
            (*sptr).cur_start_row = 0i32 as JDIMENSION;
            (*sptr).first_undef_row = 0i32 as JDIMENSION;
            (*sptr).dirty = FALSE
        }
        sptr = (*sptr).next
    }
    bptr = (*mem).virt_barray_list;
    while !bptr.is_null() {
        if (*bptr).mem_buffer.is_null() {
            minheights = (((*bptr).rows_in_array as c_long - 1i64) / (*bptr).maxaccess as c_long
                + 1i64) as size_t;
            if minheights <= max_minheights {
                (*bptr).rows_in_mem = (*bptr).rows_in_array
            } else {
                (*bptr).rows_in_mem =
                    max_minheights.wrapping_mul((*bptr).maxaccess as c_ulong) as JDIMENSION;
                jpeg_open_backing_store(
                    cinfo,
                    &mut (*bptr).b_s_info,
                    (*bptr).rows_in_array as c_long
                        * (*bptr).blocksperrow as c_long
                        * ::std::mem::size_of::<JBLOCK>() as c_ulong as c_long,
                );
                (*bptr).b_s_open = TRUE
            }
            (*bptr).mem_buffer = alloc_barray(
                cinfo,
                JPOOL_IMAGE,
                (*bptr).blocksperrow,
                (*bptr).rows_in_mem,
            );
            (*bptr).rowsperchunk = (*mem).last_rowsperchunk;
            (*bptr).cur_start_row = 0i32 as JDIMENSION;
            (*bptr).first_undef_row = 0i32 as JDIMENSION;
            (*bptr).dirty = FALSE
        }
        bptr = (*bptr).next
    }
}
unsafe extern "C" fn do_sarray_io(
    mut cinfo: j_common_ptr,
    mut ptr: jvirt_sarray_ptr,
    mut writing: boolean,
) {
    let mut bytesperrow: c_long = 0;
    let mut file_offset: c_long = 0;
    let mut byte_count: c_long = 0;
    let mut rows: c_long = 0;
    let mut thisrow: c_long = 0;
    let mut i: c_long = 0;
    bytesperrow = ((*ptr).samplesperrow as c_long as c_ulong)
        .wrapping_mul(::std::mem::size_of::<JSAMPLE>() as c_ulong) as c_long;
    file_offset = (*ptr).cur_start_row as c_long * bytesperrow;
    i = 0i32 as c_long;
    while i < (*ptr).rows_in_mem as c_long {
        rows = if ((*ptr).rowsperchunk as c_long) < (*ptr).rows_in_mem as c_long - i {
            (*ptr).rowsperchunk as c_long
        } else {
            (*ptr).rows_in_mem as c_long - i
        };
        thisrow = (*ptr).cur_start_row as c_long + i;
        rows = if rows < (*ptr).first_undef_row as c_long - thisrow {
            rows
        } else {
            (*ptr).first_undef_row as c_long - thisrow
        };
        rows = if rows < (*ptr).rows_in_array as c_long - thisrow {
            rows
        } else {
            (*ptr).rows_in_array as c_long - thisrow
        };
        /* this chunk might be past end of file! */
        if rows <= 0i32 as c_long {
            break;
        }
        byte_count = rows * bytesperrow;
        if 0 != writing {
            (*ptr)
                .b_s_info
                .write_backing_store
                .expect("non-null function pointer")(
                cinfo,
                &mut (*ptr).b_s_info,
                *(*ptr).mem_buffer.offset(i as isize) as *mut c_void,
                file_offset,
                byte_count,
            );
        } else {
            (*ptr)
                .b_s_info
                .read_backing_store
                .expect("non-null function pointer")(
                cinfo,
                &mut (*ptr).b_s_info,
                *(*ptr).mem_buffer.offset(i as isize) as *mut c_void,
                file_offset,
                byte_count,
            );
        }
        file_offset += byte_count;
        i += (*ptr).rowsperchunk as c_long
    }
}
unsafe extern "C" fn do_barray_io(
    mut cinfo: j_common_ptr,
    mut ptr: jvirt_barray_ptr,
    mut writing: boolean,
) {
    let mut bytesperrow: c_long = 0;
    let mut file_offset: c_long = 0;
    let mut byte_count: c_long = 0;
    let mut rows: c_long = 0;
    let mut thisrow: c_long = 0;
    let mut i: c_long = 0;
    bytesperrow = ((*ptr).blocksperrow as c_long as c_ulong)
        .wrapping_mul(::std::mem::size_of::<JBLOCK>() as c_ulong) as c_long;
    file_offset = (*ptr).cur_start_row as c_long * bytesperrow;
    i = 0i32 as c_long;
    while i < (*ptr).rows_in_mem as c_long {
        rows = if ((*ptr).rowsperchunk as c_long) < (*ptr).rows_in_mem as c_long - i {
            (*ptr).rowsperchunk as c_long
        } else {
            (*ptr).rows_in_mem as c_long - i
        };
        thisrow = (*ptr).cur_start_row as c_long + i;
        rows = if rows < (*ptr).first_undef_row as c_long - thisrow {
            rows
        } else {
            (*ptr).first_undef_row as c_long - thisrow
        };
        rows = if rows < (*ptr).rows_in_array as c_long - thisrow {
            rows
        } else {
            (*ptr).rows_in_array as c_long - thisrow
        };
        /* this chunk might be past end of file! */
        if rows <= 0i32 as c_long {
            break;
        }
        byte_count = rows * bytesperrow;
        if 0 != writing {
            (*ptr)
                .b_s_info
                .write_backing_store
                .expect("non-null function pointer")(
                cinfo,
                &mut (*ptr).b_s_info,
                *(*ptr).mem_buffer.offset(i as isize) as *mut c_void,
                file_offset,
                byte_count,
            );
        } else {
            (*ptr)
                .b_s_info
                .read_backing_store
                .expect("non-null function pointer")(
                cinfo,
                &mut (*ptr).b_s_info,
                *(*ptr).mem_buffer.offset(i as isize) as *mut c_void,
                file_offset,
                byte_count,
            );
        }
        file_offset += byte_count;
        i += (*ptr).rowsperchunk as c_long
    }
}
unsafe extern "C" fn access_virt_sarray(
    mut cinfo: j_common_ptr,
    mut ptr: jvirt_sarray_ptr,
    mut start_row: JDIMENSION,
    mut num_rows: JDIMENSION,
    mut writable: boolean,
) -> JSAMPARRAY {
    let mut end_row: JDIMENSION = start_row.wrapping_add(num_rows);
    let mut undef_row: JDIMENSION = 0;
    if end_row > (*ptr).rows_in_array || num_rows > (*ptr).maxaccess || (*ptr).mem_buffer.is_null()
    {
        (*(*cinfo).err).msg_code = JERR_BAD_VIRTUAL_ACCESS as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo);
    }
    if start_row < (*ptr).cur_start_row
        || end_row > (*ptr).cur_start_row.wrapping_add((*ptr).rows_in_mem)
    {
        if 0 == (*ptr).b_s_open {
            (*(*cinfo).err).msg_code = JERR_VIRTUAL_BUG as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo);
        }
        if 0 != (*ptr).dirty {
            do_sarray_io(cinfo, ptr, TRUE);
            (*ptr).dirty = FALSE
        }
        if start_row > (*ptr).cur_start_row {
            (*ptr).cur_start_row = start_row
        } else {
            let mut ltemp: c_long = 0;
            ltemp = end_row as c_long - (*ptr).rows_in_mem as c_long;
            if ltemp < 0i32 as c_long {
                ltemp = 0i32 as c_long
            }
            (*ptr).cur_start_row = ltemp as JDIMENSION
        }
        do_sarray_io(cinfo, ptr, FALSE);
    }
    if (*ptr).first_undef_row < end_row {
        if (*ptr).first_undef_row < start_row {
            if 0 != writable {
                (*(*cinfo).err).msg_code = JERR_BAD_VIRTUAL_ACCESS as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo);
            }
            undef_row = start_row
        } else {
            undef_row = (*ptr).first_undef_row
        }
        if 0 != writable {
            (*ptr).first_undef_row = end_row
        }
        if 0 != (*ptr).pre_zero {
            let mut bytesperrow: size_t = ((*ptr).samplesperrow as size_t)
                .wrapping_mul(::std::mem::size_of::<JSAMPLE>() as c_ulong);
            undef_row = (undef_row as c_uint).wrapping_sub((*ptr).cur_start_row) as JDIMENSION
                as JDIMENSION;
            end_row =
                (end_row as c_uint).wrapping_sub((*ptr).cur_start_row) as JDIMENSION as JDIMENSION;
            while undef_row < end_row {
                jzero_far(
                    *(*ptr).mem_buffer.offset(undef_row as isize) as *mut c_void,
                    bytesperrow,
                );
                undef_row = undef_row.wrapping_add(1)
            }
        } else if 0 == writable {
            (*(*cinfo).err).msg_code = JERR_BAD_VIRTUAL_ACCESS as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo);
        }
    }
    if 0 != writable {
        (*ptr).dirty = TRUE
    }
    return (*ptr)
        .mem_buffer
        .offset(start_row.wrapping_sub((*ptr).cur_start_row) as isize);
}
unsafe extern "C" fn access_virt_barray(
    mut cinfo: j_common_ptr,
    mut ptr: jvirt_barray_ptr,
    mut start_row: JDIMENSION,
    mut num_rows: JDIMENSION,
    mut writable: boolean,
) -> JBLOCKARRAY {
    let mut end_row: JDIMENSION = start_row.wrapping_add(num_rows);
    let mut undef_row: JDIMENSION = 0;
    if end_row > (*ptr).rows_in_array || num_rows > (*ptr).maxaccess || (*ptr).mem_buffer.is_null()
    {
        (*(*cinfo).err).msg_code = JERR_BAD_VIRTUAL_ACCESS as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo);
    }
    if start_row < (*ptr).cur_start_row
        || end_row > (*ptr).cur_start_row.wrapping_add((*ptr).rows_in_mem)
    {
        if 0 == (*ptr).b_s_open {
            (*(*cinfo).err).msg_code = JERR_VIRTUAL_BUG as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo);
        }
        if 0 != (*ptr).dirty {
            do_barray_io(cinfo, ptr, TRUE);
            (*ptr).dirty = FALSE
        }
        if start_row > (*ptr).cur_start_row {
            (*ptr).cur_start_row = start_row
        } else {
            let mut ltemp: c_long = 0;
            ltemp = end_row as c_long - (*ptr).rows_in_mem as c_long;
            if ltemp < 0i32 as c_long {
                ltemp = 0i32 as c_long
            }
            (*ptr).cur_start_row = ltemp as JDIMENSION
        }
        do_barray_io(cinfo, ptr, FALSE);
    }
    if (*ptr).first_undef_row < end_row {
        if (*ptr).first_undef_row < start_row {
            if 0 != writable {
                (*(*cinfo).err).msg_code = JERR_BAD_VIRTUAL_ACCESS as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo);
            }
            undef_row = start_row
        } else {
            undef_row = (*ptr).first_undef_row
        }
        if 0 != writable {
            (*ptr).first_undef_row = end_row
        }
        if 0 != (*ptr).pre_zero {
            let mut bytesperrow: size_t = ((*ptr).blocksperrow as size_t)
                .wrapping_mul(::std::mem::size_of::<JBLOCK>() as c_ulong);
            undef_row = (undef_row as c_uint).wrapping_sub((*ptr).cur_start_row) as JDIMENSION
                as JDIMENSION;
            end_row =
                (end_row as c_uint).wrapping_sub((*ptr).cur_start_row) as JDIMENSION as JDIMENSION;
            while undef_row < end_row {
                jzero_far(
                    *(*ptr).mem_buffer.offset(undef_row as isize) as *mut c_void,
                    bytesperrow,
                );
                undef_row = undef_row.wrapping_add(1)
            }
        } else if 0 == writable {
            (*(*cinfo).err).msg_code = JERR_BAD_VIRTUAL_ACCESS as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo);
        }
    }
    if 0 != writable {
        (*ptr).dirty = TRUE
    }
    return (*ptr)
        .mem_buffer
        .offset(start_row.wrapping_sub((*ptr).cur_start_row) as isize);
}
/*
 * Release all objects belonging to a specified pool.
 */
unsafe extern "C" fn free_pool(mut cinfo: j_common_ptr, mut pool_id: c_int) {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut shdr_ptr: small_pool_ptr = 0 as *mut small_pool_struct;
    let mut lhdr_ptr: large_pool_ptr = 0 as *mut large_pool_struct;
    let mut space_freed: size_t = 0;
    if pool_id < 0i32 || pool_id >= JPOOL_NUMPOOLS {
        (*(*cinfo).err).msg_code = JERR_BAD_POOL_ID as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = pool_id;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo);
    }
    if pool_id == JPOOL_IMAGE {
        let mut sptr: jvirt_sarray_ptr = 0 as *mut jvirt_sarray_control;
        let mut bptr: jvirt_barray_ptr = 0 as *mut jvirt_barray_control;
        sptr = (*mem).virt_sarray_list;
        while !sptr.is_null() {
            if 0 != (*sptr).b_s_open {
                (*sptr).b_s_open = FALSE;
                (*sptr)
                    .b_s_info
                    .close_backing_store
                    .expect("non-null function pointer")(
                    cinfo, &mut (*sptr).b_s_info
                );
            }
            sptr = (*sptr).next
        }
        (*mem).virt_sarray_list = NULL as jvirt_sarray_ptr;
        bptr = (*mem).virt_barray_list;
        while !bptr.is_null() {
            if 0 != (*bptr).b_s_open {
                (*bptr).b_s_open = FALSE;
                (*bptr)
                    .b_s_info
                    .close_backing_store
                    .expect("non-null function pointer")(
                    cinfo, &mut (*bptr).b_s_info
                );
            }
            bptr = (*bptr).next
        }
        (*mem).virt_barray_list = NULL as jvirt_barray_ptr
    }
    lhdr_ptr = (*mem).large_list[pool_id as usize];
    (*mem).large_list[pool_id as usize] = NULL as large_pool_ptr;
    while !lhdr_ptr.is_null() {
        let mut next_lhdr_ptr: large_pool_ptr = (*lhdr_ptr).next;
        space_freed = (*lhdr_ptr)
            .bytes_used
            .wrapping_add((*lhdr_ptr).bytes_left)
            .wrapping_add(::std::mem::size_of::<large_pool_hdr>() as c_ulong);
        jpeg_free_large(cinfo, lhdr_ptr as *mut c_void, space_freed);
        (*mem).total_space_allocated =
            ((*mem).total_space_allocated as c_ulong).wrapping_sub(space_freed) as size_t as size_t;
        lhdr_ptr = next_lhdr_ptr
    }
    shdr_ptr = (*mem).small_list[pool_id as usize];
    (*mem).small_list[pool_id as usize] = NULL as small_pool_ptr;
    while !shdr_ptr.is_null() {
        let mut next_shdr_ptr: small_pool_ptr = (*shdr_ptr).next;
        space_freed = (*shdr_ptr)
            .bytes_used
            .wrapping_add((*shdr_ptr).bytes_left)
            .wrapping_add(::std::mem::size_of::<small_pool_hdr>() as c_ulong);
        jpeg_free_small(cinfo, shdr_ptr as *mut c_void, space_freed);
        (*mem).total_space_allocated =
            ((*mem).total_space_allocated as c_ulong).wrapping_sub(space_freed) as size_t as size_t;
        shdr_ptr = next_shdr_ptr
    }
}
/*
 * Close up shop entirely.
 * Note that this cannot be called unless cinfo->mem is non-NULL.
 */
unsafe extern "C" fn self_destruct(mut cinfo: j_common_ptr) {
    let mut pool: c_int = 0;
    pool = JPOOL_NUMPOOLS - 1i32;
    while pool >= JPOOL_PERMANENT {
        free_pool(cinfo, pool);
        pool -= 1
    }
    jpeg_free_small(
        cinfo,
        (*cinfo).mem as *mut c_void,
        ::std::mem::size_of::<my_memory_mgr>() as c_ulong,
    );
    (*cinfo).mem = NULL as *mut jpeg_memory_mgr;
    jpeg_mem_term(cinfo);
}
/*
 * Memory manager initialization.
 * When this is called, only the error manager pointer is valid in cinfo!
 */
/* Memory manager initialization */
#[no_mangle]
pub unsafe extern "C" fn jinit_memory_mgr(mut cinfo: j_common_ptr) {
    let mut mem: my_mem_ptr = 0 as *mut my_memory_mgr;
    let mut max_to_use: c_long = 0;
    let mut pool: c_int = 0;
    let mut test_mac: size_t = 0;
    (*cinfo).mem = NULL as *mut jpeg_memory_mgr;
    if ALIGN_SIZE & ALIGN_SIZE - 1i32 != 0i32 {
        (*(*cinfo).err).msg_code = JERR_BAD_ALIGN_TYPE as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo);
    }
    test_mac = MAX_ALLOC_CHUNK as size_t;
    if test_mac as c_long != MAX_ALLOC_CHUNK
        || MAX_ALLOC_CHUNK % ALIGN_SIZE as c_long != 0i32 as c_long
    {
        (*(*cinfo).err).msg_code = JERR_BAD_ALLOC_CHUNK as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo);
    }
    max_to_use = jpeg_mem_init(cinfo);
    mem = jpeg_get_small(cinfo, ::std::mem::size_of::<my_memory_mgr>() as c_ulong) as my_mem_ptr;
    if mem.is_null() {
        jpeg_mem_term(cinfo);
        (*(*cinfo).err).msg_code = JERR_OUT_OF_MEMORY as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = 0i32;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo);
    }
    (*mem).pub_0.alloc_small = Some(
        alloc_small as unsafe extern "C" fn(_: j_common_ptr, _: c_int, _: size_t) -> *mut c_void,
    );
    (*mem).pub_0.alloc_large = Some(
        alloc_large as unsafe extern "C" fn(_: j_common_ptr, _: c_int, _: size_t) -> *mut c_void,
    );
    (*mem).pub_0.alloc_sarray = Some(
        alloc_sarray
            as unsafe extern "C" fn(
                _: j_common_ptr,
                _: c_int,
                _: JDIMENSION,
                _: JDIMENSION,
            ) -> JSAMPARRAY,
    );
    (*mem).pub_0.alloc_barray = Some(
        alloc_barray
            as unsafe extern "C" fn(
                _: j_common_ptr,
                _: c_int,
                _: JDIMENSION,
                _: JDIMENSION,
            ) -> JBLOCKARRAY,
    );
    (*mem).pub_0.request_virt_sarray = Some(
        request_virt_sarray
            as unsafe extern "C" fn(
                _: j_common_ptr,
                _: c_int,
                _: boolean,
                _: JDIMENSION,
                _: JDIMENSION,
                _: JDIMENSION,
            ) -> jvirt_sarray_ptr,
    );
    (*mem).pub_0.request_virt_barray = Some(
        request_virt_barray
            as unsafe extern "C" fn(
                _: j_common_ptr,
                _: c_int,
                _: boolean,
                _: JDIMENSION,
                _: JDIMENSION,
                _: JDIMENSION,
            ) -> jvirt_barray_ptr,
    );
    (*mem).pub_0.realize_virt_arrays =
        Some(realize_virt_arrays as unsafe extern "C" fn(_: j_common_ptr) -> ());
    (*mem).pub_0.access_virt_sarray = Some(
        access_virt_sarray
            as unsafe extern "C" fn(
                _: j_common_ptr,
                _: jvirt_sarray_ptr,
                _: JDIMENSION,
                _: JDIMENSION,
                _: boolean,
            ) -> JSAMPARRAY,
    );
    (*mem).pub_0.access_virt_barray = Some(
        access_virt_barray
            as unsafe extern "C" fn(
                _: j_common_ptr,
                _: jvirt_barray_ptr,
                _: JDIMENSION,
                _: JDIMENSION,
                _: boolean,
            ) -> JBLOCKARRAY,
    );
    (*mem).pub_0.free_pool =
        Some(free_pool as unsafe extern "C" fn(_: j_common_ptr, _: c_int) -> ());
    (*mem).pub_0.self_destruct = Some(self_destruct as unsafe extern "C" fn(_: j_common_ptr) -> ());
    (*mem).pub_0.max_alloc_chunk = MAX_ALLOC_CHUNK;
    (*mem).pub_0.max_memory_to_use = max_to_use;
    pool = JPOOL_NUMPOOLS - 1i32;
    while pool >= JPOOL_PERMANENT {
        (*mem).small_list[pool as usize] = NULL as small_pool_ptr;
        (*mem).large_list[pool as usize] = NULL as large_pool_ptr;
        pool -= 1
    }
    (*mem).virt_sarray_list = NULL as jvirt_sarray_ptr;
    (*mem).virt_barray_list = NULL as jvirt_barray_ptr;
    (*mem).total_space_allocated = ::std::mem::size_of::<my_memory_mgr>() as c_ulong;
    (*cinfo).mem = &mut (*mem).pub_0;
    let mut memenv: *mut c_char = 0 as *mut c_char;
    memenv = getenv(b"JPEGMEM\x00" as *const u8 as *const c_char);
    if !memenv.is_null() {
        let mut ch: c_char = 'x' as i32 as c_char;
        if sscanf(
            memenv,
            b"%ld%c\x00" as *const u8 as *const c_char,
            &mut max_to_use as *mut c_long,
            &mut ch as *mut c_char,
        ) > 0i32
        {
            if ch as c_int == 'm' as i32 || ch as c_int == 'M' as i32 {
                max_to_use *= 1000i64
            }
            (*mem).pub_0.max_memory_to_use = max_to_use * 1000i64
        }
    };
}
