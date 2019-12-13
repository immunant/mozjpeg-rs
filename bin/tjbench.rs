#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(extern_types, main, ptr_wrapping_offset_from, register_tool)]
pub mod tjutil_h {
    extern "C" {
        #[no_mangle]
        pub fn getTime() -> libc::c_double;
    }
}
pub mod jpeglib_h {
    pub const JMSG_LENGTH_MAX: libc::c_int = 200 as libc::c_int;
}
pub mod stddef_h {
    pub type size_t = libc::c_ulong;

    pub const NULL: libc::c_int = 0 as libc::c_int;

    pub const NULL_0: libc::c_int = 0 as libc::c_int;
}
pub mod stdlib {
    extern "C" {
        #[no_mangle]
        pub fn __ctype_toupper_loc() -> *mut *const crate::stdlib::__int32_t;
        #[no_mangle]
        pub fn __errno_location() -> *mut libc::c_int;
        #[no_mangle]
        pub fn log10(_: libc::c_double) -> libc::c_double;

        #[no_mangle]
        pub fn ceil(_: libc::c_double) -> libc::c_double;

        #[no_mangle]
        pub fn fabs(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn fclose(__stream: *mut crate::stdlib::FILE) -> libc::c_int;

        #[no_mangle]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut crate::stdlib::FILE;

        #[no_mangle]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;

        #[no_mangle]
        pub fn snprintf(
            _: *mut libc::c_char,
            _: libc::c_ulong,
            _: *const libc::c_char,
            _: ...
        ) -> libc::c_int;

        #[no_mangle]
        pub fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;

        #[no_mangle]
        pub fn ftell(__stream: *mut crate::stdlib::FILE) -> libc::c_long;

        #[no_mangle]
        pub fn fseek(
            __stream: *mut crate::stdlib::FILE,
            __off: libc::c_long,
            __whence: libc::c_int,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn fwrite(
            _: *const libc::c_void,
            _: libc::c_ulong,
            _: libc::c_ulong,
            _: *mut crate::stdlib::FILE,
        ) -> libc::c_ulong;

        #[no_mangle]
        pub fn puts(__s: *const libc::c_char) -> libc::c_int;

        #[no_mangle]
        pub fn fread(
            _: *mut libc::c_void,
            _: libc::c_ulong,
            _: libc::c_ulong,
            _: *mut crate::stdlib::FILE,
        ) -> libc::c_ulong;
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
        pub fn abs(_: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;

        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

        #[no_mangle]
        pub fn strncpy(
            _: *mut libc::c_char,
            _: *const libc::c_char,
            _: libc::c_ulong,
        ) -> *mut libc::c_char;

        #[no_mangle]
        pub fn strncmp(
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: libc::c_ulong,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

        #[no_mangle]
        pub fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;

        #[no_mangle]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
        pub type _IO_wide_data;

        pub type _IO_codecvt;

        pub type _IO_marker;
    }
    pub type FILE = crate::stdlib::_IO_FILE;
    pub const SEEK_SET: libc::c_int = 0 as libc::c_int;

    pub const SEEK_END: libc::c_int = 2 as libc::c_int;
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

    pub type _IO_lock_t = ();
    pub type __int32_t = libc::c_int;

    pub type __off_t = libc::c_long;

    pub type __off64_t = libc::c_long;
}
use ::mozjpeg::*;

#[c2rust::header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/turbojpeg.h:37"]
pub mod turbojpeg_h {
    /* *
     * Chrominance subsampling options.
     * When pixels are converted from RGB to YCbCr (see #TJCS_YCbCr) or from CMYK
     * to YCCK (see #TJCS_YCCK) as part of the JPEG compression process, some of
     * the Cb and Cr (chrominance) components can be discarded or averaged together
     * to produce a smaller image with little perceptible loss of image clarity
     * (the human eye is more sensitive to small changes in brightness than to
     * small changes in color.)  This is called "chrominance subsampling".
     */

    /* *
     * MCU block width (in pixels) for a given level of chrominance subsampling.
     * MCU block sizes:
     * - 8x8 for no subsampling or grayscale
     * - 16x8 for 4:2:2
     * - 8x16 for 4:4:0
     * - 16x16 for 4:2:0
     * - 32x8 for 4:1:1
     */

    pub static mut tjMCUWidth: [libc::c_int; 6] = [
        8 as libc::c_int,
        16 as libc::c_int,
        16 as libc::c_int,
        8 as libc::c_int,
        8 as libc::c_int,
        32 as libc::c_int,
    ];
    /* *
     * MCU block height (in pixels) for a given level of chrominance subsampling.
     * MCU block sizes:
     * - 8x8 for no subsampling or grayscale
     * - 16x8 for 4:2:2
     * - 8x16 for 4:4:0
     * - 16x16 for 4:2:0
     * - 32x8 for 4:1:1
     */

    pub static mut tjMCUHeight: [libc::c_int; 6] = [
        8 as libc::c_int,
        8 as libc::c_int,
        16 as libc::c_int,
        8 as libc::c_int,
        16 as libc::c_int,
        8 as libc::c_int,
    ];
    /* *
     * Red offset (in bytes) for a given pixel format.  This specifies the number
     * of bytes that the red component is offset from the start of the pixel.  For
     * instance, if a pixel of format TJ_BGRX is stored in <tt>char pixel[]</tt>,
     * then the red component will be <tt>pixel[tjRedOffset[TJ_BGRX]]</tt>.  This
     * will be -1 if the pixel format does not have a red component.
     */

    pub static mut tjRedOffset: [libc::c_int; 12] = [
        0 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        -(1 as libc::c_int),
        0 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        -(1 as libc::c_int),
    ];
    /* *
     * Green offset (in bytes) for a given pixel format.  This specifies the number
     * of bytes that the green component is offset from the start of the pixel.
     * For instance, if a pixel of format TJ_BGRX is stored in
     * <tt>char pixel[]</tt>, then the green component will be
     * <tt>pixel[tjGreenOffset[TJ_BGRX]]</tt>.  This will be -1 if the pixel format
     * does not have a green component.
     */

    pub static mut tjGreenOffset: [libc::c_int; 12] = [
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        2 as libc::c_int,
        -(1 as libc::c_int),
        1 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        2 as libc::c_int,
        -(1 as libc::c_int),
    ];
    /* *
     * Blue offset (in bytes) for a given pixel format.  This specifies the number
     * of bytes that the Blue component is offset from the start of the pixel.  For
     * instance, if a pixel of format TJ_BGRX is stored in <tt>char pixel[]</tt>,
     * then the blue component will be <tt>pixel[tjBlueOffset[TJ_BGRX]]</tt>.  This
     * will be -1 if the pixel format does not have a blue component.
     */

    pub static mut tjBlueOffset: [libc::c_int; 12] = [
        2 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        -(1 as libc::c_int),
        2 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        -(1 as libc::c_int),
    ];
    /* *
     * Pixel size (in bytes) for a given pixel format
     */

    pub static mut tjPixelSize: [libc::c_int; 12] = [
        3 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
    ];
    extern "C" {
        #[no_mangle]
        pub fn tjInitCompress() -> crate::turbojpeg_h::tjhandle;
        /* *
         * Compress an RGB, grayscale, or CMYK image into a JPEG image.
         *
         * @param handle a handle to a TurboJPEG compressor or transformer instance
         *
         * @param srcBuf pointer to an image buffer containing RGB, grayscale, or
         * CMYK pixels to be compressed
         *
         * @param width width (in pixels) of the source image
         *
         * @param pitch bytes per line in the source image.  Normally, this should be
         * <tt>width * #tjPixelSize[pixelFormat]</tt> if the image is unpadded, or
         * <tt>#TJPAD(width * #tjPixelSize[pixelFormat])</tt> if each line of the image
         * is padded to the nearest 32-bit boundary, as is the case for Windows
         * bitmaps.  You can also be clever and use this parameter to skip lines, etc.
         * Setting this parameter to 0 is the equivalent of setting it to
         * <tt>width * #tjPixelSize[pixelFormat]</tt>.
         *
         * @param height height (in pixels) of the source image
         *
         * @param pixelFormat pixel format of the source image (see @ref TJPF
         * "Pixel formats".)
         *
         * @param jpegBuf address of a pointer to an image buffer that will receive the
         * JPEG image.  TurboJPEG has the ability to reallocate the JPEG buffer
         * to accommodate the size of the JPEG image.  Thus, you can choose to:
         * -# pre-allocate the JPEG buffer with an arbitrary size using #tjAlloc() and
         * let TurboJPEG grow the buffer as needed,
         * -# set <tt>*jpegBuf</tt> to NULL to tell TurboJPEG to allocate the buffer
         * for you, or
         * -# pre-allocate the buffer to a "worst case" size determined by calling
         * #tjBufSize().  This should ensure that the buffer never has to be
         * re-allocated (setting #TJFLAG_NOREALLOC guarantees that it won't be.)
         * .
         * If you choose option 1, <tt>*jpegSize</tt> should be set to the size of your
         * pre-allocated buffer.  In any case, unless you have set #TJFLAG_NOREALLOC,
         * you should always check <tt>*jpegBuf</tt> upon return from this function, as
         * it may have changed.
         *
         * @param jpegSize pointer to an unsigned long variable that holds the size of
         * the JPEG image buffer.  If <tt>*jpegBuf</tt> points to a pre-allocated
         * buffer, then <tt>*jpegSize</tt> should be set to the size of the buffer.
         * Upon return, <tt>*jpegSize</tt> will contain the size of the JPEG image (in
         * bytes.)  If <tt>*jpegBuf</tt> points to a JPEG image buffer that is being
         * reused from a previous call to one of the JPEG compression functions, then
         * <tt>*jpegSize</tt> is ignored.
         *
         * @param jpegSubsamp the level of chrominance subsampling to be used when
         * generating the JPEG image (see @ref TJSAMP
         * "Chrominance subsampling options".)
         *
         * @param jpegQual the image quality of the generated JPEG image (1 = worst,
         * 100 = best)
         *
         * @param flags the bitwise OR of one or more of the @ref TJFLAG_ACCURATEDCT
         * "flags"
         *
         * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2()
         * and #tjGetErrorCode().)
        */
        #[no_mangle]
        pub fn tjCompress2(
            handle: crate::turbojpeg_h::tjhandle,
            srcBuf: *const libc::c_uchar,
            width: libc::c_int,
            pitch: libc::c_int,
            height: libc::c_int,
            pixelFormat: libc::c_int,
            jpegBuf: *mut *mut libc::c_uchar,
            jpegSize: *mut libc::c_ulong,
            jpegSubsamp: libc::c_int,
            jpegQual: libc::c_int,
            flags_0: libc::c_int,
        ) -> libc::c_int;
        /* *
         * Compress a YUV planar image into a JPEG image.
         *
         * @param handle a handle to a TurboJPEG compressor or transformer instance
         *
         * @param srcBuf pointer to an image buffer containing a YUV planar image to be
         * compressed.  The size of this buffer should match the value returned by
         * #tjBufSizeYUV2() for the given image width, height, padding, and level of
         * chrominance subsampling.  The Y, U (Cb), and V (Cr) image planes should be
         * stored sequentially in the source buffer (refer to @ref YUVnotes
         * "YUV Image Format Notes".)
         *
         * @param width width (in pixels) of the source image.  If the width is not an
         * even multiple of the MCU block width (see #tjMCUWidth), then an intermediate
         * buffer copy will be performed within TurboJPEG.
         *
         * @param pad the line padding used in the source image.  For instance, if each
         * line in each plane of the YUV image is padded to the nearest multiple of 4
         * bytes, then <tt>pad</tt> should be set to 4.
         *
         * @param height height (in pixels) of the source image.  If the height is not
         * an even multiple of the MCU block height (see #tjMCUHeight), then an
         * intermediate buffer copy will be performed within TurboJPEG.
         *
         * @param subsamp the level of chrominance subsampling used in the source
         * image (see @ref TJSAMP "Chrominance subsampling options".)
         *
         * @param jpegBuf address of a pointer to an image buffer that will receive the
         * JPEG image.  TurboJPEG has the ability to reallocate the JPEG buffer to
         * accommodate the size of the JPEG image.  Thus, you can choose to:
         * -# pre-allocate the JPEG buffer with an arbitrary size using #tjAlloc() and
         * let TurboJPEG grow the buffer as needed,
         * -# set <tt>*jpegBuf</tt> to NULL to tell TurboJPEG to allocate the buffer
         * for you, or
         * -# pre-allocate the buffer to a "worst case" size determined by calling
         * #tjBufSize().  This should ensure that the buffer never has to be
         * re-allocated (setting #TJFLAG_NOREALLOC guarantees that it won't be.)
         * .
         * If you choose option 1, <tt>*jpegSize</tt> should be set to the size of your
         * pre-allocated buffer.  In any case, unless you have set #TJFLAG_NOREALLOC,
         * you should always check <tt>*jpegBuf</tt> upon return from this function, as
         * it may have changed.
         *
         * @param jpegSize pointer to an unsigned long variable that holds the size of
         * the JPEG image buffer.  If <tt>*jpegBuf</tt> points to a pre-allocated
         * buffer, then <tt>*jpegSize</tt> should be set to the size of the buffer.
         * Upon return, <tt>*jpegSize</tt> will contain the size of the JPEG image (in
         * bytes.)  If <tt>*jpegBuf</tt> points to a JPEG image buffer that is being
         * reused from a previous call to one of the JPEG compression functions, then
         * <tt>*jpegSize</tt> is ignored.
         *
         * @param jpegQual the image quality of the generated JPEG image (1 = worst,
         * 100 = best)
         *
         * @param flags the bitwise OR of one or more of the @ref TJFLAG_ACCURATEDCT
         * "flags"
         *
         * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2()
         * and #tjGetErrorCode().)
        */
        #[no_mangle]
        pub fn tjCompressFromYUV(
            handle: crate::turbojpeg_h::tjhandle,
            srcBuf: *const libc::c_uchar,
            width: libc::c_int,
            pad: libc::c_int,
            height: libc::c_int,
            subsamp: libc::c_int,
            jpegBuf: *mut *mut libc::c_uchar,
            jpegSize: *mut libc::c_ulong,
            jpegQual: libc::c_int,
            flags_0: libc::c_int,
        ) -> libc::c_int;
        /* *
         * The maximum size of the buffer (in bytes) required to hold a JPEG image with
         * the given parameters.  The number of bytes returned by this function is
         * larger than the size of the uncompressed source image.  The reason for this
         * is that the JPEG format uses 16-bit coefficients, and it is thus possible
         * for a very high-quality JPEG image with very high-frequency content to
         * expand rather than compress when converted to the JPEG format.  Such images
         * represent a very rare corner case, but since there is no way to predict the
         * size of a JPEG image prior to compression, the corner case has to be
         * handled.
         *
         * @param width width (in pixels) of the image
         *
         * @param height height (in pixels) of the image
         *
         * @param jpegSubsamp the level of chrominance subsampling to be used when
         * generating the JPEG image (see @ref TJSAMP
         * "Chrominance subsampling options".)
         *
         * @return the maximum size of the buffer (in bytes) required to hold the
         * image, or -1 if the arguments are out of bounds.
         */
        #[no_mangle]
        pub fn tjBufSize(
            width: libc::c_int,
            height: libc::c_int,
            jpegSubsamp: libc::c_int,
        ) -> libc::c_ulong;
        /* *
         * The size of the buffer (in bytes) required to hold a YUV planar image with
         * the given parameters.
         *
         * @param width width (in pixels) of the image
         *
         * @param pad the width of each line in each plane of the image is padded to
         * the nearest multiple of this number of bytes (must be a power of 2.)
         *
         * @param height height (in pixels) of the image
         *
         * @param subsamp level of chrominance subsampling in the image (see
         * @ref TJSAMP "Chrominance subsampling options".)
         *
         * @return the size of the buffer (in bytes) required to hold the image, or
         * -1 if the arguments are out of bounds.
         */
        #[no_mangle]
        pub fn tjBufSizeYUV2(
            width: libc::c_int,
            pad: libc::c_int,
            height: libc::c_int,
            subsamp: libc::c_int,
        ) -> libc::c_ulong;
        /* *
         * Encode an RGB or grayscale image into a YUV planar image.  This function
         * uses the accelerated color conversion routines in the underlying
         * codec but does not execute any of the other steps in the JPEG compression
         * process.
         *
         * @param handle a handle to a TurboJPEG compressor or transformer instance
         *
         * @param srcBuf pointer to an image buffer containing RGB or grayscale pixels
         * to be encoded
         *
         * @param width width (in pixels) of the source image
         *
         * @param pitch bytes per line in the source image.  Normally, this should be
         * <tt>width * #tjPixelSize[pixelFormat]</tt> if the image is unpadded, or
         * <tt>#TJPAD(width * #tjPixelSize[pixelFormat])</tt> if each line of the image
         * is padded to the nearest 32-bit boundary, as is the case for Windows
         * bitmaps.  You can also be clever and use this parameter to skip lines, etc.
         * Setting this parameter to 0 is the equivalent of setting it to
         * <tt>width * #tjPixelSize[pixelFormat]</tt>.
         *
         * @param height height (in pixels) of the source image
         *
         * @param pixelFormat pixel format of the source image (see @ref TJPF
         * "Pixel formats".)
         *
         * @param dstBuf pointer to an image buffer that will receive the YUV image.
         * Use #tjBufSizeYUV2() to determine the appropriate size for this buffer based
         * on the image width, height, padding, and level of chrominance subsampling.
         * The Y, U (Cb), and V (Cr) image planes will be stored sequentially in the
         * buffer (refer to @ref YUVnotes "YUV Image Format Notes".)
         *
         * @param pad the width of each line in each plane of the YUV image will be
         * padded to the nearest multiple of this number of bytes (must be a power of
         * 2.)  To generate images suitable for X Video, <tt>pad</tt> should be set to
         * 4.
         *
         * @param subsamp the level of chrominance subsampling to be used when
         * generating the YUV image (see @ref TJSAMP
         * "Chrominance subsampling options".)  To generate images suitable for X
         * Video, <tt>subsamp</tt> should be set to @ref TJSAMP_420.  This produces an
         * image compatible with the I420 (AKA "YUV420P") format.
         *
         * @param flags the bitwise OR of one or more of the @ref TJFLAG_ACCURATEDCT
         * "flags"
         *
         * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2()
         * and #tjGetErrorCode().)
        */
        #[no_mangle]
        pub fn tjEncodeYUV3(
            handle: crate::turbojpeg_h::tjhandle,
            srcBuf: *const libc::c_uchar,
            width: libc::c_int,
            pitch: libc::c_int,
            height: libc::c_int,
            pixelFormat: libc::c_int,
            dstBuf: *mut libc::c_uchar,
            pad: libc::c_int,
            subsamp: libc::c_int,
            flags_0: libc::c_int,
        ) -> libc::c_int;
        /* *
         * Create a TurboJPEG decompressor instance.
         *
         * @return a handle to the newly-created instance, or NULL if an error
         * occurred (see #tjGetErrorStr2().)
        */
        #[no_mangle]
        pub fn tjInitDecompress() -> crate::turbojpeg_h::tjhandle;
        /* *
         * Retrieve information about a JPEG image without decompressing it.
         *
         * @param handle a handle to a TurboJPEG decompressor or transformer instance
         *
         * @param jpegBuf pointer to a buffer containing a JPEG image
         *
         * @param jpegSize size of the JPEG image (in bytes)
         *
         * @param width pointer to an integer variable that will receive the width (in
         * pixels) of the JPEG image
         *
         * @param height pointer to an integer variable that will receive the height
         * (in pixels) of the JPEG image
         *
         * @param jpegSubsamp pointer to an integer variable that will receive the
         * level of chrominance subsampling used when the JPEG image was compressed
         * (see @ref TJSAMP "Chrominance subsampling options".)
         *
         * @param jpegColorspace pointer to an integer variable that will receive one
         * of the JPEG colorspace constants, indicating the colorspace of the JPEG
         * image (see @ref TJCS "JPEG colorspaces".)
         *
         * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2()
         * and #tjGetErrorCode().)
        */
        #[no_mangle]
        pub fn tjDecompressHeader3(
            handle: crate::turbojpeg_h::tjhandle,
            jpegBuf: *const libc::c_uchar,
            jpegSize: libc::c_ulong,
            width: *mut libc::c_int,
            height: *mut libc::c_int,
            jpegSubsamp: *mut libc::c_int,
            jpegColorspace: *mut libc::c_int,
        ) -> libc::c_int;
        /* *
         * Returns a list of fractional scaling factors that the JPEG decompressor in
         * this implementation of TurboJPEG supports.
         *
         * @param numscalingfactors pointer to an integer variable that will receive
         * the number of elements in the list
         *
         * @return a pointer to a list of fractional scaling factors, or NULL if an
         * error is encountered (see #tjGetErrorStr2().)
        */
        #[no_mangle]
        pub fn tjGetScalingFactors(
            numscalingfactors: *mut libc::c_int,
        ) -> *mut crate::turbojpeg_h::tjscalingfactor;
        /* *
         * Decompress a JPEG image to an RGB, grayscale, or CMYK image.
         *
         * @param handle a handle to a TurboJPEG decompressor or transformer instance
         *
         * @param jpegBuf pointer to a buffer containing the JPEG image to decompress
         *
         * @param jpegSize size of the JPEG image (in bytes)
         *
         * @param dstBuf pointer to an image buffer that will receive the decompressed
         * image.  This buffer should normally be <tt>pitch * scaledHeight</tt> bytes
         * in size, where <tt>scaledHeight</tt> can be determined by calling
         * #TJSCALED() with the JPEG image height and one of the scaling factors
         * returned by #tjGetScalingFactors().  The <tt>dstBuf</tt> pointer may also be
         * used to decompress into a specific region of a larger buffer.
         *
         * @param width desired width (in pixels) of the destination image.  If this is
         * different than the width of the JPEG image being decompressed, then
         * TurboJPEG will use scaling in the JPEG decompressor to generate the largest
         * possible image that will fit within the desired width.  If <tt>width</tt> is
         * set to 0, then only the height will be considered when determining the
         * scaled image size.
         *
         * @param pitch bytes per line in the destination image.  Normally, this is
         * <tt>scaledWidth * #tjPixelSize[pixelFormat]</tt> if the decompressed image
         * is unpadded, else <tt>#TJPAD(scaledWidth * #tjPixelSize[pixelFormat])</tt>
         * if each line of the decompressed image is padded to the nearest 32-bit
         * boundary, as is the case for Windows bitmaps.  (NOTE: <tt>scaledWidth</tt>
         * can be determined by calling #TJSCALED() with the JPEG image width and one
         * of the scaling factors returned by #tjGetScalingFactors().)  You can also be
         * clever and use the pitch parameter to skip lines, etc.  Setting this
         * parameter to 0 is the equivalent of setting it to
         * <tt>scaledWidth * #tjPixelSize[pixelFormat]</tt>.
         *
         * @param height desired height (in pixels) of the destination image.  If this
         * is different than the height of the JPEG image being decompressed, then
         * TurboJPEG will use scaling in the JPEG decompressor to generate the largest
         * possible image that will fit within the desired height.  If <tt>height</tt>
         * is set to 0, then only the width will be considered when determining the
         * scaled image size.
         *
         * @param pixelFormat pixel format of the destination image (see @ref
         * TJPF "Pixel formats".)
         *
         * @param flags the bitwise OR of one or more of the @ref TJFLAG_ACCURATEDCT
         * "flags"
         *
         * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2()
         * and #tjGetErrorCode().)
         */
        #[no_mangle]
        pub fn tjDecompress2(
            handle: crate::turbojpeg_h::tjhandle,
            jpegBuf: *const libc::c_uchar,
            jpegSize: libc::c_ulong,
            dstBuf: *mut libc::c_uchar,
            width: libc::c_int,
            pitch: libc::c_int,
            height: libc::c_int,
            pixelFormat: libc::c_int,
            flags_0: libc::c_int,
        ) -> libc::c_int;
        /* *
         * Decompress a JPEG image to a YUV planar image.  This function performs JPEG
         * decompression but leaves out the color conversion step, so a planar YUV
         * image is generated instead of an RGB image.
         *
         * @param handle a handle to a TurboJPEG decompressor or transformer instance
         *
         * @param jpegBuf pointer to a buffer containing the JPEG image to decompress
         *
         * @param jpegSize size of the JPEG image (in bytes)
         *
         * @param dstBuf pointer to an image buffer that will receive the YUV image.
         * Use #tjBufSizeYUV2() to determine the appropriate size for this buffer based
         * on the image width, height, padding, and level of subsampling.  The Y,
         * U (Cb), and V (Cr) image planes will be stored sequentially in the buffer
         * (refer to @ref YUVnotes "YUV Image Format Notes".)
         *
         * @param width desired width (in pixels) of the YUV image.  If this is
         * different than the width of the JPEG image being decompressed, then
         * TurboJPEG will use scaling in the JPEG decompressor to generate the largest
         * possible image that will fit within the desired width.  If <tt>width</tt> is
         * set to 0, then only the height will be considered when determining the
         * scaled image size.  If the scaled width is not an even multiple of the MCU
         * block width (see #tjMCUWidth), then an intermediate buffer copy will be
         * performed within TurboJPEG.
         *
         * @param pad the width of each line in each plane of the YUV image will be
         * padded to the nearest multiple of this number of bytes (must be a power of
         * 2.)  To generate images suitable for X Video, <tt>pad</tt> should be set to
         * 4.
         *
         * @param height desired height (in pixels) of the YUV image.  If this is
         * different than the height of the JPEG image being decompressed, then
         * TurboJPEG will use scaling in the JPEG decompressor to generate the largest
         * possible image that will fit within the desired height.  If <tt>height</tt>
         * is set to 0, then only the width will be considered when determining the
         * scaled image size.  If the scaled height is not an even multiple of the MCU
         * block height (see #tjMCUHeight), then an intermediate buffer copy will be
         * performed within TurboJPEG.
         *
         * @param flags the bitwise OR of one or more of the @ref TJFLAG_ACCURATEDCT
         * "flags"
         *
         * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2()
         * and #tjGetErrorCode().)
         */
        #[no_mangle]
        pub fn tjDecompressToYUV2(
            handle: crate::turbojpeg_h::tjhandle,
            jpegBuf: *const libc::c_uchar,
            jpegSize: libc::c_ulong,
            dstBuf: *mut libc::c_uchar,
            width: libc::c_int,
            pad: libc::c_int,
            height: libc::c_int,
            flags_0: libc::c_int,
        ) -> libc::c_int;
        /* *
         * Decode a YUV planar image into an RGB or grayscale image.  This function
         * uses the accelerated color conversion routines in the underlying
         * codec but does not execute any of the other steps in the JPEG decompression
         * process.
         *
         * @param handle a handle to a TurboJPEG decompressor or transformer instance
         *
         * @param srcBuf pointer to an image buffer containing a YUV planar image to be
         * decoded.  The size of this buffer should match the value returned by
         * #tjBufSizeYUV2() for the given image width, height, padding, and level of
         * chrominance subsampling.  The Y, U (Cb), and V (Cr) image planes should be
         * stored sequentially in the source buffer (refer to @ref YUVnotes
         * "YUV Image Format Notes".)
         *
         * @param pad Use this parameter to specify that the width of each line in each
         * plane of the YUV source image is padded to the nearest multiple of this
         * number of bytes (must be a power of 2.)
         *
         * @param subsamp the level of chrominance subsampling used in the YUV source
         * image (see @ref TJSAMP "Chrominance subsampling options".)
         *
         * @param dstBuf pointer to an image buffer that will receive the decoded
         * image.  This buffer should normally be <tt>pitch * height</tt> bytes in
         * size, but the <tt>dstBuf</tt> pointer can also be used to decode into a
         * specific region of a larger buffer.
         *
         * @param width width (in pixels) of the source and destination images
         *
         * @param pitch bytes per line in the destination image.  Normally, this should
         * be <tt>width * #tjPixelSize[pixelFormat]</tt> if the destination image is
         * unpadded, or <tt>#TJPAD(width * #tjPixelSize[pixelFormat])</tt> if each line
         * of the destination image should be padded to the nearest 32-bit boundary, as
         * is the case for Windows bitmaps.  You can also be clever and use the pitch
         * parameter to skip lines, etc.  Setting this parameter to 0 is the equivalent
         * of setting it to <tt>width * #tjPixelSize[pixelFormat]</tt>.
         *
         * @param height height (in pixels) of the source and destination images
         *
         * @param pixelFormat pixel format of the destination image (see @ref TJPF
         * "Pixel formats".)
         *
         * @param flags the bitwise OR of one or more of the @ref TJFLAG_ACCURATEDCT
         * "flags"
         *
         * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2()
         * and #tjGetErrorCode().)
         */
        #[no_mangle]
        pub fn tjDecodeYUV(
            handle: crate::turbojpeg_h::tjhandle,
            srcBuf: *const libc::c_uchar,
            pad: libc::c_int,
            subsamp: libc::c_int,
            dstBuf: *mut libc::c_uchar,
            width: libc::c_int,
            pitch: libc::c_int,
            height: libc::c_int,
            pixelFormat: libc::c_int,
            flags_0: libc::c_int,
        ) -> libc::c_int;
        /* *
         * Create a new TurboJPEG transformer instance.
         *
         * @return a handle to the newly-created instance, or NULL if an error
         * occurred (see #tjGetErrorStr2().)
         */
        #[no_mangle]
        pub fn tjInitTransform() -> crate::turbojpeg_h::tjhandle;
        /* *
         * Losslessly transform a JPEG image into another JPEG image.  Lossless
         * transforms work by moving the raw DCT coefficients from one JPEG image
         * structure to another without altering the values of the coefficients.  While
         * this is typically faster than decompressing the image, transforming it, and
         * re-compressing it, lossless transforms are not free.  Each lossless
         * transform requires reading and performing Huffman decoding on all of the
         * coefficients in the source image, regardless of the size of the destination
         * image.  Thus, this function provides a means of generating multiple
         * transformed images from the same source or  applying multiple
         * transformations simultaneously, in order to eliminate the need to read the
         * source coefficients multiple times.
         *
         * @param handle a handle to a TurboJPEG transformer instance
         *
         * @param jpegBuf pointer to a buffer containing the JPEG source image to
         * transform
         *
         * @param jpegSize size of the JPEG source image (in bytes)
         *
         * @param n the number of transformed JPEG images to generate
         *
         * @param dstBufs pointer to an array of n image buffers.  <tt>dstBufs[i]</tt>
         * will receive a JPEG image that has been transformed using the parameters in
         * <tt>transforms[i]</tt>.  TurboJPEG has the ability to reallocate the JPEG
         * buffer to accommodate the size of the JPEG image.  Thus, you can choose to:
         * -# pre-allocate the JPEG buffer with an arbitrary size using #tjAlloc() and
         * let TurboJPEG grow the buffer as needed,
         * -# set <tt>dstBufs[i]</tt> to NULL to tell TurboJPEG to allocate the buffer
         * for you, or
         * -# pre-allocate the buffer to a "worst case" size determined by calling
         * #tjBufSize() with the transformed or cropped width and height.  Under normal
         * circumstances, this should ensure that the buffer never has to be
         * re-allocated (setting #TJFLAG_NOREALLOC guarantees that it won't be.)  Note,
         * however, that there are some rare cases (such as transforming images with a
         * large amount of embedded EXIF or ICC profile data) in which the output image
         * will be larger than the worst-case size, and #TJFLAG_NOREALLOC cannot be
         * used in those cases.
         * .
         * If you choose option 1, <tt>dstSizes[i]</tt> should be set to the size of
         * your pre-allocated buffer.  In any case, unless you have set
         * #TJFLAG_NOREALLOC, you should always check <tt>dstBufs[i]</tt> upon return
         * from this function, as it may have changed.
         *
         * @param dstSizes pointer to an array of n unsigned long variables that will
         * receive the actual sizes (in bytes) of each transformed JPEG image.  If
         * <tt>dstBufs[i]</tt> points to a pre-allocated buffer, then
         * <tt>dstSizes[i]</tt> should be set to the size of the buffer.  Upon return,
         * <tt>dstSizes[i]</tt> will contain the size of the JPEG image (in bytes.)
         *
         * @param transforms pointer to an array of n #tjtransform structures, each of
         * which specifies the transform parameters and/or cropping region for the
         * corresponding transformed output image.
         *
         * @param flags the bitwise OR of one or more of the @ref TJFLAG_ACCURATEDCT
         * "flags"
         *
         * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2()
         * and #tjGetErrorCode().)
         */
        #[no_mangle]
        pub fn tjTransform(
            handle: crate::turbojpeg_h::tjhandle,
            jpegBuf: *const libc::c_uchar,
            jpegSize: libc::c_ulong,
            n: libc::c_int,
            dstBufs: *mut *mut libc::c_uchar,
            dstSizes: *mut libc::c_ulong,
            transforms: *mut crate::turbojpeg_h::tjtransform,
            flags_0: libc::c_int,
        ) -> libc::c_int;
        /* *
         * Destroy a TurboJPEG compressor, decompressor, or transformer instance.
         *
         * @param handle a handle to a TurboJPEG compressor, decompressor or
         * transformer instance
         *
         * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2().)
         */
        #[no_mangle]
        pub fn tjDestroy(handle: crate::turbojpeg_h::tjhandle) -> libc::c_int;
        /* *
         * Allocate an image buffer for use with TurboJPEG.  You should always use
         * this function to allocate the JPEG destination buffer(s) for the compression
         * and transform functions unless you are disabling automatic buffer
         * (re)allocation (by setting #TJFLAG_NOREALLOC.)
         *
         * @param bytes the number of bytes to allocate
         *
         * @return a pointer to a newly-allocated buffer with the specified number of
         * bytes.
         *
         * @sa tjFree()
         */
        #[no_mangle]
        pub fn tjAlloc(bytes: libc::c_int) -> *mut libc::c_uchar;
        /* *
         * Load an uncompressed image from disk into memory.
         *
         * @param filename name of a file containing an uncompressed image in Windows
         * BMP or PBMPLUS (PPM/PGM) format
         *
         * @param width pointer to an integer variable that will receive the width (in
         * pixels) of the uncompressed image
         *
         * @param align row alignment of the image buffer to be returned (must be a
         * power of 2.)  For instance, setting this parameter to 4 will cause all rows
         * in the image buffer to be padded to the nearest 32-bit boundary, and setting
         * this parameter to 1 will cause all rows in the image buffer to be unpadded.
         *
         * @param height pointer to an integer variable that will receive the height
         * (in pixels) of the uncompressed image
         *
         * @param pixelFormat pointer to an integer variable that specifies or will
         * receive the pixel format of the uncompressed image buffer.  The behavior of
         * #tjLoadImage() will vary depending on the value of <tt>*pixelFormat</tt>
         * passed to the function:
         * - @ref TJPF_UNKNOWN : The uncompressed image buffer returned by the function
         * will use the most optimal pixel format for the file type, and
         * <tt>*pixelFormat</tt> will contain the ID of this pixel format upon
         * successful return from the function.
         * - @ref TJPF_GRAY : Only PGM files and 8-bit BMP files with a grayscale
         * colormap can be loaded.
         * - @ref TJPF_CMYK : The RGB or grayscale pixels stored in the file will be
         * converted using a quick & dirty algorithm that is suitable only for testing
         * purposes (proper conversion between CMYK and other formats requires a color
         * management system.)
         * - Other @ref TJPF "pixel formats" : The uncompressed image buffer will use
         * the specified pixel format, and pixel format conversion will be performed if
         * necessary.
         *
         * @param flags the bitwise OR of one or more of the @ref TJFLAG_BOTTOMUP
         * "flags".
         *
         * @return a pointer to a newly-allocated buffer containing the uncompressed
         * image, converted to the chosen pixel format and with the chosen row
         * alignment, or NULL if an error occurred (see #tjGetErrorStr2().)  This
         * buffer should be freed using #tjFree().
         */
        #[no_mangle]
        pub fn tjLoadImage(
            filename: *const libc::c_char,
            width: *mut libc::c_int,
            align: libc::c_int,
            height: *mut libc::c_int,
            pixelFormat: *mut libc::c_int,
            flags_0: libc::c_int,
        ) -> *mut libc::c_uchar;
        /* *
         * Save an uncompressed image from memory to disk.
         *
         * @param filename name of a file to which to save the uncompressed image.
         * The image will be stored in Windows BMP or PBMPLUS (PPM/PGM) format,
         * depending on the file extension.
         *
         * @param buffer pointer to an image buffer containing RGB, grayscale, or
         * CMYK pixels to be saved
         *
         * @param width width (in pixels) of the uncompressed image
         *
         * @param pitch bytes per line in the image buffer.  Setting this parameter to
         * 0 is the equivalent of setting it to
         * <tt>width * #tjPixelSize[pixelFormat]</tt>.
         *
         * @param height height (in pixels) of the uncompressed image
         *
         * @param pixelFormat pixel format of the image buffer (see @ref TJPF
         * "Pixel formats".)  If this parameter is set to @ref TJPF_GRAY, then the
         * image will be stored in PGM or 8-bit (indexed color) BMP format.  Otherwise,
         * the image will be stored in PPM or 24-bit BMP format.  If this parameter
         * is set to @ref TJPF_CMYK, then the CMYK pixels will be converted to RGB
         * using a quick & dirty algorithm that is suitable only for testing (proper
         * conversion between CMYK and other formats requires a color management
         * system.)
         *
         * @param flags the bitwise OR of one or more of the @ref TJFLAG_BOTTOMUP
         * "flags".
         *
         * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2().)
         */
        #[no_mangle]
        pub fn tjSaveImage(
            filename: *const libc::c_char,
            buffer: *mut libc::c_uchar,
            width: libc::c_int,
            pitch: libc::c_int,
            height: libc::c_int,
            pixelFormat: libc::c_int,
            flags_0: libc::c_int,
        ) -> libc::c_int;
        /* *
         * Free an image buffer previously allocated by TurboJPEG.  You should always
         * use this function to free JPEG destination buffer(s) that were automatically
         * (re)allocated by the compression and transform functions or that were
         * manually allocated using #tjAlloc().
         *
         * @param buffer address of the buffer to free
         *
         * @sa tjAlloc()
         */
        #[no_mangle]
        pub fn tjFree(buffer: *mut libc::c_uchar);
        /* *
         * Returns a descriptive error message explaining why the last command failed.
         *
         * @param handle a handle to a TurboJPEG compressor, decompressor, or
         * transformer instance, or NULL if the error was generated by a global
         * function (but note that retrieving the error message for a global function
         * is not thread-safe.)
         *
         * @return a descriptive error message explaining why the last command failed.
         */
        #[no_mangle]
        pub fn tjGetErrorStr2(handle: crate::turbojpeg_h::tjhandle) -> *mut libc::c_char;
        /* *
         * Returns a code indicating the severity of the last error.  See
         * @ref TJERR "Error codes".
         *
         * @param handle a handle to a TurboJPEG compressor, decompressor or
         * transformer instance
         *
         * @return a code indicating the severity of the last error.  See
         * @ref TJERR "Error codes".
         */
        #[no_mangle]
        pub fn tjGetErrorCode(handle: crate::turbojpeg_h::tjhandle) -> libc::c_int;

        #[no_mangle]
        pub fn tjGetErrorStr() -> *mut libc::c_char;
    }
    pub type TJSAMP = libc::c_uint;
    /* *
     * The number of pixel formats
     */
    /* *
     * Pixel formats
     */

    pub type TJPF = libc::c_int;
    /* *
     * The number of JPEG colorspaces
     */
    /* *
     * JPEG colorspaces
     */

    pub type TJCS = libc::c_uint;
    /* *
     * The number of error codes
     */
    /* *
     * Error codes
     */

    pub type TJERR = libc::c_uint;
    /* *
     * The number of transform operations
     */
    /* *
     * Transform operations for #tjTransform()
     */

    pub type TJXOP = libc::c_uint;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct tjscalingfactor {
        pub num: libc::c_int,
        pub denom: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct tjregion {
        pub x: libc::c_int,
        pub y: libc::c_int,
        pub w: libc::c_int,
        pub h: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct tjtransform {
        pub r: crate::turbojpeg_h::tjregion,
        pub op: libc::c_int,
        pub options: libc::c_int,
        pub data: *mut libc::c_void,
        pub customFilter: Option<
            unsafe extern "C" fn(
                _: *mut libc::c_short,
                _: crate::turbojpeg_h::tjregion,
                _: crate::turbojpeg_h::tjregion,
                _: libc::c_int,
                _: libc::c_int,
                _: *mut crate::turbojpeg_h::tjtransform,
            ) -> libc::c_int,
        >,
    }
    /* *
     * TurboJPEG instance handle
     */

    pub type tjhandle = *mut libc::c_void;
    /* *
     * 4:1:1 chrominance subsampling.  The JPEG or YUV image will contain one
     * chrominance component for every 4x1 block of pixels in the source image.
     * JPEG images compressed with 4:1:1 subsampling will be almost exactly the
     * same size as those compressed with 4:2:0 subsampling, and in the
     * aggregate, both subsampling methods produce approximately the same
     * perceptual quality.  However, 4:1:1 is better able to reproduce sharp
     * horizontal features.
     *
     * @note 4:1:1 subsampling is not fully accelerated in libjpeg-turbo.
     */

    pub const TJSAMP_411: crate::turbojpeg_h::TJSAMP = 5;
    /* *
     * 4:4:0 chrominance subsampling.  The JPEG or YUV image will contain one
     * chrominance component for every 1x2 block of pixels in the source image.
     *
     * @note 4:4:0 subsampling is not fully accelerated in libjpeg-turbo.
     */

    pub const TJSAMP_440: crate::turbojpeg_h::TJSAMP = 4;
    /* *
     * Grayscale.  The JPEG or YUV image will contain no chrominance components.
     */

    pub const TJSAMP_GRAY: crate::turbojpeg_h::TJSAMP = 3;
    /* *
     * 4:2:0 chrominance subsampling.  The JPEG or YUV image will contain one
     * chrominance component for every 2x2 block of pixels in the source image.
     */

    pub const TJSAMP_420: crate::turbojpeg_h::TJSAMP = 2;
    /* *
     * 4:2:2 chrominance subsampling.  The JPEG or YUV image will contain one
     * chrominance component for every 2x1 block of pixels in the source image.
     */

    pub const TJSAMP_422: crate::turbojpeg_h::TJSAMP = 1;
    /* *
     * 4:4:4 chrominance subsampling (no chrominance subsampling).  The JPEG or
     * YUV image will contain one chrominance component for every pixel in the
     * source image.
     */

    pub const TJSAMP_444: crate::turbojpeg_h::TJSAMP = 0;

    /* *
     * Unknown pixel format.  Currently this is only used by #tjLoadImage().
     */

    pub const TJPF_UNKNOWN: crate::turbojpeg_h::TJPF = -1;
    /* *
     * CMYK pixel format.  Unlike RGB, which is an additive color model used
     * primarily for display, CMYK (Cyan/Magenta/Yellow/Key) is a subtractive
     * color model used primarily for printing.  In the CMYK color model, the
     * value of each color component typically corresponds to an amount of cyan,
     * magenta, yellow, or black ink that is applied to a white background.  In
     * order to convert between CMYK and RGB, it is necessary to use a color
     * management system (CMS.)  A CMS will attempt to map colors within the
     * printer's gamut to perceptually similar colors in the display's gamut and
     * vice versa, but the mapping is typically not 1:1 or reversible, nor can it
     * be defined with a simple formula.  Thus, such a conversion is out of scope
     * for a codec library.  However, the TurboJPEG API allows for compressing
     * CMYK pixels into a YCCK JPEG image (see #TJCS_YCCK) and decompressing YCCK
     * JPEG images into CMYK pixels.
     */

    pub const TJPF_CMYK: crate::turbojpeg_h::TJPF = 11;
    /* *
     * ARGB pixel format.  This is the same as @ref TJPF_XRGB, except that when
     * decompressing, the X component is guaranteed to be 0xFF, which can be
     * interpreted as an opaque alpha channel.
     */

    pub const TJPF_ARGB: crate::turbojpeg_h::TJPF = 10;
    /* *
     * ABGR pixel format.  This is the same as @ref TJPF_XBGR, except that when
     * decompressing, the X component is guaranteed to be 0xFF, which can be
     * interpreted as an opaque alpha channel.
     */

    pub const TJPF_ABGR: crate::turbojpeg_h::TJPF = 9;
    /* *
     * BGRA pixel format.  This is the same as @ref TJPF_BGRX, except that when
     * decompressing, the X component is guaranteed to be 0xFF, which can be
     * interpreted as an opaque alpha channel.
     */

    pub const TJPF_BGRA: crate::turbojpeg_h::TJPF = 8;
    /* *
     * RGBA pixel format.  This is the same as @ref TJPF_RGBX, except that when
     * decompressing, the X component is guaranteed to be 0xFF, which can be
     * interpreted as an opaque alpha channel.
     */

    pub const TJPF_RGBA: crate::turbojpeg_h::TJPF = 7;
    /* *
     * Grayscale pixel format.  Each 1-byte pixel represents a luminance
     * (brightness) level from 0 to 255.
     */

    pub const TJPF_GRAY: crate::turbojpeg_h::TJPF = 6;
    /* *
     * XRGB pixel format.  The red, green, and blue components in the image are
     * stored in 4-byte pixels in the order B, G, R from highest to lowest byte
     * address within each pixel.  The X component is ignored when compressing
     * and undefined when decompressing.
     */

    pub const TJPF_XRGB: crate::turbojpeg_h::TJPF = 5;
    /* *
     * XBGR pixel format.  The red, green, and blue components in the image are
     * stored in 4-byte pixels in the order R, G, B from highest to lowest byte
     * address within each pixel.  The X component is ignored when compressing
     * and undefined when decompressing.
     */

    pub const TJPF_XBGR: crate::turbojpeg_h::TJPF = 4;
    /* *
     * BGRX pixel format.  The red, green, and blue components in the image are
     * stored in 4-byte pixels in the order B, G, R from lowest to highest byte
     * address within each pixel.  The X component is ignored when compressing
     * and undefined when decompressing.
     */

    pub const TJPF_BGRX: crate::turbojpeg_h::TJPF = 3;
    /* *
     * RGBX pixel format.  The red, green, and blue components in the image are
     * stored in 4-byte pixels in the order R, G, B from lowest to highest byte
     * address within each pixel.  The X component is ignored when compressing
     * and undefined when decompressing.
     */

    pub const TJPF_RGBX: crate::turbojpeg_h::TJPF = 2;
    /* *
     * BGR pixel format.  The red, green, and blue components in the image are
     * stored in 3-byte pixels in the order B, G, R from lowest to highest byte
     * address within each pixel.
     */

    pub const TJPF_BGR: crate::turbojpeg_h::TJPF = 1;
    /* *
     * RGB pixel format.  The red, green, and blue components in the image are
     * stored in 3-byte pixels in the order R, G, B from lowest to highest byte
     * address within each pixel.
     */

    pub const TJPF_RGB: crate::turbojpeg_h::TJPF = 0;

    /* *
     * YCCK colorspace.  YCCK (AKA "YCbCrK") is not an absolute colorspace but
     * rather a mathematical transformation of CMYK designed solely for storage
     * and transmission.  It is to CMYK as YCbCr is to RGB.  CMYK pixels can be
     * reversibly transformed into YCCK, and as with YCbCr, the chrominance
     * components in the YCCK pixels can be subsampled without incurring major
     * perceptual loss.  YCCK JPEG images can only be compressed from and
     * decompressed to CMYK pixels.
     */

    pub const TJCS_YCCK: crate::turbojpeg_h::TJCS = 4;
    /* *
     * CMYK colorspace.  When compressing the JPEG image, the C, M, Y, and K
     * components in the source image are reordered into image planes, but no
     * colorspace conversion or subsampling is performed.  CMYK JPEG images can
     * only be decompressed to CMYK pixels.
     */

    pub const TJCS_CMYK: crate::turbojpeg_h::TJCS = 3;
    /* *
     * Grayscale colorspace.  The JPEG image retains only the luminance data (Y
     * component), and any color data from the source image is discarded.
     * Grayscale JPEG images can be compressed from and decompressed to any of
     * the extended RGB pixel formats or grayscale, or they can be decompressed
     * to YUV planar images.
     */

    pub const TJCS_GRAY: crate::turbojpeg_h::TJCS = 2;
    /* *
     * YCbCr colorspace.  YCbCr is not an absolute colorspace but rather a
     * mathematical transformation of RGB designed solely for storage and
     * transmission.  YCbCr images must be converted to RGB before they can
     * actually be displayed.  In the YCbCr colorspace, the Y (luminance)
     * component represents the black & white portion of the original image, and
     * the Cb and Cr (chrominance) components represent the color portion of the
     * original image.  Originally, the analog equivalent of this transformation
     * allowed the same signal to drive both black & white and color televisions,
     * but JPEG images use YCbCr primarily because it allows the color data to be
     * optionally subsampled for the purposes of reducing bandwidth or disk
     * space.  YCbCr is the most common JPEG colorspace, and YCbCr JPEG images
     * can be compressed from and decompressed to any of the extended RGB pixel
     * formats or grayscale, or they can be decompressed to YUV planar images.
     */

    pub const TJCS_YCbCr: crate::turbojpeg_h::TJCS = 1;
    /* *
     * RGB colorspace.  When compressing the JPEG image, the R, G, and B
     * components in the source image are reordered into image planes, but no
     * colorspace conversion or subsampling is performed.  RGB JPEG images can be
     * decompressed to any of the extended RGB pixel formats or grayscale, but
     * they cannot be decompressed to YUV images.
     */

    pub const TJCS_RGB: crate::turbojpeg_h::TJCS = 0;

    /* *
     * The error was fatal and non-recoverable.
     */

    pub const TJERR_FATAL: crate::turbojpeg_h::TJERR = 1;
    /* *
     * The error was non-fatal and recoverable, but the image may still be
     * corrupt.
     */

    pub const TJERR_WARNING: crate::turbojpeg_h::TJERR = 0;

    /* *
     * Rotate image counter-clockwise by 90 degrees.  This transform is imperfect
     * if there are any partial MCU blocks on the right edge (see
     * #TJXOPT_PERFECT.)
     */

    pub const TJXOP_ROT270: crate::turbojpeg_h::TJXOP = 7;
    /* *
     * Rotate image 180 degrees.  This transform is imperfect if there are any
     * partial MCU blocks in the image (see #TJXOPT_PERFECT.)
     */

    pub const TJXOP_ROT180: crate::turbojpeg_h::TJXOP = 6;
    /* *
     * Rotate image clockwise by 90 degrees.  This transform is imperfect if
     * there are any partial MCU blocks on the bottom edge (see
     * #TJXOPT_PERFECT.)
     */

    pub const TJXOP_ROT90: crate::turbojpeg_h::TJXOP = 5;
    /* *
     * Transverse transpose image (flip/mirror along upper right to lower left
     * axis.)  This transform is imperfect if there are any partial MCU blocks in
     * the image (see #TJXOPT_PERFECT.)
     */

    pub const TJXOP_TRANSVERSE: crate::turbojpeg_h::TJXOP = 4;
    /* *
     * Transpose image (flip/mirror along upper left to lower right axis.)  This
     * transform is always perfect.
     */

    pub const TJXOP_TRANSPOSE: crate::turbojpeg_h::TJXOP = 3;
    /* *
     * Flip (mirror) image vertically.  This transform is imperfect if there are
     * any partial MCU blocks on the bottom edge (see #TJXOPT_PERFECT.)
     */

    pub const TJXOP_VFLIP: crate::turbojpeg_h::TJXOP = 2;
    /* *
     * Flip (mirror) image horizontally.  This transform is imperfect if there
     * are any partial MCU blocks on the right edge (see #TJXOPT_PERFECT.)
     */

    pub const TJXOP_HFLIP: crate::turbojpeg_h::TJXOP = 1;
    /* *
     * Do not transform the position of the image pixels
     */

    pub const TJXOP_NONE: crate::turbojpeg_h::TJXOP = 0;

    /*
     * Copyright (C)2009-2015, 2017 D. R. Commander.  All Rights Reserved.
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
    /* *
     * @addtogroup TurboJPEG
     * TurboJPEG API.  This API provides an interface for generating, decoding, and
     * transforming planar YUV and JPEG images in memory.
     *
     * @anchor YUVnotes
     * YUV Image Format Notes
     * ----------------------
     * Technically, the JPEG format uses the YCbCr colorspace (which is technically
     * not a colorspace but a color transform), but per the convention of the
     * digital video community, the TurboJPEG API uses "YUV" to refer to an image
     * format consisting of Y, Cb, and Cr image planes.
     *
     * Each plane is simply a 2D array of bytes, each byte representing the value
     * of one of the components (Y, Cb, or Cr) at a particular location in the
     * image.  The width and height of each plane are determined by the image
     * width, height, and level of chrominance subsampling.   The luminance plane
     * width is the image width padded to the nearest multiple of the horizontal
     * subsampling factor (2 in the case of 4:2:0 and 4:2:2, 4 in the case of
     * 4:1:1, 1 in the case of 4:4:4 or grayscale.)  Similarly, the luminance plane
     * height is the image height padded to the nearest multiple of the vertical
     * subsampling factor (2 in the case of 4:2:0 or 4:4:0, 1 in the case of 4:4:4
     * or grayscale.)  This is irrespective of any additional padding that may be
     * specified as an argument to the various YUV functions.  The chrominance
     * plane width is equal to the luminance plane width divided by the horizontal
     * subsampling factor, and the chrominance plane height is equal to the
     * luminance plane height divided by the vertical subsampling factor.
     *
     * For example, if the source image is 35 x 35 pixels and 4:2:2 subsampling is
     * used, then the luminance plane would be 36 x 35 bytes, and each of the
     * chrominance planes would be 18 x 35 bytes.  If you specify a line padding of
     * 4 bytes on top of this, then the luminance plane would be 36 x 35 bytes, and
     * each of the chrominance planes would be 20 x 35 bytes.
     *
     * @{
     */
    /* *
     * The number of chrominance subsampling options
     */

    pub const TJ_NUMSAMP: libc::c_int = 6 as libc::c_int;

    /* *
     * The uncompressed source/destination image is stored in bottom-up (Windows,
     * OpenGL) order, not top-down (X11) order.
     */

    pub const TJFLAG_BOTTOMUP: libc::c_int = 2 as libc::c_int;
    /* *
     * When decompressing an image that was compressed using chrominance
     * subsampling, use the fastest chrominance upsampling algorithm available in
     * the underlying codec.  The default is to use smooth upsampling, which
     * creates a smooth transition between neighboring chrominance components in
     * order to reduce upsampling artifacts in the decompressed image.
     */

    pub const TJFLAG_FASTUPSAMPLE: libc::c_int = 256 as libc::c_int;
    /* *
     * Disable buffer (re)allocation.  If passed to one of the JPEG compression or
     * transform functions, this flag will cause those functions to generate an
     * error if the JPEG image buffer is invalid or too small rather than
     * attempting to allocate or reallocate that buffer.  This reproduces the
     * behavior of earlier versions of TurboJPEG.
     */

    pub const TJFLAG_NOREALLOC: libc::c_int = 1024 as libc::c_int;
    /* *
     * Use the fastest DCT/IDCT algorithm available in the underlying codec.  The
     * default if this flag is not specified is implementation-specific.  For
     * example, the implementation of TurboJPEG for libjpeg[-turbo] uses the fast
     * algorithm by default when compressing, because this has been shown to have
     * only a very slight effect on accuracy, but it uses the accurate algorithm
     * when decompressing, because this has been shown to have a larger effect.
     */

    pub const TJFLAG_FASTDCT: libc::c_int = 2048 as libc::c_int;
    /* *
     * Use the most accurate DCT/IDCT algorithm available in the underlying codec.
     * The default if this flag is not specified is implementation-specific.  For
     * example, the implementation of TurboJPEG for libjpeg[-turbo] uses the fast
     * algorithm by default when compressing, because this has been shown to have
     * only a very slight effect on accuracy, but it uses the accurate algorithm
     * when decompressing, because this has been shown to have a larger effect.
     */

    pub const TJFLAG_ACCURATEDCT: libc::c_int = 4096 as libc::c_int;
    /* *
     * Immediately discontinue the current compression/decompression/transform
     * operation if the underlying codec throws a warning (non-fatal error).  The
     * default behavior is to allow the operation to complete unless a fatal error
     * is encountered.
     */

    pub const TJFLAG_STOPONWARNING: libc::c_int = 8192 as libc::c_int;
    /* *
     * Use progressive entropy coding in JPEG images generated by the compression
     * and transform functions.  Progressive entropy coding will generally improve
     * compression relative to baseline entropy coding (the default), but it will
     * reduce compression and decompression performance considerably.
     */

    pub const TJFLAG_PROGRESSIVE: libc::c_int = 16384 as libc::c_int;
    /* *
     * This option will cause #tjTransform() to return an error if the transform is
     * not perfect.  Lossless transforms operate on MCU blocks, whose size depends
     * on the level of chrominance subsampling used (see #tjMCUWidth
     * and #tjMCUHeight.)  If the image's width or height is not evenly divisible
     * by the MCU block size, then there will be partial MCU blocks on the right
     * and/or bottom edges.  It is not possible to move these partial MCU blocks to
     * the top or left of the image, so any transform that would require that is
     * "imperfect."  If this option is not specified, then any partial MCU blocks
     * that cannot be transformed will be left in place, which will create
     * odd-looking strips on the right or bottom edge of the image.
     */
    /* *
     * This option will cause #tjTransform() to discard any partial MCU blocks that
     * cannot be transformed.
     */

    pub const TJXOPT_TRIM: libc::c_int = 2 as libc::c_int;
    /* *
     * This option will enable lossless cropping.  See #tjTransform() for more
     * information.
     */

    pub const TJXOPT_CROP: libc::c_int = 4 as libc::c_int;
    /* *
     * This option will discard the color data in the input image and produce
     * a grayscale output image.
     */

    pub const TJXOPT_GRAY: libc::c_int = 8 as libc::c_int;
    /* *
     * This option will prevent #tjTransform() from outputting a JPEG image for
     * this particular transform (this can be used in conjunction with a custom
     * filter to capture the transformed DCT coefficients without transcoding
     * them.)
     */

    pub const TJXOPT_NOOUTPUT: libc::c_int = 16 as libc::c_int;
    /* *
     * This option will enable progressive entropy coding in the output image
     * generated by this particular transform.  Progressive entropy coding will
     * generally improve compression relative to baseline entropy coding (the
     * default), but it will reduce compression and decompression performance
     * considerably.
     */
    /* *
     * This option will prevent #tjTransform() from copying any extra markers
     * (including EXIF and ICC profile data) from the source image to the output
     * image.
     */

    pub const TJXOPT_COPYNONE: libc::c_int = 64 as libc::c_int;
    /* Deprecated functions and macros */
    /* Backward compatibility functions and macros (nothing to see here) */

    pub const TJ_GRAYSCALE: libc::c_int = crate::turbojpeg_h::TJSAMP_GRAY as libc::c_int;

    /* *
     * @}
     */
}

#[c2rust::header_src = "/usr/include/stdlib.h:30"]
pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
        return crate::stdlib::strtol(
            __nptr,
            crate::stddef_h::NULL as *mut libc::c_void as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as libc::c_int;
    }
    use crate::stddef_h::NULL;
}
#[c2rust::header_src = "/usr/include/bits/stdlib-float.h:30"]
pub mod stdlib_float_h {
    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
        return crate::stdlib::strtod(
            __nptr,
            crate::stddef_h::NULL as *mut libc::c_void as *mut *mut libc::c_char,
        );
    }
    use crate::stddef_h::NULL;
    use crate::stdlib::strtod;
}

#[c2rust::header_src = "/usr/include/ctype.h:32"]
pub mod ctype_h {
    #[inline]

    pub unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
        return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
}

pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
pub use crate::stddef_h::NULL_0;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::stdlib::abs;
pub use crate::stdlib::exit;
pub use crate::stdlib::fclose;
pub use crate::stdlib::fopen;
pub use crate::stdlib::fread;
pub use crate::stdlib::free;
pub use crate::stdlib::fseek;
pub use crate::stdlib::ftell;
pub use crate::stdlib::fwrite;
pub use crate::stdlib::malloc;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
pub use crate::stdlib::printf;
pub use crate::stdlib::puts;
pub use crate::stdlib::snprintf;
pub use crate::stdlib::sscanf;
use crate::stdlib::strcasecmp;
use crate::stdlib::strchr;
use crate::stdlib::strerror;
use crate::stdlib::strlen;
use crate::stdlib::strncmp;
use crate::stdlib::strncpy;
use crate::stdlib::strrchr;
pub use crate::stdlib::strtod;
pub use crate::stdlib::strtol;
pub use crate::stdlib::SEEK_END;
pub use crate::stdlib::SEEK_SET;
pub use crate::stdlib_float_h::atof;
pub use crate::stdlib_h::atoi;
pub use crate::turbojpeg_h::tjAlloc;
pub use crate::turbojpeg_h::tjBlueOffset;
pub use crate::turbojpeg_h::tjBufSize;
pub use crate::turbojpeg_h::tjBufSizeYUV2;
pub use crate::turbojpeg_h::tjCompress2;
pub use crate::turbojpeg_h::tjCompressFromYUV;
pub use crate::turbojpeg_h::tjDecodeYUV;
pub use crate::turbojpeg_h::tjDecompress2;
pub use crate::turbojpeg_h::tjDecompressHeader3;
pub use crate::turbojpeg_h::tjDecompressToYUV2;
pub use crate::turbojpeg_h::tjDestroy;
pub use crate::turbojpeg_h::tjEncodeYUV3;
pub use crate::turbojpeg_h::tjFree;
pub use crate::turbojpeg_h::tjGetErrorCode;
pub use crate::turbojpeg_h::tjGetErrorStr;
pub use crate::turbojpeg_h::tjGetErrorStr2;
pub use crate::turbojpeg_h::tjGetScalingFactors;
pub use crate::turbojpeg_h::tjGreenOffset;
pub use crate::turbojpeg_h::tjInitCompress;
pub use crate::turbojpeg_h::tjInitDecompress;
pub use crate::turbojpeg_h::tjInitTransform;
pub use crate::turbojpeg_h::tjLoadImage;
pub use crate::turbojpeg_h::tjMCUHeight;
pub use crate::turbojpeg_h::tjMCUWidth;
pub use crate::turbojpeg_h::tjPixelSize;
pub use crate::turbojpeg_h::tjRedOffset;
pub use crate::turbojpeg_h::tjSaveImage;
pub use crate::turbojpeg_h::tjTransform;
pub use crate::turbojpeg_h::tjhandle;
pub use crate::turbojpeg_h::tjregion;
pub use crate::turbojpeg_h::tjscalingfactor;
pub use crate::turbojpeg_h::tjtransform;
pub use crate::turbojpeg_h::TJCS_YCbCr;
pub use crate::turbojpeg_h::TJCS;
pub use crate::turbojpeg_h::TJCS_CMYK;
pub use crate::turbojpeg_h::TJCS_GRAY;
pub use crate::turbojpeg_h::TJCS_RGB;
pub use crate::turbojpeg_h::TJCS_YCCK;
pub use crate::turbojpeg_h::TJERR;
pub use crate::turbojpeg_h::TJERR_FATAL;
pub use crate::turbojpeg_h::TJERR_WARNING;
pub use crate::turbojpeg_h::TJFLAG_ACCURATEDCT;
pub use crate::turbojpeg_h::TJFLAG_BOTTOMUP;
pub use crate::turbojpeg_h::TJFLAG_FASTDCT;
pub use crate::turbojpeg_h::TJFLAG_FASTUPSAMPLE;
pub use crate::turbojpeg_h::TJFLAG_NOREALLOC;
pub use crate::turbojpeg_h::TJFLAG_PROGRESSIVE;
pub use crate::turbojpeg_h::TJFLAG_STOPONWARNING;
pub use crate::turbojpeg_h::TJPF;
pub use crate::turbojpeg_h::TJPF_ABGR;
pub use crate::turbojpeg_h::TJPF_ARGB;
pub use crate::turbojpeg_h::TJPF_BGR;
pub use crate::turbojpeg_h::TJPF_BGRA;
pub use crate::turbojpeg_h::TJPF_BGRX;
pub use crate::turbojpeg_h::TJPF_CMYK;
pub use crate::turbojpeg_h::TJPF_GRAY;
pub use crate::turbojpeg_h::TJPF_RGB;
pub use crate::turbojpeg_h::TJPF_RGBA;
pub use crate::turbojpeg_h::TJPF_RGBX;
pub use crate::turbojpeg_h::TJPF_UNKNOWN;
pub use crate::turbojpeg_h::TJPF_XBGR;
pub use crate::turbojpeg_h::TJPF_XRGB;
pub use crate::turbojpeg_h::TJSAMP;
pub use crate::turbojpeg_h::TJSAMP_411;
pub use crate::turbojpeg_h::TJSAMP_420;
pub use crate::turbojpeg_h::TJSAMP_422;
pub use crate::turbojpeg_h::TJSAMP_440;
pub use crate::turbojpeg_h::TJSAMP_444;
pub use crate::turbojpeg_h::TJSAMP_GRAY;
pub use crate::turbojpeg_h::TJXOP;
pub use crate::turbojpeg_h::TJXOPT_COPYNONE;
pub use crate::turbojpeg_h::TJXOPT_CROP;
pub use crate::turbojpeg_h::TJXOPT_GRAY;
pub use crate::turbojpeg_h::TJXOPT_NOOUTPUT;
pub use crate::turbojpeg_h::TJXOPT_TRIM;
pub use crate::turbojpeg_h::TJXOP_HFLIP;
pub use crate::turbojpeg_h::TJXOP_NONE;
pub use crate::turbojpeg_h::TJXOP_ROT180;
pub use crate::turbojpeg_h::TJXOP_ROT270;
pub use crate::turbojpeg_h::TJXOP_ROT90;
pub use crate::turbojpeg_h::TJXOP_TRANSPOSE;
pub use crate::turbojpeg_h::TJXOP_TRANSVERSE;
pub use crate::turbojpeg_h::TJXOP_VFLIP;
pub use crate::turbojpeg_h::TJ_GRAYSCALE;
pub use crate::turbojpeg_h::TJ_NUMSAMP;

pub use crate::ctype_h::toupper;
pub use crate::jpeglib_h::JMSG_LENGTH_MAX;
pub use crate::stdlib::__ctype_toupper_loc;
use crate::stdlib::__errno_location;
use crate::stdlib::ceil;
use crate::stdlib::fabs;
use crate::stdlib::log10;
use crate::tjutil_h::getTime;
/*
 * Copyright (C)2009-2018 D. R. Commander.  All Rights Reserved.
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
#[no_mangle]

pub static mut tjErrorStr: [libc::c_char; 200] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];
#[no_mangle]

pub static mut tjErrorMsg: [libc::c_char; 200] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];
#[no_mangle]

pub static mut tjErrorLine: libc::c_int = -(1 as libc::c_int);
#[no_mangle]

pub static mut tjErrorCode: libc::c_int = -(1 as libc::c_int);
#[no_mangle]

pub static mut flags: libc::c_int = crate::turbojpeg_h::TJFLAG_NOREALLOC;
#[no_mangle]

pub static mut compOnly: libc::c_int = 0 as libc::c_int;
#[no_mangle]

pub static mut decompOnly: libc::c_int = 0 as libc::c_int;
#[no_mangle]

pub static mut doYUV: libc::c_int = 0 as libc::c_int;
#[no_mangle]

pub static mut quiet: libc::c_int = 0 as libc::c_int;
#[no_mangle]

pub static mut doTile: libc::c_int = 0 as libc::c_int;
#[no_mangle]

pub static mut pf: libc::c_int = crate::turbojpeg_h::TJPF_BGR as libc::c_int;
#[no_mangle]

pub static mut yuvPad: libc::c_int = 1 as libc::c_int;
#[no_mangle]

pub static mut doWrite: libc::c_int = 1 as libc::c_int;
#[no_mangle]

pub static mut ext: *mut libc::c_char =
    b"ppm\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]

pub static mut pixFormatStr: [*const libc::c_char; 12] = [
    b"RGB\x00" as *const u8 as *const libc::c_char,
    b"BGR\x00" as *const u8 as *const libc::c_char,
    b"RGBX\x00" as *const u8 as *const libc::c_char,
    b"BGRX\x00" as *const u8 as *const libc::c_char,
    b"XBGR\x00" as *const u8 as *const libc::c_char,
    b"XRGB\x00" as *const u8 as *const libc::c_char,
    b"GRAY\x00" as *const u8 as *const libc::c_char,
    b"\x00" as *const u8 as *const libc::c_char,
    b"\x00" as *const u8 as *const libc::c_char,
    b"\x00" as *const u8 as *const libc::c_char,
    b"\x00" as *const u8 as *const libc::c_char,
    b"CMYK\x00" as *const u8 as *const libc::c_char,
];
#[no_mangle]

pub static mut subNameLong: [*const libc::c_char; 6] = [
    b"4:4:4\x00" as *const u8 as *const libc::c_char,
    b"4:2:2\x00" as *const u8 as *const libc::c_char,
    b"4:2:0\x00" as *const u8 as *const libc::c_char,
    b"GRAY\x00" as *const u8 as *const libc::c_char,
    b"4:4:0\x00" as *const u8 as *const libc::c_char,
    b"4:1:1\x00" as *const u8 as *const libc::c_char,
];
#[no_mangle]

pub static mut csName: [*const libc::c_char; 5] = [
    b"RGB\x00" as *const u8 as *const libc::c_char,
    b"YCbCr\x00" as *const u8 as *const libc::c_char,
    b"GRAY\x00" as *const u8 as *const libc::c_char,
    b"CMYK\x00" as *const u8 as *const libc::c_char,
    b"YCCK\x00" as *const u8 as *const libc::c_char,
];
#[no_mangle]

pub static mut subName: [*const libc::c_char; 6] = [
    b"444\x00" as *const u8 as *const libc::c_char,
    b"422\x00" as *const u8 as *const libc::c_char,
    b"420\x00" as *const u8 as *const libc::c_char,
    b"GRAY\x00" as *const u8 as *const libc::c_char,
    b"440\x00" as *const u8 as *const libc::c_char,
    b"411\x00" as *const u8 as *const libc::c_char,
];
#[no_mangle]

pub static mut scalingFactors: *mut crate::turbojpeg_h::tjscalingfactor =
    crate::stddef_h::NULL_0 as *mut crate::turbojpeg_h::tjscalingfactor;
#[no_mangle]

pub static mut sf: crate::turbojpeg_h::tjscalingfactor = {
    let mut init = crate::turbojpeg_h::tjscalingfactor {
        num: 1 as libc::c_int,
        denom: 1 as libc::c_int,
    };
    init
};
#[no_mangle]

pub static mut nsf: libc::c_int = 0 as libc::c_int;
#[no_mangle]

pub static mut xformOp: libc::c_int = crate::turbojpeg_h::TJXOP_NONE as libc::c_int;
#[no_mangle]

pub static mut xformOpt: libc::c_int = 0 as libc::c_int;
#[no_mangle]

pub static mut customFilter: Option<
    unsafe extern "C" fn(
        _: *mut libc::c_short,
        _: crate::turbojpeg_h::tjregion,
        _: crate::turbojpeg_h::tjregion,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut crate::turbojpeg_h::tjtransform,
    ) -> libc::c_int,
> = None;
#[no_mangle]

pub static mut benchTime: libc::c_double = 5.0f64;
#[no_mangle]

pub static mut warmup: libc::c_double = 1.0f64;
#[no_mangle]

pub unsafe extern "C" fn formatName(
    mut subsamp: libc::c_int,
    mut cs: libc::c_int,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    if cs == crate::turbojpeg_h::TJCS_YCbCr as libc::c_int {
        return subNameLong[subsamp as usize] as *mut libc::c_char;
    } else if cs == crate::turbojpeg_h::TJCS_YCCK as libc::c_int
        || cs == crate::turbojpeg_h::TJCS_CMYK as libc::c_int
    {
        crate::stdlib::snprintf(
            buf,
            80 as libc::c_int as libc::c_ulong,
            b"%s %s\x00" as *const u8 as *const libc::c_char,
            csName[cs as usize],
            subNameLong[subsamp as usize],
        );
        return buf;
    } else {
        return csName[cs as usize] as *mut libc::c_char;
    };
}
#[no_mangle]

pub unsafe extern "C" fn sigfig(
    mut val: libc::c_double,
    mut figs: libc::c_int,
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
) -> *mut libc::c_char {
    let mut format: [libc::c_char; 80] = [0; 80];
    let mut digitsAfterDecimal: libc::c_int =
        figs - crate::stdlib::ceil(crate::stdlib::log10(crate::stdlib::fabs(val))) as libc::c_int;
    if digitsAfterDecimal < 1 as libc::c_int {
        crate::stdlib::snprintf(
            format.as_mut_ptr(),
            80 as libc::c_int as libc::c_ulong,
            b"%%.0f\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        crate::stdlib::snprintf(
            format.as_mut_ptr(),
            80 as libc::c_int as libc::c_ulong,
            b"%%.%df\x00" as *const u8 as *const libc::c_char,
            digitsAfterDecimal,
        );
    }
    crate::stdlib::snprintf(buf, len as libc::c_ulong, format.as_mut_ptr(), val);
    return buf;
}
/* Custom DCT filter which produces a negative of the image */
#[no_mangle]

pub unsafe extern "C" fn dummyDCTFilter(
    mut coeffs: *mut libc::c_short,
    mut arrayRegion: crate::turbojpeg_h::tjregion,
    mut planeRegion: crate::turbojpeg_h::tjregion,
    mut componentIndex: libc::c_int,
    mut transformIndex: libc::c_int,
    mut transform: *mut crate::turbojpeg_h::tjtransform,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < arrayRegion.w * arrayRegion.h {
        *coeffs.offset(i as isize) = -(*coeffs.offset(i as isize) as libc::c_int) as libc::c_short;
        i += 1
    }
    return 0 as libc::c_int;
}
/* Decompression test */
#[no_mangle]

pub unsafe extern "C" fn decomp(
    mut srcBuf: *mut libc::c_uchar,
    mut jpegBuf: *mut *mut libc::c_uchar,
    mut jpegSize: *mut libc::c_ulong,
    mut dstBuf: *mut libc::c_uchar,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut subsamp: libc::c_int,
    mut jpegQual: libc::c_int,
    mut fileName: *mut libc::c_char,
    mut tilew: libc::c_int,
    mut tileh: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut tempStr: [libc::c_char; 1024] = [0; 1024];
    let mut sizeStr: [libc::c_char; 20] =
        *::std::mem::transmute::<&[u8; 20], &mut [libc::c_char; 20]>(
            b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        );
    let mut qualStr: [libc::c_char; 6] =
        *::std::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"\x00\x00\x00\x00\x00\x00");
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut file: *mut crate::stdlib::FILE = crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE;
    let mut handle: crate::turbojpeg_h::tjhandle = crate::stddef_h::NULL_0 as *mut libc::c_void;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut iter: libc::c_int = 0 as libc::c_int;
    let mut dstBufAlloc: libc::c_int = 0 as libc::c_int;
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut elapsed: libc::c_double = 0.;
    let mut elapsedDecode: libc::c_double = 0.;
    let mut ps: libc::c_int = tjPixelSize[pf as usize];
    let mut scaledw: libc::c_int = (w * sf.num + sf.denom - 1 as libc::c_int) / sf.denom;
    let mut scaledh: libc::c_int = (h * sf.num + sf.denom - 1 as libc::c_int) / sf.denom;
    let mut pitch: libc::c_int = scaledw * ps;
    let mut ntilesw: libc::c_int = (w + tilew - 1 as libc::c_int) / tilew;
    let mut ntilesh: libc::c_int = (h + tileh - 1 as libc::c_int) / tileh;
    let mut dstPtr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut dstPtr2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut yuvBuf: *mut libc::c_uchar = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
    if jpegQual > 0 as libc::c_int {
        crate::stdlib::snprintf(
            qualStr.as_mut_ptr(),
            6 as libc::c_int as libc::c_ulong,
            b"_Q%d\x00" as *const u8 as *const libc::c_char,
            jpegQual,
        );
        qualStr[5 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char
    }
    handle = crate::turbojpeg_h::tjInitDecompress();
    if handle.is_null() {
        let mut _tjErrorCode: libc::c_int = crate::turbojpeg_h::tjGetErrorCode(handle);
        let mut _tjErrorStr: *mut libc::c_char = crate::turbojpeg_h::tjGetErrorStr2(handle);
        if flags & crate::turbojpeg_h::TJFLAG_STOPONWARNING == 0
            && _tjErrorCode == crate::turbojpeg_h::TJERR_WARNING as libc::c_int
        {
            if crate::stdlib::strncmp(
                tjErrorStr.as_mut_ptr(),
                _tjErrorStr,
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            ) != 0
                || crate::stdlib::strncmp(
                    tjErrorMsg.as_mut_ptr(),
                    b"executing tjInitDecompress()\x00" as *const u8 as *const libc::c_char,
                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                ) != 0
                || tjErrorCode != _tjErrorCode
                || tjErrorLine != 160 as libc::c_int
            {
                crate::stdlib::strncpy(
                    tjErrorStr.as_mut_ptr(),
                    _tjErrorStr,
                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                );
                crate::stdlib::strncpy(
                    tjErrorMsg.as_mut_ptr(),
                    b"executing tjInitDecompress()\x00" as *const u8 as *const libc::c_char,
                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                );
                tjErrorCode = _tjErrorCode;
                tjErrorLine = 160 as libc::c_int;
                crate::stdlib::printf(
                    b"WARNING in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                    160 as libc::c_int,
                    b"executing tjInitDecompress()\x00" as *const u8 as *const libc::c_char,
                    _tjErrorStr,
                );
            }
            current_block = 11194104282611034094;
        } else {
            crate::stdlib::printf(
                b"%s in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                if _tjErrorCode == crate::turbojpeg_h::TJERR_WARNING as libc::c_int {
                    b"WARNING\x00" as *const u8 as *const libc::c_char
                } else {
                    b"ERROR\x00" as *const u8 as *const libc::c_char
                },
                160 as libc::c_int,
                b"executing tjInitDecompress()\x00" as *const u8 as *const libc::c_char,
                _tjErrorStr,
            );
            retval = -(1 as libc::c_int);
            current_block = 126259514807107346;
        }
    } else {
        current_block = 11194104282611034094;
    }
    match current_block {
        11194104282611034094 => {
            if dstBuf.is_null() {
                dstBuf =
                    crate::stdlib::malloc((pitch * scaledh) as libc::c_ulong) as *mut libc::c_uchar;
                if dstBuf.is_null() {
                    crate::stdlib::printf(
                        b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                        164 as libc::c_int,
                        b"allocating destination buffer\x00" as *const u8 as *const libc::c_char,
                        crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                    );
                    retval = -(1 as libc::c_int);
                    current_block = 126259514807107346;
                } else {
                    dstBufAlloc = 1 as libc::c_int;
                    current_block = 8693738493027456495;
                }
            } else {
                current_block = 8693738493027456495;
            }
            match current_block {
                126259514807107346 => {}
                _ => {
                    /* Set the destination buffer to gray so we know whether the decompressor
                    attempted to write to it */
                    crate::stdlib::memset(
                        dstBuf as *mut libc::c_void,
                        127 as libc::c_int,
                        (pitch * scaledh) as libc::c_ulong,
                    );
                    if doYUV != 0 {
                        let mut width: libc::c_int = if doTile != 0 { tilew } else { scaledw };
                        let mut height: libc::c_int = if doTile != 0 { tileh } else { scaledh };
                        let mut yuvSize: libc::c_int =
                            crate::turbojpeg_h::tjBufSizeYUV2(width, yuvPad, height, subsamp)
                                as libc::c_int;
                        yuvBuf =
                            crate::stdlib::malloc(yuvSize as libc::c_ulong) as *mut libc::c_uchar;
                        if yuvBuf.is_null() {
                            crate::stdlib::printf(
                                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                177 as libc::c_int,
                                b"allocating YUV buffer\x00" as *const u8 as *const libc::c_char,
                                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                            );
                            retval = -(1 as libc::c_int);
                            current_block = 126259514807107346;
                        } else {
                            crate::stdlib::memset(
                                yuvBuf as *mut libc::c_void,
                                127 as libc::c_int,
                                yuvSize as libc::c_ulong,
                            );
                            current_block = 11743904203796629665;
                        }
                    } else {
                        current_block = 11743904203796629665;
                    }
                    match current_block {
                        126259514807107346 => {}
                        _ => {
                            /* Benchmark */
                            iter = -(1 as libc::c_int);
                            elapsedDecode = 0.0f64;
                            elapsed = elapsedDecode;
                            's_213: loop {
                                let mut tile: libc::c_int = 0 as libc::c_int;
                                let mut start: libc::c_double = crate::tjutil_h::getTime();
                                row = 0 as libc::c_int;
                                dstPtr = dstBuf;
                                while row < ntilesh {
                                    col = 0 as libc::c_int;
                                    dstPtr2 = dstPtr;
                                    while col < ntilesw {
                                        let mut width_0: libc::c_int = if doTile != 0 {
                                            if tilew < w - col * tilew {
                                                tilew
                                            } else {
                                                (w) - col * tilew
                                            }
                                        } else {
                                            scaledw
                                        };
                                        let mut height_0: libc::c_int = if doTile != 0 {
                                            if tileh < h - row * tileh {
                                                tileh
                                            } else {
                                                (h) - row * tileh
                                            }
                                        } else {
                                            scaledh
                                        };
                                        if doYUV != 0 {
                                            let mut startDecode: libc::c_double = 0.;
                                            if crate::turbojpeg_h::tjDecompressToYUV2(
                                                handle,
                                                *jpegBuf.offset(tile as isize),
                                                *jpegSize.offset(tile as isize),
                                                yuvBuf,
                                                width_0,
                                                yuvPad,
                                                height_0,
                                                flags,
                                            ) == -(1 as libc::c_int)
                                            {
                                                let mut _tjErrorCode_0: libc::c_int =
                                                    crate::turbojpeg_h::tjGetErrorCode(handle);
                                                let mut _tjErrorStr_0: *mut libc::c_char =
                                                    crate::turbojpeg_h::tjGetErrorStr2(handle);
                                                if flags & crate::turbojpeg_h::TJFLAG_STOPONWARNING
                                                    == 0
                                                    && _tjErrorCode_0
                                                        == crate::turbojpeg_h::TJERR_WARNING
                                                            as libc::c_int
                                                {
                                                    if crate::stdlib::strncmp(
                                                        tjErrorStr.as_mut_ptr(),
                                                        _tjErrorStr_0,
                                                        crate::jpeglib_h::JMSG_LENGTH_MAX
                                                            as libc::c_ulong,
                                                    ) != 0
                                                        || crate::stdlib::strncmp(
                                                            tjErrorMsg.as_mut_ptr(),
                                                            b"executing tjDecompressToYUV2()\x00"
                                                                as *const u8
                                                                as *const libc::c_char,
                                                            crate::jpeglib_h::JMSG_LENGTH_MAX
                                                                as libc::c_ulong,
                                                        ) != 0
                                                        || tjErrorCode != _tjErrorCode_0
                                                        || tjErrorLine != 200 as libc::c_int
                                                    {
                                                        crate::stdlib::strncpy(
                                                            tjErrorStr.as_mut_ptr(),
                                                            _tjErrorStr_0,
                                                            crate::jpeglib_h::JMSG_LENGTH_MAX
                                                                as libc::c_ulong,
                                                        );
                                                        crate::stdlib::strncpy(
                                                            tjErrorMsg.as_mut_ptr(),
                                                            b"executing tjDecompressToYUV2()\x00"
                                                                as *const u8
                                                                as *const libc::c_char,
                                                            crate::jpeglib_h::JMSG_LENGTH_MAX
                                                                as libc::c_ulong,
                                                        );
                                                        tjErrorCode = _tjErrorCode_0;
                                                        tjErrorLine = 200 as libc::c_int;
                                                        crate::stdlib::printf(b"WARNING in line %d while %s:\n%s\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   200 as
                                                                       libc::c_int,
                                                                   b"executing tjDecompressToYUV2()\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   _tjErrorStr_0);
                                                    }
                                                } else {
                                                    crate::stdlib::printf(
                                                        b"%s in line %d while %s:\n%s\n\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        if _tjErrorCode_0
                                                            == crate::turbojpeg_h::TJERR_WARNING
                                                                as libc::c_int
                                                        {
                                                            b"WARNING\x00" as *const u8
                                                                as *const libc::c_char
                                                        } else {
                                                            b"ERROR\x00" as *const u8
                                                                as *const libc::c_char
                                                        },
                                                        200 as libc::c_int,
                                                        b"executing tjDecompressToYUV2()\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        _tjErrorStr_0,
                                                    );
                                                    retval = -(1 as libc::c_int);
                                                    current_block = 126259514807107346;
                                                    break 's_213;
                                                }
                                            }
                                            startDecode = crate::tjutil_h::getTime();
                                            if crate::turbojpeg_h::tjDecodeYUV(
                                                handle, yuvBuf, yuvPad, subsamp, dstPtr2, width_0,
                                                pitch, height_0, pf, flags,
                                            ) == -(1 as libc::c_int)
                                            {
                                                let mut _tjErrorCode_1: libc::c_int =
                                                    crate::turbojpeg_h::tjGetErrorCode(handle);
                                                let mut _tjErrorStr_1: *mut libc::c_char =
                                                    crate::turbojpeg_h::tjGetErrorStr2(handle);
                                                if flags & crate::turbojpeg_h::TJFLAG_STOPONWARNING
                                                    == 0
                                                    && _tjErrorCode_1
                                                        == crate::turbojpeg_h::TJERR_WARNING
                                                            as libc::c_int
                                                {
                                                    if crate::stdlib::strncmp(
                                                        tjErrorStr.as_mut_ptr(),
                                                        _tjErrorStr_1,
                                                        crate::jpeglib_h::JMSG_LENGTH_MAX
                                                            as libc::c_ulong,
                                                    ) != 0
                                                        || crate::stdlib::strncmp(
                                                            tjErrorMsg.as_mut_ptr(),
                                                            b"executing tjDecodeYUV()\x00"
                                                                as *const u8
                                                                as *const libc::c_char,
                                                            crate::jpeglib_h::JMSG_LENGTH_MAX
                                                                as libc::c_ulong,
                                                        ) != 0
                                                        || tjErrorCode != _tjErrorCode_1
                                                        || tjErrorLine != 204 as libc::c_int
                                                    {
                                                        crate::stdlib::strncpy(
                                                            tjErrorStr.as_mut_ptr(),
                                                            _tjErrorStr_1,
                                                            crate::jpeglib_h::JMSG_LENGTH_MAX
                                                                as libc::c_ulong,
                                                        );
                                                        crate::stdlib::strncpy(
                                                            tjErrorMsg.as_mut_ptr(),
                                                            b"executing tjDecodeYUV()\x00"
                                                                as *const u8
                                                                as *const libc::c_char,
                                                            crate::jpeglib_h::JMSG_LENGTH_MAX
                                                                as libc::c_ulong,
                                                        );
                                                        tjErrorCode = _tjErrorCode_1;
                                                        tjErrorLine = 204 as libc::c_int;
                                                        crate::stdlib::printf(b"WARNING in line %d while %s:\n%s\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   204 as
                                                                       libc::c_int,
                                                                   b"executing tjDecodeYUV()\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   _tjErrorStr_1);
                                                    }
                                                } else {
                                                    crate::stdlib::printf(
                                                        b"%s in line %d while %s:\n%s\n\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        if _tjErrorCode_1
                                                            == crate::turbojpeg_h::TJERR_WARNING
                                                                as libc::c_int
                                                        {
                                                            b"WARNING\x00" as *const u8
                                                                as *const libc::c_char
                                                        } else {
                                                            b"ERROR\x00" as *const u8
                                                                as *const libc::c_char
                                                        },
                                                        204 as libc::c_int,
                                                        b"executing tjDecodeYUV()\x00" as *const u8
                                                            as *const libc::c_char,
                                                        _tjErrorStr_1,
                                                    );
                                                    retval = -(1 as libc::c_int);
                                                    current_block = 126259514807107346;
                                                    break 's_213;
                                                }
                                            }
                                            if iter >= 0 as libc::c_int {
                                                elapsedDecode +=
                                                    crate::tjutil_h::getTime() - startDecode
                                            }
                                        } else if crate::turbojpeg_h::tjDecompress2(
                                            handle,
                                            *jpegBuf.offset(tile as isize),
                                            *jpegSize.offset(tile as isize),
                                            dstPtr2,
                                            width_0,
                                            pitch,
                                            height_0,
                                            pf,
                                            flags,
                                        ) == -(1 as libc::c_int)
                                        {
                                            let mut _tjErrorCode_2: libc::c_int =
                                                crate::turbojpeg_h::tjGetErrorCode(handle);
                                            let mut _tjErrorStr_2: *mut libc::c_char =
                                                crate::turbojpeg_h::tjGetErrorStr2(handle);
                                            if flags & crate::turbojpeg_h::TJFLAG_STOPONWARNING == 0
                                                && _tjErrorCode_2
                                                    == crate::turbojpeg_h::TJERR_WARNING
                                                        as libc::c_int
                                            {
                                                if crate::stdlib::strncmp(
                                                    tjErrorStr.as_mut_ptr(),
                                                    _tjErrorStr_2,
                                                    crate::jpeglib_h::JMSG_LENGTH_MAX
                                                        as libc::c_ulong,
                                                ) != 0
                                                    || crate::stdlib::strncmp(
                                                        tjErrorMsg.as_mut_ptr(),
                                                        b"executing tjDecompress2()\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        crate::jpeglib_h::JMSG_LENGTH_MAX
                                                            as libc::c_ulong,
                                                    ) != 0
                                                    || tjErrorCode != _tjErrorCode_2
                                                    || tjErrorLine != 209 as libc::c_int
                                                {
                                                    crate::stdlib::strncpy(
                                                        tjErrorStr.as_mut_ptr(),
                                                        _tjErrorStr_2,
                                                        crate::jpeglib_h::JMSG_LENGTH_MAX
                                                            as libc::c_ulong,
                                                    );
                                                    crate::stdlib::strncpy(
                                                        tjErrorMsg.as_mut_ptr(),
                                                        b"executing tjDecompress2()\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        crate::jpeglib_h::JMSG_LENGTH_MAX
                                                            as libc::c_ulong,
                                                    );
                                                    tjErrorCode = _tjErrorCode_2;
                                                    tjErrorLine = 209 as libc::c_int;
                                                    crate::stdlib::printf(
                                                        b"WARNING in line %d while %s:\n%s\n\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        209 as libc::c_int,
                                                        b"executing tjDecompress2()\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        _tjErrorStr_2,
                                                    );
                                                }
                                            } else {
                                                crate::stdlib::printf(
                                                    b"%s in line %d while %s:\n%s\n\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    if _tjErrorCode_2
                                                        == crate::turbojpeg_h::TJERR_WARNING
                                                            as libc::c_int
                                                    {
                                                        b"WARNING\x00" as *const u8
                                                            as *const libc::c_char
                                                    } else {
                                                        b"ERROR\x00" as *const u8
                                                            as *const libc::c_char
                                                    },
                                                    209 as libc::c_int,
                                                    b"executing tjDecompress2()\x00" as *const u8
                                                        as *const libc::c_char,
                                                    _tjErrorStr_2,
                                                );
                                                retval = -(1 as libc::c_int);
                                                current_block = 126259514807107346;
                                                break 's_213;
                                            }
                                        }
                                        col += 1;
                                        tile += 1;
                                        dstPtr2 = dstPtr2.offset((ps * tilew) as isize)
                                    }
                                    row += 1;
                                    dstPtr = dstPtr.offset((pitch * tileh) as isize)
                                }
                                elapsed += crate::tjutil_h::getTime() - start;
                                if iter >= 0 as libc::c_int {
                                    iter += 1;
                                    if elapsed >= benchTime {
                                        current_block = 14851765859726653900;
                                        break;
                                    }
                                } else if elapsed >= warmup {
                                    iter = 0 as libc::c_int;
                                    elapsedDecode = 0.0f64;
                                    elapsed = elapsedDecode
                                }
                            }
                            match current_block {
                                126259514807107346 => {}
                                _ => {
                                    if doYUV != 0 {
                                        elapsed -= elapsedDecode
                                    }
                                    if crate::turbojpeg_h::tjDestroy(handle) == -(1 as libc::c_int)
                                    {
                                        let mut _tjErrorCode_3: libc::c_int =
                                            crate::turbojpeg_h::tjGetErrorCode(handle);
                                        let mut _tjErrorStr_3: *mut libc::c_char =
                                            crate::turbojpeg_h::tjGetErrorStr2(handle);
                                        if flags & crate::turbojpeg_h::TJFLAG_STOPONWARNING == 0
                                            && _tjErrorCode_3
                                                == crate::turbojpeg_h::TJERR_WARNING as libc::c_int
                                        {
                                            if crate::stdlib::strncmp(
                                                tjErrorStr.as_mut_ptr(),
                                                _tjErrorStr_3,
                                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                            ) != 0
                                                || crate::stdlib::strncmp(
                                                    tjErrorMsg.as_mut_ptr(),
                                                    b"executing tjDestroy()\x00" as *const u8
                                                        as *const libc::c_char,
                                                    crate::jpeglib_h::JMSG_LENGTH_MAX
                                                        as libc::c_ulong,
                                                ) != 0
                                                || tjErrorCode != _tjErrorCode_3
                                                || tjErrorLine != 223 as libc::c_int
                                            {
                                                crate::stdlib::strncpy(
                                                    tjErrorStr.as_mut_ptr(),
                                                    _tjErrorStr_3,
                                                    crate::jpeglib_h::JMSG_LENGTH_MAX
                                                        as libc::c_ulong,
                                                );
                                                crate::stdlib::strncpy(
                                                    tjErrorMsg.as_mut_ptr(),
                                                    b"executing tjDestroy()\x00" as *const u8
                                                        as *const libc::c_char,
                                                    crate::jpeglib_h::JMSG_LENGTH_MAX
                                                        as libc::c_ulong,
                                                );
                                                tjErrorCode = _tjErrorCode_3;
                                                tjErrorLine = 223 as libc::c_int;
                                                crate::stdlib::printf(
                                                    b"WARNING in line %d while %s:\n%s\n\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    223 as libc::c_int,
                                                    b"executing tjDestroy()\x00" as *const u8
                                                        as *const libc::c_char,
                                                    _tjErrorStr_3,
                                                );
                                            }
                                            current_block = 17917672080766325409;
                                        } else {
                                            crate::stdlib::printf(
                                                b"%s in line %d while %s:\n%s\n\x00" as *const u8
                                                    as *const libc::c_char,
                                                if _tjErrorCode_3
                                                    == crate::turbojpeg_h::TJERR_WARNING
                                                        as libc::c_int
                                                {
                                                    b"WARNING\x00" as *const u8
                                                        as *const libc::c_char
                                                } else {
                                                    b"ERROR\x00" as *const u8 as *const libc::c_char
                                                },
                                                223 as libc::c_int,
                                                b"executing tjDestroy()\x00" as *const u8
                                                    as *const libc::c_char,
                                                _tjErrorStr_3,
                                            );
                                            retval = -(1 as libc::c_int);
                                            current_block = 126259514807107346;
                                        }
                                    } else {
                                        current_block = 17917672080766325409;
                                    }
                                    match current_block {
                                        126259514807107346 => {}
                                        _ => {
                                            handle = crate::stddef_h::NULL_0 as *mut libc::c_void;
                                            if quiet != 0 {
                                                crate::stdlib::printf(
                                                    b"%-6s%s\x00" as *const u8
                                                        as *const libc::c_char,
                                                    sigfig(
                                                        (w * h) as libc::c_double / 1000000.0f64
                                                            * iter as libc::c_double
                                                            / elapsed,
                                                        4 as libc::c_int,
                                                        tempStr.as_mut_ptr(),
                                                        1024 as libc::c_int,
                                                    ),
                                                    if quiet == 2 as libc::c_int {
                                                        b"\n\x00" as *const u8
                                                            as *const libc::c_char
                                                    } else {
                                                        b"  \x00" as *const u8
                                                            as *const libc::c_char
                                                    },
                                                );
                                                if doYUV != 0 {
                                                    crate::stdlib::printf(
                                                        b"%s\n\x00" as *const u8
                                                            as *const libc::c_char,
                                                        sigfig(
                                                            (w * h) as libc::c_double
                                                                / 1000000.0f64
                                                                * iter as libc::c_double
                                                                / elapsedDecode,
                                                            4 as libc::c_int,
                                                            tempStr.as_mut_ptr(),
                                                            1024 as libc::c_int,
                                                        ),
                                                    );
                                                } else if quiet != 2 as libc::c_int {
                                                    crate::stdlib::printf(
                                                        b"\n\x00" as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                }
                                            } else {
                                                crate::stdlib::printf(
                                                    b"%s --> Frame rate:         %f fps\n\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    if doYUV != 0 {
                                                        b"Decomp to YUV\x00" as *const u8
                                                            as *const libc::c_char
                                                    } else {
                                                        b"Decompress   \x00" as *const u8
                                                            as *const libc::c_char
                                                    },
                                                    iter as libc::c_double / elapsed,
                                                );
                                                crate::stdlib::printf(b"                  Throughput:         %f Megapixels/sec\n\x00"
                                                           as *const u8 as
                                                           *const libc::c_char,
                                                       (w * h) as
                                                           libc::c_double /
                                                           1000000.0f64 *
                                                           iter as
                                                               libc::c_double
                                                           / elapsed);
                                                if doYUV != 0 {
                                                    crate::stdlib::printf(b"YUV Decode    --> Frame rate:         %f fps\n\x00"
                                                               as *const u8 as
                                                               *const libc::c_char,
                                                           iter as
                                                               libc::c_double
                                                               /
                                                               elapsedDecode);
                                                    crate::stdlib::printf(b"                  Throughput:         %f Megapixels/sec\n\x00"
                                                               as *const u8 as
                                                               *const libc::c_char,
                                                           (w * h) as
                                                               libc::c_double
                                                               / 1000000.0f64
                                                               *
                                                               iter as
                                                                   libc::c_double
                                                               /
                                                               elapsedDecode);
                                                }
                                            }
                                            if !(doWrite == 0) {
                                                if sf.num != 1 as libc::c_int
                                                    || sf.denom != 1 as libc::c_int
                                                {
                                                    crate::stdlib::snprintf(
                                                        sizeStr.as_mut_ptr(),
                                                        20 as libc::c_int as libc::c_ulong,
                                                        b"%d_%d\x00" as *const u8
                                                            as *const libc::c_char,
                                                        sf.num,
                                                        sf.denom,
                                                    );
                                                } else if tilew != w || tileh != h {
                                                    crate::stdlib::snprintf(
                                                        sizeStr.as_mut_ptr(),
                                                        20 as libc::c_int as libc::c_ulong,
                                                        b"%dx%d\x00" as *const u8
                                                            as *const libc::c_char,
                                                        tilew,
                                                        tileh,
                                                    );
                                                } else {
                                                    crate::stdlib::snprintf(
                                                        sizeStr.as_mut_ptr(),
                                                        20 as libc::c_int as libc::c_ulong,
                                                        b"full\x00" as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                }
                                                if decompOnly != 0 {
                                                    crate::stdlib::snprintf(
                                                        tempStr.as_mut_ptr(),
                                                        1024 as libc::c_int as libc::c_ulong,
                                                        b"%s_%s.%s\x00" as *const u8
                                                            as *const libc::c_char,
                                                        fileName,
                                                        sizeStr.as_mut_ptr(),
                                                        ext,
                                                    );
                                                } else {
                                                    crate::stdlib::snprintf(
                                                        tempStr.as_mut_ptr(),
                                                        1024 as libc::c_int as libc::c_ulong,
                                                        b"%s_%s%s_%s.%s\x00" as *const u8
                                                            as *const libc::c_char,
                                                        fileName,
                                                        subName[subsamp as usize],
                                                        qualStr.as_mut_ptr(),
                                                        sizeStr.as_mut_ptr(),
                                                        ext,
                                                    );
                                                }
                                                if crate::turbojpeg_h::tjSaveImage(
                                                    tempStr.as_mut_ptr(),
                                                    dstBuf,
                                                    scaledw,
                                                    0 as libc::c_int,
                                                    scaledh,
                                                    pf,
                                                    flags,
                                                ) == -(1 as libc::c_int)
                                                {
                                                    crate::stdlib::printf(
                                                        b"ERROR in line %d while %s:\n%s\n\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        263 as libc::c_int,
                                                        b"saving bitmap\x00" as *const u8
                                                            as *const libc::c_char,
                                                        crate::turbojpeg_h::tjGetErrorStr2(
                                                            crate::stddef_h::NULL_0
                                                                as *mut libc::c_void,
                                                        ),
                                                    );
                                                    retval = -(1 as libc::c_int)
                                                } else {
                                                    ptr = crate::stdlib::strrchr(
                                                        tempStr.as_mut_ptr(),
                                                        '.' as i32,
                                                    );
                                                    crate::stdlib::snprintf(
                                                        ptr,
                                                        (1024 as libc::c_int as libc::c_long
                                                            - ptr.wrapping_offset_from(
                                                                tempStr.as_mut_ptr(),
                                                            )
                                                                as libc::c_long)
                                                            as libc::c_ulong,
                                                        b"-err.%s\x00" as *const u8
                                                            as *const libc::c_char,
                                                        ext,
                                                    );
                                                    if !srcBuf.is_null()
                                                        && sf.num == 1 as libc::c_int
                                                        && sf.denom == 1 as libc::c_int
                                                    {
                                                        if quiet == 0 {
                                                            crate::stdlib::printf(b"Compression error written to %s.\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   tempStr.as_mut_ptr());
                                                        }
                                                        if subsamp
                                                            == crate::turbojpeg_h::TJ_GRAYSCALE
                                                        {
                                                            let mut index: libc::c_int = 0;
                                                            let mut index2: libc::c_int = 0;
                                                            row = 0 as libc::c_int;
                                                            index = 0 as libc::c_int;
                                                            while row < h {
                                                                col = 0 as libc::c_int;
                                                                index2 = index;
                                                                while col < w {
                                                                    let mut rindex: libc::c_int =
                                                                        index2
                                                                            + tjRedOffset
                                                                                [pf as usize];
                                                                    let mut gindex: libc::c_int =
                                                                        index2
                                                                            + tjGreenOffset
                                                                                [pf as usize];
                                                                    let mut bindex: libc::c_int =
                                                                        index2
                                                                            + tjBlueOffset
                                                                                [pf as usize];
                                                                    let mut y: libc::c_int =
                                                                        (*srcBuf
                                                                            .offset(rindex as isize)
                                                                            as libc::c_double
                                                                            * 0.299f64
                                                                            + *srcBuf.offset(
                                                                                gindex as isize,
                                                                            )
                                                                                as libc::c_double
                                                                                * 0.587f64
                                                                            + *srcBuf.offset(
                                                                                bindex as isize,
                                                                            )
                                                                                as libc::c_double
                                                                                * 0.114f64
                                                                            + 0.5f64)
                                                                            as libc::c_int;
                                                                    if y > 255 as libc::c_int {
                                                                        y = 255 as libc::c_int
                                                                    }
                                                                    if y < 0 as libc::c_int {
                                                                        y = 0 as libc::c_int
                                                                    }
                                                                    *dstBuf
                                                                        .offset(rindex as isize) =
                                                                        crate::stdlib::abs(
                                                                            *dstBuf.offset(
                                                                                rindex as isize,
                                                                            )
                                                                                as libc::c_int
                                                                                - y,
                                                                        )
                                                                            as libc::c_uchar;
                                                                    *dstBuf
                                                                        .offset(gindex as isize) =
                                                                        crate::stdlib::abs(
                                                                            *dstBuf.offset(
                                                                                gindex as isize,
                                                                            )
                                                                                as libc::c_int
                                                                                - y,
                                                                        )
                                                                            as libc::c_uchar;
                                                                    *dstBuf
                                                                        .offset(bindex as isize) =
                                                                        crate::stdlib::abs(
                                                                            *dstBuf.offset(
                                                                                bindex as isize,
                                                                            )
                                                                                as libc::c_int
                                                                                - y,
                                                                        )
                                                                            as libc::c_uchar;
                                                                    col += 1;
                                                                    index2 += ps
                                                                }
                                                                row += 1;
                                                                index += pitch
                                                            }
                                                        } else {
                                                            row = 0 as libc::c_int;
                                                            while row < h {
                                                                col = 0 as libc::c_int;
                                                                while col < w * ps {
                                                                    *dstBuf.offset(
                                                                        (pitch * row + col)
                                                                            as isize,
                                                                    ) = crate::stdlib::abs(
                                                                        *dstBuf.offset(
                                                                            (pitch * row + col)
                                                                                as isize,
                                                                        )
                                                                            as libc::c_int
                                                                            - *srcBuf.offset(
                                                                                (pitch * row + col)
                                                                                    as isize,
                                                                            )
                                                                                as libc::c_int,
                                                                    )
                                                                        as libc::c_uchar;
                                                                    col += 1
                                                                }
                                                                row += 1
                                                            }
                                                        }
                                                        if crate::turbojpeg_h::tjSaveImage(
                                                            tempStr.as_mut_ptr(),
                                                            dstBuf,
                                                            w,
                                                            0 as libc::c_int,
                                                            h,
                                                            pf,
                                                            flags,
                                                        ) == -(1 as libc::c_int)
                                                        {
                                                            crate::stdlib::printf(b"ERROR in line %d while %s:\n%s\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   294 as
                                                                       libc::c_int,
                                                                   b"saving bitmap\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   crate::turbojpeg_h::tjGetErrorStr2(crate::stddef_h::NULL_0
                                                                                      as
                                                                                      *mut libc::c_void));
                                                            retval = -(1 as libc::c_int)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !file.is_null() {
        crate::stdlib::fclose(file);
    }
    if !handle.is_null() {
        crate::turbojpeg_h::tjDestroy(handle);
    }
    if !dstBuf.is_null() && dstBufAlloc != 0 {
        crate::stdlib::free(dstBuf as *mut libc::c_void);
    }
    if !yuvBuf.is_null() {
        crate::stdlib::free(yuvBuf as *mut libc::c_void);
    }
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn fullTest(
    mut srcBuf: *mut libc::c_uchar,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut subsamp: libc::c_int,
    mut jpegQual: libc::c_int,
    mut fileName: *mut libc::c_char,
) -> libc::c_int {
    let mut tempStr: [libc::c_char; 1024] = [0; 1024];
    let mut tempStr2: [libc::c_char; 80] = [0; 80];
    let mut file: *mut crate::stdlib::FILE = crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE;
    let mut handle: crate::turbojpeg_h::tjhandle = crate::stddef_h::NULL_0 as *mut libc::c_void;
    let mut jpegBuf: *mut *mut libc::c_uchar = crate::stddef_h::NULL_0 as *mut *mut libc::c_uchar;
    let mut yuvBuf: *mut libc::c_uchar = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
    let mut tmpBuf: *mut libc::c_uchar = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
    let mut srcPtr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut srcPtr2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut start: libc::c_double = 0.;
    let mut elapsed: libc::c_double = 0.;
    let mut elapsedEncode: libc::c_double = 0.;
    let mut totalJpegSize: libc::c_int = 0 as libc::c_int;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tilew: libc::c_int = w;
    let mut tileh: libc::c_int = h;
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut iter: libc::c_int = 0;
    let mut yuvSize: libc::c_int = 0 as libc::c_int;
    let mut jpegSize: *mut libc::c_ulong = crate::stddef_h::NULL_0 as *mut libc::c_ulong;
    let mut ps: libc::c_int = tjPixelSize[pf as usize];
    let mut ntilesw: libc::c_int = 1 as libc::c_int;
    let mut ntilesh: libc::c_int = 1 as libc::c_int;
    let mut pitch: libc::c_int = w * ps;
    let mut pfStr: *const libc::c_char = pixFormatStr[pf as usize];
    tmpBuf = crate::stdlib::malloc((pitch * h) as libc::c_ulong) as *mut libc::c_uchar;
    if tmpBuf.is_null() {
        crate::stdlib::printf(
            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
            323 as libc::c_int,
            b"allocating temporary image buffer\x00" as *const u8 as *const libc::c_char,
            crate::stdlib::strerror(*crate::stdlib::__errno_location()),
        );
        retval = -(1 as libc::c_int)
    } else {
        if quiet == 0 {
            crate::stdlib::printf(
                b">>>>>  %s (%s) <--> JPEG %s Q%d  <<<<<\n\x00" as *const u8 as *const libc::c_char,
                pfStr,
                if flags & crate::turbojpeg_h::TJFLAG_BOTTOMUP != 0 {
                    b"Bottom-up\x00" as *const u8 as *const libc::c_char
                } else {
                    b"Top-down\x00" as *const u8 as *const libc::c_char
                },
                subNameLong[subsamp as usize],
                jpegQual,
            );
        }
        tilew = (if doTile != 0 { 8 as libc::c_int } else { w });
        tileh = (if doTile != 0 { 8 as libc::c_int } else { h });
        's_73: loop {
            if tilew > w {
                tilew = w
            }
            if tileh > h {
                tileh = h
            }
            ntilesw = (w + tilew - 1 as libc::c_int) / tilew;
            ntilesh = (h + tileh - 1 as libc::c_int) / tileh;
            jpegBuf = crate::stdlib::malloc(
                (::std::mem::size_of::<*mut libc::c_uchar>() as libc::c_ulong)
                    .wrapping_mul(ntilesw as libc::c_ulong)
                    .wrapping_mul(ntilesh as libc::c_ulong),
            ) as *mut *mut libc::c_uchar;
            if jpegBuf.is_null() {
                crate::stdlib::printf(
                    b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                    339 as libc::c_int,
                    b"allocating JPEG tile array\x00" as *const u8 as *const libc::c_char,
                    crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                );
                retval = -(1 as libc::c_int);
                break;
            } else {
                crate::stdlib::memset(
                    jpegBuf as *mut libc::c_void,
                    0 as libc::c_int,
                    (::std::mem::size_of::<*mut libc::c_uchar>() as libc::c_ulong)
                        .wrapping_mul(ntilesw as libc::c_ulong)
                        .wrapping_mul(ntilesh as libc::c_ulong),
                );
                jpegSize = crate::stdlib::malloc(
                    (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                        .wrapping_mul(ntilesw as libc::c_ulong)
                        .wrapping_mul(ntilesh as libc::c_ulong),
                ) as *mut libc::c_ulong;
                if jpegSize.is_null() {
                    crate::stdlib::printf(
                        b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                        343 as libc::c_int,
                        b"allocating JPEG size array\x00" as *const u8 as *const libc::c_char,
                        crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                    );
                    retval = -(1 as libc::c_int);
                    break;
                } else {
                    crate::stdlib::memset(
                        jpegSize as *mut libc::c_void,
                        0 as libc::c_int,
                        (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                            .wrapping_mul(ntilesw as libc::c_ulong)
                            .wrapping_mul(ntilesh as libc::c_ulong),
                    );
                    if flags & crate::turbojpeg_h::TJFLAG_NOREALLOC != 0 as libc::c_int {
                        i = 0 as libc::c_int;
                        while i < ntilesw * ntilesh {
                            let ref mut fresh0 = *jpegBuf.offset(i as isize);
                            *fresh0 = crate::turbojpeg_h::tjAlloc(crate::turbojpeg_h::tjBufSize(
                                tilew, tileh, subsamp,
                            )
                                as libc::c_int);
                            if (*fresh0).is_null() {
                                crate::stdlib::printf(
                                    b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    350 as libc::c_int,
                                    b"allocating JPEG tiles\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                                );
                                retval = -(1 as libc::c_int);
                                break 's_73;
                            } else {
                                i += 1
                            }
                        }
                    }
                    /* Compression test */
                    if quiet == 1 as libc::c_int {
                        crate::stdlib::printf(
                            b"%-4s (%s)  %-5s    %-3d   \x00" as *const u8 as *const libc::c_char,
                            pfStr,
                            if flags & crate::turbojpeg_h::TJFLAG_BOTTOMUP != 0 {
                                b"BU\x00" as *const u8 as *const libc::c_char
                            } else {
                                b"TD\x00" as *const u8 as *const libc::c_char
                            },
                            subNameLong[subsamp as usize],
                            jpegQual,
                        );
                    }
                    i = 0 as libc::c_int;
                    while i < h {
                        crate::stdlib::memcpy(
                            &mut *tmpBuf.offset((pitch * i) as isize) as *mut libc::c_uchar
                                as *mut libc::c_void,
                            &mut *srcBuf.offset((w * ps * i) as isize) as *mut libc::c_uchar
                                as *const libc::c_void,
                            (w * ps) as libc::c_ulong,
                        );
                        i += 1
                    }
                    handle = crate::turbojpeg_h::tjInitCompress();
                    if handle.is_null() {
                        let mut _tjErrorCode: libc::c_int =
                            crate::turbojpeg_h::tjGetErrorCode(handle);
                        let mut _tjErrorStr: *mut libc::c_char =
                            crate::turbojpeg_h::tjGetErrorStr2(handle);
                        if flags & crate::turbojpeg_h::TJFLAG_STOPONWARNING == 0
                            && _tjErrorCode == crate::turbojpeg_h::TJERR_WARNING as libc::c_int
                        {
                            if crate::stdlib::strncmp(
                                tjErrorStr.as_mut_ptr(),
                                _tjErrorStr,
                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                            ) != 0
                                || crate::stdlib::strncmp(
                                    tjErrorMsg.as_mut_ptr(),
                                    b"executing tjInitCompress()\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                ) != 0
                                || tjErrorCode != _tjErrorCode
                                || tjErrorLine != 361 as libc::c_int
                            {
                                crate::stdlib::strncpy(
                                    tjErrorStr.as_mut_ptr(),
                                    _tjErrorStr,
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                );
                                crate::stdlib::strncpy(
                                    tjErrorMsg.as_mut_ptr(),
                                    b"executing tjInitCompress()\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                );
                                tjErrorCode = _tjErrorCode;
                                tjErrorLine = 361 as libc::c_int;
                                crate::stdlib::printf(
                                    b"WARNING in line %d while %s:\n%s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    361 as libc::c_int,
                                    b"executing tjInitCompress()\x00" as *const u8
                                        as *const libc::c_char,
                                    _tjErrorStr,
                                );
                            }
                        } else {
                            crate::stdlib::printf(
                                b"%s in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                if _tjErrorCode == crate::turbojpeg_h::TJERR_WARNING as libc::c_int
                                {
                                    b"WARNING\x00" as *const u8 as *const libc::c_char
                                } else {
                                    b"ERROR\x00" as *const u8 as *const libc::c_char
                                },
                                361 as libc::c_int,
                                b"executing tjInitCompress()\x00" as *const u8
                                    as *const libc::c_char,
                                _tjErrorStr,
                            );
                            retval = -(1 as libc::c_int);
                            break;
                        }
                    }
                    if doYUV != 0 {
                        yuvSize = crate::turbojpeg_h::tjBufSizeYUV2(tilew, yuvPad, tileh, subsamp)
                            as libc::c_int;
                        yuvBuf =
                            crate::stdlib::malloc(yuvSize as libc::c_ulong) as *mut libc::c_uchar;
                        if yuvBuf.is_null() {
                            crate::stdlib::printf(
                                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                366 as libc::c_int,
                                b"allocating YUV buffer\x00" as *const u8 as *const libc::c_char,
                                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                            );
                            retval = -(1 as libc::c_int);
                            break;
                        } else {
                            crate::stdlib::memset(
                                yuvBuf as *mut libc::c_void,
                                127 as libc::c_int,
                                yuvSize as libc::c_ulong,
                            );
                        }
                    }
                    /* Benchmark */
                    iter = -(1 as libc::c_int);
                    elapsedEncode = 0.0f64;
                    elapsed = elapsedEncode;
                    loop {
                        let mut tile: libc::c_int = 0 as libc::c_int;
                        totalJpegSize = 0 as libc::c_int;
                        start = crate::tjutil_h::getTime();
                        row = 0 as libc::c_int;
                        srcPtr = srcBuf;
                        while row < ntilesh {
                            col = 0 as libc::c_int;
                            srcPtr2 = srcPtr;
                            while col < ntilesw {
                                let mut width: libc::c_int = if tilew < w - col * tilew {
                                    tilew
                                } else {
                                    (w) - col * tilew
                                };
                                let mut height: libc::c_int = if tileh < h - row * tileh {
                                    tileh
                                } else {
                                    (h) - row * tileh
                                };
                                if doYUV != 0 {
                                    let mut startEncode: libc::c_double =
                                        crate::tjutil_h::getTime();
                                    if crate::turbojpeg_h::tjEncodeYUV3(
                                        handle, srcPtr2, width, pitch, height, pf, yuvBuf, yuvPad,
                                        subsamp, flags,
                                    ) == -(1 as libc::c_int)
                                    {
                                        let mut _tjErrorCode_0: libc::c_int =
                                            crate::turbojpeg_h::tjGetErrorCode(handle);
                                        let mut _tjErrorStr_0: *mut libc::c_char =
                                            crate::turbojpeg_h::tjGetErrorStr2(handle);
                                        if flags & crate::turbojpeg_h::TJFLAG_STOPONWARNING == 0
                                            && _tjErrorCode_0
                                                == crate::turbojpeg_h::TJERR_WARNING as libc::c_int
                                        {
                                            if crate::stdlib::strncmp(
                                                tjErrorStr.as_mut_ptr(),
                                                _tjErrorStr_0,
                                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                            ) != 0
                                                || crate::stdlib::strncmp(
                                                    tjErrorMsg.as_mut_ptr(),
                                                    b"executing tjEncodeYUV3()\x00" as *const u8
                                                        as *const libc::c_char,
                                                    crate::jpeglib_h::JMSG_LENGTH_MAX
                                                        as libc::c_ulong,
                                                ) != 0
                                                || tjErrorCode != _tjErrorCode_0
                                                || tjErrorLine != 390 as libc::c_int
                                            {
                                                crate::stdlib::strncpy(
                                                    tjErrorStr.as_mut_ptr(),
                                                    _tjErrorStr_0,
                                                    crate::jpeglib_h::JMSG_LENGTH_MAX
                                                        as libc::c_ulong,
                                                );
                                                crate::stdlib::strncpy(
                                                    tjErrorMsg.as_mut_ptr(),
                                                    b"executing tjEncodeYUV3()\x00" as *const u8
                                                        as *const libc::c_char,
                                                    crate::jpeglib_h::JMSG_LENGTH_MAX
                                                        as libc::c_ulong,
                                                );
                                                tjErrorCode = _tjErrorCode_0;
                                                tjErrorLine = 390 as libc::c_int;
                                                crate::stdlib::printf(
                                                    b"WARNING in line %d while %s:\n%s\n\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    390 as libc::c_int,
                                                    b"executing tjEncodeYUV3()\x00" as *const u8
                                                        as *const libc::c_char,
                                                    _tjErrorStr_0,
                                                );
                                            }
                                        } else {
                                            crate::stdlib::printf(
                                                b"%s in line %d while %s:\n%s\n\x00" as *const u8
                                                    as *const libc::c_char,
                                                if _tjErrorCode_0
                                                    == crate::turbojpeg_h::TJERR_WARNING
                                                        as libc::c_int
                                                {
                                                    b"WARNING\x00" as *const u8
                                                        as *const libc::c_char
                                                } else {
                                                    b"ERROR\x00" as *const u8 as *const libc::c_char
                                                },
                                                390 as libc::c_int,
                                                b"executing tjEncodeYUV3()\x00" as *const u8
                                                    as *const libc::c_char,
                                                _tjErrorStr_0,
                                            );
                                            retval = -(1 as libc::c_int);
                                            break 's_73;
                                        }
                                    }
                                    if iter >= 0 as libc::c_int {
                                        elapsedEncode += crate::tjutil_h::getTime() - startEncode
                                    }
                                    if crate::turbojpeg_h::tjCompressFromYUV(
                                        handle,
                                        yuvBuf,
                                        width,
                                        yuvPad,
                                        height,
                                        subsamp,
                                        &mut *jpegBuf.offset(tile as isize),
                                        &mut *jpegSize.offset(tile as isize),
                                        jpegQual,
                                        flags,
                                    ) == -(1 as libc::c_int)
                                    {
                                        let mut _tjErrorCode_1: libc::c_int =
                                            crate::turbojpeg_h::tjGetErrorCode(handle);
                                        let mut _tjErrorStr_1: *mut libc::c_char =
                                            crate::turbojpeg_h::tjGetErrorStr2(handle);
                                        if flags & crate::turbojpeg_h::TJFLAG_STOPONWARNING == 0
                                            && _tjErrorCode_1
                                                == crate::turbojpeg_h::TJERR_WARNING as libc::c_int
                                        {
                                            if crate::stdlib::strncmp(
                                                tjErrorStr.as_mut_ptr(),
                                                _tjErrorStr_1,
                                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                            ) != 0
                                                || crate::stdlib::strncmp(
                                                    tjErrorMsg.as_mut_ptr(),
                                                    b"executing tjCompressFromYUV()\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    crate::jpeglib_h::JMSG_LENGTH_MAX
                                                        as libc::c_ulong,
                                                ) != 0
                                                || tjErrorCode != _tjErrorCode_1
                                                || tjErrorLine != 395 as libc::c_int
                                            {
                                                crate::stdlib::strncpy(
                                                    tjErrorStr.as_mut_ptr(),
                                                    _tjErrorStr_1,
                                                    crate::jpeglib_h::JMSG_LENGTH_MAX
                                                        as libc::c_ulong,
                                                );
                                                crate::stdlib::strncpy(
                                                    tjErrorMsg.as_mut_ptr(),
                                                    b"executing tjCompressFromYUV()\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    crate::jpeglib_h::JMSG_LENGTH_MAX
                                                        as libc::c_ulong,
                                                );
                                                tjErrorCode = _tjErrorCode_1;
                                                tjErrorLine = 395 as libc::c_int;
                                                crate::stdlib::printf(
                                                    b"WARNING in line %d while %s:\n%s\n\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    395 as libc::c_int,
                                                    b"executing tjCompressFromYUV()\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    _tjErrorStr_1,
                                                );
                                            }
                                        } else {
                                            crate::stdlib::printf(
                                                b"%s in line %d while %s:\n%s\n\x00" as *const u8
                                                    as *const libc::c_char,
                                                if _tjErrorCode_1
                                                    == crate::turbojpeg_h::TJERR_WARNING
                                                        as libc::c_int
                                                {
                                                    b"WARNING\x00" as *const u8
                                                        as *const libc::c_char
                                                } else {
                                                    b"ERROR\x00" as *const u8 as *const libc::c_char
                                                },
                                                395 as libc::c_int,
                                                b"executing tjCompressFromYUV()\x00" as *const u8
                                                    as *const libc::c_char,
                                                _tjErrorStr_1,
                                            );
                                            retval = -(1 as libc::c_int);
                                            break 's_73;
                                        }
                                    }
                                } else if crate::turbojpeg_h::tjCompress2(
                                    handle,
                                    srcPtr2,
                                    width,
                                    pitch,
                                    height,
                                    pf,
                                    &mut *jpegBuf.offset(tile as isize),
                                    &mut *jpegSize.offset(tile as isize),
                                    subsamp,
                                    jpegQual,
                                    flags,
                                ) == -(1 as libc::c_int)
                                {
                                    let mut _tjErrorCode_2: libc::c_int =
                                        crate::turbojpeg_h::tjGetErrorCode(handle);
                                    let mut _tjErrorStr_2: *mut libc::c_char =
                                        crate::turbojpeg_h::tjGetErrorStr2(handle);
                                    if flags & crate::turbojpeg_h::TJFLAG_STOPONWARNING == 0
                                        && _tjErrorCode_2
                                            == crate::turbojpeg_h::TJERR_WARNING as libc::c_int
                                    {
                                        if crate::stdlib::strncmp(
                                            tjErrorStr.as_mut_ptr(),
                                            _tjErrorStr_2,
                                            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                        ) != 0
                                            || crate::stdlib::strncmp(
                                                tjErrorMsg.as_mut_ptr(),
                                                b"executing tjCompress2()\x00" as *const u8
                                                    as *const libc::c_char,
                                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                            ) != 0
                                            || tjErrorCode != _tjErrorCode_2
                                            || tjErrorLine != 400 as libc::c_int
                                        {
                                            crate::stdlib::strncpy(
                                                tjErrorStr.as_mut_ptr(),
                                                _tjErrorStr_2,
                                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                            );
                                            crate::stdlib::strncpy(
                                                tjErrorMsg.as_mut_ptr(),
                                                b"executing tjCompress2()\x00" as *const u8
                                                    as *const libc::c_char,
                                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                            );
                                            tjErrorCode = _tjErrorCode_2;
                                            tjErrorLine = 400 as libc::c_int;
                                            crate::stdlib::printf(
                                                b"WARNING in line %d while %s:\n%s\n\x00"
                                                    as *const u8
                                                    as *const libc::c_char,
                                                400 as libc::c_int,
                                                b"executing tjCompress2()\x00" as *const u8
                                                    as *const libc::c_char,
                                                _tjErrorStr_2,
                                            );
                                        }
                                    } else {
                                        crate::stdlib::printf(
                                            b"%s in line %d while %s:\n%s\n\x00" as *const u8
                                                as *const libc::c_char,
                                            if _tjErrorCode_2
                                                == crate::turbojpeg_h::TJERR_WARNING as libc::c_int
                                            {
                                                b"WARNING\x00" as *const u8 as *const libc::c_char
                                            } else {
                                                b"ERROR\x00" as *const u8 as *const libc::c_char
                                            },
                                            400 as libc::c_int,
                                            b"executing tjCompress2()\x00" as *const u8
                                                as *const libc::c_char,
                                            _tjErrorStr_2,
                                        );
                                        retval = -(1 as libc::c_int);
                                        break 's_73;
                                    }
                                }
                                totalJpegSize = (totalJpegSize as libc::c_ulong)
                                    .wrapping_add(*jpegSize.offset(tile as isize))
                                    as libc::c_int
                                    as libc::c_int;
                                col += 1;
                                tile += 1;
                                srcPtr2 = srcPtr2.offset((ps * tilew) as isize)
                            }
                            row += 1;
                            srcPtr = srcPtr.offset((pitch * tileh) as isize)
                        }
                        elapsed += crate::tjutil_h::getTime() - start;
                        if iter >= 0 as libc::c_int {
                            iter += 1;
                            if elapsed >= benchTime {
                                break;
                            }
                        } else if elapsed >= warmup {
                            iter = 0 as libc::c_int;
                            elapsedEncode = 0.0f64;
                            elapsed = elapsedEncode
                        }
                    }
                    if doYUV != 0 {
                        elapsed -= elapsedEncode
                    }
                    if crate::turbojpeg_h::tjDestroy(handle) == -(1 as libc::c_int) {
                        let mut _tjErrorCode_3: libc::c_int =
                            crate::turbojpeg_h::tjGetErrorCode(handle);
                        let mut _tjErrorStr_3: *mut libc::c_char =
                            crate::turbojpeg_h::tjGetErrorStr2(handle);
                        if flags & crate::turbojpeg_h::TJFLAG_STOPONWARNING == 0
                            && _tjErrorCode_3 == crate::turbojpeg_h::TJERR_WARNING as libc::c_int
                        {
                            if crate::stdlib::strncmp(
                                tjErrorStr.as_mut_ptr(),
                                _tjErrorStr_3,
                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                            ) != 0
                                || crate::stdlib::strncmp(
                                    tjErrorMsg.as_mut_ptr(),
                                    b"executing tjDestroy()\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                ) != 0
                                || tjErrorCode != _tjErrorCode_3
                                || tjErrorLine != 416 as libc::c_int
                            {
                                crate::stdlib::strncpy(
                                    tjErrorStr.as_mut_ptr(),
                                    _tjErrorStr_3,
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                );
                                crate::stdlib::strncpy(
                                    tjErrorMsg.as_mut_ptr(),
                                    b"executing tjDestroy()\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                );
                                tjErrorCode = _tjErrorCode_3;
                                tjErrorLine = 416 as libc::c_int;
                                crate::stdlib::printf(
                                    b"WARNING in line %d while %s:\n%s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    416 as libc::c_int,
                                    b"executing tjDestroy()\x00" as *const u8
                                        as *const libc::c_char,
                                    _tjErrorStr_3,
                                );
                            }
                        } else {
                            crate::stdlib::printf(
                                b"%s in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                if _tjErrorCode_3
                                    == crate::turbojpeg_h::TJERR_WARNING as libc::c_int
                                {
                                    b"WARNING\x00" as *const u8 as *const libc::c_char
                                } else {
                                    b"ERROR\x00" as *const u8 as *const libc::c_char
                                },
                                416 as libc::c_int,
                                b"executing tjDestroy()\x00" as *const u8 as *const libc::c_char,
                                _tjErrorStr_3,
                            );
                            retval = -(1 as libc::c_int);
                            break;
                        }
                    }
                    handle = crate::stddef_h::NULL_0 as *mut libc::c_void;
                    if quiet == 1 as libc::c_int {
                        crate::stdlib::printf(
                            b"%-5d  %-5d   \x00" as *const u8 as *const libc::c_char,
                            tilew,
                            tileh,
                        );
                    }
                    if quiet != 0 {
                        if doYUV != 0 {
                            crate::stdlib::printf(
                                b"%-6s%s\x00" as *const u8 as *const libc::c_char,
                                sigfig(
                                    (w * h) as libc::c_double / 1000000.0f64
                                        * iter as libc::c_double
                                        / elapsedEncode,
                                    4 as libc::c_int,
                                    tempStr.as_mut_ptr(),
                                    1024 as libc::c_int,
                                ),
                                if quiet == 2 as libc::c_int {
                                    b"\n\x00" as *const u8 as *const libc::c_char
                                } else {
                                    b"  \x00" as *const u8 as *const libc::c_char
                                },
                            );
                        }
                        crate::stdlib::printf(
                            b"%-6s%s\x00" as *const u8 as *const libc::c_char,
                            sigfig(
                                (w * h) as libc::c_double / 1000000.0f64 * iter as libc::c_double
                                    / elapsed,
                                4 as libc::c_int,
                                tempStr.as_mut_ptr(),
                                1024 as libc::c_int,
                            ),
                            if quiet == 2 as libc::c_int {
                                b"\n\x00" as *const u8 as *const libc::c_char
                            } else {
                                b"  \x00" as *const u8 as *const libc::c_char
                            },
                        );
                        crate::stdlib::printf(
                            b"%-6s%s\x00" as *const u8 as *const libc::c_char,
                            sigfig(
                                (w * h * ps) as libc::c_double / totalJpegSize as libc::c_double,
                                4 as libc::c_int,
                                tempStr2.as_mut_ptr(),
                                80 as libc::c_int,
                            ),
                            if quiet == 2 as libc::c_int {
                                b"\n\x00" as *const u8 as *const libc::c_char
                            } else {
                                b"  \x00" as *const u8 as *const libc::c_char
                            },
                        );
                    } else {
                        crate::stdlib::printf(
                            b"\n%s size: %d x %d\n\x00" as *const u8 as *const libc::c_char,
                            if doTile != 0 {
                                b"Tile\x00" as *const u8 as *const libc::c_char
                            } else {
                                b"Image\x00" as *const u8 as *const libc::c_char
                            },
                            tilew,
                            tileh,
                        );
                        if doYUV != 0 {
                            crate::stdlib::printf(
                                b"Encode YUV    --> Frame rate:         %f fps\n\x00" as *const u8
                                    as *const libc::c_char,
                                iter as libc::c_double / elapsedEncode,
                            );
                            crate::stdlib::printf(
                                b"                  Output image size:  %d bytes\n\x00" as *const u8
                                    as *const libc::c_char,
                                yuvSize,
                            );
                            crate::stdlib::printf(
                                b"                  Compression ratio:  %f:1\n\x00" as *const u8
                                    as *const libc::c_char,
                                (w * h * ps) as libc::c_double / yuvSize as libc::c_double,
                            );
                            crate::stdlib::printf(
                                b"                  Throughput:         %f Megapixels/sec\n\x00"
                                    as *const u8
                                    as *const libc::c_char,
                                (w * h) as libc::c_double / 1000000.0f64 * iter as libc::c_double
                                    / elapsedEncode,
                            );
                            crate::stdlib::printf(
                                b"                  Output bit stream:  %f Megabits/sec\n\x00"
                                    as *const u8
                                    as *const libc::c_char,
                                yuvSize as libc::c_double * 8.0f64 / 1000000.0f64
                                    * iter as libc::c_double
                                    / elapsedEncode,
                            );
                        }
                        crate::stdlib::printf(
                            b"%s --> Frame rate:         %f fps\n\x00" as *const u8
                                as *const libc::c_char,
                            if doYUV != 0 {
                                b"Comp from YUV\x00" as *const u8 as *const libc::c_char
                            } else {
                                b"Compress     \x00" as *const u8 as *const libc::c_char
                            },
                            iter as libc::c_double / elapsed,
                        );
                        crate::stdlib::printf(
                            b"                  Output image size:  %d bytes\n\x00" as *const u8
                                as *const libc::c_char,
                            totalJpegSize,
                        );
                        crate::stdlib::printf(
                            b"                  Compression ratio:  %f:1\n\x00" as *const u8
                                as *const libc::c_char,
                            (w * h * ps) as libc::c_double / totalJpegSize as libc::c_double,
                        );
                        crate::stdlib::printf(
                            b"                  Throughput:         %f Megapixels/sec\n\x00"
                                as *const u8 as *const libc::c_char,
                            (w * h) as libc::c_double / 1000000.0f64 * iter as libc::c_double
                                / elapsed,
                        );
                        crate::stdlib::printf(
                            b"                  Output bit stream:  %f Megabits/sec\n\x00"
                                as *const u8 as *const libc::c_char,
                            totalJpegSize as libc::c_double * 8.0f64 / 1000000.0f64
                                * iter as libc::c_double
                                / elapsed,
                        );
                    }
                    if tilew == w && tileh == h && doWrite != 0 {
                        crate::stdlib::snprintf(
                            tempStr.as_mut_ptr(),
                            1024 as libc::c_int as libc::c_ulong,
                            b"%s_%s_Q%d.jpg\x00" as *const u8 as *const libc::c_char,
                            fileName,
                            subName[subsamp as usize],
                            jpegQual,
                        );
                        file = crate::stdlib::fopen(
                            tempStr.as_mut_ptr(),
                            b"wb\x00" as *const u8 as *const libc::c_char,
                        );
                        if file.is_null() {
                            crate::stdlib::printf(
                                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                463 as libc::c_int,
                                b"opening reference image\x00" as *const u8 as *const libc::c_char,
                                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                            );
                            retval = -(1 as libc::c_int);
                            break;
                        } else if crate::stdlib::fwrite(
                            *jpegBuf.offset(0 as libc::c_int as isize) as *const libc::c_void,
                            *jpegSize.offset(0 as libc::c_int as isize),
                            1 as libc::c_int as libc::c_ulong,
                            file,
                        ) != 1 as libc::c_int as libc::c_ulong
                        {
                            crate::stdlib::printf(
                                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                465 as libc::c_int,
                                b"writing reference image\x00" as *const u8 as *const libc::c_char,
                                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                            );
                            retval = -(1 as libc::c_int);
                            break;
                        } else {
                            crate::stdlib::fclose(file);
                            file = crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE;
                            if quiet == 0 {
                                crate::stdlib::printf(
                                    b"Reference image written to %s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    tempStr.as_mut_ptr(),
                                );
                            }
                        }
                    }
                    /* Decompression test */
                    if compOnly == 0 {
                        if decomp(
                            srcBuf, jpegBuf, jpegSize, tmpBuf, w, h, subsamp, jpegQual, fileName,
                            tilew, tileh,
                        ) == -(1 as libc::c_int)
                        {
                            break;
                        }
                    }
                    i = 0 as libc::c_int;
                    while i < ntilesw * ntilesh {
                        if !(*jpegBuf.offset(i as isize)).is_null() {
                            crate::turbojpeg_h::tjFree(*jpegBuf.offset(i as isize));
                        }
                        let ref mut fresh1 = *jpegBuf.offset(i as isize);
                        *fresh1 = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
                        i += 1
                    }
                    crate::stdlib::free(jpegBuf as *mut libc::c_void);
                    jpegBuf = crate::stddef_h::NULL_0 as *mut *mut libc::c_uchar;
                    crate::stdlib::free(jpegSize as *mut libc::c_void);
                    jpegSize = crate::stddef_h::NULL_0 as *mut libc::c_ulong;
                    if doYUV != 0 {
                        crate::stdlib::free(yuvBuf as *mut libc::c_void);
                        yuvBuf = crate::stddef_h::NULL_0 as *mut libc::c_uchar
                    }
                    if tilew == w && tileh == h {
                        break;
                    }
                    tilew *= 2 as libc::c_int;
                    tileh *= 2 as libc::c_int
                }
            }
        }
    }
    if !file.is_null() {
        crate::stdlib::fclose(file);
        file = crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE
    }
    if !jpegBuf.is_null() {
        i = 0 as libc::c_int;
        while i < ntilesw * ntilesh {
            if !(*jpegBuf.offset(i as isize)).is_null() {
                crate::turbojpeg_h::tjFree(*jpegBuf.offset(i as isize));
            }
            let ref mut fresh2 = *jpegBuf.offset(i as isize);
            *fresh2 = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
            i += 1
        }
        crate::stdlib::free(jpegBuf as *mut libc::c_void);
        jpegBuf = crate::stddef_h::NULL_0 as *mut *mut libc::c_uchar
    }
    if !yuvBuf.is_null() {
        crate::stdlib::free(yuvBuf as *mut libc::c_void);
        yuvBuf = crate::stddef_h::NULL_0 as *mut libc::c_uchar
    }
    if !jpegSize.is_null() {
        crate::stdlib::free(jpegSize as *mut libc::c_void);
        jpegSize = crate::stddef_h::NULL_0 as *mut libc::c_ulong
    }
    if !tmpBuf.is_null() {
        crate::stdlib::free(tmpBuf as *mut libc::c_void);
        tmpBuf = crate::stddef_h::NULL_0 as *mut libc::c_uchar
    }
    if !handle.is_null() {
        crate::turbojpeg_h::tjDestroy(handle);
        handle = crate::stddef_h::NULL_0 as *mut libc::c_void
    }
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn decompTest(mut fileName: *mut libc::c_char) -> libc::c_int {
    let mut current_block: u64;
    let mut file: *mut crate::stdlib::FILE = crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE;
    let mut handle: crate::turbojpeg_h::tjhandle = crate::stddef_h::NULL_0 as *mut libc::c_void;
    let mut jpegBuf: *mut *mut libc::c_uchar = crate::stddef_h::NULL_0 as *mut *mut libc::c_uchar;
    let mut srcBuf: *mut libc::c_uchar = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
    let mut jpegSize: *mut libc::c_ulong = crate::stddef_h::NULL_0 as *mut libc::c_ulong;
    let mut srcSize: libc::c_ulong = 0;
    let mut totalJpegSize: libc::c_ulong = 0;
    let mut t: *mut crate::turbojpeg_h::tjtransform =
        crate::stddef_h::NULL_0 as *mut crate::turbojpeg_h::tjtransform;
    let mut start: libc::c_double = 0.;
    let mut elapsed: libc::c_double = 0.;
    let mut ps: libc::c_int = tjPixelSize[pf as usize];
    let mut tile: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut iter: libc::c_int = 0;
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut decompsrc: libc::c_int = 0 as libc::c_int;
    let mut temp: *mut libc::c_char = crate::stddef_h::NULL_0 as *mut libc::c_char;
    let mut tempStr: [libc::c_char; 80] = [0; 80];
    let mut tempStr2: [libc::c_char; 80] = [0; 80];
    /* Original image */
    let mut w: libc::c_int = 0 as libc::c_int;
    let mut h: libc::c_int = 0 as libc::c_int;
    let mut tilew: libc::c_int = 0;
    let mut tileh: libc::c_int = 0;
    let mut ntilesw: libc::c_int = 1 as libc::c_int;
    let mut ntilesh: libc::c_int = 1 as libc::c_int;
    let mut subsamp: libc::c_int = -(1 as libc::c_int);
    let mut cs: libc::c_int = -(1 as libc::c_int);
    /* Transformed image */
    let mut tw: libc::c_int = 0;
    let mut th: libc::c_int = 0;
    let mut ttilew: libc::c_int = 0;
    let mut ttileh: libc::c_int = 0;
    let mut tntilesw: libc::c_int = 0;
    let mut tntilesh: libc::c_int = 0;
    let mut tsubsamp: libc::c_int = 0;
    file = crate::stdlib::fopen(fileName, b"rb\x00" as *const u8 as *const libc::c_char);
    if file.is_null() {
        crate::stdlib::printf(
            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
            524 as libc::c_int,
            b"opening file\x00" as *const u8 as *const libc::c_char,
            crate::stdlib::strerror(*crate::stdlib::__errno_location()),
        );
        retval = -(1 as libc::c_int)
    } else if crate::stdlib::fseek(
        file,
        0 as libc::c_int as libc::c_long,
        crate::stdlib::SEEK_END,
    ) < 0 as libc::c_int
        || {
            srcSize = crate::stdlib::ftell(file) as libc::c_ulong;
            (srcSize) == -(1 as libc::c_int) as libc::c_ulong
        }
    {
        crate::stdlib::printf(
            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
            527 as libc::c_int,
            b"determining file size\x00" as *const u8 as *const libc::c_char,
            crate::stdlib::strerror(*crate::stdlib::__errno_location()),
        );
        retval = -(1 as libc::c_int)
    } else {
        srcBuf = crate::stdlib::malloc(srcSize) as *mut libc::c_uchar;
        if srcBuf.is_null() {
            crate::stdlib::printf(
                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                529 as libc::c_int,
                b"allocating memory\x00" as *const u8 as *const libc::c_char,
                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
            );
            retval = -(1 as libc::c_int)
        } else if crate::stdlib::fseek(
            file,
            0 as libc::c_int as libc::c_long,
            crate::stdlib::SEEK_SET,
        ) < 0 as libc::c_int
        {
            crate::stdlib::printf(
                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                531 as libc::c_int,
                b"setting file position\x00" as *const u8 as *const libc::c_char,
                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
            );
            retval = -(1 as libc::c_int)
        } else if crate::stdlib::fread(
            srcBuf as *mut libc::c_void,
            srcSize,
            1 as libc::c_int as libc::c_ulong,
            file,
        ) < 1 as libc::c_int as libc::c_ulong
        {
            crate::stdlib::printf(
                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                533 as libc::c_int,
                b"reading JPEG data\x00" as *const u8 as *const libc::c_char,
                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
            );
            retval = -(1 as libc::c_int)
        } else {
            crate::stdlib::fclose(file);
            file = crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE;
            temp = crate::stdlib::strrchr(fileName, '.' as i32);
            if !temp.is_null() {
                *temp = '\u{0}' as i32 as libc::c_char
            }
            handle = crate::turbojpeg_h::tjInitTransform();
            if handle.is_null() {
                let mut _tjErrorCode: libc::c_int = crate::turbojpeg_h::tjGetErrorCode(handle);
                let mut _tjErrorStr: *mut libc::c_char = crate::turbojpeg_h::tjGetErrorStr2(handle);
                if flags & crate::turbojpeg_h::TJFLAG_STOPONWARNING == 0
                    && _tjErrorCode == crate::turbojpeg_h::TJERR_WARNING as libc::c_int
                {
                    if crate::stdlib::strncmp(
                        tjErrorStr.as_mut_ptr(),
                        _tjErrorStr,
                        crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                    ) != 0
                        || crate::stdlib::strncmp(
                            tjErrorMsg.as_mut_ptr(),
                            b"executing tjInitTransform()\x00" as *const u8 as *const libc::c_char,
                            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                        ) != 0
                        || tjErrorCode != _tjErrorCode
                        || tjErrorLine != 540 as libc::c_int
                    {
                        crate::stdlib::strncpy(
                            tjErrorStr.as_mut_ptr(),
                            _tjErrorStr,
                            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                        );
                        crate::stdlib::strncpy(
                            tjErrorMsg.as_mut_ptr(),
                            b"executing tjInitTransform()\x00" as *const u8 as *const libc::c_char,
                            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                        );
                        tjErrorCode = _tjErrorCode;
                        tjErrorLine = 540 as libc::c_int;
                        crate::stdlib::printf(
                            b"WARNING in line %d while %s:\n%s\n\x00" as *const u8
                                as *const libc::c_char,
                            540 as libc::c_int,
                            b"executing tjInitTransform()\x00" as *const u8 as *const libc::c_char,
                            _tjErrorStr,
                        );
                    }
                    current_block = 9441801433784995173;
                } else {
                    crate::stdlib::printf(
                        b"%s in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                        if _tjErrorCode == crate::turbojpeg_h::TJERR_WARNING as libc::c_int {
                            b"WARNING\x00" as *const u8 as *const libc::c_char
                        } else {
                            b"ERROR\x00" as *const u8 as *const libc::c_char
                        },
                        540 as libc::c_int,
                        b"executing tjInitTransform()\x00" as *const u8 as *const libc::c_char,
                        _tjErrorStr,
                    );
                    retval = -(1 as libc::c_int);
                    current_block = 18327231901424809080;
                }
            } else {
                current_block = 9441801433784995173;
            }
            match current_block {
                18327231901424809080 => {}
                _ => {
                    if crate::turbojpeg_h::tjDecompressHeader3(
                        handle,
                        srcBuf,
                        srcSize,
                        &mut w,
                        &mut h,
                        &mut subsamp,
                        &mut cs,
                    ) == -(1 as libc::c_int)
                    {
                        let mut _tjErrorCode_0: libc::c_int =
                            crate::turbojpeg_h::tjGetErrorCode(handle);
                        let mut _tjErrorStr_0: *mut libc::c_char =
                            crate::turbojpeg_h::tjGetErrorStr2(handle);
                        if flags & crate::turbojpeg_h::TJFLAG_STOPONWARNING == 0
                            && _tjErrorCode_0 == crate::turbojpeg_h::TJERR_WARNING as libc::c_int
                        {
                            if crate::stdlib::strncmp(
                                tjErrorStr.as_mut_ptr(),
                                _tjErrorStr_0,
                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                            ) != 0
                                || crate::stdlib::strncmp(
                                    tjErrorMsg.as_mut_ptr(),
                                    b"executing tjDecompressHeader3()\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                ) != 0
                                || tjErrorCode != _tjErrorCode_0
                                || tjErrorLine != 543 as libc::c_int
                            {
                                crate::stdlib::strncpy(
                                    tjErrorStr.as_mut_ptr(),
                                    _tjErrorStr_0,
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                );
                                crate::stdlib::strncpy(
                                    tjErrorMsg.as_mut_ptr(),
                                    b"executing tjDecompressHeader3()\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                );
                                tjErrorCode = _tjErrorCode_0;
                                tjErrorLine = 543 as libc::c_int;
                                crate::stdlib::printf(
                                    b"WARNING in line %d while %s:\n%s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    543 as libc::c_int,
                                    b"executing tjDecompressHeader3()\x00" as *const u8
                                        as *const libc::c_char,
                                    _tjErrorStr_0,
                                );
                            }
                            current_block = 168769493162332264;
                        } else {
                            crate::stdlib::printf(
                                b"%s in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                if _tjErrorCode_0
                                    == crate::turbojpeg_h::TJERR_WARNING as libc::c_int
                                {
                                    b"WARNING\x00" as *const u8 as *const libc::c_char
                                } else {
                                    b"ERROR\x00" as *const u8 as *const libc::c_char
                                },
                                543 as libc::c_int,
                                b"executing tjDecompressHeader3()\x00" as *const u8
                                    as *const libc::c_char,
                                _tjErrorStr_0,
                            );
                            retval = -(1 as libc::c_int);
                            current_block = 18327231901424809080;
                        }
                    } else {
                        current_block = 168769493162332264;
                    }
                    match current_block {
                        18327231901424809080 => {}
                        _ => {
                            if cs == crate::turbojpeg_h::TJCS_YCCK as libc::c_int
                                || cs == crate::turbojpeg_h::TJCS_CMYK as libc::c_int
                            {
                                pf = crate::turbojpeg_h::TJPF_CMYK as libc::c_int;
                                ps = tjPixelSize[pf as usize]
                            }
                            if quiet == 1 as libc::c_int {
                                crate::stdlib::printf(
                                    b"All performance values in Mpixels/sec\n\n\x00" as *const u8
                                        as *const libc::c_char,
                                );
                                crate::stdlib::printf(b"Bitmap     JPEG   JPEG     %s  %s   Xform   Comp    Decomp  \x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       if doTile != 0 {
                                           b"Tile \x00" as *const u8 as
                                               *const libc::c_char
                                       } else {
                                           b"Image\x00" as *const u8 as
                                               *const libc::c_char
                                       },
                                       if doTile != 0 {
                                           b"Tile \x00" as *const u8 as
                                               *const libc::c_char
                                       } else {
                                           b"Image\x00" as *const u8 as
                                               *const libc::c_char
                                       });
                                if doYUV != 0 {
                                    crate::stdlib::printf(
                                        b"Decode\x00" as *const u8 as *const libc::c_char,
                                    );
                                }
                                crate::stdlib::printf(
                                    b"\n\x00" as *const u8 as *const libc::c_char,
                                );
                                crate::stdlib::printf(b"Format     CS     Subsamp  Width  Height  Perf    Ratio   Perf    \x00"
                                           as *const u8 as
                                           *const libc::c_char);
                                if doYUV != 0 {
                                    crate::stdlib::printf(
                                        b"Perf\x00" as *const u8 as *const libc::c_char,
                                    );
                                }
                                crate::stdlib::printf(
                                    b"\n\n\x00" as *const u8 as *const libc::c_char,
                                );
                            } else if quiet == 0 {
                                crate::stdlib::printf(
                                    b">>>>>  JPEG %s --> %s (%s)  <<<<<\n\x00" as *const u8
                                        as *const libc::c_char,
                                    formatName(subsamp, cs, tempStr.as_mut_ptr()),
                                    pixFormatStr[pf as usize],
                                    if flags & crate::turbojpeg_h::TJFLAG_BOTTOMUP != 0 {
                                        b"Bottom-up\x00" as *const u8 as *const libc::c_char
                                    } else {
                                        b"Top-down\x00" as *const u8 as *const libc::c_char
                                    },
                                );
                            }
                            tilew = (if doTile != 0 { 16 as libc::c_int } else { w });
                            tileh = (if doTile != 0 { 16 as libc::c_int } else { h });
                            's_381: loop {
                                if tilew > w {
                                    tilew = w
                                }
                                if tileh > h {
                                    tileh = h
                                }
                                ntilesw = (w + tilew - 1 as libc::c_int) / tilew;
                                ntilesh = (h + tileh - 1 as libc::c_int) / tileh;
                                jpegBuf = crate::stdlib::malloc(
                                    (::std::mem::size_of::<*mut libc::c_uchar>() as libc::c_ulong)
                                        .wrapping_mul(ntilesw as libc::c_ulong)
                                        .wrapping_mul(ntilesh as libc::c_ulong),
                                )
                                    as *mut *mut libc::c_uchar;
                                if jpegBuf.is_null() {
                                    crate::stdlib::printf(
                                        b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                            as *const libc::c_char,
                                        571 as libc::c_int,
                                        b"allocating JPEG tile array\x00" as *const u8
                                            as *const libc::c_char,
                                        crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                                    );
                                    retval = -(1 as libc::c_int);
                                    break;
                                } else {
                                    crate::stdlib::memset(
                                        jpegBuf as *mut libc::c_void,
                                        0 as libc::c_int,
                                        (::std::mem::size_of::<*mut libc::c_uchar>()
                                            as libc::c_ulong)
                                            .wrapping_mul(ntilesw as libc::c_ulong)
                                            .wrapping_mul(ntilesh as libc::c_ulong),
                                    );
                                    jpegSize = crate::stdlib::malloc(
                                        (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                            .wrapping_mul(ntilesw as libc::c_ulong)
                                            .wrapping_mul(ntilesh as libc::c_ulong),
                                    )
                                        as *mut libc::c_ulong;
                                    if jpegSize.is_null() {
                                        crate::stdlib::printf(
                                            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                                as *const libc::c_char,
                                            575 as libc::c_int,
                                            b"allocating JPEG size array\x00" as *const u8
                                                as *const libc::c_char,
                                            crate::stdlib::strerror(
                                                *crate::stdlib::__errno_location(),
                                            ),
                                        );
                                        retval = -(1 as libc::c_int);
                                        break;
                                    } else {
                                        crate::stdlib::memset(
                                            jpegSize as *mut libc::c_void,
                                            0 as libc::c_int,
                                            (::std::mem::size_of::<libc::c_ulong>()
                                                as libc::c_ulong)
                                                .wrapping_mul(ntilesw as libc::c_ulong)
                                                .wrapping_mul(ntilesh as libc::c_ulong),
                                        );
                                        if flags & crate::turbojpeg_h::TJFLAG_NOREALLOC
                                            != 0 as libc::c_int
                                            || doTile == 0
                                        {
                                            i = 0 as libc::c_int;
                                            while i < ntilesw * ntilesh {
                                                let ref mut fresh3 = *jpegBuf.offset(i as isize);
                                                *fresh3 = crate::turbojpeg_h::tjAlloc(
                                                    crate::turbojpeg_h::tjBufSize(
                                                        tilew, tileh, subsamp,
                                                    )
                                                        as libc::c_int,
                                                );
                                                if (*fresh3).is_null() {
                                                    crate::stdlib::printf(
                                                        b"ERROR in line %d while %s:\n%s\n\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        582 as libc::c_int,
                                                        b"allocating JPEG tiles\x00" as *const u8
                                                            as *const libc::c_char,
                                                        crate::stdlib::strerror(
                                                            *crate::stdlib::__errno_location(),
                                                        ),
                                                    );
                                                    retval = -(1 as libc::c_int);
                                                    break 's_381;
                                                } else {
                                                    i += 1
                                                }
                                            }
                                        }
                                        tw = w;
                                        th = h;
                                        ttilew = tilew;
                                        ttileh = tileh;
                                        if quiet == 0 {
                                            crate::stdlib::printf(
                                                b"\n%s size: %d x %d\x00" as *const u8
                                                    as *const libc::c_char,
                                                if doTile != 0 {
                                                    b"Tile\x00" as *const u8 as *const libc::c_char
                                                } else {
                                                    b"Image\x00" as *const u8 as *const libc::c_char
                                                },
                                                ttilew,
                                                ttileh,
                                            );
                                            if sf.num != 1 as libc::c_int
                                                || sf.denom != 1 as libc::c_int
                                            {
                                                crate::stdlib::printf(
                                                    b" --> %d x %d\x00" as *const u8
                                                        as *const libc::c_char,
                                                    (tw * sf.num + sf.denom - 1 as libc::c_int)
                                                        / sf.denom,
                                                    (th * sf.num + sf.denom - 1 as libc::c_int)
                                                        / sf.denom,
                                                );
                                            }
                                            crate::stdlib::printf(
                                                b"\n\x00" as *const u8 as *const libc::c_char,
                                            );
                                        } else if quiet == 1 as libc::c_int {
                                            crate::stdlib::printf(
                                                b"%-4s (%s)  %-5s  %-5s    \x00" as *const u8
                                                    as *const libc::c_char,
                                                pixFormatStr[pf as usize],
                                                if flags & crate::turbojpeg_h::TJFLAG_BOTTOMUP != 0
                                                {
                                                    b"BU\x00" as *const u8 as *const libc::c_char
                                                } else {
                                                    b"TD\x00" as *const u8 as *const libc::c_char
                                                },
                                                csName[cs as usize],
                                                subNameLong[subsamp as usize],
                                            );
                                            crate::stdlib::printf(
                                                b"%-5d  %-5d   \x00" as *const u8
                                                    as *const libc::c_char,
                                                tilew,
                                                tileh,
                                            );
                                        }
                                        tsubsamp = subsamp;
                                        if doTile != 0
                                            || xformOp
                                                != crate::turbojpeg_h::TJXOP_NONE as libc::c_int
                                            || xformOpt != 0 as libc::c_int
                                            || customFilter.is_some()
                                        {
                                            t = crate::stdlib::malloc(
                                                (::std::mem::size_of::<
                                                    crate::turbojpeg_h::tjtransform,
                                                >()
                                                    as libc::c_ulong)
                                                    .wrapping_mul(ntilesw as libc::c_ulong)
                                                    .wrapping_mul(ntilesh as libc::c_ulong),
                                            )
                                                as *mut crate::turbojpeg_h::tjtransform;
                                            if t.is_null() {
                                                crate::stdlib::printf(
                                                    b"ERROR in line %d while %s:\n%s\n\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    602 as libc::c_int,
                                                    b"allocating image transform array\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    crate::stdlib::strerror(
                                                        *crate::stdlib::__errno_location(),
                                                    ),
                                                );
                                                retval = -(1 as libc::c_int);
                                                break;
                                            } else {
                                                if xformOp
                                                    == crate::turbojpeg_h::TJXOP_TRANSPOSE
                                                        as libc::c_int
                                                    || xformOp
                                                        == crate::turbojpeg_h::TJXOP_TRANSVERSE
                                                            as libc::c_int
                                                    || xformOp
                                                        == crate::turbojpeg_h::TJXOP_ROT90
                                                            as libc::c_int
                                                    || xformOp
                                                        == crate::turbojpeg_h::TJXOP_ROT270
                                                            as libc::c_int
                                                {
                                                    tw = h;
                                                    th = w;
                                                    ttilew = tileh;
                                                    ttileh = tilew
                                                }
                                                if xformOpt & crate::turbojpeg_h::TJXOPT_GRAY != 0 {
                                                    tsubsamp = crate::turbojpeg_h::TJ_GRAYSCALE
                                                }
                                                if xformOp
                                                    == crate::turbojpeg_h::TJXOP_HFLIP
                                                        as libc::c_int
                                                    || xformOp
                                                        == crate::turbojpeg_h::TJXOP_ROT180
                                                            as libc::c_int
                                                {
                                                    tw = tw - tw % tjMCUWidth[tsubsamp as usize]
                                                }
                                                if xformOp
                                                    == crate::turbojpeg_h::TJXOP_VFLIP
                                                        as libc::c_int
                                                    || xformOp
                                                        == crate::turbojpeg_h::TJXOP_ROT180
                                                            as libc::c_int
                                                {
                                                    th = th - th % tjMCUHeight[tsubsamp as usize]
                                                }
                                                if xformOp
                                                    == crate::turbojpeg_h::TJXOP_TRANSVERSE
                                                        as libc::c_int
                                                    || xformOp
                                                        == crate::turbojpeg_h::TJXOP_ROT90
                                                            as libc::c_int
                                                {
                                                    tw = tw - tw % tjMCUHeight[tsubsamp as usize]
                                                }
                                                if xformOp
                                                    == crate::turbojpeg_h::TJXOP_TRANSVERSE
                                                        as libc::c_int
                                                    || xformOp
                                                        == crate::turbojpeg_h::TJXOP_ROT270
                                                            as libc::c_int
                                                {
                                                    th = th - th % tjMCUWidth[tsubsamp as usize]
                                                }
                                                tntilesw =
                                                    (tw + ttilew - 1 as libc::c_int) / ttilew;
                                                tntilesh =
                                                    (th + ttileh - 1 as libc::c_int) / ttileh;
                                                if xformOp
                                                    == crate::turbojpeg_h::TJXOP_TRANSPOSE
                                                        as libc::c_int
                                                    || xformOp
                                                        == crate::turbojpeg_h::TJXOP_TRANSVERSE
                                                            as libc::c_int
                                                    || xformOp
                                                        == crate::turbojpeg_h::TJXOP_ROT90
                                                            as libc::c_int
                                                    || xformOp
                                                        == crate::turbojpeg_h::TJXOP_ROT270
                                                            as libc::c_int
                                                {
                                                    if tsubsamp
                                                        == crate::turbojpeg_h::TJSAMP_422
                                                            as libc::c_int
                                                    {
                                                        tsubsamp = crate::turbojpeg_h::TJSAMP_440
                                                            as libc::c_int
                                                    } else if tsubsamp
                                                        == crate::turbojpeg_h::TJSAMP_440
                                                            as libc::c_int
                                                    {
                                                        tsubsamp = crate::turbojpeg_h::TJSAMP_422
                                                            as libc::c_int
                                                    }
                                                }
                                                row = 0 as libc::c_int;
                                                tile = 0 as libc::c_int;
                                                while row < tntilesh {
                                                    col = 0 as libc::c_int;
                                                    while col < tntilesw {
                                                        (*t.offset(tile as isize)).r.w =
                                                            if ttilew < tw - col * ttilew {
                                                                ttilew
                                                            } else {
                                                                (tw) - col * ttilew
                                                            };
                                                        (*t.offset(tile as isize)).r.h =
                                                            if ttileh < th - row * ttileh {
                                                                ttileh
                                                            } else {
                                                                (th) - row * ttileh
                                                            };
                                                        (*t.offset(tile as isize)).r.x =
                                                            col * ttilew;
                                                        (*t.offset(tile as isize)).r.y =
                                                            row * ttileh;
                                                        (*t.offset(tile as isize)).op = xformOp;
                                                        (*t.offset(tile as isize)).options =
                                                            xformOpt
                                                                | crate::turbojpeg_h::TJXOPT_TRIM;
                                                        let ref mut fresh4 =
                                                            (*t.offset(tile as isize)).customFilter;
                                                        *fresh4 = customFilter;
                                                        if (*t.offset(tile as isize)).options
                                                            & crate::turbojpeg_h::TJXOPT_NOOUTPUT
                                                            != 0
                                                            && !(*jpegBuf.offset(tile as isize))
                                                                .is_null()
                                                        {
                                                            crate::turbojpeg_h::tjFree(
                                                                *jpegBuf.offset(tile as isize),
                                                            );
                                                            let ref mut fresh5 =
                                                                *jpegBuf.offset(tile as isize);
                                                            *fresh5 = crate::stddef_h::NULL_0
                                                                as *mut libc::c_uchar
                                                        }
                                                        col += 1;
                                                        tile += 1
                                                    }
                                                    row += 1
                                                }
                                                iter = -(1 as libc::c_int);
                                                elapsed = 0.0f64;
                                                loop {
                                                    start = crate::tjutil_h::getTime();
                                                    if crate::turbojpeg_h::tjTransform(
                                                        handle,
                                                        srcBuf,
                                                        srcSize,
                                                        tntilesw * tntilesh,
                                                        jpegBuf,
                                                        jpegSize,
                                                        t,
                                                        flags,
                                                    ) == -(1 as libc::c_int)
                                                    {
                                                        let mut _tjErrorCode_1: libc::c_int =
                                                            crate::turbojpeg_h::tjGetErrorCode(
                                                                handle,
                                                            );
                                                        let mut _tjErrorStr_1: *mut libc::c_char =
                                                            crate::turbojpeg_h::tjGetErrorStr2(
                                                                handle,
                                                            );
                                                        if flags &
                                                                   crate::turbojpeg_h::TJFLAG_STOPONWARNING
                                                                   == 0 &&
                                                                   _tjErrorCode_1
                                                                       ==
                                                                       crate::turbojpeg_h::TJERR_WARNING
                                                                           as
                                                                           libc::c_int
                                                               {
                                                                if crate::stdlib::strncmp(tjErrorStr.as_mut_ptr(),
                                                                           _tjErrorStr_1,
                                                                           crate::jpeglib_h::JMSG_LENGTH_MAX
                                                                               as
                                                                               libc::c_ulong)
                                                                       != 0 ||
                                                                       crate::stdlib::strncmp(tjErrorMsg.as_mut_ptr(),
                                                                               b"executing tjTransform()\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const libc::c_char,
                                                                               crate::jpeglib_h::JMSG_LENGTH_MAX
                                                                                   as
                                                                                   libc::c_ulong)
                                                                           !=
                                                                           0
                                                                       ||
                                                                       tjErrorCode
                                                                           !=
                                                                           _tjErrorCode_1
                                                                       ||
                                                                       tjErrorLine
                                                                           !=
                                                                           648
                                                                               as
                                                                               libc::c_int
                                                                   {
                                                                    crate::stdlib::strncpy(tjErrorStr.as_mut_ptr(),
                                                                            _tjErrorStr_1,
                                                                            crate::jpeglib_h::JMSG_LENGTH_MAX
                                                                                as
                                                                                libc::c_ulong);
                                                                    crate::stdlib::strncpy(tjErrorMsg.as_mut_ptr(),
                                                                            b"executing tjTransform()\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char,
                                                                            crate::jpeglib_h::JMSG_LENGTH_MAX
                                                                                as
                                                                                libc::c_ulong);
                                                                    tjErrorCode
                                                                        =
                                                                        _tjErrorCode_1;
                                                                    tjErrorLine
                                                                        =
                                                                        648 as
                                                                            libc::c_int;
                                                                    crate::stdlib::printf(b"WARNING in line %d while %s:\n%s\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char,
                                                                           648
                                                                               as
                                                                               libc::c_int,
                                                                           b"executing tjTransform()\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char,
                                                                           _tjErrorStr_1);
                                                                }
                                                            } else {
                                                                crate::stdlib::printf(b"%s in line %d while %s:\n%s\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char,
                                                                       if _tjErrorCode_1
                                                                              ==
                                                                              crate::turbojpeg_h::TJERR_WARNING
                                                                                  as
                                                                                  libc::c_int
                                                                          {
                                                                           b"WARNING\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char
                                                                       } else {
                                                                           b"ERROR\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char
                                                                       },
                                                                       648 as
                                                                           libc::c_int,
                                                                       b"executing tjTransform()\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char,
                                                                       _tjErrorStr_1);
                                                                retval =
                                                                    -(1 as
                                                                          libc::c_int);
                                                                break 's_381 ;
                                                            }
                                                    }
                                                    elapsed += crate::tjutil_h::getTime() - start;
                                                    if iter >= 0 as libc::c_int {
                                                        iter += 1;
                                                        if elapsed >= benchTime {
                                                            break;
                                                        }
                                                    } else if elapsed >= warmup {
                                                        iter = 0 as libc::c_int;
                                                        elapsed = 0.0f64
                                                    }
                                                }
                                                crate::stdlib::free(t as *mut libc::c_void);
                                                t = crate::stddef_h::NULL_0
                                                    as *mut crate::turbojpeg_h::tjtransform;
                                                tile = 0 as libc::c_int;
                                                totalJpegSize = 0 as libc::c_int as libc::c_ulong;
                                                while tile < tntilesw * tntilesh {
                                                    totalJpegSize = totalJpegSize.wrapping_add(
                                                        *jpegSize.offset(tile as isize),
                                                    );
                                                    tile += 1
                                                }
                                                if quiet != 0 {
                                                    crate::stdlib::printf(
                                                        b"%-6s%s%-6s%s\x00" as *const u8
                                                            as *const libc::c_char,
                                                        sigfig(
                                                            (w * h) as libc::c_double
                                                                / 1000000.0f64
                                                                / elapsed,
                                                            4 as libc::c_int,
                                                            tempStr.as_mut_ptr(),
                                                            80 as libc::c_int,
                                                        ),
                                                        if quiet == 2 as libc::c_int {
                                                            b"\n\x00" as *const u8
                                                                as *const libc::c_char
                                                        } else {
                                                            b"  \x00" as *const u8
                                                                as *const libc::c_char
                                                        },
                                                        sigfig(
                                                            (w * h * ps) as libc::c_double
                                                                / totalJpegSize as libc::c_double,
                                                            4 as libc::c_int,
                                                            tempStr2.as_mut_ptr(),
                                                            80 as libc::c_int,
                                                        ),
                                                        if quiet == 2 as libc::c_int {
                                                            b"\n\x00" as *const u8
                                                                as *const libc::c_char
                                                        } else {
                                                            b"  \x00" as *const u8
                                                                as *const libc::c_char
                                                        },
                                                    );
                                                } else if quiet == 0 {
                                                    crate::stdlib::printf(b"Transform     --> Frame rate:         %f fps\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               1.0f64 /
                                                                   elapsed);
                                                    crate::stdlib::printf(b"                  Output image size:  %lu bytes\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               totalJpegSize);
                                                    crate::stdlib::printf(b"                  Compression ratio:  %f:1\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (w * h * ps) as
                                                                   libc::c_double
                                                                   /
                                                                   totalJpegSize
                                                                       as
                                                                       libc::c_double);
                                                    crate::stdlib::printf(b"                  Throughput:         %f Megapixels/sec\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (w * h) as
                                                                   libc::c_double
                                                                   /
                                                                   1000000.0f64
                                                                   / elapsed);
                                                    crate::stdlib::printf(b"                  Output bit stream:  %f Megabits/sec\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               totalJpegSize
                                                                   as
                                                                   libc::c_double
                                                                   * 8.0f64 /
                                                                   1000000.0f64
                                                                   / elapsed);
                                                }
                                            }
                                        } else {
                                            if quiet == 1 as libc::c_int {
                                                crate::stdlib::printf(
                                                    b"N/A     N/A     \x00" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            }
                                            crate::turbojpeg_h::tjFree(
                                                *jpegBuf.offset(0 as libc::c_int as isize),
                                            );
                                            let ref mut fresh6 =
                                                *jpegBuf.offset(0 as libc::c_int as isize);
                                            *fresh6 = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
                                            decompsrc = 1 as libc::c_int
                                        }
                                        if w == tilew {
                                            ttilew = tw
                                        }
                                        if h == tileh {
                                            ttileh = th
                                        }
                                        if xformOpt & crate::turbojpeg_h::TJXOPT_NOOUTPUT == 0 {
                                            if decomp(
                                                crate::stddef_h::NULL_0 as *mut libc::c_uchar,
                                                (if decompsrc != 0 {
                                                    &mut srcBuf
                                                } else {
                                                    jpegBuf
                                                }),
                                                (if decompsrc != 0 {
                                                    &mut srcSize
                                                } else {
                                                    jpegSize
                                                }),
                                                crate::stddef_h::NULL_0 as *mut libc::c_uchar,
                                                tw,
                                                th,
                                                tsubsamp,
                                                0 as libc::c_int,
                                                fileName,
                                                ttilew,
                                                ttileh,
                                            ) == -(1 as libc::c_int)
                                            {
                                                break;
                                            }
                                        } else if quiet == 1 as libc::c_int {
                                            crate::stdlib::printf(
                                                b"N/A\n\x00" as *const u8 as *const libc::c_char,
                                            );
                                        }
                                        i = 0 as libc::c_int;
                                        while i < ntilesw * ntilesh {
                                            crate::turbojpeg_h::tjFree(*jpegBuf.offset(i as isize));
                                            let ref mut fresh7 = *jpegBuf.offset(i as isize);
                                            *fresh7 = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
                                            i += 1
                                        }
                                        crate::stdlib::free(jpegBuf as *mut libc::c_void);
                                        jpegBuf =
                                            crate::stddef_h::NULL_0 as *mut *mut libc::c_uchar;
                                        if !jpegSize.is_null() {
                                            crate::stdlib::free(jpegSize as *mut libc::c_void);
                                            jpegSize = crate::stddef_h::NULL_0 as *mut libc::c_ulong
                                        }
                                        if tilew == w && tileh == h {
                                            break;
                                        }
                                        tilew *= 2 as libc::c_int;
                                        tileh *= 2 as libc::c_int
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !file.is_null() {
        crate::stdlib::fclose(file);
        file = crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE
    }
    if !jpegBuf.is_null() {
        i = 0 as libc::c_int;
        while i < ntilesw * ntilesh {
            if !(*jpegBuf.offset(i as isize)).is_null() {
                crate::turbojpeg_h::tjFree(*jpegBuf.offset(i as isize));
            }
            let ref mut fresh8 = *jpegBuf.offset(i as isize);
            *fresh8 = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
            i += 1
        }
        crate::stdlib::free(jpegBuf as *mut libc::c_void);
        jpegBuf = crate::stddef_h::NULL_0 as *mut *mut libc::c_uchar
    }
    if !jpegSize.is_null() {
        crate::stdlib::free(jpegSize as *mut libc::c_void);
        jpegSize = crate::stddef_h::NULL_0 as *mut libc::c_ulong
    }
    if !srcBuf.is_null() {
        crate::stdlib::free(srcBuf as *mut libc::c_void);
        srcBuf = crate::stddef_h::NULL_0 as *mut libc::c_uchar
    }
    if !t.is_null() {
        crate::stdlib::free(t as *mut libc::c_void);
        t = crate::stddef_h::NULL_0 as *mut crate::turbojpeg_h::tjtransform
    }
    if !handle.is_null() {
        crate::turbojpeg_h::tjDestroy(handle);
        handle = crate::stddef_h::NULL_0 as *mut libc::c_void
    }
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn usage(mut progName: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    crate::stdlib::printf(
        b"USAGE: %s\n\x00" as *const u8 as *const libc::c_char,
        progName,
    );
    crate::stdlib::printf(
        b"       <Inputfile (BMP|PPM)> <Quality> [options]\n\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"       %s\n\x00" as *const u8 as *const libc::c_char,
        progName,
    );
    crate::stdlib::printf(
        b"       <Inputfile (JPG)> [options]\n\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"Options:\n\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-alloc = Dynamically allocate JPEG image buffers\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-bmp = Generate output images in Windows Bitmap format (default = PPM)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-bottomup = Test bottom-up compression/decompression\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-tile = Test performance of the codec when the image is encoded as separate\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     tiles of varying sizes.\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-rgb, -bgr, -rgbx, -bgrx, -xbgr, -xrgb =\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     Test the specified color conversion path in the codec (default = BGR)\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-cmyk = Indirectly test YCCK JPEG compression/decompression (the source\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     and destination bitmaps are still RGB.  The conversion is done\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     internally prior to compression or after decompression.)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-fastupsample = Use the fastest chrominance upsampling algorithm available in\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"     the underlying codec\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-fastdct = Use the fastest DCT/IDCT algorithms available in the underlying\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"     codec\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-accuratedct = Use the most accurate DCT/IDCT algorithms available in the\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"     underlying codec\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-progressive = Use progressive entropy coding in JPEG images generated by\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     compression and transform operations.\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-subsamp <s> = When testing JPEG compression, this option specifies the level\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     of chrominance subsampling to use (<s> = 444, 422, 440, 420, 411, or\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     GRAY).  The default is to test Grayscale, 4:2:0, 4:2:2, and 4:4:4 in\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"     sequence.\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-quiet = Output results in tabular rather than verbose format\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-yuv = Test YUV encoding/decoding functions\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-yuvpad <p> = If testing YUV encoding/decoding, this specifies the number of\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     bytes to which each row of each plane in the intermediate YUV image is\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"     padded (default = 1)\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-scale M/N = Scale down the width/height of the decompressed JPEG image by a\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"     factor of M/N (M/N = \x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < nsf {
        crate::stdlib::printf(
            b"%d/%d\x00" as *const u8 as *const libc::c_char,
            (*scalingFactors.offset(i as isize)).num,
            (*scalingFactors.offset(i as isize)).denom,
        );
        if nsf == 2 as libc::c_int && i != nsf - 1 as libc::c_int {
            crate::stdlib::printf(b" or \x00" as *const u8 as *const libc::c_char);
        } else if nsf > 2 as libc::c_int {
            if i != nsf - 1 as libc::c_int {
                crate::stdlib::printf(b", \x00" as *const u8 as *const libc::c_char);
            }
            if i == nsf - 2 as libc::c_int {
                crate::stdlib::printf(b"or \x00" as *const u8 as *const libc::c_char);
            }
        }
        if i % 8 as libc::c_int == 0 as libc::c_int && i != 0 as libc::c_int {
            crate::stdlib::printf(b"\n     \x00" as *const u8 as *const libc::c_char);
        }
        i += 1
    }
    crate::stdlib::printf(b")\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-hflip, -vflip, -transpose, -transverse, -rot90, -rot180, -rot270 =\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     Perform the corresponding lossless transform prior to\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     decompression (these options are mutually exclusive)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-grayscale = Perform lossless grayscale conversion prior to decompression\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     test (can be combined with the other transforms above)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-copynone = Do not copy any extra markers (including EXIF and ICC profile data)\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     when transforming the image.\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-benchtime <t> = Run each benchmark for at least <t> seconds (default = 5.0)\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-warmup <t> = Run each benchmark for <t> seconds (default = 1.0) prior to\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     starting the timer, in order to prime the caches and thus improve the\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     consistency of the results.\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-componly = Stop after running compression tests.  Do not test decompression.\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-nowrite = Do not write reference or output images (improves consistency of\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     performance measurements.)\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-stoponwarning = Immediately discontinue the current\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     compression/decompression/transform operation if the underlying codec\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     throws a warning (non-fatal error)\n\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"NOTE:  If the quality is specified as a range (e.g. 90-100), a separate\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"test will be performed for all quality values in the range.\n\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::exit(1 as libc::c_int);
}

unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut current_block: u64;
    let mut srcBuf: *mut libc::c_uchar = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
    let mut w: libc::c_int = 0 as libc::c_int;
    let mut h: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut minQual: libc::c_int = -(1 as libc::c_int);
    let mut maxQual: libc::c_int = -(1 as libc::c_int);
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut minArg: libc::c_int = 2 as libc::c_int;
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut subsamp: libc::c_int = -(1 as libc::c_int);
    scalingFactors = crate::turbojpeg_h::tjGetScalingFactors(&mut nsf);
    if scalingFactors.is_null() || nsf == 0 as libc::c_int {
        crate::stdlib::printf(
            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
            804 as libc::c_int,
            b"executing tjGetScalingFactors()\x00" as *const u8 as *const libc::c_char,
            crate::turbojpeg_h::tjGetErrorStr(),
        );
        retval = -(1 as libc::c_int)
    } else {
        if argc < minArg {
            usage(*argv.offset(0 as libc::c_int as isize));
        }
        temp = crate::stdlib::strrchr(*argv.offset(1 as libc::c_int as isize), '.' as i32);
        if !temp.is_null() {
            if crate::stdlib::strcasecmp(temp, b".bmp\x00" as *const u8 as *const libc::c_char) == 0
            {
                ext = b"bmp\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            if crate::stdlib::strcasecmp(temp, b".jpg\x00" as *const u8 as *const libc::c_char) == 0
                || crate::stdlib::strcasecmp(temp, b".jpeg\x00" as *const u8 as *const libc::c_char)
                    == 0
            {
                decompOnly = 1 as libc::c_int
            }
        }
        crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
        if decompOnly == 0 {
            minArg = 3 as libc::c_int;
            if argc < minArg {
                usage(*argv.offset(0 as libc::c_int as isize));
            }
            minQual = atoi(*argv.offset(2 as libc::c_int as isize));
            if minQual < 1 as libc::c_int || minQual > 100 as libc::c_int {
                crate::stdlib::puts(
                    b"ERROR: Quality must be between 1 and 100.\x00" as *const u8
                        as *const libc::c_char,
                );
                crate::stdlib::exit(1 as libc::c_int);
            }
            temp = crate::stdlib::strchr(*argv.offset(2 as libc::c_int as isize), '-' as i32);
            if !(!temp.is_null()
                && crate::stdlib::strlen(temp) > 1 as libc::c_int as libc::c_ulong
                && crate::stdlib::sscanf(
                    &mut *temp.offset(1 as libc::c_int as isize) as *mut libc::c_char,
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    &mut maxQual as *mut libc::c_int,
                ) == 1 as libc::c_int
                && maxQual > minQual
                && maxQual >= 1 as libc::c_int
                && maxQual <= 100 as libc::c_int)
            {
                maxQual = minQual
            }
        }
        if argc > minArg {
            i = minArg;
            while i < argc {
                if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-tile\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    doTile = 1 as libc::c_int;
                    xformOpt |= crate::turbojpeg_h::TJXOPT_CROP
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-fastupsample\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    crate::stdlib::printf(
                        b"Using fast upsampling code\n\n\x00" as *const u8 as *const libc::c_char,
                    );
                    flags |= crate::turbojpeg_h::TJFLAG_FASTUPSAMPLE
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-fastdct\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    crate::stdlib::printf(
                        b"Using fastest DCT/IDCT algorithm\n\n\x00" as *const u8
                            as *const libc::c_char,
                    );
                    flags |= crate::turbojpeg_h::TJFLAG_FASTDCT
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-accuratedct\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    crate::stdlib::printf(
                        b"Using most accurate DCT/IDCT algorithm\n\n\x00" as *const u8
                            as *const libc::c_char,
                    );
                    flags |= crate::turbojpeg_h::TJFLAG_ACCURATEDCT
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-progressive\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    crate::stdlib::printf(
                        b"Using progressive entropy coding\n\n\x00" as *const u8
                            as *const libc::c_char,
                    );
                    flags |= crate::turbojpeg_h::TJFLAG_PROGRESSIVE
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-rgb\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    pf = crate::turbojpeg_h::TJPF_RGB as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-rgbx\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    pf = crate::turbojpeg_h::TJPF_RGBX as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-bgr\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    pf = crate::turbojpeg_h::TJPF_BGR as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-bgrx\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    pf = crate::turbojpeg_h::TJPF_BGRX as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-xbgr\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    pf = crate::turbojpeg_h::TJPF_XBGR as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-xrgb\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    pf = crate::turbojpeg_h::TJPF_XRGB as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-cmyk\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    pf = crate::turbojpeg_h::TJPF_CMYK as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-bottomup\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    flags |= crate::turbojpeg_h::TJFLAG_BOTTOMUP
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-quiet\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    quiet = 1 as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-qq\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    quiet = 2 as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-scale\x00" as *const u8 as *const libc::c_char,
                ) == 0
                    && i < argc - 1 as libc::c_int
                {
                    let mut temp1: libc::c_int = 0 as libc::c_int;
                    let mut temp2: libc::c_int = 0 as libc::c_int;
                    let mut match_0: libc::c_int = 0 as libc::c_int;
                    i += 1;
                    if crate::stdlib::sscanf(
                        *argv.offset(i as isize),
                        b"%d/%d\x00" as *const u8 as *const libc::c_char,
                        &mut temp1 as *mut libc::c_int,
                        &mut temp2 as *mut libc::c_int,
                    ) == 2 as libc::c_int
                    {
                        j = 0 as libc::c_int;
                        while j < nsf {
                            if temp1 as libc::c_double / temp2 as libc::c_double
                                == (*scalingFactors.offset(j as isize)).num as libc::c_double
                                    / (*scalingFactors.offset(j as isize)).denom as libc::c_double
                            {
                                sf = *scalingFactors.offset(j as isize);
                                match_0 = 1 as libc::c_int;
                                break;
                            } else {
                                j += 1
                            }
                        }
                        if match_0 == 0 {
                            usage(*argv.offset(0 as libc::c_int as isize));
                        }
                    } else {
                        usage(*argv.offset(0 as libc::c_int as isize));
                    }
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-hflip\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    xformOp = crate::turbojpeg_h::TJXOP_HFLIP as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-vflip\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    xformOp = crate::turbojpeg_h::TJXOP_VFLIP as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-transpose\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    xformOp = crate::turbojpeg_h::TJXOP_TRANSPOSE as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-transverse\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    xformOp = crate::turbojpeg_h::TJXOP_TRANSVERSE as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-rot90\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    xformOp = crate::turbojpeg_h::TJXOP_ROT90 as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-rot180\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    xformOp = crate::turbojpeg_h::TJXOP_ROT180 as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-rot270\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    xformOp = crate::turbojpeg_h::TJXOP_ROT270 as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-grayscale\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    xformOpt |= crate::turbojpeg_h::TJXOPT_GRAY
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-custom\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    customFilter = Some(
                        dummyDCTFilter
                            as unsafe extern "C" fn(
                                _: *mut libc::c_short,
                                _: crate::turbojpeg_h::tjregion,
                                _: crate::turbojpeg_h::tjregion,
                                _: libc::c_int,
                                _: libc::c_int,
                                _: *mut crate::turbojpeg_h::tjtransform,
                            ) -> libc::c_int,
                    )
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-nooutput\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    xformOpt |= crate::turbojpeg_h::TJXOPT_NOOUTPUT
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-copynone\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    xformOpt |= crate::turbojpeg_h::TJXOPT_COPYNONE
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-benchtime\x00" as *const u8 as *const libc::c_char,
                ) == 0
                    && i < argc - 1 as libc::c_int
                {
                    i += 1;
                    let mut temp_0: libc::c_double = atof(*argv.offset(i as isize));
                    if temp_0 > 0.0f64 {
                        benchTime = temp_0
                    } else {
                        usage(*argv.offset(0 as libc::c_int as isize));
                    }
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-warmup\x00" as *const u8 as *const libc::c_char,
                ) == 0
                    && i < argc - 1 as libc::c_int
                {
                    i += 1;
                    let mut temp_1: libc::c_double = atof(*argv.offset(i as isize));
                    if temp_1 >= 0.0f64 {
                        warmup = temp_1
                    } else {
                        usage(*argv.offset(0 as libc::c_int as isize));
                    }
                    crate::stdlib::printf(
                        b"Warmup time = %.1f seconds\n\n\x00" as *const u8 as *const libc::c_char,
                        warmup,
                    );
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-alloc\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    flags &= !crate::turbojpeg_h::TJFLAG_NOREALLOC
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-bmp\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    ext = b"bmp\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-yuv\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    crate::stdlib::printf(
                        b"Testing YUV planar encoding/decoding\n\n\x00" as *const u8
                            as *const libc::c_char,
                    );
                    doYUV = 1 as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-yuvpad\x00" as *const u8 as *const libc::c_char,
                ) == 0
                    && i < argc - 1 as libc::c_int
                {
                    i += 1;
                    let mut temp_2: libc::c_int = atoi(*argv.offset(i as isize));
                    if temp_2 >= 1 as libc::c_int {
                        yuvPad = temp_2
                    }
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-subsamp\x00" as *const u8 as *const libc::c_char,
                ) == 0
                    && i < argc - 1 as libc::c_int
                {
                    i += 1;
                    if ({
                        let mut __res: libc::c_int = 0;
                        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = *(*argv.offset(i as isize))
                                    .offset(0 as libc::c_int as isize)
                                    as libc::c_int;
                                __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int
                                {
                                    __c
                                } else {
                                    *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                                })
                            } else {
                                __res = toupper(
                                    *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                                        as libc::c_int,
                                )
                            }
                        } else {
                            __res = *(*crate::stdlib::__ctype_toupper_loc()).offset(
                                *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                                    as libc::c_int as isize,
                            )
                        }
                        __res
                    }) == 'G' as i32
                    {
                        subsamp = crate::turbojpeg_h::TJSAMP_GRAY as libc::c_int
                    } else {
                        let mut temp_3: libc::c_int = atoi(*argv.offset(i as isize));
                        match temp_3 {
                            444 => subsamp = crate::turbojpeg_h::TJSAMP_444 as libc::c_int,
                            422 => subsamp = crate::turbojpeg_h::TJSAMP_422 as libc::c_int,
                            440 => subsamp = crate::turbojpeg_h::TJSAMP_440 as libc::c_int,
                            420 => subsamp = crate::turbojpeg_h::TJSAMP_420 as libc::c_int,
                            411 => subsamp = crate::turbojpeg_h::TJSAMP_411 as libc::c_int,
                            _ => {}
                        }
                    }
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-componly\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    compOnly = 1 as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-nowrite\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    doWrite = 0 as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"-stoponwarning\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    flags |= crate::turbojpeg_h::TJFLAG_STOPONWARNING
                } else {
                    usage(*argv.offset(0 as libc::c_int as isize));
                }
                i += 1
            }
        }
        if (sf.num != 1 as libc::c_int || sf.denom != 1 as libc::c_int) && doTile != 0 {
            crate::stdlib::printf(
                b"Disabling tiled compression/decompression tests, because those tests do not\n\x00"
                    as *const u8 as *const libc::c_char,
            );
            crate::stdlib::printf(
                b"work when scaled decompression is enabled.\n\x00" as *const u8
                    as *const libc::c_char,
            );
            doTile = 0 as libc::c_int
        }
        if flags & crate::turbojpeg_h::TJFLAG_NOREALLOC == 0 as libc::c_int && doTile != 0 {
            crate::stdlib::printf(
                b"Disabling tiled compression/decompression tests, because those tests do not\n\x00"
                    as *const u8 as *const libc::c_char,
            );
            crate::stdlib::printf(
                b"work when dynamic JPEG buffer allocation is enabled.\n\n\x00" as *const u8
                    as *const libc::c_char,
            );
            doTile = 0 as libc::c_int
        }
        if decompOnly == 0 {
            srcBuf = crate::turbojpeg_h::tjLoadImage(
                *argv.offset(1 as libc::c_int as isize),
                &mut w,
                1 as libc::c_int,
                &mut h,
                &mut pf,
                flags,
            );
            if srcBuf.is_null() {
                crate::stdlib::printf(
                    b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                    962 as libc::c_int,
                    b"loading bitmap\x00" as *const u8 as *const libc::c_char,
                    crate::turbojpeg_h::tjGetErrorStr2(
                        crate::stddef_h::NULL_0 as *mut libc::c_void,
                    ),
                );
                retval = -(1 as libc::c_int);
                current_block = 15940078839392993310;
            } else {
                temp = crate::stdlib::strrchr(*argv.offset(1 as libc::c_int as isize), '.' as i32);
                if !temp.is_null() {
                    *temp = '\u{0}' as i32 as libc::c_char
                }
                current_block = 11359721434352816539;
            }
        } else {
            current_block = 11359721434352816539;
        }
        match current_block {
            15940078839392993310 => {}
            _ => {
                if quiet == 1 as libc::c_int && decompOnly == 0 {
                    crate::stdlib::printf(
                        b"All performance values in Mpixels/sec\n\n\x00" as *const u8
                            as *const libc::c_char,
                    );
                    crate::stdlib::printf(
                        b"Bitmap     JPEG     JPEG  %s  %s   \x00" as *const u8
                            as *const libc::c_char,
                        if doTile != 0 {
                            b"Tile \x00" as *const u8 as *const libc::c_char
                        } else {
                            b"Image\x00" as *const u8 as *const libc::c_char
                        },
                        if doTile != 0 {
                            b"Tile \x00" as *const u8 as *const libc::c_char
                        } else {
                            b"Image\x00" as *const u8 as *const libc::c_char
                        },
                    );
                    if doYUV != 0 {
                        crate::stdlib::printf(b"Encode  \x00" as *const u8 as *const libc::c_char);
                    }
                    crate::stdlib::printf(
                        b"Comp    Comp    Decomp  \x00" as *const u8 as *const libc::c_char,
                    );
                    if doYUV != 0 {
                        crate::stdlib::printf(b"Decode\x00" as *const u8 as *const libc::c_char);
                    }
                    crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                    crate::stdlib::printf(
                        b"Format     Subsamp  Qual  Width  Height  \x00" as *const u8
                            as *const libc::c_char,
                    );
                    if doYUV != 0 {
                        crate::stdlib::printf(b"Perf    \x00" as *const u8 as *const libc::c_char);
                    }
                    crate::stdlib::printf(
                        b"Perf    Ratio   Perf    \x00" as *const u8 as *const libc::c_char,
                    );
                    if doYUV != 0 {
                        crate::stdlib::printf(b"Perf\x00" as *const u8 as *const libc::c_char);
                    }
                    crate::stdlib::printf(b"\n\n\x00" as *const u8 as *const libc::c_char);
                }
                if decompOnly != 0 {
                    decompTest(*argv.offset(1 as libc::c_int as isize));
                    crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                } else if subsamp >= 0 as libc::c_int && subsamp < crate::turbojpeg_h::TJ_NUMSAMP {
                    i = maxQual;
                    while i >= minQual {
                        fullTest(
                            srcBuf,
                            w,
                            h,
                            subsamp,
                            i,
                            *argv.offset(1 as libc::c_int as isize),
                        );
                        i -= 1
                    }
                    crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                } else {
                    if pf != crate::turbojpeg_h::TJPF_CMYK as libc::c_int {
                        i = maxQual;
                        while i >= minQual {
                            fullTest(
                                srcBuf,
                                w,
                                h,
                                crate::turbojpeg_h::TJSAMP_GRAY as libc::c_int,
                                i,
                                *argv.offset(1 as libc::c_int as isize),
                            );
                            i -= 1
                        }
                        crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                    }
                    i = maxQual;
                    while i >= minQual {
                        fullTest(
                            srcBuf,
                            w,
                            h,
                            crate::turbojpeg_h::TJSAMP_420 as libc::c_int,
                            i,
                            *argv.offset(1 as libc::c_int as isize),
                        );
                        i -= 1
                    }
                    crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                    i = maxQual;
                    while i >= minQual {
                        fullTest(
                            srcBuf,
                            w,
                            h,
                            crate::turbojpeg_h::TJSAMP_422 as libc::c_int,
                            i,
                            *argv.offset(1 as libc::c_int as isize),
                        );
                        i -= 1
                    }
                    crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                    i = maxQual;
                    while i >= minQual {
                        fullTest(
                            srcBuf,
                            w,
                            h,
                            crate::turbojpeg_h::TJSAMP_444 as libc::c_int,
                            i,
                            *argv.offset(1 as libc::c_int as isize),
                        );
                        i -= 1
                    }
                    crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                }
            }
        }
    }
    if !srcBuf.is_null() {
        crate::turbojpeg_h::tjFree(srcBuf);
    }
    return retval;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
