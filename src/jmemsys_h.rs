pub type backing_store_info = crate::jmemsys_h::backing_store_struct;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct backing_store_struct {
    pub read_backing_store: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_common_ptr,
            _: crate::jmemsys_h::backing_store_ptr,
            _: *mut libc::c_void,
            _: libc::c_long,
            _: libc::c_long,
        ) -> (),
    >,
    pub write_backing_store: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_common_ptr,
            _: crate::jmemsys_h::backing_store_ptr,
            _: *mut libc::c_void,
            _: libc::c_long,
            _: libc::c_long,
        ) -> (),
    >,
    pub close_backing_store: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_common_ptr,
            _: crate::jmemsys_h::backing_store_ptr,
        ) -> (),
    >,
    pub temp_file: *mut crate::stdlib::FILE,
    pub temp_name: [libc::c_char; 64],
}
/* Methods for reading/writing/closing this backing-store object */

/* Private fields for system-dependent backing-store management */

/* For a typical implementation with temp files, we need: */

/* stdio reference to temp file */

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
pub type backing_store_ptr = *mut crate::jmemsys_h::backing_store_struct;
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
pub const MAX_ALLOC_CHUNK: libc::c_long = 1000000000 as libc::c_long;
