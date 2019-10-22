extern "C" {
    #[no_mangle]
    pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;

    #[no_mangle]
    pub fn __ctype_tolower_loc() -> *mut *const crate::stdlib::__int32_t;

    #[no_mangle]
    pub fn __ctype_toupper_loc() -> *mut *const crate::stdlib::__int32_t;
    #[no_mangle]
    pub fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    pub fn setlocale(__category: libc::c_int, __locale: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn _setjmp(_: *mut crate::stdlib::__jmp_buf_tag) -> libc::c_int;

    #[no_mangle]
    pub fn longjmp(_: *mut crate::stdlib::__jmp_buf_tag, _: libc::c_int) -> !;
    #[no_mangle]
    pub fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;

    #[no_mangle]
    pub fn ceilf(_: libc::c_float) -> libc::c_float;

    #[no_mangle]
    pub fn log10(_: libc::c_double) -> libc::c_double;

    #[no_mangle]
    pub fn ceil(_: libc::c_double) -> libc::c_double;

    #[no_mangle]
    pub fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    pub static mut stdin: *mut crate::stdlib::FILE;

    #[no_mangle]
    pub static mut stdout: *mut crate::stdlib::FILE;

    #[no_mangle]
    pub fn fflush(__stream: *mut crate::stdlib::FILE) -> libc::c_int;

    #[no_mangle]
    pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;

    #[no_mangle]
    pub fn perror(__s: *const libc::c_char);

    #[no_mangle]
    pub fn feof(__stream: *mut crate::stdlib::FILE) -> libc::c_int;

    #[no_mangle]
    pub fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;

    #[no_mangle]
    pub fn puts(__s: *const libc::c_char) -> libc::c_int;

    #[no_mangle]
    pub static mut stderr: *mut crate::stdlib::FILE;

    #[no_mangle]
    pub fn fclose(__stream: *mut crate::stdlib::FILE) -> libc::c_int;

    #[no_mangle]
    pub fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut crate::stdlib::FILE;

    #[no_mangle]
    pub fn fprintf(_: *mut crate::stdlib::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;

    #[no_mangle]
    pub fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;

    #[no_mangle]
    pub fn getc(__stream: *mut crate::stdlib::FILE) -> libc::c_int;

    #[no_mangle]
    pub fn ungetc(__c: libc::c_int, __stream: *mut crate::stdlib::FILE) -> libc::c_int;

    #[no_mangle]
    pub fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut crate::stdlib::FILE,
    ) -> libc::c_ulong;

    #[no_mangle]
    pub fn fseek(
        __stream: *mut crate::stdlib::FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;

    #[no_mangle]
    pub fn ftell(__stream: *mut crate::stdlib::FILE) -> libc::c_long;

    #[no_mangle]
    pub fn putc(__c: libc::c_int, __stream: *mut crate::stdlib::FILE) -> libc::c_int;

    #[no_mangle]
    pub fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut crate::stdlib::FILE,
    ) -> libc::c_ulong;

    #[no_mangle]
    pub fn ferror(__stream: *mut crate::stdlib::FILE) -> libc::c_int;

    #[no_mangle]
    pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    pub fn abs(_: libc::c_int) -> libc::c_int;

    #[no_mangle]
    pub fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;

    #[no_mangle]
    pub fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;

    #[no_mangle]
    pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;

    #[no_mangle]
    pub fn free(__ptr: *mut libc::c_void);

    #[no_mangle]
    pub fn exit(_: libc::c_int) -> !;

    #[no_mangle]
    pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

    #[no_mangle]
    pub fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;

    #[no_mangle]
    pub fn random() -> libc::c_long;

    #[no_mangle]
    pub fn putenv(__string: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

    #[no_mangle]
    pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

    #[no_mangle]
    pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

    #[no_mangle]
    pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

    #[no_mangle]
    pub fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;

    #[no_mangle]
    pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;

    #[no_mangle]
    pub fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;

    #[no_mangle]
    pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> libc::c_int;

    #[no_mangle]
    pub fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

    #[no_mangle]
    pub fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

    #[no_mangle]
    pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

    #[no_mangle]
    pub fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    pub type _IO_marker;

    pub type _IO_codecvt;

    pub type _IO_wide_data;
    #[no_mangle]
    pub fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut crate::stdlib::stat,
    ) -> libc::c_int;
    #[no_mangle]
    pub fn gettimeofday(
        __tv: *mut crate::stdlib::timeval,
        __tz: crate::stdlib::__timezone_ptr_t,
    ) -> libc::c_int;
    #[no_mangle]
    pub fn unlink(__name: *const libc::c_char) -> libc::c_int;

    #[no_mangle]
    pub fn lseek(
        __fd: libc::c_int,
        __offset: crate::stdlib::__off_t,
        __whence: libc::c_int,
    ) -> crate::stdlib::__off_t;

    #[no_mangle]
    pub fn close(__fd: libc::c_int) -> libc::c_int;

    #[no_mangle]
    pub fn read(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __nbytes: crate::stddef_h::size_t,
    ) -> crate::stdlib::ssize_t;
}
// =============== BEGIN FILE_h ================
pub type FILE = crate::stdlib::_IO_FILE;
// ================ END FILE_h ================
// =============== BEGIN __FILE_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut crate::stdlib::_IO_marker,
    pub _chain: *mut crate::stdlib::_IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: crate::stdlib::__off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: crate::stdlib::__off64_t,
    pub _codecvt: *mut crate::stdlib::_IO_codecvt,
    pub _wide_data: *mut crate::stdlib::_IO_wide_data,
    pub _freeres_list: *mut crate::stdlib::_IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: crate::stddef_h::size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
use crate::stddef_h::size_t;
// ================ END __FILE_h ================
// =============== BEGIN __sigset_t_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
// ================ END __sigset_t_h ================
// =============== BEGIN ctype_h ================
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISupper: crate::stdlib::C2RustUnnamed_0 = 256;
pub const _ISalnum: crate::stdlib::C2RustUnnamed_0 = 8;
pub const _ISpunct: crate::stdlib::C2RustUnnamed_0 = 4;
pub const _IScntrl: crate::stdlib::C2RustUnnamed_0 = 2;
pub const _ISblank: crate::stdlib::C2RustUnnamed_0 = 1;
pub const _ISgraph: crate::stdlib::C2RustUnnamed_0 = 32768;
pub const _ISprint: crate::stdlib::C2RustUnnamed_0 = 16384;
pub const _ISspace: crate::stdlib::C2RustUnnamed_0 = 8192;
pub const _ISxdigit: crate::stdlib::C2RustUnnamed_0 = 4096;
pub const _ISdigit: crate::stdlib::C2RustUnnamed_0 = 2048;
pub const _ISalpha: crate::stdlib::C2RustUnnamed_0 = 1024;
pub const _ISlower: crate::stdlib::C2RustUnnamed_0 = 512;
#[inline]
pub unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -128i32 && __c < 256i32 {
        *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
pub unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -128i32 && __c < 256i32 {
        *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
// ================ END ctype_h ================
// =============== BEGIN fcntl_linux_h ================
pub const O_RDONLY: libc::c_int = 0i32;
// ================ END fcntl_linux_h ================
// =============== BEGIN include_locale_h ================
pub const LC_CTYPE: libc::c_int = crate::stdlib::__LC_CTYPE;
// ================ END include_locale_h ================
// =============== BEGIN include_setjmp_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: crate::stdlib::__jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: crate::stdlib::__sigset_t,
}
pub type jmp_buf = [crate::stdlib::__jmp_buf_tag; 1];
// ================ END include_setjmp_h ================
// =============== BEGIN locale_h ================
pub const __LC_CTYPE: libc::c_int = 0i32;
// ================ END locale_h ================
// =============== BEGIN setjmp_h ================
pub type __jmp_buf = [libc::c_long; 8];
// ================ END setjmp_h ================
// =============== BEGIN stat_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat {
    pub st_dev: crate::stdlib::__dev_t,
    pub st_ino: crate::stdlib::__ino_t,
    pub st_nlink: crate::stdlib::__nlink_t,
    pub st_mode: crate::stdlib::__mode_t,
    pub st_uid: crate::stdlib::__uid_t,
    pub st_gid: crate::stdlib::__gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: crate::stdlib::__dev_t,
    pub st_size: crate::stdlib::__off_t,
    pub st_blksize: crate::stdlib::__blksize_t,
    pub st_blocks: crate::stdlib::__blkcnt_t,
    pub st_atim: crate::stdlib::timespec,
    pub st_mtim: crate::stdlib::timespec,
    pub st_ctim: crate::stdlib::timespec,
    pub __glibc_reserved: [crate::stdlib::__syscall_slong_t; 3],
}
pub const _STAT_VER_LINUX: libc::c_int = 1i32;
pub const _STAT_VER: libc::c_int = crate::stdlib::_STAT_VER_LINUX;
// ================ END stat_h ================
// =============== BEGIN stdint_h ================
pub const SIZE_MAX: libc::c_ulong = 18446744073709551615u64;
// ================ END stdint_h ================
// =============== BEGIN stdio_h ================
pub const SEEK_END: libc::c_int = 2i32;
pub const SEEK_SET: libc::c_int = 0i32;
pub const EOF: libc::c_int = -1i32;
// ================ END stdio_h ================
// =============== BEGIN stdlib_float_h ================
#[inline]
pub unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return crate::stdlib::strtod(
        __nptr,
        crate::stddef_h::NULL as *mut libc::c_void as *mut *mut libc::c_char,
    );
}
// ================ END stdlib_float_h ================
// =============== BEGIN stdlib_h ================
#[inline]
pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return crate::stdlib::strtol(
        __nptr,
        crate::stddef_h::NULL as *mut libc::c_void as *mut *mut libc::c_char,
        10i32,
    ) as libc::c_int;
}
pub const EXIT_SUCCESS: libc::c_int = 0i32;
pub const EXIT_FAILURE: libc::c_int = 1i32;
pub const RAND_MAX: libc::c_int = 2147483647i32;
use crate::stddef_h::NULL;
// ================ END stdlib_h ================
// =============== BEGIN struct_FILE_h ================
pub type _IO_lock_t = ();
// ================ END struct_FILE_h ================
// =============== BEGIN struct_timespec_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: crate::stdlib::__time_t,
    pub tv_nsec: crate::stdlib::__syscall_slong_t,
}
// ================ END struct_timespec_h ================
// =============== BEGIN struct_timeval_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: crate::stdlib::__time_t,
    pub tv_usec: crate::stdlib::__suseconds_t,
}
// ================ END struct_timeval_h ================
// =============== BEGIN sys_stat_h ================
#[inline]
pub unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut crate::stdlib::stat,
) -> libc::c_int {
    return crate::stdlib::__fxstat(crate::stdlib::_STAT_VER, __fd, __statbuf);
}
// ================ END sys_stat_h ================
// =============== BEGIN sys_types_h ================
pub type off_t = crate::stdlib::__off_t;
pub type ssize_t = crate::stdlib::__ssize_t;
// ================ END sys_types_h ================
// =============== BEGIN time_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
pub type __timezone_ptr_t = *mut crate::stdlib::timezone;
// ================ END time_h ================
// =============== BEGIN types_h ================
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
