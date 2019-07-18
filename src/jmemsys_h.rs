use libc::c_char;
use libc::c_long;
use libc::c_void;
extern "C" {
    #[no_mangle]
    pub fn jpeg_get_small(cinfo: j_common_ptr, sizeofobject: size_t) -> *mut c_void;

    #[no_mangle]
    pub fn jpeg_free_small(cinfo: j_common_ptr, object: *mut c_void, sizeofobject: size_t);
    #[no_mangle]
    pub fn jpeg_get_large(cinfo: j_common_ptr, sizeofobject: size_t) -> *mut c_void;

    #[no_mangle]
    pub fn jpeg_free_large(cinfo: j_common_ptr, object: *mut c_void, sizeofobject: size_t);
    #[no_mangle]
    pub fn jpeg_mem_available(
        cinfo: j_common_ptr,
        min_bytes_needed: size_t,
        max_bytes_needed: size_t,
        already_allocated: size_t,
    ) -> size_t;
    #[no_mangle]
    pub fn jpeg_open_backing_store(
        cinfo: j_common_ptr,
        info: backing_store_ptr,
        total_bytes_needed: c_long,
    );
    #[no_mangle]
    pub fn jpeg_mem_init(cinfo: j_common_ptr) -> c_long;

    #[no_mangle]
    pub fn jpeg_mem_term(cinfo: j_common_ptr);
}
// =============== BEGIN jmemsys_h ================
use crate::jpeglib_h::j_common_ptr;
use crate::stdlib::FILE;
use ::libc;
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
    pub close_backing_store:
        Option<unsafe extern "C" fn(_: j_common_ptr, _: backing_store_ptr) -> ()>,
    pub temp_file: *mut FILE,
    pub temp_name: [c_char; 64],
}
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
