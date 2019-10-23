pub use super::jerror::{
    C2RustUnnamed_3, JERR_ARITH_NOTIMPL, JERR_BAD_ALIGN_TYPE, JERR_BAD_ALLOC_CHUNK,
    JERR_BAD_BUFFER_MODE, JERR_BAD_COMPONENT_ID, JERR_BAD_CROP_SPEC, JERR_BAD_DCTSIZE,
    JERR_BAD_DCT_COEF, JERR_BAD_HUFF_TABLE, JERR_BAD_IN_COLORSPACE, JERR_BAD_J_COLORSPACE,
    JERR_BAD_LENGTH, JERR_BAD_LIB_VERSION, JERR_BAD_MCU_SIZE, JERR_BAD_PARAM, JERR_BAD_PARAM_VALUE,
    JERR_BAD_POOL_ID, JERR_BAD_PRECISION, JERR_BAD_PROGRESSION, JERR_BAD_PROG_SCRIPT,
    JERR_BAD_SAMPLING, JERR_BAD_SCAN_SCRIPT, JERR_BAD_STATE, JERR_BAD_STRUCT_SIZE,
    JERR_BAD_VIRTUAL_ACCESS, JERR_BUFFER_SIZE, JERR_CANT_SUSPEND, JERR_CCIR601_NOTIMPL,
    JERR_COMPONENT_COUNT, JERR_CONVERSION_NOTIMPL, JERR_DAC_INDEX, JERR_DAC_VALUE, JERR_DHT_INDEX,
    JERR_DQT_INDEX, JERR_EMPTY_IMAGE, JERR_EMS_READ, JERR_EMS_WRITE, JERR_EOI_EXPECTED,
    JERR_FILE_READ, JERR_FILE_WRITE, JERR_FRACT_SAMPLE_NOTIMPL, JERR_HUFF_CLEN_OVERFLOW,
    JERR_HUFF_MISSING_CODE, JERR_IMAGE_TOO_BIG, JERR_INPUT_EMPTY, JERR_INPUT_EOF,
    JERR_MISMATCHED_QUANT_TABLE, JERR_MISSING_DATA, JERR_MODE_CHANGE, JERR_NOTIMPL,
    JERR_NOT_COMPILED, JERR_NO_BACKING_STORE, JERR_NO_HUFF_TABLE, JERR_NO_IMAGE,
    JERR_NO_QUANT_TABLE, JERR_NO_SOI, JERR_OUT_OF_MEMORY, JERR_QUANT_COMPONENTS,
    JERR_QUANT_FEW_COLORS, JERR_QUANT_MANY_COLORS, JERR_SOF_DUPLICATE, JERR_SOF_NO_SOS,
    JERR_SOF_UNSUPPORTED, JERR_SOI_DUPLICATE, JERR_SOS_NO_SOF, JERR_TFILE_CREATE, JERR_TFILE_READ,
    JERR_TFILE_SEEK, JERR_TFILE_WRITE, JERR_TOO_LITTLE_DATA, JERR_UNKNOWN_MARKER,
    JERR_UNSUPPORTED_SUSPEND, JERR_VIRTUAL_BUG, JERR_WIDTH_OVERFLOW, JERR_XMS_READ, JERR_XMS_WRITE,
    JMSG_COPYRIGHT, JMSG_LASTMSGCODE, JMSG_NOMESSAGE, JMSG_VERSION, JTRC_16BIT_TABLES, JTRC_ADOBE,
    JTRC_APP0, JTRC_APP14, JTRC_DAC, JTRC_DHT, JTRC_DQT, JTRC_DRI, JTRC_EMS_CLOSE, JTRC_EMS_OPEN,
    JTRC_EOI, JTRC_HUFFBITS, JTRC_JFIF, JTRC_JFIF_BADTHUMBNAILSIZE, JTRC_JFIF_EXTENSION,
    JTRC_JFIF_THUMBNAIL, JTRC_MISC_MARKER, JTRC_PARMLESS_MARKER, JTRC_QUANTVALS,
    JTRC_QUANT_3_NCOLORS, JTRC_QUANT_NCOLORS, JTRC_QUANT_SELECTED, JTRC_RECOVERY_ACTION, JTRC_RST,
    JTRC_SMOOTH_NOTIMPL, JTRC_SOF, JTRC_SOF_COMPONENT, JTRC_SOI, JTRC_SOS, JTRC_SOS_COMPONENT,
    JTRC_SOS_PARAMS, JTRC_TFILE_CLOSE, JTRC_TFILE_OPEN, JTRC_THUMB_JPEG, JTRC_THUMB_PALETTE,
    JTRC_THUMB_RGB, JTRC_UNKNOWN_IDS, JTRC_XMS_CLOSE, JTRC_XMS_OPEN, JWRN_ADOBE_XFORM,
    JWRN_BOGUS_ICC, JWRN_BOGUS_PROGRESSION, JWRN_EXTRANEOUS_DATA, JWRN_HIT_MARKER,
    JWRN_HUFF_BAD_CODE, JWRN_JFIF_MAJOR, JWRN_JPEG_EOF, JWRN_MUST_RESYNC, JWRN_NOT_SEQUENTIAL,
    JWRN_TOO_MUCH_DATA,
};
pub use crate::jmemsys_h::{
    backing_store_info, backing_store_ptr, backing_store_struct, jpeg_free_large, jpeg_free_small,
    jpeg_get_large, jpeg_get_small, jpeg_mem_available, jpeg_mem_init, jpeg_mem_term,
    jpeg_open_backing_store, MAX_ALLOC_CHUNK,
};
pub use crate::jmorecfg_h::{boolean, FALSE, JCOEF, JDIMENSION, JSAMPLE, TRUE};
use crate::jpegint_h::jzero_far;
pub use crate::jpeglib_h::{
    j_common_ptr, jpeg_common_struct, jpeg_error_mgr, jpeg_memory_mgr, jpeg_progress_mgr,
    jvirt_barray_control, jvirt_barray_ptr, jvirt_sarray_control, jvirt_sarray_ptr,
    C2RustUnnamed_2, JBLOCK, JBLOCKARRAY, JBLOCKROW, JPOOL_IMAGE, JPOOL_NUMPOOLS, JPOOL_PERMANENT,
    JSAMPARRAY, JSAMPROW,
};
pub use crate::stddef_h::{size_t, NULL};
pub use crate::stdlib::{
    _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, __off64_t, __off_t, FILE, SIZE_MAX,
    _IO_FILE,
};
use crate::stdlib::{getenv, sscanf};
use libc::{self, c_char, c_int, c_long, c_ulong, c_void};

pub type my_mem_ptr = *mut my_memory_mgr;

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

unsafe extern "C" fn round_up_pow2(mut a: size_t, mut b: size_t) -> size_t {
    return a + b - 1u64 & !(b - 1u64);
}

pub const ALIGN_SIZE: c_int = 32i32;
/* System-dependent control info */
/* optional extra stuff for statistics */
/* MEM_STATS */

unsafe extern "C" fn out_of_memory(mut cinfo: j_common_ptr, mut which: c_int)
/* Report an out-of-memory error and stop execution */
/* If we compiled MEM_STATS support, report alloc requests before dying */
{
    (*(*cinfo).err).msg_code = super::jerror::JERR_OUT_OF_MEMORY as c_int;
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

static mut first_pool_slop: [size_t; 2] = [1600u64, 16000u64];

static mut extra_pool_slop: [size_t; 2] = [0u64, 5000u64];

pub const MIN_SLOP: c_int = 50i32;
/* greater than 0 to avoid futile looping */

unsafe extern "C" fn alloc_small(
    mut cinfo: j_common_ptr,
    mut pool_id: c_int,
    mut sizeofobject: size_t,
) -> *mut c_void
/* Allocate a "small" object */ {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;

    /*
     * Round up the requested size to a multiple of ALIGN_SIZE in order
     * to assure alignment for the next object allocated in the same pool
     * and so that algorithms can straddle outside the proper area up
     * to the next alignment.
     */
    if sizeofobject > MAX_ALLOC_CHUNK as c_ulong {
        /* This prevents overflow/wrap-around in round_up_pow2() if sizeofobject
        is close to SIZE_MAX. */
        out_of_memory(cinfo, 7i32);
    }
    sizeofobject = round_up_pow2(sizeofobject, ALIGN_SIZE as size_t);
    /* Check for unsatisfiable request (do now to ensure no overflow below) */
    if ::std::mem::size_of::<small_pool_hdr>() as c_ulong + sizeofobject + ALIGN_SIZE as c_ulong
        - 1u64
        > MAX_ALLOC_CHUNK as c_ulong
    {
        out_of_memory(cinfo, 1i32); /* request exceeds malloc's ability */
    }
    /* See if space is available in any existing pool */
    if pool_id < 0i32 || pool_id >= JPOOL_NUMPOOLS {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_POOL_ID as c_int; /* safety check */
        (*(*cinfo).err).msg_parm.i[0] = pool_id; /* found pool with enough space */
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }

    let mut prev_hdr_ptr: small_pool_ptr = NULL as small_pool_ptr;
    let mut hdr_ptr: small_pool_ptr = (*mem).small_list[pool_id as usize];
    while !hdr_ptr.is_null() {
        if (*hdr_ptr).bytes_left >= sizeofobject {
            break;
        }
        prev_hdr_ptr = hdr_ptr;
        hdr_ptr = (*hdr_ptr).next
    }
    /* Time to make a new pool? */
    if hdr_ptr.is_null() {
        let mut slop: size_t = 0;
        let mut min_request: size_t = ::std::mem::size_of::<small_pool_hdr>() as c_ulong
            + sizeofobject
            + ALIGN_SIZE as c_ulong
            - 1u64;
        if prev_hdr_ptr.is_null() {
            /* first pool in class? */
            slop = first_pool_slop[pool_id as usize]
        } else {
            slop = extra_pool_slop[pool_id as usize]
        }
        /* Don't ask for more than MAX_ALLOC_CHUNK */
        if slop > MAX_ALLOC_CHUNK as c_ulong - min_request {
            slop = MAX_ALLOC_CHUNK as c_ulong - min_request
        }
        loop
        /* Try to get space, if fail reduce slop and try again */
        {
            hdr_ptr = jpeg_get_small(cinfo, min_request + slop) as small_pool_ptr;
            if !hdr_ptr.is_null() {
                break;
            }
            slop = slop / 2u64;
            if slop < MIN_SLOP as c_ulong {
                /* give up when it gets real small */
                out_of_memory(cinfo, 2i32);
            }
        }
        (*mem).total_space_allocated = (*mem).total_space_allocated + (min_request + slop);
        /* Success, initialize the new pool header and add to end of list */
        (*hdr_ptr).next = NULL as small_pool_ptr;
        (*hdr_ptr).bytes_used = 0u64;
        (*hdr_ptr).bytes_left = sizeofobject + slop;
        if prev_hdr_ptr.is_null() {
            /* first pool in class? */
            (*mem).small_list[pool_id as usize] = hdr_ptr
        } else {
            (*prev_hdr_ptr).next = hdr_ptr
        }
    }
    let mut data_ptr: *mut c_char = hdr_ptr as *mut c_char; /* point to first data byte in pool... */
    data_ptr = data_ptr.offset(::std::mem::size_of::<small_pool_hdr>() as isize); /* ...by skipping the header... */
    if data_ptr as size_t % ALIGN_SIZE as c_ulong != 0 {
        /* ...and adjust for alignment */
        data_ptr = data_ptr
            .offset((ALIGN_SIZE as c_ulong - data_ptr as size_t % ALIGN_SIZE as c_ulong) as isize)
    } /* point to place for object */
    data_ptr = data_ptr.offset((*hdr_ptr).bytes_used as isize);
    (*hdr_ptr).bytes_used = (*hdr_ptr).bytes_used + sizeofobject;
    (*hdr_ptr).bytes_left = (*hdr_ptr).bytes_left - sizeofobject;
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
) -> *mut c_void
/* Allocate a "large" object */ {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;

    /*
     * Round up the requested size to a multiple of ALIGN_SIZE so that
     * algorithms can straddle outside the proper area up to the next
     * alignment.
     */
    if sizeofobject > MAX_ALLOC_CHUNK as c_ulong {
        /* This prevents overflow/wrap-around in round_up_pow2() if sizeofobject
        is close to SIZE_MAX. */
        out_of_memory(cinfo, 8i32);
    }
    sizeofobject = round_up_pow2(sizeofobject, ALIGN_SIZE as size_t);
    /* Check for unsatisfiable request (do now to ensure no overflow below) */
    if ::std::mem::size_of::<large_pool_hdr>() as c_ulong + sizeofobject + ALIGN_SIZE as c_ulong
        - 1u64
        > MAX_ALLOC_CHUNK as c_ulong
    {
        out_of_memory(cinfo, 3i32); /* request exceeds malloc's ability */
    }
    /* Always make a new pool */
    if pool_id < 0i32 || pool_id >= JPOOL_NUMPOOLS {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_POOL_ID as c_int; /* safety check */
        (*(*cinfo).err).msg_parm.i[0] = pool_id; /* jpeg_get_large failed */
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    let mut hdr_ptr: large_pool_ptr = jpeg_get_large(
        cinfo,
        sizeofobject + ::std::mem::size_of::<large_pool_hdr>() as c_ulong + ALIGN_SIZE as c_ulong
            - 1u64,
    ) as large_pool_ptr;
    if hdr_ptr.is_null() {
        out_of_memory(cinfo, 4i32);
    }
    (*mem).total_space_allocated = (*mem).total_space_allocated
        + (sizeofobject
            + ::std::mem::size_of::<large_pool_hdr>() as c_ulong
            + ALIGN_SIZE as c_ulong
            - 1u64);
    /* Success, initialize the new pool header and add to list */
    (*hdr_ptr).next = (*mem).large_list[pool_id as usize];
    /* We maintain space counts in each pool header for statistical purposes,
     * even though they are not needed for allocation.
     */
    (*hdr_ptr).bytes_used = sizeofobject; /* point to first data byte in pool... */
    (*hdr_ptr).bytes_left = 0u64; /* ...by skipping the header... */
    (*mem).large_list[pool_id as usize] = hdr_ptr;
    let mut data_ptr: *mut c_char = hdr_ptr as *mut c_char;
    data_ptr = data_ptr.offset(::std::mem::size_of::<small_pool_hdr>() as isize);
    if data_ptr as size_t % ALIGN_SIZE as c_ulong != 0 {
        /* ...and adjust for alignment */
        data_ptr = data_ptr
            .offset((ALIGN_SIZE as c_ulong - data_ptr as size_t % ALIGN_SIZE as c_ulong) as isize)
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
) -> JSAMPARRAY
/* Allocate a 2-D sample array */ {
    let mut rowsperchunk: JDIMENSION = 0;
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;

    /* Make sure each row is properly aligned */
    if ALIGN_SIZE as c_ulong % ::std::mem::size_of::<JSAMPLE>() as c_ulong != 0u64 {
        out_of_memory(cinfo, 5i32); /* safety check */
    }
    if samplesperrow as c_long > MAX_ALLOC_CHUNK {
        /* This prevents overflow/wrap-around in round_up_pow2() if sizeofobject
        is close to SIZE_MAX. */
        out_of_memory(cinfo, 9i32);
    }
    samplesperrow = round_up_pow2(
        samplesperrow as size_t,
        (2i32 * ALIGN_SIZE) as c_ulong / ::std::mem::size_of::<JSAMPLE>() as c_ulong,
    ) as JDIMENSION;
    let mut ltemp: c_long = ((MAX_ALLOC_CHUNK as c_ulong
        - ::std::mem::size_of::<large_pool_hdr>() as c_ulong)
        / (samplesperrow as c_ulong * ::std::mem::size_of::<JSAMPLE>() as c_ulong))
        as c_long;
    if ltemp <= 0i64 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_WIDTH_OVERFLOW as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    if ltemp < numrows as c_long {
        rowsperchunk = ltemp as JDIMENSION
    } else {
        rowsperchunk = numrows
    }
    (*mem).last_rowsperchunk = rowsperchunk;

    let mut result: JSAMPARRAY = alloc_small(
        cinfo,
        pool_id,
        numrows as c_ulong * ::std::mem::size_of::<JSAMPROW>() as c_ulong,
    ) as JSAMPARRAY;
    let mut currow: JDIMENSION = 0u32;
    while currow < numrows {
        rowsperchunk = if rowsperchunk < numrows - currow {
            rowsperchunk
        } else {
            numrows - currow
        };

        let mut workspace: JSAMPROW = alloc_large(
            cinfo,
            pool_id,
            rowsperchunk as size_t
                * samplesperrow as size_t
                * ::std::mem::size_of::<JSAMPLE>() as c_ulong,
        ) as JSAMPROW;
        let mut i: JDIMENSION = rowsperchunk;
        while i > 0u32 {
            let fresh0 = currow;
            currow += 1;
            let ref mut fresh1 = *result.offset(fresh0 as isize);
            *fresh1 = workspace;
            workspace = workspace.offset(samplesperrow as isize);
            i -= 1
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
) -> JBLOCKARRAY
/* Allocate a 2-D coefficient-block array */ {
    let mut rowsperchunk: JDIMENSION = 0;
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;

    /* Make sure each row is properly aligned */
    if ::std::mem::size_of::<JBLOCK>() as c_ulong % ALIGN_SIZE as c_ulong != 0u64 {
        out_of_memory(cinfo, 6i32); /* safety check */
    }
    let mut ltemp: c_long = ((MAX_ALLOC_CHUNK as c_ulong
        - ::std::mem::size_of::<large_pool_hdr>() as c_ulong)
        / (blocksperrow as c_ulong * ::std::mem::size_of::<JBLOCK>() as c_ulong))
        as c_long;
    if ltemp <= 0i64 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_WIDTH_OVERFLOW as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    if ltemp < numrows as c_long {
        rowsperchunk = ltemp as JDIMENSION
    } else {
        rowsperchunk = numrows
    }
    (*mem).last_rowsperchunk = rowsperchunk;

    let mut result: JBLOCKARRAY = alloc_small(
        cinfo,
        pool_id,
        numrows as c_ulong * ::std::mem::size_of::<JBLOCKROW>() as c_ulong,
    ) as JBLOCKARRAY;
    let mut currow: JDIMENSION = 0u32;
    while currow < numrows {
        rowsperchunk = if rowsperchunk < numrows - currow {
            rowsperchunk
        } else {
            numrows - currow
        };

        let mut workspace: JBLOCKROW = alloc_large(
            cinfo,
            pool_id,
            rowsperchunk as size_t
                * blocksperrow as size_t
                * ::std::mem::size_of::<JBLOCK>() as c_ulong,
        ) as JBLOCKROW;
        let mut i: JDIMENSION = rowsperchunk;
        while i > 0u32 {
            let fresh2 = currow;
            currow += 1;
            let ref mut fresh3 = *result.offset(fresh2 as isize);
            *fresh3 = workspace;
            workspace = workspace.offset(blocksperrow as isize);
            i -= 1
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
) -> jvirt_sarray_ptr
/* Request a virtual 2-D sample array */ {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;

    /* Only IMAGE-lifetime virtual arrays are currently supported */
    if pool_id != JPOOL_IMAGE {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_POOL_ID as c_int; /* safety check */
        (*(*cinfo).err).msg_parm.i[0] = pool_id;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    let mut result: jvirt_sarray_ptr = alloc_small(
        cinfo,
        pool_id,
        ::std::mem::size_of::<jvirt_sarray_control>() as c_ulong,
    ) as jvirt_sarray_ptr; /* marks array not yet realized */
    (*result).mem_buffer = NULL as JSAMPARRAY; /* no associated backing-store object */
    (*result).rows_in_array = numrows; /* add to list of virtual arrays */
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
) -> jvirt_barray_ptr
/* Request a virtual 2-D coefficient-block array */ {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;

    /* Only IMAGE-lifetime virtual arrays are currently supported */
    if pool_id != JPOOL_IMAGE {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_POOL_ID as c_int; /* safety check */
        (*(*cinfo).err).msg_parm.i[0] = pool_id;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    let mut result: jvirt_barray_ptr = alloc_small(
        cinfo,
        pool_id,
        ::std::mem::size_of::<jvirt_barray_control>() as c_ulong,
    ) as jvirt_barray_ptr; /* marks array not yet realized */
    (*result).mem_buffer = NULL as JBLOCKARRAY; /* no associated backing-store object */
    (*result).rows_in_array = numrows; /* add to list of virtual arrays */
    (*result).blocksperrow = blocksperrow;
    (*result).maxaccess = maxaccess;
    (*result).pre_zero = pre_zero;
    (*result).b_s_open = FALSE;
    (*result).next = (*mem).virt_barray_list;
    (*mem).virt_barray_list = result;
    return result;
}

unsafe extern "C" fn realize_virt_arrays(mut cinfo: j_common_ptr)
/* Allocate the in-memory buffers for any unrealized virtual arrays */
{
    let mut minheights: size_t = 0;
    let mut max_minheights: size_t = 0;
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;

    /* Compute the minimum space needed (maxaccess rows in each buffer)
     * and the maximum space needed (full image height in each buffer).
     * These may be of use to the system-dependent jpeg_mem_available routine.
     */

    let mut space_per_minheight: size_t = 0u64;
    let mut maximum_space: size_t = 0u64;
    let mut sptr: jvirt_sarray_ptr = (*mem).virt_sarray_list;
    while !sptr.is_null() {
        if (*sptr).mem_buffer.is_null() {
            /* if not realized yet */
            let mut new_space: size_t =
                ((*sptr).rows_in_array as c_long * (*sptr).samplesperrow as c_long) as c_ulong
                    * ::std::mem::size_of::<JSAMPLE>() as c_ulong;
            space_per_minheight += ((*sptr).maxaccess as c_long * (*sptr).samplesperrow as c_long)
                as c_ulong
                * ::std::mem::size_of::<JSAMPLE>() as c_ulong;
            if SIZE_MAX - maximum_space < new_space {
                out_of_memory(cinfo, 10i32);
            }
            maximum_space += new_space
        }
        sptr = (*sptr).next
    }
    let mut bptr: jvirt_barray_ptr = (*mem).virt_barray_list;
    while !bptr.is_null() {
        if (*bptr).mem_buffer.is_null() {
            /* if not realized yet */
            let mut new_space_0: size_t =
                ((*bptr).rows_in_array as c_long * (*bptr).blocksperrow as c_long) as c_ulong
                    * ::std::mem::size_of::<JBLOCK>() as c_ulong; /* no unrealized arrays, no work */
            space_per_minheight += ((*bptr).maxaccess as c_long * (*bptr).blocksperrow as c_long)
                as c_ulong
                * ::std::mem::size_of::<JBLOCK>() as c_ulong;
            if SIZE_MAX - maximum_space < new_space_0 {
                out_of_memory(cinfo, 11i32);
            }
            maximum_space += new_space_0
        }
        bptr = (*bptr).next
    }
    if space_per_minheight <= 0u64 {
        return;
    }
    let mut avail_mem: size_t = jpeg_mem_available(
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
        max_minheights = avail_mem / space_per_minheight;
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
            minheights = (((*sptr).rows_in_array as c_long - 1i64) / (*sptr).maxaccess as c_long
                + 1i64) as size_t;
            if minheights <= max_minheights {
                /* This buffer fits in memory */
                (*sptr).rows_in_mem = (*sptr).rows_in_array
            } else {
                /* It doesn't fit in memory, create backing store. */
                (*sptr).rows_in_mem = (max_minheights * (*sptr).maxaccess as c_ulong) as JDIMENSION;
                jpeg_open_backing_store(
                    cinfo,
                    &mut (*sptr).b_s_info,
                    (*sptr).rows_in_array as c_long
                        * (*sptr).samplesperrow as c_long
                        * ::std::mem::size_of::<JSAMPLE>() as c_long,
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
            (*sptr).cur_start_row = 0u32;
            (*sptr).first_undef_row = 0u32;
            (*sptr).dirty = FALSE
        }
        sptr = (*sptr).next
    }
    bptr = (*mem).virt_barray_list;
    while !bptr.is_null() {
        if (*bptr).mem_buffer.is_null() {
            /* if not realized yet */
            minheights = (((*bptr).rows_in_array as c_long - 1i64) / (*bptr).maxaccess as c_long
                + 1i64) as size_t;
            if minheights <= max_minheights {
                /* This buffer fits in memory */
                (*bptr).rows_in_mem = (*bptr).rows_in_array
            } else {
                /* It doesn't fit in memory, create backing store. */
                (*bptr).rows_in_mem = (max_minheights * (*bptr).maxaccess as c_ulong) as JDIMENSION;
                jpeg_open_backing_store(
                    cinfo,
                    &mut (*bptr).b_s_info,
                    (*bptr).rows_in_array as c_long
                        * (*bptr).blocksperrow as c_long
                        * ::std::mem::size_of::<JBLOCK>() as c_long,
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
            (*bptr).cur_start_row = 0u32;
            (*bptr).first_undef_row = 0u32;
            (*bptr).dirty = FALSE
        }
        bptr = (*bptr).next
    }
}

unsafe extern "C" fn do_sarray_io(
    mut cinfo: j_common_ptr,
    mut ptr: jvirt_sarray_ptr,
    mut writing: boolean,
)
/* Do backing store read or write of a virtual sample array */
{
    let mut bytesperrow: c_long =
        ((*ptr).samplesperrow as c_ulong * ::std::mem::size_of::<JSAMPLE>() as c_ulong) as c_long;
    let mut file_offset: c_long = (*ptr).cur_start_row as c_long * bytesperrow;
    let mut i: c_long = 0i64;
    while i < (*ptr).rows_in_mem as c_long {
        let mut rows: c_long = if ((*ptr).rowsperchunk as c_long) < (*ptr).rows_in_mem as c_long - i
        {
            (*ptr).rowsperchunk as c_long
        } else {
            ((*ptr).rows_in_mem as c_long) - i
        };
        let mut thisrow: c_long = (*ptr).cur_start_row as c_long + i;
        rows = if rows < (*ptr).first_undef_row as c_long - thisrow {
            rows
        } else {
            ((*ptr).first_undef_row as c_long) - thisrow
        };
        /* Transfer no more than fits in file */
        rows = if rows < (*ptr).rows_in_array as c_long - thisrow {
            rows
        } else {
            ((*ptr).rows_in_array as c_long) - thisrow
        };
        if rows <= 0i64 {
            break;
        }
        let mut byte_count: c_long = rows * bytesperrow;
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
                *(*ptr).mem_buffer.offset(i as isize) as *mut c_void,
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
)
/* Do backing store read or write of a virtual coefficient-block array */
{
    let mut bytesperrow: c_long =
        ((*ptr).blocksperrow as c_ulong * ::std::mem::size_of::<JBLOCK>() as c_ulong) as c_long;
    let mut file_offset: c_long = (*ptr).cur_start_row as c_long * bytesperrow;
    let mut i: c_long = 0i64;
    while i < (*ptr).rows_in_mem as c_long {
        let mut rows: c_long = if ((*ptr).rowsperchunk as c_long) < (*ptr).rows_in_mem as c_long - i
        {
            (*ptr).rowsperchunk as c_long
        } else {
            ((*ptr).rows_in_mem as c_long) - i
        };
        let mut thisrow: c_long = (*ptr).cur_start_row as c_long + i;
        rows = if rows < (*ptr).first_undef_row as c_long - thisrow {
            rows
        } else {
            ((*ptr).first_undef_row as c_long) - thisrow
        };
        /* Transfer no more than fits in file */
        rows = if rows < (*ptr).rows_in_array as c_long - thisrow {
            rows
        } else {
            ((*ptr).rows_in_array as c_long) - thisrow
        };
        if rows <= 0i64 {
            break;
        }
        let mut byte_count: c_long = rows * bytesperrow;
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
                *(*ptr).mem_buffer.offset(i as isize) as *mut c_void,
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
) -> JSAMPARRAY
/* Access the part of a virtual sample array starting at start_row */
/* and extending for num_rows rows.  writable is true if  */
/* caller intends to modify the accessed area. */ {
    let mut end_row: JDIMENSION = start_row + num_rows;

    /* debugging check */
    if end_row > (*ptr).rows_in_array || num_rows > (*ptr).maxaccess || (*ptr).mem_buffer.is_null()
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_VIRTUAL_ACCESS as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    /* Make the desired part of the virtual array accessible */
    if start_row < (*ptr).cur_start_row || end_row > (*ptr).cur_start_row + (*ptr).rows_in_mem {
        if (*ptr).b_s_open == 0 {
            (*(*cinfo).err).msg_code = super::jerror::JERR_VIRTUAL_BUG as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
        }
        /* Flush old buffer contents if necessary */
        if (*ptr).dirty != 0 {
            do_sarray_io(cinfo, ptr, TRUE);
            (*ptr).dirty = FALSE
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
            /* don't fall off front end of file */
            let mut ltemp: c_long = end_row as c_long - (*ptr).rows_in_mem as c_long;
            if ltemp < 0i64 {
                ltemp = 0i64
            }
            (*ptr).cur_start_row = ltemp as JDIMENSION
        }
        /* Read in the selected part of the array.
         * During the initial write pass, we will do no actual read
         * because the selected part is all undefined.
         */
        do_sarray_io(cinfo, ptr, FALSE);
    }
    /* Ensure the accessed part of the array is defined; prezero if needed.
     * To improve locality of access, we only prezero the part of the array
     * that the caller is about to access, not the entire in-memory array.
     */
    if (*ptr).first_undef_row < end_row {
        let mut undef_row: JDIMENSION = 0;
        if (*ptr).first_undef_row < start_row {
            if writable != 0 {
                /* writer skipped over a section of array */
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_VIRTUAL_ACCESS as c_int;
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
            let mut bytesperrow: size_t =
                (*ptr).samplesperrow as size_t * ::std::mem::size_of::<JSAMPLE>() as c_ulong;
            undef_row -= (*ptr).cur_start_row;
            end_row -= (*ptr).cur_start_row;
            while undef_row < end_row {
                jzero_far(
                    *(*ptr).mem_buffer.offset(undef_row as isize) as *mut c_void,
                    bytesperrow,
                );
                undef_row += 1
            }
        } else if writable == 0 {
            /* reader looking at undefined data */
            (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_VIRTUAL_ACCESS as c_int;
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
        (*ptr).dirty = TRUE
    }
    /* Return address of proper part of the buffer */
    return (*ptr)
        .mem_buffer
        .offset((start_row - (*ptr).cur_start_row) as isize);
}

unsafe extern "C" fn access_virt_barray(
    mut cinfo: j_common_ptr,
    mut ptr: jvirt_barray_ptr,
    mut start_row: JDIMENSION,
    mut num_rows: JDIMENSION,
    mut writable: boolean,
) -> JBLOCKARRAY
/* Access the part of a virtual block array starting at start_row */
/* and extending for num_rows rows.  writable is true if  */
/* caller intends to modify the accessed area. */ {
    let mut end_row: JDIMENSION = start_row + num_rows;

    /* debugging check */
    if end_row > (*ptr).rows_in_array || num_rows > (*ptr).maxaccess || (*ptr).mem_buffer.is_null()
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_VIRTUAL_ACCESS as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    /* Make the desired part of the virtual array accessible */
    if start_row < (*ptr).cur_start_row || end_row > (*ptr).cur_start_row + (*ptr).rows_in_mem {
        if (*ptr).b_s_open == 0 {
            (*(*cinfo).err).msg_code = super::jerror::JERR_VIRTUAL_BUG as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
        }
        /* Flush old buffer contents if necessary */
        if (*ptr).dirty != 0 {
            do_barray_io(cinfo, ptr, TRUE);
            (*ptr).dirty = FALSE
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
            /* don't fall off front end of file */
            let mut ltemp: c_long = end_row as c_long - (*ptr).rows_in_mem as c_long;
            if ltemp < 0i64 {
                ltemp = 0i64
            }
            (*ptr).cur_start_row = ltemp as JDIMENSION
        }
        /* Read in the selected part of the array.
         * During the initial write pass, we will do no actual read
         * because the selected part is all undefined.
         */
        do_barray_io(cinfo, ptr, FALSE);
    }
    /* Ensure the accessed part of the array is defined; prezero if needed.
     * To improve locality of access, we only prezero the part of the array
     * that the caller is about to access, not the entire in-memory array.
     */
    if (*ptr).first_undef_row < end_row {
        let mut undef_row: JDIMENSION = 0;
        if (*ptr).first_undef_row < start_row {
            if writable != 0 {
                /* writer skipped over a section of array */
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_VIRTUAL_ACCESS as c_int;
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
            let mut bytesperrow: size_t =
                (*ptr).blocksperrow as size_t * ::std::mem::size_of::<JBLOCK>() as c_ulong;
            undef_row -= (*ptr).cur_start_row;
            end_row -= (*ptr).cur_start_row;
            while undef_row < end_row {
                jzero_far(
                    *(*ptr).mem_buffer.offset(undef_row as isize) as *mut c_void,
                    bytesperrow,
                );
                undef_row += 1
            }
        } else if writable == 0 {
            /* reader looking at undefined data */
            (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_VIRTUAL_ACCESS as c_int;
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
        (*ptr).dirty = TRUE
    }
    /* Return address of proper part of the buffer */
    return (*ptr)
        .mem_buffer
        .offset((start_row - (*ptr).cur_start_row) as isize);
}
/*
 * Release all objects belonging to a specified pool.
 */

unsafe extern "C" fn free_pool(mut cinfo: j_common_ptr, mut pool_id: c_int) {
    let mut space_freed: size_t = 0;
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr; /* safety check */

    if pool_id < 0i32 || pool_id >= JPOOL_NUMPOOLS {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_POOL_ID as c_int;
        (*(*cinfo).err).msg_parm.i[0] = pool_id;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    /* If freeing IMAGE pool, close any virtual arrays first */
    if pool_id == JPOOL_IMAGE {
        let mut sptr: jvirt_sarray_ptr = (*mem).virt_sarray_list;
        while !sptr.is_null() {
            if (*sptr).b_s_open != 0 {
                /* there may be no backing store */
                (*sptr).b_s_open = FALSE; /* prevent recursive close if error */
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
        (*mem).virt_sarray_list = NULL as jvirt_sarray_ptr;
        let mut bptr: jvirt_barray_ptr = (*mem).virt_barray_list;
        while !bptr.is_null() {
            if (*bptr).b_s_open != 0 {
                /* there may be no backing store */
                (*bptr).b_s_open = FALSE; /* prevent recursive close if error */
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
        (*mem).virt_barray_list = NULL as jvirt_barray_ptr
    }
    let mut lhdr_ptr: large_pool_ptr = (*mem).large_list[pool_id as usize];
    (*mem).large_list[pool_id as usize] = NULL as large_pool_ptr;
    while !lhdr_ptr.is_null() {
        let mut next_lhdr_ptr: large_pool_ptr = (*lhdr_ptr).next;
        space_freed = (*lhdr_ptr).bytes_used
            + (*lhdr_ptr).bytes_left
            + ::std::mem::size_of::<large_pool_hdr>() as c_ulong;
        jpeg_free_large(cinfo, lhdr_ptr as *mut c_void, space_freed);
        (*mem).total_space_allocated = (*mem).total_space_allocated - space_freed;
        lhdr_ptr = next_lhdr_ptr
    }
    let mut shdr_ptr: small_pool_ptr = (*mem).small_list[pool_id as usize];
    (*mem).small_list[pool_id as usize] = NULL as small_pool_ptr;
    while !shdr_ptr.is_null() {
        let mut next_shdr_ptr: small_pool_ptr = (*shdr_ptr).next;
        space_freed = (*shdr_ptr).bytes_used
            + (*shdr_ptr).bytes_left
            + ::std::mem::size_of::<small_pool_hdr>() as c_ulong;
        jpeg_free_small(cinfo, shdr_ptr as *mut c_void, space_freed);
        (*mem).total_space_allocated = (*mem).total_space_allocated - space_freed;
        shdr_ptr = next_shdr_ptr
    }
}
/*
 * Close up shop entirely.
 * Note that this cannot be called unless cinfo->mem is non-NULL.
 */

unsafe extern "C" fn self_destruct(mut cinfo: j_common_ptr) {
    /* Close all backing store, release all memory.
     * Releasing pools in reverse order might help avoid fragmentation
     * with some (brain-damaged) malloc libraries.
     */
    let mut pool: c_int = JPOOL_NUMPOOLS - 1i32;
    while pool >= JPOOL_PERMANENT {
        free_pool(cinfo, pool);
        pool -= 1
    }
    /* Release the memory manager control block too. */
    jpeg_free_small(
        cinfo,
        (*cinfo).mem as *mut c_void,
        ::std::mem::size_of::<my_memory_mgr>() as c_ulong,
    ); /* ensures I will be called only once */
    (*cinfo).mem = NULL as *mut jpeg_memory_mgr;
    jpeg_mem_term(cinfo);
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

pub unsafe extern "C" fn jinit_memory_mgr(mut cinfo: j_common_ptr) {
    (*cinfo).mem = NULL as *mut jpeg_memory_mgr;
    /* Check for configuration errors.
     * sizeof(ALIGN_TYPE) should be a power of 2; otherwise, it probably
     * doesn't reflect any real hardware alignment requirement.
     * The test is a little tricky: for X>0, X and X-1 have no one-bits
     * in common if and only if X is a power of 2, ie has only one one-bit.
     * Some compilers may give an "unreachable code" warning here; ignore it.
     */
    if ALIGN_SIZE & ALIGN_SIZE - 1i32 != 0i32 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_ALIGN_TYPE as c_int;
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
    let mut test_mac: size_t = MAX_ALLOC_CHUNK as size_t; /* system-dependent initialization */
    if test_mac as c_long != MAX_ALLOC_CHUNK || MAX_ALLOC_CHUNK % ALIGN_SIZE as c_long != 0i64 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_ALLOC_CHUNK as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }

    let mut max_to_use: c_long = jpeg_mem_init(cinfo);
    let mut mem: my_mem_ptr =
        jpeg_get_small(cinfo, ::std::mem::size_of::<my_memory_mgr>() as c_ulong) as my_mem_ptr; /* system-dependent cleanup */
    if mem.is_null() {
        jpeg_mem_term(cinfo);
        (*(*cinfo).err).msg_code = super::jerror::JERR_OUT_OF_MEMORY as c_int;
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
    /* Make MAX_ALLOC_CHUNK accessible to other modules */
    (*mem).pub_0.max_alloc_chunk = MAX_ALLOC_CHUNK;
    /* Initialize working state */
    (*mem).pub_0.max_memory_to_use = max_to_use;
    let mut pool: c_int = JPOOL_NUMPOOLS - 1i32;
    while pool >= JPOOL_PERMANENT {
        (*mem).small_list[pool as usize] = NULL as small_pool_ptr;
        (*mem).large_list[pool as usize] = NULL as large_pool_ptr;
        pool -= 1
    }
    (*mem).virt_sarray_list = NULL as jvirt_sarray_ptr;
    (*mem).virt_barray_list = NULL as jvirt_barray_ptr;
    (*mem).total_space_allocated = ::std::mem::size_of::<my_memory_mgr>() as c_ulong;
    /* Declare ourselves open for business */
    (*cinfo).mem = &mut (*mem).pub_0;
    /* Check for an environment variable JPEGMEM; if found, override the
     * default max_memory setting from jpeg_mem_init.  Note that the
     * surrounding application may again override this value.
     * If your system doesn't support getenv(), define NO_GETENV to disable
     * this feature.
     */

    let mut memenv: *mut c_char = getenv(b"JPEGMEM\x00".as_ptr() as *const c_char);
    if !memenv.is_null() {
        let mut ch: c_char = 'x' as c_char;
        if sscanf(
            memenv,
            b"%ld%c\x00".as_ptr() as *const c_char,
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
