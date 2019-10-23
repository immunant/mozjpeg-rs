use libc::{c_char, c_void, c_long};extern "C" {
    #[no_mangle]
    pub fn jpeg_get_small(
        cinfo: j_common_ptr,
        sizeofobject: size_t,
    ) -> *mut c_void;

    #[no_mangle]
    pub fn jpeg_free_small(
        cinfo: j_common_ptr,
        object: *mut c_void,
        sizeofobject: size_t,
    );
    /*
     * These two functions are used to allocate and release large chunks of
     * memory (up to the total free space designated by jpeg_mem_available).
     * These are identical to the jpeg_get/free_small routines; but we keep them
     * separate anyway, in case a different allocation strategy is desirable for
     * large chunks.
     */
    #[no_mangle]
    pub fn jpeg_get_large(
        cinfo: j_common_ptr,
        sizeofobject: size_t,
    ) -> *mut c_void;

    #[no_mangle]
    pub fn jpeg_free_large(
        cinfo: j_common_ptr,
        object: *mut c_void,
        sizeofobject: size_t,
    );
    /*
     * This routine computes the total space still available for allocation by
     * jpeg_get_large.  If more space than this is needed, backing store will be
     * used.  NOTE: any memory already allocated must not be counted.
     *
     * There is a minimum space requirement, corresponding to the minimum
     * feasible buffer sizes; jmemmgr.c will request that much space even if
     * jpeg_mem_available returns zero.  The maximum space needed, enough to hold
     * all working storage in memory, is also passed in case it is useful.
     * Finally, the total space already allocated is passed.  If no better
     * method is available, cinfo->mem->max_memory_to_use - already_allocated
     * is often a suitable calculation.
     *
     * It is OK for jpeg_mem_available to underestimate the space available
     * (that'll just lead to more backing-store access than is really necessary).
     * However, an overestimate will lead to failure.  Hence it's wise to subtract
     * a slop factor from the true available space.  5% should be enough.
     *
     * On machines with lots of virtual memory, any large constant may be returned.
     * Conversely, zero may be returned to always use the minimum amount of memory.
     */
    #[no_mangle]
    pub fn jpeg_mem_available(
        cinfo: j_common_ptr,
        min_bytes_needed: size_t,
        max_bytes_needed: size_t,
        already_allocated: size_t,
    ) -> size_t;
    /*
     * Initial opening of a backing-store object.  This must fill in the
     * read/write/close pointers in the object.  The read/write routines
     * may take an error exit if the specified maximum file size is exceeded.
     * (If jpeg_mem_available always returns a large value, this routine can
     * just take an error exit.)
     */
    #[no_mangle]
    pub fn jpeg_open_backing_store(
        cinfo: j_common_ptr,
        info: backing_store_ptr,
        total_bytes_needed: c_long,
    );
    /*
     * These routines take care of any system-dependent initialization and
     * cleanup required.  jpeg_mem_init will be called before anything is
     * allocated (and, therefore, nothing in cinfo is of use except the error
     * manager pointer).  It should return a suitable default value for
     * max_memory_to_use; this may subsequently be overridden by the surrounding
     * application.  (Note that max_memory_to_use is only important if
     * jpeg_mem_available chooses to consult it ... no one else will.)
     * jpeg_mem_term may assume that all requested memory has been freed and that
     * all opened backing-store objects have been closed.
     */
    #[no_mangle]
    pub fn jpeg_mem_init(cinfo: j_common_ptr) -> c_long;

    #[no_mangle]
    pub fn jpeg_mem_term(cinfo: j_common_ptr);
}

use crate::stdlib::FILE;use crate::jpeglib_h::j_common_ptr;
pub type backing_store_info = backing_store_struct;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct backing_store_struct {
    pub read_backing_store: Option<
        unsafe extern "C" fn(
            _: j_common_ptr,
            _: backing_store_ptr,
            _: *mut c_void,
            _: c_long,
            _: c_long,
        ) -> (),
    >,
    pub write_backing_store: Option<
        unsafe extern "C" fn(
            _: j_common_ptr,
            _: backing_store_ptr,
            _: *mut c_void,
            _: c_long,
            _: c_long,
        ) -> (),
    >,
    pub close_backing_store: Option<
        unsafe extern "C" fn(
            _: j_common_ptr,
            _: backing_store_ptr,
        ) -> (),
    >,
    pub temp_file: *mut FILE,
    pub temp_name: [c_char; 64],
}
/* name of temp file */

/*
 * This structure holds whatever state is needed to access a single
 * backing-store object.  The read/write/close method pointers are called
 * by jmemmgr.c to manipulate the backing-store object; all other fields
 * are private to the system-dependent backing store routines.
 */

/* max length of a temporary file's name */

/* DOS-specific junk */

/* USE_MSDOS_MEMMGR */

/* Mac-specific junk */

/* USE_MAC_MEMMGR */
pub type backing_store_ptr = *mut backing_store_struct;

use crate::stddef_h::size_t;
/*
 * The macro MAX_ALLOC_CHUNK designates the maximum number of bytes that may
 * be requested in a single call to jpeg_get_large (and jpeg_get_small for that
 * matter, but that case should never come into play).  This macro was needed
 * to model the 64Kb-segment-size limit of far addressing on 80x86 machines.
 * On machines with flat address spaces, any large constant may be used.
 *
 * NB: jmemmgr.c expects that MAX_ALLOC_CHUNK will be representable as type
 * size_t and will be a multiple of sizeof(align_type).
 */

/* may be overridden in jconfig.h */
pub const MAX_ALLOC_CHUNK: c_long = 1000000000i64;
