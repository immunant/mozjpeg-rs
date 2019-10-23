use libc;

pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

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
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JPOOL_NUMPOOLS;
pub use crate::jpeglib_h::JPOOL_PERMANENT;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::src::jerror::C2RustUnnamed_3;
pub use crate::src::jerror::JERR_ARITH_NOTIMPL;
pub use crate::src::jerror::JERR_BAD_ALIGN_TYPE;
pub use crate::src::jerror::JERR_BAD_ALLOC_CHUNK;
pub use crate::src::jerror::JERR_BAD_BUFFER_MODE;
pub use crate::src::jerror::JERR_BAD_COMPONENT_ID;
pub use crate::src::jerror::JERR_BAD_CROP_SPEC;
pub use crate::src::jerror::JERR_BAD_DCTSIZE;
pub use crate::src::jerror::JERR_BAD_DCT_COEF;
pub use crate::src::jerror::JERR_BAD_HUFF_TABLE;
pub use crate::src::jerror::JERR_BAD_IN_COLORSPACE;
pub use crate::src::jerror::JERR_BAD_J_COLORSPACE;
pub use crate::src::jerror::JERR_BAD_LENGTH;
pub use crate::src::jerror::JERR_BAD_LIB_VERSION;
pub use crate::src::jerror::JERR_BAD_MCU_SIZE;
pub use crate::src::jerror::JERR_BAD_PARAM;
pub use crate::src::jerror::JERR_BAD_PARAM_VALUE;
pub use crate::src::jerror::JERR_BAD_POOL_ID;
pub use crate::src::jerror::JERR_BAD_PRECISION;
pub use crate::src::jerror::JERR_BAD_PROGRESSION;
pub use crate::src::jerror::JERR_BAD_PROG_SCRIPT;
pub use crate::src::jerror::JERR_BAD_SAMPLING;
pub use crate::src::jerror::JERR_BAD_SCAN_SCRIPT;
pub use crate::src::jerror::JERR_BAD_STATE;
pub use crate::src::jerror::JERR_BAD_STRUCT_SIZE;
pub use crate::src::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use crate::src::jerror::JERR_BUFFER_SIZE;
pub use crate::src::jerror::JERR_CANT_SUSPEND;
pub use crate::src::jerror::JERR_CCIR601_NOTIMPL;
pub use crate::src::jerror::JERR_COMPONENT_COUNT;
pub use crate::src::jerror::JERR_CONVERSION_NOTIMPL;
pub use crate::src::jerror::JERR_DAC_INDEX;
pub use crate::src::jerror::JERR_DAC_VALUE;
pub use crate::src::jerror::JERR_DHT_INDEX;
pub use crate::src::jerror::JERR_DQT_INDEX;
pub use crate::src::jerror::JERR_EMPTY_IMAGE;
pub use crate::src::jerror::JERR_EMS_READ;
pub use crate::src::jerror::JERR_EMS_WRITE;
pub use crate::src::jerror::JERR_EOI_EXPECTED;
pub use crate::src::jerror::JERR_FILE_READ;
pub use crate::src::jerror::JERR_FILE_WRITE;
pub use crate::src::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use crate::src::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use crate::src::jerror::JERR_HUFF_MISSING_CODE;
pub use crate::src::jerror::JERR_IMAGE_TOO_BIG;
pub use crate::src::jerror::JERR_INPUT_EMPTY;
pub use crate::src::jerror::JERR_INPUT_EOF;
pub use crate::src::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use crate::src::jerror::JERR_MISSING_DATA;
pub use crate::src::jerror::JERR_MODE_CHANGE;
pub use crate::src::jerror::JERR_NOTIMPL;
pub use crate::src::jerror::JERR_NOT_COMPILED;
pub use crate::src::jerror::JERR_NO_BACKING_STORE;
pub use crate::src::jerror::JERR_NO_HUFF_TABLE;
pub use crate::src::jerror::JERR_NO_IMAGE;
pub use crate::src::jerror::JERR_NO_QUANT_TABLE;
pub use crate::src::jerror::JERR_NO_SOI;
pub use crate::src::jerror::JERR_OUT_OF_MEMORY;
pub use crate::src::jerror::JERR_QUANT_COMPONENTS;
pub use crate::src::jerror::JERR_QUANT_FEW_COLORS;
pub use crate::src::jerror::JERR_QUANT_MANY_COLORS;
pub use crate::src::jerror::JERR_SOF_DUPLICATE;
pub use crate::src::jerror::JERR_SOF_NO_SOS;
pub use crate::src::jerror::JERR_SOF_UNSUPPORTED;
pub use crate::src::jerror::JERR_SOI_DUPLICATE;
pub use crate::src::jerror::JERR_SOS_NO_SOF;
pub use crate::src::jerror::JERR_TFILE_CREATE;
pub use crate::src::jerror::JERR_TFILE_READ;
pub use crate::src::jerror::JERR_TFILE_SEEK;
pub use crate::src::jerror::JERR_TFILE_WRITE;
pub use crate::src::jerror::JERR_TOO_LITTLE_DATA;
pub use crate::src::jerror::JERR_UNKNOWN_MARKER;
pub use crate::src::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use crate::src::jerror::JERR_VIRTUAL_BUG;
pub use crate::src::jerror::JERR_WIDTH_OVERFLOW;
pub use crate::src::jerror::JERR_XMS_READ;
pub use crate::src::jerror::JERR_XMS_WRITE;
pub use crate::src::jerror::JMSG_COPYRIGHT;
pub use crate::src::jerror::JMSG_LASTMSGCODE;
pub use crate::src::jerror::JMSG_NOMESSAGE;
pub use crate::src::jerror::JMSG_VERSION;
pub use crate::src::jerror::JTRC_16BIT_TABLES;
pub use crate::src::jerror::JTRC_ADOBE;
pub use crate::src::jerror::JTRC_APP0;
pub use crate::src::jerror::JTRC_APP14;
pub use crate::src::jerror::JTRC_DAC;
pub use crate::src::jerror::JTRC_DHT;
pub use crate::src::jerror::JTRC_DQT;
pub use crate::src::jerror::JTRC_DRI;
pub use crate::src::jerror::JTRC_EMS_CLOSE;
pub use crate::src::jerror::JTRC_EMS_OPEN;
pub use crate::src::jerror::JTRC_EOI;
pub use crate::src::jerror::JTRC_HUFFBITS;
pub use crate::src::jerror::JTRC_JFIF;
pub use crate::src::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use crate::src::jerror::JTRC_JFIF_EXTENSION;
pub use crate::src::jerror::JTRC_JFIF_THUMBNAIL;
pub use crate::src::jerror::JTRC_MISC_MARKER;
pub use crate::src::jerror::JTRC_PARMLESS_MARKER;
pub use crate::src::jerror::JTRC_QUANTVALS;
pub use crate::src::jerror::JTRC_QUANT_3_NCOLORS;
pub use crate::src::jerror::JTRC_QUANT_NCOLORS;
pub use crate::src::jerror::JTRC_QUANT_SELECTED;
pub use crate::src::jerror::JTRC_RECOVERY_ACTION;
pub use crate::src::jerror::JTRC_RST;
pub use crate::src::jerror::JTRC_SMOOTH_NOTIMPL;
pub use crate::src::jerror::JTRC_SOF;
pub use crate::src::jerror::JTRC_SOF_COMPONENT;
pub use crate::src::jerror::JTRC_SOI;
pub use crate::src::jerror::JTRC_SOS;
pub use crate::src::jerror::JTRC_SOS_COMPONENT;
pub use crate::src::jerror::JTRC_SOS_PARAMS;
pub use crate::src::jerror::JTRC_TFILE_CLOSE;
pub use crate::src::jerror::JTRC_TFILE_OPEN;
pub use crate::src::jerror::JTRC_THUMB_JPEG;
pub use crate::src::jerror::JTRC_THUMB_PALETTE;
pub use crate::src::jerror::JTRC_THUMB_RGB;
pub use crate::src::jerror::JTRC_UNKNOWN_IDS;
pub use crate::src::jerror::JTRC_XMS_CLOSE;
pub use crate::src::jerror::JTRC_XMS_OPEN;
pub use crate::src::jerror::JWRN_ADOBE_XFORM;
pub use crate::src::jerror::JWRN_BOGUS_ICC;
pub use crate::src::jerror::JWRN_BOGUS_PROGRESSION;
pub use crate::src::jerror::JWRN_EXTRANEOUS_DATA;
pub use crate::src::jerror::JWRN_HIT_MARKER;
pub use crate::src::jerror::JWRN_HUFF_BAD_CODE;
pub use crate::src::jerror::JWRN_JFIF_MAJOR;
pub use crate::src::jerror::JWRN_JPEG_EOF;
pub use crate::src::jerror::JWRN_MUST_RESYNC;
pub use crate::src::jerror::JWRN_NOT_SEQUENTIAL;
pub use crate::src::jerror::JWRN_TOO_MUCH_DATA;
use crate::stdlib::getenv;
use crate::stdlib::sscanf;
pub use crate::stdlib::SIZE_MAX;

pub type my_mem_ptr = *mut my_memory_mgr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_memory_mgr {
    pub pub_0: crate::jpeglib_h::jpeg_memory_mgr,
    pub small_list: [small_pool_ptr; 2],
    pub large_list: [large_pool_ptr; 2],
    pub virt_sarray_list: crate::jpeglib_h::jvirt_sarray_ptr,
    pub virt_barray_list: crate::jpeglib_h::jvirt_barray_ptr,
    pub total_space_allocated: crate::stddef_h::size_t,
    pub last_rowsperchunk: crate::jmorecfg_h::JDIMENSION,
}

pub type large_pool_ptr = *mut large_pool_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct large_pool_struct {
    pub next: large_pool_ptr,
    pub bytes_used: crate::stddef_h::size_t,
    pub bytes_left: crate::stddef_h::size_t,
}

pub type small_pool_ptr = *mut small_pool_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct small_pool_struct {
    pub next: small_pool_ptr,
    pub bytes_used: crate::stddef_h::size_t,
    pub bytes_left: crate::stddef_h::size_t,
}

pub type small_pool_hdr = small_pool_struct;

pub type large_pool_hdr = large_pool_struct;

unsafe extern "C" fn round_up_pow2(
    mut a: crate::stddef_h::size_t,
    mut b: crate::stddef_h::size_t,
) -> crate::stddef_h::size_t {
    return  a + b - 1u64
        & !(b - 1u64);
}

pub const ALIGN_SIZE: libc::c_int = 32i32;
/* System-dependent control info */
/* optional extra stuff for statistics */
/* MEM_STATS */

unsafe extern "C" fn out_of_memory(
    mut cinfo: crate::jpeglib_h::j_common_ptr,
    mut which: libc::c_int,
)
/* Report an out-of-memory error and stop execution */
/* If we compiled MEM_STATS support, report alloc requests before dying */
{
    (*(*cinfo).err).msg_code = crate::src::jerror::JERR_OUT_OF_MEMORY as libc::c_int;
    (*(*cinfo).err).msg_parm.i[0] = which;
    Some(
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer"),
    )
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

static mut first_pool_slop: [crate::stddef_h::size_t; 2] = [
    1600u64,
    16000u64,
];

static mut extra_pool_slop: [crate::stddef_h::size_t; 2] = [
    0u64,
    5000u64,
];

pub const MIN_SLOP: libc::c_int = 50i32;
/* greater than 0 to avoid futile looping */

unsafe extern "C" fn alloc_small(
    mut cinfo: crate::jpeglib_h::j_common_ptr,
    mut pool_id: libc::c_int,
    mut sizeofobject: crate::stddef_h::size_t,
) -> *mut libc::c_void
/* Allocate a "small" object */ {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut hdr_ptr: small_pool_ptr = ::std::ptr::null_mut::< small_pool_struct>();
    let mut prev_hdr_ptr: small_pool_ptr = ::std::ptr::null_mut::< small_pool_struct>();
    let mut data_ptr: *mut libc::c_char = ::std::ptr::null_mut::< libc::c_char>();
    let mut min_request: crate::stddef_h::size_t = 0;
    let mut slop: crate::stddef_h::size_t = 0;
    /*
     * Round up the requested size to a multiple of ALIGN_SIZE in order
     * to assure alignment for the next object allocated in the same pool
     * and so that algorithms can straddle outside the proper area up
     * to the next alignment.
     */
    if sizeofobject > crate::jmemsys_h::MAX_ALLOC_CHUNK as libc::c_ulong {
        /* This prevents overflow/wrap-around in round_up_pow2() if sizeofobject
        is close to SIZE_MAX. */
        out_of_memory(cinfo, 7i32);
    }
    sizeofobject = round_up_pow2(sizeofobject, ALIGN_SIZE as crate::stddef_h::size_t);
    /* Check for unsatisfiable request (do now to ensure no overflow below) */
    if ::std::mem::size_of::<small_pool_hdr>() as libc::c_ulong + sizeofobject +
    ALIGN_SIZE as libc::c_ulong - 1u64
        > crate::jmemsys_h::MAX_ALLOC_CHUNK as libc::c_ulong
    {
        out_of_memory(cinfo, 1i32); /* request exceeds malloc's ability */
    }
    /* See if space is available in any existing pool */
    if pool_id < 0i32 || pool_id >= crate::jpeglib_h::JPOOL_NUMPOOLS {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_POOL_ID as libc::c_int; /* safety check */
        (*(*cinfo).err).msg_parm.i[0] = pool_id; /* found pool with enough space */
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    prev_hdr_ptr = crate::stddef_h::NULL as small_pool_ptr;
    hdr_ptr = (*mem).small_list[pool_id as usize];
    while !hdr_ptr.is_null() {
        if (*hdr_ptr).bytes_left >= sizeofobject {
            break;
        }
        prev_hdr_ptr = hdr_ptr;
        hdr_ptr = (*hdr_ptr).next
    }
    /* Time to make a new pool? */
    if hdr_ptr.is_null() {
        /* min_request is what we need now, slop is what will be leftover */
        min_request = ::std::mem::size_of::<small_pool_hdr>() as libc::c_ulong + sizeofobject +
    ALIGN_SIZE as libc::c_ulong - 1u64;
        if prev_hdr_ptr.is_null() {
            /* first pool in class? */
            slop = first_pool_slop[pool_id as usize]
        } else {
            slop = extra_pool_slop[pool_id as usize]
        }
        /* Don't ask for more than MAX_ALLOC_CHUNK */
        if slop > crate::jmemsys_h::MAX_ALLOC_CHUNK as libc::c_ulong - min_request {
            slop = crate::jmemsys_h::MAX_ALLOC_CHUNK as libc::c_ulong - min_request
        }
        loop
        /* Try to get space, if fail reduce slop and try again */
        {
            hdr_ptr = crate::jmemsys_h::jpeg_get_small(cinfo,  min_request + slop)
                as small_pool_ptr;
            if !hdr_ptr.is_null() {
                break;
            }
            slop = slop / 2u64;
            if slop < MIN_SLOP as libc::c_ulong {
                /* give up when it gets real small */
                out_of_memory(cinfo, 2i32);
            }
        }
        (*mem).total_space_allocated = (*mem).total_space_allocated + (min_request + slop);
        /* Success, initialize the new pool header and add to end of list */
        (*hdr_ptr).next = crate::stddef_h::NULL as small_pool_ptr;
        (*hdr_ptr).bytes_used = 0u64;
        (*hdr_ptr).bytes_left =  sizeofobject + slop;
        if prev_hdr_ptr.is_null() {
            /* first pool in class? */
            (*mem).small_list[pool_id as usize] = hdr_ptr
        } else {
            (*prev_hdr_ptr).next = hdr_ptr
        }
    }
    /* OK, allocate the object from the current pool */
    data_ptr = hdr_ptr as *mut libc::c_char; /* point to first data byte in pool... */
    data_ptr = data_ptr.offset(::std::mem::size_of::<small_pool_hdr>() as isize); /* ...by skipping the header... */
    if data_ptr as crate::stddef_h::size_t % ALIGN_SIZE as libc::c_ulong != 0 {
        /* ...and adjust for alignment */
        data_ptr = data_ptr.offset((ALIGN_SIZE as libc::c_ulong -
    data_ptr as crate::stddef_h::size_t % ALIGN_SIZE as libc::c_ulong) as isize)
    } /* point to place for object */
    data_ptr = data_ptr.offset((*hdr_ptr).bytes_used as isize);
    (*hdr_ptr).bytes_used = (*hdr_ptr).bytes_used + sizeofobject;
    (*hdr_ptr).bytes_left = (*hdr_ptr).bytes_left - sizeofobject;
    return data_ptr as *mut libc::c_void;
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
    mut cinfo: crate::jpeglib_h::j_common_ptr,
    mut pool_id: libc::c_int,
    mut sizeofobject: crate::stddef_h::size_t,
) -> *mut libc::c_void
/* Allocate a "large" object */ {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut hdr_ptr: large_pool_ptr = ::std::ptr::null_mut::< large_pool_struct>();
    let mut data_ptr: *mut libc::c_char = ::std::ptr::null_mut::< libc::c_char>();
    /*
     * Round up the requested size to a multiple of ALIGN_SIZE so that
     * algorithms can straddle outside the proper area up to the next
     * alignment.
     */
    if sizeofobject > crate::jmemsys_h::MAX_ALLOC_CHUNK as libc::c_ulong {
        /* This prevents overflow/wrap-around in round_up_pow2() if sizeofobject
        is close to SIZE_MAX. */
        out_of_memory(cinfo, 8i32);
    }
    sizeofobject = round_up_pow2(sizeofobject, ALIGN_SIZE as crate::stddef_h::size_t);
    /* Check for unsatisfiable request (do now to ensure no overflow below) */
    if ::std::mem::size_of::<large_pool_hdr>() as libc::c_ulong + sizeofobject +
    ALIGN_SIZE as libc::c_ulong - 1u64
        > crate::jmemsys_h::MAX_ALLOC_CHUNK as libc::c_ulong
    {
        out_of_memory(cinfo, 3i32); /* request exceeds malloc's ability */
    }
    /* Always make a new pool */
    if pool_id < 0i32 || pool_id >= crate::jpeglib_h::JPOOL_NUMPOOLS {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_POOL_ID as libc::c_int; /* safety check */
        (*(*cinfo).err).msg_parm.i[0] = pool_id; /* jpeg_get_large failed */
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    hdr_ptr = crate::jmemsys_h::jpeg_get_large(
        cinfo,
        
        sizeofobject + ::std::mem::size_of::<large_pool_hdr>() as libc::c_ulong +
    ALIGN_SIZE as libc::c_ulong - 1u64,
    ) as large_pool_ptr;
    if hdr_ptr.is_null() {
        out_of_memory(cinfo, 4i32);
    }
    (*mem).total_space_allocated = (*mem).total_space_allocated +
    (
        sizeofobject + ::std::mem::size_of::<large_pool_hdr>() as libc::c_ulong +
         ALIGN_SIZE as libc::c_ulong - 1u64);
    /* Success, initialize the new pool header and add to list */
    (*hdr_ptr).next = (*mem).large_list[pool_id as usize];
    /* We maintain space counts in each pool header for statistical purposes,
     * even though they are not needed for allocation.
     */
    (*hdr_ptr).bytes_used = sizeofobject; /* point to first data byte in pool... */
    (*hdr_ptr).bytes_left = 0u64; /* ...by skipping the header... */
    (*mem).large_list[pool_id as usize] = hdr_ptr;
    data_ptr = hdr_ptr as *mut libc::c_char;
    data_ptr = data_ptr.offset(::std::mem::size_of::<small_pool_hdr>() as isize);
    if data_ptr as crate::stddef_h::size_t % ALIGN_SIZE as libc::c_ulong != 0 {
        /* ...and adjust for alignment */
        data_ptr = data_ptr.offset((ALIGN_SIZE as libc::c_ulong -
    data_ptr as crate::stddef_h::size_t % ALIGN_SIZE as libc::c_ulong) as isize)
    }
    return data_ptr as *mut libc::c_void;
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
    mut cinfo: crate::jpeglib_h::j_common_ptr,
    mut pool_id: libc::c_int,
    mut samplesperrow: crate::jmorecfg_h::JDIMENSION,
    mut numrows: crate::jmorecfg_h::JDIMENSION,
) -> crate::jpeglib_h::JSAMPARRAY
/* Allocate a 2-D sample array */ {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut result: crate::jpeglib_h::JSAMPARRAY = ::std::ptr::null_mut::< crate::jpeglib_h::JSAMPROW>();
    let mut workspace: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut rowsperchunk: crate::jmorecfg_h::JDIMENSION = 0;
    let mut currow: crate::jmorecfg_h::JDIMENSION = 0;
    let mut i: crate::jmorecfg_h::JDIMENSION = 0;
    let mut ltemp: libc::c_long = 0;
    /* Make sure each row is properly aligned */
    if ALIGN_SIZE as libc::c_ulong %
    ::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>() as libc::c_ulong
        != 0u64
    {
        out_of_memory(cinfo, 5i32); /* safety check */
    }
    if samplesperrow as libc::c_long > crate::jmemsys_h::MAX_ALLOC_CHUNK {
        /* This prevents overflow/wrap-around in round_up_pow2() if sizeofobject
        is close to SIZE_MAX. */
        out_of_memory(cinfo, 9i32);
    }
    samplesperrow = round_up_pow2(
        samplesperrow as crate::stddef_h::size_t,
        (2i32 * ALIGN_SIZE) as libc::c_ulong /
    ::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>() as libc::c_ulong,
    ) as crate::jmorecfg_h::JDIMENSION;
    /* Calculate max # of rows allowed in one allocation chunk */
    ltemp = ((crate::jmemsys_h::MAX_ALLOC_CHUNK as libc::c_ulong -
     ::std::mem::size_of::<large_pool_hdr>() as libc::c_ulong) /
    (samplesperrow as libc::c_ulong *
         ::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>() as libc::c_ulong)) as libc::c_long;
    if ltemp <= 0i64 {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_WIDTH_OVERFLOW as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    if ltemp < numrows as libc::c_long {
        rowsperchunk = ltemp as crate::jmorecfg_h::JDIMENSION
    } else {
        rowsperchunk = numrows
    }
    (*mem).last_rowsperchunk = rowsperchunk;
    /* Get space for row pointers (small object) */
    result = alloc_small(
        cinfo,
        pool_id,
        numrows as libc::c_ulong *
    ::std::mem::size_of::<crate::jpeglib_h::JSAMPROW>() as libc::c_ulong,
    ) as crate::jpeglib_h::JSAMPARRAY;
    /* Get the rows themselves (large objects) */
    currow = 0u32;
    while currow < numrows {
        rowsperchunk = if rowsperchunk <  numrows - currow {
            rowsperchunk
        } else {
            
            numrows - currow
        };
        workspace = alloc_large(
            cinfo,
            pool_id,
            rowsperchunk as crate::stddef_h::size_t *
    samplesperrow as crate::stddef_h::size_t *
    ::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>() as libc::c_ulong,
        ) as crate::jpeglib_h::JSAMPROW;
        i = rowsperchunk;
        while i > 0u32 {
            let fresh0 = currow;
            currow =  currow + 1;
            let ref mut fresh1 = *result.offset(fresh0 as isize);
            *fresh1 = workspace;
            workspace = workspace.offset(samplesperrow as isize);
            i =  i - 1
        }
    }
    return result;
}
/*
 * Creation of 2-D coefficient-block arrays.
 * This is essentially the same as the code for sample arrays, above.
 */

unsafe extern "C" fn alloc_barray(
    mut cinfo: crate::jpeglib_h::j_common_ptr,
    mut pool_id: libc::c_int,
    mut blocksperrow: crate::jmorecfg_h::JDIMENSION,
    mut numrows: crate::jmorecfg_h::JDIMENSION,
) -> crate::jpeglib_h::JBLOCKARRAY
/* Allocate a 2-D coefficient-block array */ {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut result: crate::jpeglib_h::JBLOCKARRAY = ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCKROW>();
    let mut workspace: crate::jpeglib_h::JBLOCKROW = ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCK>();
    let mut rowsperchunk: crate::jmorecfg_h::JDIMENSION = 0;
    let mut currow: crate::jmorecfg_h::JDIMENSION = 0;
    let mut i: crate::jmorecfg_h::JDIMENSION = 0;
    let mut ltemp: libc::c_long = 0;
    /* Make sure each row is properly aligned */
    if ::std::mem::size_of::<crate::jpeglib_h::JBLOCK>() as libc::c_ulong %
    ALIGN_SIZE as libc::c_ulong
        != 0u64
    {
        out_of_memory(cinfo, 6i32); /* safety check */
    }
    /* Calculate max # of rows allowed in one allocation chunk */
    ltemp = ((crate::jmemsys_h::MAX_ALLOC_CHUNK as libc::c_ulong -
     ::std::mem::size_of::<large_pool_hdr>() as libc::c_ulong) /
    (blocksperrow as libc::c_ulong *
         ::std::mem::size_of::<crate::jpeglib_h::JBLOCK>() as libc::c_ulong)) as libc::c_long;
    if ltemp <= 0i64 {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_WIDTH_OVERFLOW as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    if ltemp < numrows as libc::c_long {
        rowsperchunk = ltemp as crate::jmorecfg_h::JDIMENSION
    } else {
        rowsperchunk = numrows
    }
    (*mem).last_rowsperchunk = rowsperchunk;
    /* Get space for row pointers (small object) */
    result = alloc_small(
        cinfo,
        pool_id,
        numrows as libc::c_ulong *
    ::std::mem::size_of::<crate::jpeglib_h::JBLOCKROW>() as libc::c_ulong,
    ) as crate::jpeglib_h::JBLOCKARRAY;
    /* Get the rows themselves (large objects) */
    currow = 0u32;
    while currow < numrows {
        rowsperchunk = if rowsperchunk <  numrows - currow {
            rowsperchunk
        } else {
            
            numrows - currow
        };
        workspace = alloc_large(
            cinfo,
            pool_id,
            rowsperchunk as crate::stddef_h::size_t *
    blocksperrow as crate::stddef_h::size_t *
    ::std::mem::size_of::<crate::jpeglib_h::JBLOCK>() as libc::c_ulong,
        ) as crate::jpeglib_h::JBLOCKROW;
        i = rowsperchunk;
        while i > 0u32 {
            let fresh2 = currow;
            currow =  currow + 1;
            let ref mut fresh3 = *result.offset(fresh2 as isize);
            *fresh3 = workspace;
            workspace = workspace.offset(blocksperrow as isize);
            i =  i - 1
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
    mut cinfo: crate::jpeglib_h::j_common_ptr,
    mut pool_id: libc::c_int,
    mut pre_zero: crate::jmorecfg_h::boolean,
    mut samplesperrow: crate::jmorecfg_h::JDIMENSION,
    mut numrows: crate::jmorecfg_h::JDIMENSION,
    mut maxaccess: crate::jmorecfg_h::JDIMENSION,
) -> crate::jpeglib_h::jvirt_sarray_ptr
/* Request a virtual 2-D sample array */ {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut result: crate::jpeglib_h::jvirt_sarray_ptr =
        ::std::ptr::null_mut::< crate::jpeglib_h::jvirt_sarray_control>();
    /* Only IMAGE-lifetime virtual arrays are currently supported */
    if pool_id != crate::jpeglib_h::JPOOL_IMAGE {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_POOL_ID as libc::c_int; /* safety check */
        (*(*cinfo).err).msg_parm.i[0] = pool_id;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    /* get control block */
    result = alloc_small(
        cinfo,
        pool_id,
        ::std::mem::size_of::<crate::jpeglib_h::jvirt_sarray_control>() as libc::c_ulong,
    ) as crate::jpeglib_h::jvirt_sarray_ptr; /* marks array not yet realized */
    (*result).mem_buffer = crate::stddef_h::NULL as crate::jpeglib_h::JSAMPARRAY; /* no associated backing-store object */
    (*result).rows_in_array = numrows; /* add to list of virtual arrays */
    (*result).samplesperrow = samplesperrow;
    (*result).maxaccess = maxaccess;
    (*result).pre_zero = pre_zero;
    (*result).b_s_open = crate::jmorecfg_h::FALSE;
    (*result).next = (*mem).virt_sarray_list;
    (*mem).virt_sarray_list = result;
    return result;
}

unsafe extern "C" fn request_virt_barray(
    mut cinfo: crate::jpeglib_h::j_common_ptr,
    mut pool_id: libc::c_int,
    mut pre_zero: crate::jmorecfg_h::boolean,
    mut blocksperrow: crate::jmorecfg_h::JDIMENSION,
    mut numrows: crate::jmorecfg_h::JDIMENSION,
    mut maxaccess: crate::jmorecfg_h::JDIMENSION,
) -> crate::jpeglib_h::jvirt_barray_ptr
/* Request a virtual 2-D coefficient-block array */ {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut result: crate::jpeglib_h::jvirt_barray_ptr =
        ::std::ptr::null_mut::< crate::jpeglib_h::jvirt_barray_control>();
    /* Only IMAGE-lifetime virtual arrays are currently supported */
    if pool_id != crate::jpeglib_h::JPOOL_IMAGE {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_POOL_ID as libc::c_int; /* safety check */
        (*(*cinfo).err).msg_parm.i[0] = pool_id;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    /* get control block */
    result = alloc_small(
        cinfo,
        pool_id,
        ::std::mem::size_of::<crate::jpeglib_h::jvirt_barray_control>() as libc::c_ulong,
    ) as crate::jpeglib_h::jvirt_barray_ptr; /* marks array not yet realized */
    (*result).mem_buffer = crate::stddef_h::NULL as crate::jpeglib_h::JBLOCKARRAY; /* no associated backing-store object */
    (*result).rows_in_array = numrows; /* add to list of virtual arrays */
    (*result).blocksperrow = blocksperrow;
    (*result).maxaccess = maxaccess;
    (*result).pre_zero = pre_zero;
    (*result).b_s_open = crate::jmorecfg_h::FALSE;
    (*result).next = (*mem).virt_barray_list;
    (*mem).virt_barray_list = result;
    return result;
}

unsafe extern "C" fn realize_virt_arrays(mut cinfo: crate::jpeglib_h::j_common_ptr)
/* Allocate the in-memory buffers for any unrealized virtual arrays */
{
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut space_per_minheight: crate::stddef_h::size_t = 0;
    let mut maximum_space: crate::stddef_h::size_t = 0;
    let mut avail_mem: crate::stddef_h::size_t = 0;
    let mut minheights: crate::stddef_h::size_t = 0;
    let mut max_minheights: crate::stddef_h::size_t = 0;
    let mut sptr: crate::jpeglib_h::jvirt_sarray_ptr =
        ::std::ptr::null_mut::< crate::jpeglib_h::jvirt_sarray_control>();
    let mut bptr: crate::jpeglib_h::jvirt_barray_ptr =
        ::std::ptr::null_mut::< crate::jpeglib_h::jvirt_barray_control>();
    /* Compute the minimum space needed (maxaccess rows in each buffer)
     * and the maximum space needed (full image height in each buffer).
     * These may be of use to the system-dependent jpeg_mem_available routine.
     */
    space_per_minheight = 0u64;
    maximum_space = 0u64;
    sptr = (*mem).virt_sarray_list;
    while !sptr.is_null() {
        if (*sptr).mem_buffer.is_null() {
            /* if not realized yet */
            let mut new_space: crate::stddef_h::size_t = ((*sptr).rows_in_array as libc::c_long
                * (*sptr).samplesperrow as libc::c_long)
                as libc::c_ulong *
    ::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>() as libc::c_ulong;
            space_per_minheight = space_per_minheight +
    ((*sptr).maxaccess as libc::c_long * (*sptr).samplesperrow as libc::c_long)
                    as libc::c_ulong *
        
                        ::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>() as libc::c_ulong;
            if  crate::stdlib::SIZE_MAX - maximum_space < new_space {
                out_of_memory(cinfo, 10i32);
            }
            maximum_space = maximum_space + new_space
        }
        sptr = (*sptr).next
    }
    bptr = (*mem).virt_barray_list;
    while !bptr.is_null() {
        if (*bptr).mem_buffer.is_null() {
            /* if not realized yet */
            let mut new_space_0: crate::stddef_h::size_t = ((*bptr).rows_in_array as libc::c_long
                * (*bptr).blocksperrow as libc::c_long)
                as libc::c_ulong *
    ::std::mem::size_of::<crate::jpeglib_h::JBLOCK>() as libc::c_ulong; /* no unrealized arrays, no work */
            space_per_minheight = space_per_minheight +
    ((*bptr).maxaccess as libc::c_long * (*bptr).blocksperrow as libc::c_long)
                    as libc::c_ulong *
        
                        ::std::mem::size_of::<crate::jpeglib_h::JBLOCK>() as libc::c_ulong;
            if  crate::stdlib::SIZE_MAX - maximum_space < new_space_0 {
                out_of_memory(cinfo, 11i32);
            }
            maximum_space = maximum_space + new_space_0
        }
        bptr = (*bptr).next
    }
    if space_per_minheight <= 0u64 {
        return;
    }
    /* Determine amount of memory to actually use; this is system-dependent. */
    avail_mem = crate::jmemsys_h::jpeg_mem_available(
        cinfo,
        space_per_minheight,
        maximum_space,
        (*mem).total_space_allocated,
    );
    /* If the maximum space needed is available, make all the buffers full
     * height; otherwise parcel it out with the same number of minheights
     * in each buffer.
     */
    if avail_mem >= maximum_space {
        max_minheights = 1000000000u64
    } else {
        max_minheights =  avail_mem / space_per_minheight;
        /* If there doesn't seem to be enough space, try to get the minimum
         * anyway.  This allows a "stub" implementation of jpeg_mem_available().
         */
        if max_minheights <= 0u64 {
            max_minheights = 1u64
        }
    }
    /* Allocate the in-memory buffers and initialize backing store as needed. */
    sptr = (*mem).virt_sarray_list;
    while !sptr.is_null() {
        if (*sptr).mem_buffer.is_null() {
            /* if not realized yet */
            minheights = (((*sptr).rows_in_array as libc::c_long - 1i64)
                / (*sptr).maxaccess as libc::c_long
                + 1i64) as crate::stddef_h::size_t;
            if minheights <= max_minheights {
                /* This buffer fits in memory */
                (*sptr).rows_in_mem = (*sptr).rows_in_array
            } else {
                /* It doesn't fit in memory, create backing store. */
                (*sptr).rows_in_mem = ( max_minheights * (*sptr).maxaccess as libc::c_ulong)
                    as crate::jmorecfg_h::JDIMENSION;
                crate::jmemsys_h::jpeg_open_backing_store(
                    cinfo,
                    &mut (*sptr).b_s_info,
                    (*sptr).rows_in_array as libc::c_long
                        * (*sptr).samplesperrow as libc::c_long
                        *  ::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>()
                            as libc::c_long,
                );
                (*sptr).b_s_open = crate::jmorecfg_h::TRUE
            }
            (*sptr).mem_buffer = alloc_sarray(
                cinfo,
                crate::jpeglib_h::JPOOL_IMAGE,
                (*sptr).samplesperrow,
                (*sptr).rows_in_mem,
            );
            (*sptr).rowsperchunk = (*mem).last_rowsperchunk;
            (*sptr).cur_start_row = 0u32;
            (*sptr).first_undef_row = 0u32;
            (*sptr).dirty = crate::jmorecfg_h::FALSE
        }
        sptr = (*sptr).next
    }
    bptr = (*mem).virt_barray_list;
    while !bptr.is_null() {
        if (*bptr).mem_buffer.is_null() {
            /* if not realized yet */
            minheights = (((*bptr).rows_in_array as libc::c_long - 1i64)
                / (*bptr).maxaccess as libc::c_long
                + 1i64) as crate::stddef_h::size_t;
            if minheights <= max_minheights {
                /* This buffer fits in memory */
                (*bptr).rows_in_mem = (*bptr).rows_in_array
            } else {
                /* It doesn't fit in memory, create backing store. */
                (*bptr).rows_in_mem = ( max_minheights * (*bptr).maxaccess as libc::c_ulong)
                    as crate::jmorecfg_h::JDIMENSION;
                crate::jmemsys_h::jpeg_open_backing_store(
                    cinfo,
                    &mut (*bptr).b_s_info,
                    (*bptr).rows_in_array as libc::c_long
                        * (*bptr).blocksperrow as libc::c_long
                        *  ::std::mem::size_of::<crate::jpeglib_h::JBLOCK>()
                            as libc::c_long,
                );
                (*bptr).b_s_open = crate::jmorecfg_h::TRUE
            }
            (*bptr).mem_buffer = alloc_barray(
                cinfo,
                crate::jpeglib_h::JPOOL_IMAGE,
                (*bptr).blocksperrow,
                (*bptr).rows_in_mem,
            );
            (*bptr).rowsperchunk = (*mem).last_rowsperchunk;
            (*bptr).cur_start_row = 0u32;
            (*bptr).first_undef_row = 0u32;
            (*bptr).dirty = crate::jmorecfg_h::FALSE
        }
        bptr = (*bptr).next
    }
}

unsafe extern "C" fn do_sarray_io(
    mut cinfo: crate::jpeglib_h::j_common_ptr,
    mut ptr: crate::jpeglib_h::jvirt_sarray_ptr,
    mut writing: crate::jmorecfg_h::boolean,
)
/* Do backing store read or write of a virtual sample array */
{
    let mut bytesperrow: libc::c_long = 0;
    let mut file_offset: libc::c_long = 0;
    let mut byte_count: libc::c_long = 0;
    let mut rows: libc::c_long = 0;
    let mut thisrow: libc::c_long = 0;
    let mut i: libc::c_long = 0;
    bytesperrow = ((*ptr).samplesperrow as libc::c_ulong *
    ::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>() as libc::c_ulong)
        as libc::c_long;
    file_offset = (*ptr).cur_start_row as libc::c_long * bytesperrow;
    /* Loop to read or write each allocation chunk in mem_buffer */
    i = 0i64;
    while i < (*ptr).rows_in_mem as libc::c_long {
        /* One chunk, but check for short chunk at end of buffer */
        rows = if ((*ptr).rowsperchunk as libc::c_long) < (*ptr).rows_in_mem as libc::c_long - i {
            (*ptr).rowsperchunk as libc::c_long
        } else {
            ((*ptr).rows_in_mem as libc::c_long) - i
        };
        /* Transfer no more than is currently defined */
        thisrow = (*ptr).cur_start_row as libc::c_long + i;
        rows = if rows < (*ptr).first_undef_row as libc::c_long - thisrow {
            rows
        } else {
            ((*ptr).first_undef_row as libc::c_long) - thisrow
        };
        /* Transfer no more than fits in file */
        rows = if rows < (*ptr).rows_in_array as libc::c_long - thisrow {
            rows
        } else {
            ((*ptr).rows_in_array as libc::c_long) - thisrow
        };
        if rows <= 0i64 {
            break;
        }
        byte_count = rows * bytesperrow;
        if writing != 0 {
            Some(
                (*ptr)
                    .b_s_info
                    .write_backing_store
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                &mut (*ptr).b_s_info,
                *(*ptr).mem_buffer.offset(i as isize) as *mut libc::c_void,
                file_offset,
                byte_count,
            );
        } else {
            Some(
                (*ptr)
                    .b_s_info
                    .read_backing_store
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                &mut (*ptr).b_s_info,
                *(*ptr).mem_buffer.offset(i as isize) as *mut libc::c_void,
                file_offset,
                byte_count,
            );
        }
        file_offset += byte_count;
        i += (*ptr).rowsperchunk as libc::c_long
    }
}

unsafe extern "C" fn do_barray_io(
    mut cinfo: crate::jpeglib_h::j_common_ptr,
    mut ptr: crate::jpeglib_h::jvirt_barray_ptr,
    mut writing: crate::jmorecfg_h::boolean,
)
/* Do backing store read or write of a virtual coefficient-block array */
{
    let mut bytesperrow: libc::c_long = 0;
    let mut file_offset: libc::c_long = 0;
    let mut byte_count: libc::c_long = 0;
    let mut rows: libc::c_long = 0;
    let mut thisrow: libc::c_long = 0;
    let mut i: libc::c_long = 0;
    bytesperrow = ((*ptr).blocksperrow as libc::c_ulong *
    ::std::mem::size_of::<crate::jpeglib_h::JBLOCK>() as libc::c_ulong)
        as libc::c_long;
    file_offset = (*ptr).cur_start_row as libc::c_long * bytesperrow;
    /* Loop to read or write each allocation chunk in mem_buffer */
    i = 0i64;
    while i < (*ptr).rows_in_mem as libc::c_long {
        /* One chunk, but check for short chunk at end of buffer */
        rows = if ((*ptr).rowsperchunk as libc::c_long) < (*ptr).rows_in_mem as libc::c_long - i {
            (*ptr).rowsperchunk as libc::c_long
        } else {
            ((*ptr).rows_in_mem as libc::c_long) - i
        };
        /* Transfer no more than is currently defined */
        thisrow = (*ptr).cur_start_row as libc::c_long + i;
        rows = if rows < (*ptr).first_undef_row as libc::c_long - thisrow {
            rows
        } else {
            ((*ptr).first_undef_row as libc::c_long) - thisrow
        };
        /* Transfer no more than fits in file */
        rows = if rows < (*ptr).rows_in_array as libc::c_long - thisrow {
            rows
        } else {
            ((*ptr).rows_in_array as libc::c_long) - thisrow
        };
        if rows <= 0i64 {
            break;
        }
        byte_count = rows * bytesperrow;
        if writing != 0 {
            Some(
                (*ptr)
                    .b_s_info
                    .write_backing_store
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                &mut (*ptr).b_s_info,
                *(*ptr).mem_buffer.offset(i as isize) as *mut libc::c_void,
                file_offset,
                byte_count,
            );
        } else {
            Some(
                (*ptr)
                    .b_s_info
                    .read_backing_store
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                &mut (*ptr).b_s_info,
                *(*ptr).mem_buffer.offset(i as isize) as *mut libc::c_void,
                file_offset,
                byte_count,
            );
        }
        file_offset += byte_count;
        i += (*ptr).rowsperchunk as libc::c_long
    }
}

unsafe extern "C" fn access_virt_sarray(
    mut cinfo: crate::jpeglib_h::j_common_ptr,
    mut ptr: crate::jpeglib_h::jvirt_sarray_ptr,
    mut start_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: crate::jmorecfg_h::JDIMENSION,
    mut writable: crate::jmorecfg_h::boolean,
) -> crate::jpeglib_h::JSAMPARRAY
/* Access the part of a virtual sample array starting at start_row */
/* and extending for num_rows rows.  writable is true if  */
/* caller intends to modify the accessed area. */ {
    let mut end_row: crate::jmorecfg_h::JDIMENSION =  start_row + num_rows;
    let mut undef_row: crate::jmorecfg_h::JDIMENSION = 0;
    /* debugging check */
    if end_row > (*ptr).rows_in_array || num_rows > (*ptr).maxaccess || (*ptr).mem_buffer.is_null()
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_VIRTUAL_ACCESS as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    /* Make the desired part of the virtual array accessible */
    if start_row < (*ptr).cur_start_row
        || end_row >  (*ptr).cur_start_row + (*ptr).rows_in_mem
    {
        if (*ptr).b_s_open == 0 {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_VIRTUAL_BUG as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
        }
        /* Flush old buffer contents if necessary */
        if (*ptr).dirty != 0 {
            do_sarray_io(cinfo, ptr, crate::jmorecfg_h::TRUE);
            (*ptr).dirty = crate::jmorecfg_h::FALSE
        }
        /* Decide what part of virtual array to access.
         * Algorithm: if target address > current window, assume forward scan,
         * load starting at target address.  If target address < current window,
         * assume backward scan, load so that target area is top of window.
         * Note that when switching from forward write to forward read, will have
         * start_row = 0, so the limiting case applies and we load from 0 anyway.
         */
        if start_row > (*ptr).cur_start_row {
            (*ptr).cur_start_row = start_row
        } else {
            /* use long arithmetic here to avoid overflow & unsigned problems */
            let mut ltemp: libc::c_long = 0; /* don't fall off front end of file */
            ltemp = end_row as libc::c_long - (*ptr).rows_in_mem as libc::c_long;
            if ltemp < 0i64 {
                ltemp = 0i64
            }
            (*ptr).cur_start_row = ltemp as crate::jmorecfg_h::JDIMENSION
        }
        /* Read in the selected part of the array.
         * During the initial write pass, we will do no actual read
         * because the selected part is all undefined.
         */
        do_sarray_io(cinfo, ptr, crate::jmorecfg_h::FALSE);
    }
    /* Ensure the accessed part of the array is defined; prezero if needed.
     * To improve locality of access, we only prezero the part of the array
     * that the caller is about to access, not the entire in-memory array.
     */
    if (*ptr).first_undef_row < end_row {
        if (*ptr).first_undef_row < start_row {
            if writable != 0 {
                /* writer skipped over a section of array */
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_BAD_VIRTUAL_ACCESS as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
            }
            undef_row = start_row
        /* but reader is allowed to read ahead */
        } else {
            undef_row = (*ptr).first_undef_row
        } /* make indexes relative to buffer */
        if writable != 0 {
            (*ptr).first_undef_row = end_row
        }
        if (*ptr).pre_zero != 0 {
            let mut bytesperrow: crate::stddef_h::size_t = (*ptr).samplesperrow
                as crate::stddef_h::size_t *
    ::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>() as libc::c_ulong;
            undef_row = undef_row - (*ptr).cur_start_row;
            end_row = end_row - (*ptr).cur_start_row;
            while undef_row < end_row {
                crate::jpegint_h::jzero_far(
                    *(*ptr).mem_buffer.offset(undef_row as isize) as *mut libc::c_void,
                    bytesperrow,
                );
                undef_row =  undef_row + 1
            }
        } else if writable == 0 {
            /* reader looking at undefined data */
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_VIRTUAL_ACCESS as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
        }
    }
    /* Flag the buffer dirty if caller will write in it */
    if writable != 0 {
        (*ptr).dirty = crate::jmorecfg_h::TRUE
    }
    /* Return address of proper part of the buffer */
    return (*ptr)
        .mem_buffer
        .offset((start_row - (*ptr).cur_start_row) as isize);
}

unsafe extern "C" fn access_virt_barray(
    mut cinfo: crate::jpeglib_h::j_common_ptr,
    mut ptr: crate::jpeglib_h::jvirt_barray_ptr,
    mut start_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: crate::jmorecfg_h::JDIMENSION,
    mut writable: crate::jmorecfg_h::boolean,
) -> crate::jpeglib_h::JBLOCKARRAY
/* Access the part of a virtual block array starting at start_row */
/* and extending for num_rows rows.  writable is true if  */
/* caller intends to modify the accessed area. */ {
    let mut end_row: crate::jmorecfg_h::JDIMENSION =  start_row + num_rows;
    let mut undef_row: crate::jmorecfg_h::JDIMENSION = 0;
    /* debugging check */
    if end_row > (*ptr).rows_in_array || num_rows > (*ptr).maxaccess || (*ptr).mem_buffer.is_null()
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_VIRTUAL_ACCESS as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    /* Make the desired part of the virtual array accessible */
    if start_row < (*ptr).cur_start_row
        || end_row >  (*ptr).cur_start_row + (*ptr).rows_in_mem
    {
        if (*ptr).b_s_open == 0 {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_VIRTUAL_BUG as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
        }
        /* Flush old buffer contents if necessary */
        if (*ptr).dirty != 0 {
            do_barray_io(cinfo, ptr, crate::jmorecfg_h::TRUE);
            (*ptr).dirty = crate::jmorecfg_h::FALSE
        }
        /* Decide what part of virtual array to access.
         * Algorithm: if target address > current window, assume forward scan,
         * load starting at target address.  If target address < current window,
         * assume backward scan, load so that target area is top of window.
         * Note that when switching from forward write to forward read, will have
         * start_row = 0, so the limiting case applies and we load from 0 anyway.
         */
        if start_row > (*ptr).cur_start_row {
            (*ptr).cur_start_row = start_row
        } else {
            /* use long arithmetic here to avoid overflow & unsigned problems */
            let mut ltemp: libc::c_long = 0; /* don't fall off front end of file */
            ltemp = end_row as libc::c_long - (*ptr).rows_in_mem as libc::c_long;
            if ltemp < 0i64 {
                ltemp = 0i64
            }
            (*ptr).cur_start_row = ltemp as crate::jmorecfg_h::JDIMENSION
        }
        /* Read in the selected part of the array.
         * During the initial write pass, we will do no actual read
         * because the selected part is all undefined.
         */
        do_barray_io(cinfo, ptr, crate::jmorecfg_h::FALSE);
    }
    /* Ensure the accessed part of the array is defined; prezero if needed.
     * To improve locality of access, we only prezero the part of the array
     * that the caller is about to access, not the entire in-memory array.
     */
    if (*ptr).first_undef_row < end_row {
        if (*ptr).first_undef_row < start_row {
            if writable != 0 {
                /* writer skipped over a section of array */
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_BAD_VIRTUAL_ACCESS as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
            }
            undef_row = start_row
        /* but reader is allowed to read ahead */
        } else {
            undef_row = (*ptr).first_undef_row
        } /* make indexes relative to buffer */
        if writable != 0 {
            (*ptr).first_undef_row = end_row
        }
        if (*ptr).pre_zero != 0 {
            let mut bytesperrow: crate::stddef_h::size_t = (*ptr).blocksperrow
                as crate::stddef_h::size_t *
    ::std::mem::size_of::<crate::jpeglib_h::JBLOCK>() as libc::c_ulong;
            undef_row = undef_row - (*ptr).cur_start_row;
            end_row = end_row - (*ptr).cur_start_row;
            while undef_row < end_row {
                crate::jpegint_h::jzero_far(
                    *(*ptr).mem_buffer.offset(undef_row as isize) as *mut libc::c_void,
                    bytesperrow,
                );
                undef_row =  undef_row + 1
            }
        } else if writable == 0 {
            /* reader looking at undefined data */
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_VIRTUAL_ACCESS as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
        }
    }
    /* Flag the buffer dirty if caller will write in it */
    if writable != 0 {
        (*ptr).dirty = crate::jmorecfg_h::TRUE
    }
    /* Return address of proper part of the buffer */
    return (*ptr)
        .mem_buffer
        .offset((start_row - (*ptr).cur_start_row) as isize);
}
/*
 * Release all objects belonging to a specified pool.
 */

unsafe extern "C" fn free_pool(
    mut cinfo: crate::jpeglib_h::j_common_ptr,
    mut pool_id: libc::c_int,
) {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr; /* safety check */
    let mut shdr_ptr: small_pool_ptr = ::std::ptr::null_mut::< small_pool_struct>();
    let mut lhdr_ptr: large_pool_ptr = ::std::ptr::null_mut::< large_pool_struct>();
    let mut space_freed: crate::stddef_h::size_t = 0;
    if pool_id < 0i32 || pool_id >= crate::jpeglib_h::JPOOL_NUMPOOLS {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_POOL_ID as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = pool_id;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    /* If freeing IMAGE pool, close any virtual arrays first */
    if pool_id == crate::jpeglib_h::JPOOL_IMAGE {
        let mut sptr: crate::jpeglib_h::jvirt_sarray_ptr =
            ::std::ptr::null_mut::< crate::jpeglib_h::jvirt_sarray_control>();
        let mut bptr: crate::jpeglib_h::jvirt_barray_ptr =
            ::std::ptr::null_mut::< crate::jpeglib_h::jvirt_barray_control>();
        sptr = (*mem).virt_sarray_list;
        while !sptr.is_null() {
            if (*sptr).b_s_open != 0 {
                /* there may be no backing store */
                (*sptr).b_s_open = crate::jmorecfg_h::FALSE; /* prevent recursive close if error */
                Some(
                    (*sptr)
                        .b_s_info
                        .close_backing_store
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo, &mut (*sptr).b_s_info);
            }
            sptr = (*sptr).next
        }
        (*mem).virt_sarray_list = crate::stddef_h::NULL as crate::jpeglib_h::jvirt_sarray_ptr;
        bptr = (*mem).virt_barray_list;
        while !bptr.is_null() {
            if (*bptr).b_s_open != 0 {
                /* there may be no backing store */
                (*bptr).b_s_open = crate::jmorecfg_h::FALSE; /* prevent recursive close if error */
                Some(
                    (*bptr)
                        .b_s_info
                        .close_backing_store
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo, &mut (*bptr).b_s_info);
            }
            bptr = (*bptr).next
        }
        (*mem).virt_barray_list = crate::stddef_h::NULL as crate::jpeglib_h::jvirt_barray_ptr
    }
    /* Release large objects */
    lhdr_ptr = (*mem).large_list[pool_id as usize];
    (*mem).large_list[pool_id as usize] = crate::stddef_h::NULL as large_pool_ptr;
    while !lhdr_ptr.is_null() {
        let mut next_lhdr_ptr: large_pool_ptr = (*lhdr_ptr).next;
        space_freed =  (*lhdr_ptr)
            .bytes_used + (*lhdr_ptr).bytes_left +
    ::std::mem::size_of::<large_pool_hdr>() as libc::c_ulong;
        crate::jmemsys_h::jpeg_free_large(cinfo, lhdr_ptr as *mut libc::c_void, space_freed);
        (*mem).total_space_allocated =
            (*mem).total_space_allocated - space_freed;
        lhdr_ptr = next_lhdr_ptr
    }
    /* Release small objects */
    shdr_ptr = (*mem).small_list[pool_id as usize];
    (*mem).small_list[pool_id as usize] = crate::stddef_h::NULL as small_pool_ptr;
    while !shdr_ptr.is_null() {
        let mut next_shdr_ptr: small_pool_ptr = (*shdr_ptr).next;
        space_freed =  (*shdr_ptr)
            .bytes_used + (*shdr_ptr).bytes_left +
    ::std::mem::size_of::<small_pool_hdr>() as libc::c_ulong;
        crate::jmemsys_h::jpeg_free_small(cinfo, shdr_ptr as *mut libc::c_void, space_freed);
        (*mem).total_space_allocated =
            (*mem).total_space_allocated - space_freed;
        shdr_ptr = next_shdr_ptr
    }
}
/*
 * Close up shop entirely.
 * Note that this cannot be called unless cinfo->mem is non-NULL.
 */

unsafe extern "C" fn self_destruct(mut cinfo: crate::jpeglib_h::j_common_ptr) {
    let mut pool: libc::c_int = 0;
    /* Close all backing store, release all memory.
     * Releasing pools in reverse order might help avoid fragmentation
     * with some (brain-damaged) malloc libraries.
     */
    pool = crate::jpeglib_h::JPOOL_NUMPOOLS - 1i32;
    while pool >= crate::jpeglib_h::JPOOL_PERMANENT {
        free_pool(cinfo, pool);
        pool -= 1
    }
    /* Release the memory manager control block too. */
    crate::jmemsys_h::jpeg_free_small(
        cinfo,
        (*cinfo).mem as *mut libc::c_void,
        ::std::mem::size_of::<my_memory_mgr>() as libc::c_ulong,
    ); /* ensures I will be called only once */
    (*cinfo).mem = crate::stddef_h::NULL as *mut crate::jpeglib_h::jpeg_memory_mgr;
    crate::jmemsys_h::jpeg_mem_term(cinfo);
    /* system-dependent cleanup */
}
/* Entropy encoding */
/* Marker writing */
/* These routines are exported to allow insertion of extra markers */
/* Probably only COM and APPn markers should be written this way */
/* Declarations for decompression modules */
/* Master control module */
/* State variables made visible to other modules */
/* True during 1st pass for 2-pass quant */
/* Partial decompression variables */
/* Input control module */
/* State variables made visible to other modules */
/* True if file has multiple scans */
/* True when EOI has been consumed */
/* Main buffer control (downsampled-data buffer) */
/* Coefficient buffer control */
/* Pointer to array of coefficient virtual arrays, or NULL if none */
/* Decompression postprocessing (color quantization buffer control) */
/* Marker reading & parsing */
/* Read markers until SOS or EOI.
 * Returns same codes as are defined for jpeg_consume_input:
 * JPEG_SUSPENDED, JPEG_REACHED_SOS, or JPEG_REACHED_EOI.
 */
/* Read a restart marker --- exported for use by entropy decoder only */
/* State of marker reader --- nominally internal, but applications
 * supplying COM or APPn handlers might like to know the state.
 */
/* found SOI? */
/* found SOF? */
/* next restart number expected (0-7) */
/* # of bytes skipped looking for a marker */
/* Entropy decoding */
/* This is here to share code between baseline and progressive decoders; */
/* other modules probably should not use it */
/* set TRUE after emitting warning */
/* Inverse DCT (also performs dequantization) */
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
/*
 * Memory manager initialization.
 * When this is called, only the error manager pointer is valid in cinfo!
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_memory_mgr(mut cinfo: crate::jpeglib_h::j_common_ptr) {
    let mut mem: my_mem_ptr = ::std::ptr::null_mut::< my_memory_mgr>(); /* for safety if init fails */
    let mut max_to_use: libc::c_long = 0;
    let mut pool: libc::c_int = 0;
    let mut test_mac: crate::stddef_h::size_t = 0;
    (*cinfo).mem = crate::stddef_h::NULL as *mut crate::jpeglib_h::jpeg_memory_mgr;
    /* Check for configuration errors.
     * sizeof(ALIGN_TYPE) should be a power of 2; otherwise, it probably
     * doesn't reflect any real hardware alignment requirement.
     * The test is a little tricky: for X>0, X and X-1 have no one-bits
     * in common if and only if X is a power of 2, ie has only one one-bit.
     * Some compilers may give an "unreachable code" warning here; ignore it.
     */
    if ALIGN_SIZE & ALIGN_SIZE - 1i32 != 0i32 {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_ALIGN_TYPE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    /* MAX_ALLOC_CHUNK must be representable as type size_t, and must be
     * a multiple of ALIGN_SIZE.
     * Again, an "unreachable code" warning may be ignored here.
     * But a "constant too large" warning means you need to fix MAX_ALLOC_CHUNK.
     */
    test_mac = crate::jmemsys_h::MAX_ALLOC_CHUNK as crate::stddef_h::size_t; /* system-dependent initialization */
    if test_mac as libc::c_long != crate::jmemsys_h::MAX_ALLOC_CHUNK
        || crate::jmemsys_h::MAX_ALLOC_CHUNK % ALIGN_SIZE as libc::c_long != 0i64
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_ALLOC_CHUNK as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    max_to_use = crate::jmemsys_h::jpeg_mem_init(cinfo);
    /* Attempt to allocate memory manager's control block */
    mem = crate::jmemsys_h::jpeg_get_small(
        cinfo,
        ::std::mem::size_of::<my_memory_mgr>() as libc::c_ulong,
    ) as my_mem_ptr; /* system-dependent cleanup */
    if mem.is_null() {
        crate::jmemsys_h::jpeg_mem_term(cinfo);
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_OUT_OF_MEMORY as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = 0i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    /* OK, fill in the method pointers */
    (*mem).pub_0.alloc_small = Some(
        alloc_small
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_common_ptr,
                _: libc::c_int,
                _: crate::stddef_h::size_t,
            ) -> *mut libc::c_void,
    );
    (*mem).pub_0.alloc_large = Some(
        alloc_large
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_common_ptr,
                _: libc::c_int,
                _: crate::stddef_h::size_t,
            ) -> *mut libc::c_void,
    );
    (*mem).pub_0.alloc_sarray = Some(
        alloc_sarray
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_common_ptr,
                _: libc::c_int,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::JDIMENSION,
            ) -> crate::jpeglib_h::JSAMPARRAY,
    );
    (*mem).pub_0.alloc_barray = Some(
        alloc_barray
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_common_ptr,
                _: libc::c_int,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::JDIMENSION,
            ) -> crate::jpeglib_h::JBLOCKARRAY,
    );
    (*mem).pub_0.request_virt_sarray = Some(
        request_virt_sarray
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_common_ptr,
                _: libc::c_int,
                _: crate::jmorecfg_h::boolean,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::JDIMENSION,
            ) -> crate::jpeglib_h::jvirt_sarray_ptr,
    );
    (*mem).pub_0.request_virt_barray = Some(
        request_virt_barray
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_common_ptr,
                _: libc::c_int,
                _: crate::jmorecfg_h::boolean,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::JDIMENSION,
            ) -> crate::jpeglib_h::jvirt_barray_ptr,
    );
    (*mem).pub_0.realize_virt_arrays =
        Some(realize_virt_arrays as unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ());
    (*mem).pub_0.access_virt_sarray = Some(
        access_virt_sarray
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_common_ptr,
                _: crate::jpeglib_h::jvirt_sarray_ptr,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::boolean,
            ) -> crate::jpeglib_h::JSAMPARRAY,
    );
    (*mem).pub_0.access_virt_barray = Some(
        access_virt_barray
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_common_ptr,
                _: crate::jpeglib_h::jvirt_barray_ptr,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jmorecfg_h::boolean,
            ) -> crate::jpeglib_h::JBLOCKARRAY,
    );
    (*mem).pub_0.free_pool = Some(
        free_pool as unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr, _: libc::c_int) -> (),
    );
    (*mem).pub_0.self_destruct =
        Some(self_destruct as unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ());
    /* Make MAX_ALLOC_CHUNK accessible to other modules */
    (*mem).pub_0.max_alloc_chunk = crate::jmemsys_h::MAX_ALLOC_CHUNK;
    /* Initialize working state */
    (*mem).pub_0.max_memory_to_use = max_to_use;
    pool = crate::jpeglib_h::JPOOL_NUMPOOLS - 1i32;
    while pool >= crate::jpeglib_h::JPOOL_PERMANENT {
        (*mem).small_list[pool as usize] = crate::stddef_h::NULL as small_pool_ptr;
        (*mem).large_list[pool as usize] = crate::stddef_h::NULL as large_pool_ptr;
        pool -= 1
    }
    (*mem).virt_sarray_list = crate::stddef_h::NULL as crate::jpeglib_h::jvirt_sarray_ptr;
    (*mem).virt_barray_list = crate::stddef_h::NULL as crate::jpeglib_h::jvirt_barray_ptr;
    (*mem).total_space_allocated = ::std::mem::size_of::<my_memory_mgr>() as libc::c_ulong;
    /* Declare ourselves open for business */
    (*cinfo).mem = &mut (*mem).pub_0;
    /* Check for an environment variable JPEGMEM; if found, override the
     * default max_memory setting from jpeg_mem_init.  Note that the
     * surrounding application may again override this value.
     * If your system doesn't support getenv(), define NO_GETENV to disable
     * this feature.
     */
    let mut memenv: *mut libc::c_char = ::std::ptr::null_mut::< libc::c_char>();
    memenv = crate::stdlib::getenv(b"JPEGMEM\x00".as_ptr() as *const libc::c_char);
    if !memenv.is_null() {
        let mut ch: libc::c_char =  'x' as libc::c_char;
        if crate::stdlib::sscanf(
            memenv,
            
            b"%ld%c\x00".as_ptr() as *const libc::c_char,
            &mut max_to_use as *mut libc::c_long,
            &mut ch as *mut libc::c_char,
        ) > 0i32
        {
            if ch as libc::c_int == 'm' as i32 || ch as libc::c_int == 'M' as i32 {
                max_to_use *= 1000i64
            }
            (*mem).pub_0.max_memory_to_use = max_to_use * 1000i64
        }
    };
}
