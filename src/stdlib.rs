use libc::c_int;use libc::c_ulong;use libc::c_float;use libc::c_long;use libc::c_schar;use libc::c_uint;use libc::c_void;use libc::c_ushort;use libc::c_double;use libc::c_char;extern "C" {
    #[no_mangle]
    pub fn __ctype_b_loc() -> *mut *const c_ushort;

    #[no_mangle]
    pub fn __ctype_tolower_loc() -> *mut *const __int32_t;

    #[no_mangle]
    pub fn __ctype_toupper_loc() -> *mut *const __int32_t;
    #[no_mangle]
    pub fn __errno_location() -> *mut c_int;
    #[no_mangle]
    pub fn open(__file: *const c_char, __oflag: c_int, _: ...) -> c_int;
    #[no_mangle]
    pub fn setlocale(__category: c_int, __locale: *const c_char) -> *mut c_char;
    #[no_mangle]
    pub fn _setjmp(_: *mut __jmp_buf_tag) -> c_int;

    #[no_mangle]
    pub fn longjmp(_: *mut __jmp_buf_tag, _: c_int) -> !;
    #[no_mangle]
    pub fn pow(_: c_double, _: c_double) -> c_double;

    #[no_mangle]
    pub fn ceilf(_: c_float) -> c_float;

    #[no_mangle]
    pub fn log10(_: c_double) -> c_double;

    #[no_mangle]
    pub fn ceil(_: c_double) -> c_double;

    #[no_mangle]
    pub fn fabs(_: c_double) -> c_double;
    #[no_mangle]
    pub static mut stdin: *mut FILE;

    #[no_mangle]
    pub static mut stdout: *mut FILE;

    #[no_mangle]
    pub fn fflush(__stream: *mut FILE) -> c_int;

    #[no_mangle]
    pub fn sprintf(_: *mut c_char, _: *const c_char, _: ...) -> c_int;

    #[no_mangle]
    pub fn perror(__s: *const c_char);

    #[no_mangle]
    pub fn feof(__stream: *mut FILE) -> c_int;

    #[no_mangle]
    pub fn snprintf(
        _: *mut c_char,
        _: c_ulong,
        _: *const c_char,
        _: ...
    ) -> c_int;

    #[no_mangle]
    pub fn puts(__s: *const c_char) -> c_int;

    #[no_mangle]
    pub static mut stderr: *mut FILE;

    #[no_mangle]
    pub fn fclose(__stream: *mut FILE) -> c_int;

    #[no_mangle]
    pub fn fopen(_: *const c_char, _: *const c_char) -> *mut FILE;

    #[no_mangle]
    pub fn fprintf(_: *mut FILE, _: *const c_char, _: ...) -> c_int;

    #[no_mangle]
    pub fn sscanf(_: *const c_char, _: *const c_char, _: ...) -> c_int;

    #[no_mangle]
    pub fn getc(__stream: *mut FILE) -> c_int;

    #[no_mangle]
    pub fn ungetc(__c: c_int, __stream: *mut FILE) -> c_int;

    #[no_mangle]
    pub fn fread(
        _: *mut c_void,
        _: c_ulong,
        _: c_ulong,
        _: *mut FILE,
    ) -> c_ulong;

    #[no_mangle]
    pub fn fseek(
        __stream: *mut FILE,
        __off: c_long,
        __whence: c_int,
    ) -> c_int;

    #[no_mangle]
    pub fn ftell(__stream: *mut FILE) -> c_long;

    #[no_mangle]
    pub fn putc(__c: c_int, __stream: *mut FILE) -> c_int;

    #[no_mangle]
    pub fn fwrite(
        _: *const c_void,
        _: c_ulong,
        _: c_ulong,
        _: *mut FILE,
    ) -> c_ulong;

    #[no_mangle]
    pub fn ferror(__stream: *mut FILE) -> c_int;

    #[no_mangle]
    pub fn printf(_: *const c_char, _: ...) -> c_int;
    #[no_mangle]
    pub fn abs(_: c_int) -> c_int;

    #[no_mangle]
    pub fn strtod(_: *const c_char, _: *mut *mut c_char) -> c_double;

    #[no_mangle]
    pub fn strtol(
        _: *const c_char,
        _: *mut *mut c_char,
        _: c_int,
    ) -> c_long;

    #[no_mangle]
    pub fn malloc(_: c_ulong) -> *mut c_void;

    #[no_mangle]
    pub fn free(__ptr: *mut c_void);

    #[no_mangle]
    pub fn exit(_: c_int) -> !;

    #[no_mangle]
    pub fn realloc(_: *mut c_void, _: c_ulong) -> *mut c_void;

    #[no_mangle]
    pub fn getenv(__name: *const c_char) -> *mut c_char;

    #[no_mangle]
    pub fn random() -> c_long;

    #[no_mangle]
    pub fn putenv(__string: *mut c_char) -> c_int;
    #[no_mangle]
    pub fn strcmp(_: *const c_char, _: *const c_char) -> c_int;

    #[no_mangle]
    pub fn strcpy(_: *mut c_char, _: *const c_char) -> *mut c_char;

    #[no_mangle]
    pub fn strcat(_: *mut c_char, _: *const c_char) -> *mut c_char;

    #[no_mangle]
    pub fn memset(_: *mut c_void, _: c_int, _: c_ulong) -> *mut c_void;

    #[no_mangle]
    pub fn memcpy(
        _: *mut c_void,
        _: *const c_void,
        _: c_ulong,
    ) -> *mut c_void;

    #[no_mangle]
    pub fn strlen(_: *const c_char) -> c_ulong;

    #[no_mangle]
    pub fn strncpy(
        _: *mut c_char,
        _: *const c_char,
        _: c_ulong,
    ) -> *mut c_char;

    #[no_mangle]
    pub fn strncmp(_: *const c_char, _: *const c_char, _: c_ulong)
        -> c_int;

    #[no_mangle]
    pub fn strchr(_: *const c_char, _: c_int) -> *mut c_char;

    #[no_mangle]
    pub fn strrchr(_: *const c_char, _: c_int) -> *mut c_char;

    #[no_mangle]
    pub fn strerror(_: c_int) -> *mut c_char;
    #[no_mangle]
    pub fn strcasecmp(_: *const c_char, _: *const c_char) -> c_int;

    #[no_mangle]
    pub fn strncasecmp(
        _: *const c_char,
        _: *const c_char,
        _: c_ulong,
    ) -> c_int;
    pub type _IO_marker;

    pub type _IO_codecvt;

    pub type _IO_wide_data;
    #[no_mangle]
    pub fn __fxstat(
        __ver: c_int,
        __fildes: c_int,
        __stat_buf: *mut stat,
    ) -> c_int;
    #[no_mangle]
    pub fn gettimeofday(
        __tv: *mut timeval,
        __tz: __timezone_ptr_t,
    ) -> c_int;
    #[no_mangle]
    pub fn unlink(__name: *const c_char) -> c_int;

    #[no_mangle]
    pub fn lseek(
        __fd: c_int,
        __offset: __off_t,
        __whence: c_int,
    ) -> __off_t;

    #[no_mangle]
    pub fn close(__fd: c_int) -> c_int;

    #[no_mangle]
    pub fn read(
        __fd: c_int,
        __buf: *mut c_void,
        __nbytes: size_t,
    ) -> ssize_t;
}
// =============== BEGIN FILE_h ================
pub type FILE = _IO_FILE;
// ================ END FILE_h ================
// =============== BEGIN __FILE_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: c_int,
    pub _IO_read_ptr: *mut c_char,
    pub _IO_read_end: *mut c_char,
    pub _IO_read_base: *mut c_char,
    pub _IO_write_base: *mut c_char,
    pub _IO_write_ptr: *mut c_char,
    pub _IO_write_end: *mut c_char,
    pub _IO_buf_base: *mut c_char,
    pub _IO_buf_end: *mut c_char,
    pub _IO_save_base: *mut c_char,
    pub _IO_backup_base: *mut c_char,
    pub _IO_save_end: *mut c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: c_int,
    pub _flags2: c_int,
    pub _old_offset: __off_t,
    pub _cur_column: c_ushort,
    pub _vtable_offset: c_schar,
    pub _shortbuf: [c_char; 1],
    pub _lock: *mut c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut c_void,
    pub __pad5: size_t,
    pub _mode: c_int,
    pub _unused2: [c_char; 20],
}
use crate::stddef_h::size_t;
// ================ END __FILE_h ================
// =============== BEGIN __sigset_t_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sigset_t {
    pub __val: [c_ulong; 16],
}
// ================ END __sigset_t_h ================
// =============== BEGIN ctype_h ================
pub type C2RustUnnamed_0 = c_uint;
pub const _ISupper: C2RustUnnamed_0 = 256;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
#[inline]
pub unsafe extern "C" fn tolower(mut __c: c_int) -> c_int {
    return if __c >= -128i32 && __c < 256i32 {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
pub unsafe extern "C" fn toupper(mut __c: c_int) -> c_int {
    return if __c >= -128i32 && __c < 256i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
// ================ END ctype_h ================
// =============== BEGIN fcntl_linux_h ================
pub const O_RDONLY: c_int = 0i32;
// ================ END fcntl_linux_h ================
// =============== BEGIN include_locale_h ================
pub const LC_CTYPE: c_int = __LC_CTYPE;
// ================ END include_locale_h ================
// =============== BEGIN include_setjmp_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
// ================ END include_setjmp_h ================
// =============== BEGIN locale_h ================
pub const __LC_CTYPE: c_int = 0i32;
// ================ END locale_h ================
// =============== BEGIN setjmp_h ================
pub type __jmp_buf = [c_long; 8];
// ================ END setjmp_h ================
// =============== BEGIN stat_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub const _STAT_VER_LINUX: c_int = 1i32;
pub const _STAT_VER: c_int = _STAT_VER_LINUX;
// ================ END stat_h ================
// =============== BEGIN stdint_h ================
pub const SIZE_MAX: c_ulong = 18446744073709551615u64;
// ================ END stdint_h ================
// =============== BEGIN stdio_h ================
pub const SEEK_END: c_int = 2i32;
pub const SEEK_SET: c_int = 0i32;
pub const EOF: c_int = -1i32;
// ================ END stdio_h ================
// =============== BEGIN stdlib_float_h ================
#[inline]
pub unsafe extern "C" fn atof(mut __nptr: *const c_char) -> c_double {
    return strtod(
        __nptr,
        
        NULL as *mut *mut c_char,
    );
}
// ================ END stdlib_float_h ================
// =============== BEGIN stdlib_h ================
#[inline]
pub unsafe extern "C" fn atoi(mut __nptr: *const c_char) -> c_int {
    return strtol(
        __nptr,
        
        NULL as *mut *mut c_char,
        10i32,
    ) as c_int;
}
pub const EXIT_SUCCESS: c_int = 0i32;
pub const EXIT_FAILURE: c_int = 1i32;
pub const RAND_MAX: c_int = 2147483647i32;
use crate::stddef_h::NULL;
// ================ END stdlib_h ================
// =============== BEGIN struct_FILE_h ================
pub type _IO_lock_t = ();
// ================ END struct_FILE_h ================
// =============== BEGIN struct_timespec_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
// ================ END struct_timespec_h ================
// =============== BEGIN struct_timeval_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
// ================ END struct_timeval_h ================
// =============== BEGIN sys_stat_h ================
#[inline]
pub unsafe extern "C" fn fstat(
    mut __fd: c_int,
    mut __statbuf: *mut stat,
) -> c_int {
    return __fxstat(_STAT_VER, __fd, __statbuf);
}
// ================ END sys_stat_h ================
// =============== BEGIN sys_types_h ================
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
// ================ END sys_types_h ================
// =============== BEGIN time_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timezone {
    pub tz_minuteswest: c_int,
    pub tz_dsttime: c_int,
}
pub type __timezone_ptr_t = *mut timezone;
// ================ END time_h ================
// =============== BEGIN types_h ================
pub type __int32_t = c_int;
pub type __off_t = c_long;
pub type __off64_t = c_long;
pub type __dev_t = c_ulong;
pub type __uid_t = c_uint;
pub type __gid_t = c_uint;
pub type __ino_t = c_ulong;
pub type __mode_t = c_uint;
pub type __nlink_t = c_ulong;
pub type __time_t = c_long;
pub type __blksize_t = c_long;
pub type __blkcnt_t = c_long;
pub type __ssize_t = c_long;
pub type __syscall_slong_t = c_long;
pub type __suseconds_t = c_long;
