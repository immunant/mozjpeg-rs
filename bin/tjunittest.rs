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
#![feature(extern_types, main, register_tool)]
pub mod jmorecfg_h {
    pub type JSAMPLE = libc::c_uchar;
}
pub mod md5_h {
    extern "C" {
        #[no_mangle]
        pub fn MD5File(_: *const libc::c_char, _: *mut libc::c_char) -> *mut libc::c_char;
    }
}
pub mod stddef_h {
    pub type size_t = libc::c_ulong;

    pub const NULL: libc::c_int = 0 as libc::c_int;
}
pub mod stdlib {
    extern "C" {
        #[no_mangle]
        pub fn __errno_location() -> *mut libc::c_int;
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
        pub fn fwrite(
            _: *const libc::c_void,
            _: libc::c_ulong,
            _: libc::c_ulong,
            _: *mut crate::stdlib::FILE,
        ) -> libc::c_ulong;
        #[no_mangle]
        pub fn random() -> libc::c_long;

        #[no_mangle]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;

        #[no_mangle]
        pub fn free(__ptr: *mut libc::c_void);

        #[no_mangle]
        pub fn exit(_: libc::c_int) -> !;
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

        #[no_mangle]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
        pub type _IO_wide_data;

        pub type _IO_codecvt;

        pub type _IO_marker;
        #[no_mangle]
        pub fn unlink(__name: *const libc::c_char) -> libc::c_int;
    }
    pub type FILE = crate::stdlib::_IO_FILE;
    pub const RAND_MAX: libc::c_int = 2147483647 as libc::c_int;
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
    pub type __off_t = libc::c_long;

    pub type __off64_t = libc::c_long;
}
use ::mozjpeg::*;

#[c2rust::header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/turbojpeg.h:38"]
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
     * Alpha offset (in bytes) for a given pixel format.  This specifies the number
     * of bytes that the Alpha component is offset from the start of the pixel.
     * For instance, if a pixel of format TJ_BGRA is stored in
     * <tt>char pixel[]</tt>, then the alpha component will be
     * <tt>pixel[tjAlphaOffset[TJ_BGRA]]</tt>.  This will be -1 if the pixel format
     * does not have an alpha component.
     */

    pub static mut tjAlphaOffset: [libc::c_int; 12] = [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
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
            flags: libc::c_int,
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
            pad_0: libc::c_int,
            height: libc::c_int,
            subsamp: libc::c_int,
            jpegBuf: *mut *mut libc::c_uchar,
            jpegSize: *mut libc::c_ulong,
            jpegQual: libc::c_int,
            flags: libc::c_int,
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
            pad_0: libc::c_int,
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
            pad_0: libc::c_int,
            subsamp: libc::c_int,
            flags: libc::c_int,
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
            flags: libc::c_int,
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
            pad_0: libc::c_int,
            height: libc::c_int,
            flags: libc::c_int,
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
            pad_0: libc::c_int,
            subsamp: libc::c_int,
            dstBuf: *mut libc::c_uchar,
            width: libc::c_int,
            pitch: libc::c_int,
            height: libc::c_int,
            pixelFormat: libc::c_int,
            flags: libc::c_int,
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
            flags: libc::c_int,
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
            flags: libc::c_int,
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

        #[no_mangle]
        pub fn tjDecompressHeader2(
            handle: crate::turbojpeg_h::tjhandle,
            jpegBuf: *mut libc::c_uchar,
            jpegSize: libc::c_ulong,
            width: *mut libc::c_int,
            height: *mut libc::c_int,
            jpegSubsamp: *mut libc::c_int,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn tjGetErrorStr() -> *mut libc::c_char;
    }
    pub type TJSAMP = libc::c_uint;
    /* *
     * Pixel formats
     */

    pub type TJPF = libc::c_int;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct tjscalingfactor {
        pub num: libc::c_int,
        pub denom: libc::c_int,
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
     * The number of pixel formats
     */

    pub const TJ_NUMPF: libc::c_int = 12 as libc::c_int;

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
     * @}
     */
}

#[c2rust::header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/cmyk.h:40"]
pub mod cmyk_h {
    /*
     * cmyk.h
     *
     * Copyright (C) 2017-2018, D. R. Commander.
     * For conditions of distribution and use, see the accompanying README.ijg
     * file.
     *
     * This file contains convenience functions for performing quick & dirty
     * CMYK<->RGB conversion.  This algorithm is suitable for testing purposes
     * only.  Properly converting between CMYK and RGB requires a color management
     * system.
     */
    /* Fully reversible */
    /* Fully reversible only for C/M/Y/K values generated with rgb_to_cmyk() */
    #[inline(always)]

    pub unsafe extern "C" fn cmyk_to_rgb(
        mut c: crate::jmorecfg_h::JSAMPLE,
        mut m: crate::jmorecfg_h::JSAMPLE,
        mut y: crate::jmorecfg_h::JSAMPLE,
        mut k: crate::jmorecfg_h::JSAMPLE,
        mut r: *mut crate::jmorecfg_h::JSAMPLE,
        mut g: *mut crate::jmorecfg_h::JSAMPLE,
        mut b: *mut crate::jmorecfg_h::JSAMPLE,
    ) {
        *r = (c as libc::c_double * k as libc::c_double / 255.0f64 + 0.5f64)
            as crate::jmorecfg_h::JSAMPLE;
        *g = (m as libc::c_double * k as libc::c_double / 255.0f64 + 0.5f64)
            as crate::jmorecfg_h::JSAMPLE;
        *b = (y as libc::c_double * k as libc::c_double / 255.0f64 + 0.5f64)
            as crate::jmorecfg_h::JSAMPLE;
    }
    #[inline(always)]

    pub unsafe extern "C" fn rgb_to_cmyk(
        mut r: crate::jmorecfg_h::JSAMPLE,
        mut g: crate::jmorecfg_h::JSAMPLE,
        mut b: crate::jmorecfg_h::JSAMPLE,
        mut c: *mut crate::jmorecfg_h::JSAMPLE,
        mut m: *mut crate::jmorecfg_h::JSAMPLE,
        mut y: *mut crate::jmorecfg_h::JSAMPLE,
        mut k: *mut crate::jmorecfg_h::JSAMPLE,
    ) {
        let mut ctmp: libc::c_double = 1.0f64 - r as libc::c_double / 255.0f64;
        let mut mtmp: libc::c_double = 1.0f64 - g as libc::c_double / 255.0f64;
        let mut ytmp: libc::c_double = 1.0f64 - b as libc::c_double / 255.0f64;
        let mut ktmp: libc::c_double = if (if ctmp < mtmp { ctmp } else { mtmp }) < ytmp {
            if ctmp < mtmp {
                ctmp
            } else {
                mtmp
            }
        } else {
            ytmp
        };
        if ktmp == 1.0f64 {
            ytmp = 0.0f64;
            mtmp = ytmp;
            ctmp = mtmp
        } else {
            ctmp = (ctmp - ktmp) / (1.0f64 - ktmp);
            mtmp = (mtmp - ktmp) / (1.0f64 - ktmp);
            ytmp = (ytmp - ktmp) / (1.0f64 - ktmp)
        }
        *c = (255.0f64 - ctmp * 255.0f64 + 0.5f64) as crate::jmorecfg_h::JSAMPLE;
        *m = (255.0f64 - mtmp * 255.0f64 + 0.5f64) as crate::jmorecfg_h::JSAMPLE;
        *y = (255.0f64 - ytmp * 255.0f64 + 0.5f64) as crate::jmorecfg_h::JSAMPLE;
        *k = (255.0f64 - ktmp * 255.0f64 + 0.5f64) as crate::jmorecfg_h::JSAMPLE;
    }
    use crate::jmorecfg_h::JSAMPLE;
    /* CMYK_H */
}

pub use crate::jmorecfg_h::JSAMPLE;
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
pub use crate::turbojpeg_h::tjAlloc;
pub use crate::turbojpeg_h::tjAlphaOffset;
pub use crate::turbojpeg_h::tjBlueOffset;
pub use crate::turbojpeg_h::tjBufSize;
pub use crate::turbojpeg_h::tjBufSizeYUV2;
pub use crate::turbojpeg_h::tjCompress2;
pub use crate::turbojpeg_h::tjCompressFromYUV;
pub use crate::turbojpeg_h::tjDecodeYUV;
pub use crate::turbojpeg_h::tjDecompress2;
pub use crate::turbojpeg_h::tjDecompressHeader2;
pub use crate::turbojpeg_h::tjDecompressToYUV2;
pub use crate::turbojpeg_h::tjDestroy;
pub use crate::turbojpeg_h::tjEncodeYUV3;
pub use crate::turbojpeg_h::tjFree;
pub use crate::turbojpeg_h::tjGetErrorStr;
pub use crate::turbojpeg_h::tjGetScalingFactors;
pub use crate::turbojpeg_h::tjGreenOffset;
pub use crate::turbojpeg_h::tjInitCompress;
pub use crate::turbojpeg_h::tjInitDecompress;
pub use crate::turbojpeg_h::tjLoadImage;
pub use crate::turbojpeg_h::tjMCUHeight;
pub use crate::turbojpeg_h::tjMCUWidth;
pub use crate::turbojpeg_h::tjPixelSize;
pub use crate::turbojpeg_h::tjRedOffset;
pub use crate::turbojpeg_h::tjSaveImage;
pub use crate::turbojpeg_h::tjhandle;
pub use crate::turbojpeg_h::tjscalingfactor;
pub use crate::turbojpeg_h::TJFLAG_BOTTOMUP;
pub use crate::turbojpeg_h::TJFLAG_FASTUPSAMPLE;
pub use crate::turbojpeg_h::TJFLAG_NOREALLOC;
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
pub use crate::turbojpeg_h::TJ_NUMPF;
pub use crate::turbojpeg_h::TJ_NUMSAMP;

pub use crate::cmyk_h::cmyk_to_rgb;
pub use crate::cmyk_h::rgb_to_cmyk;
use crate::md5_h::MD5File;
use crate::stdlib::__errno_location;
pub use crate::stdlib::exit;
use crate::stdlib::fclose;
use crate::stdlib::fopen;
pub use crate::stdlib::free;
use crate::stdlib::fwrite;
pub use crate::stdlib::malloc;
use crate::stdlib::memset;
use crate::stdlib::printf;
pub use crate::stdlib::random;
use crate::stdlib::snprintf;
use crate::stdlib::strcasecmp;
use crate::stdlib::strerror;
use crate::stdlib::unlink;
pub use crate::stdlib::RAND_MAX;
/*
 * Copyright (C)2009-2014, 2017-2018 D. R. Commander.  All Rights Reserved.
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
/*
 * This program tests the various code paths in the TurboJPEG C Wrapper
 */
#[no_mangle]

pub unsafe extern "C" fn usage(mut progName: *mut libc::c_char) {
    crate::stdlib::printf(
        b"\nUSAGE: %s [options]\n\n\x00" as *const u8 as *const libc::c_char,
        progName,
    );
    crate::stdlib::printf(b"Options:\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-yuv = test YUV encoding/decoding support\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-noyuvpad = do not pad each line of each Y, U, and V plane to the nearest\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"            4-byte boundary\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-alloc = test automatic buffer allocation\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-bmp = tjLoadImage()/tjSaveImage() unit test\n\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::exit(1 as libc::c_int);
}
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

pub static mut subName: [*const libc::c_char; 6] = [
    b"444\x00" as *const u8 as *const libc::c_char,
    b"422\x00" as *const u8 as *const libc::c_char,
    b"420\x00" as *const u8 as *const libc::c_char,
    b"GRAY\x00" as *const u8 as *const libc::c_char,
    b"440\x00" as *const u8 as *const libc::c_char,
    b"411\x00" as *const u8 as *const libc::c_char,
];
#[no_mangle]

pub static mut pixFormatStr: [*const libc::c_char; 12] = [
    b"RGB\x00" as *const u8 as *const libc::c_char,
    b"BGR\x00" as *const u8 as *const libc::c_char,
    b"RGBX\x00" as *const u8 as *const libc::c_char,
    b"BGRX\x00" as *const u8 as *const libc::c_char,
    b"XBGR\x00" as *const u8 as *const libc::c_char,
    b"XRGB\x00" as *const u8 as *const libc::c_char,
    b"Grayscale\x00" as *const u8 as *const libc::c_char,
    b"RGBA\x00" as *const u8 as *const libc::c_char,
    b"BGRA\x00" as *const u8 as *const libc::c_char,
    b"ABGR\x00" as *const u8 as *const libc::c_char,
    b"ARGB\x00" as *const u8 as *const libc::c_char,
    b"CMYK\x00" as *const u8 as *const libc::c_char,
];
#[no_mangle]

pub static mut _3byteFormats: [libc::c_int; 2] = [
    crate::turbojpeg_h::TJPF_RGB as libc::c_int,
    crate::turbojpeg_h::TJPF_BGR as libc::c_int,
];
#[no_mangle]

pub static mut _4byteFormats: [libc::c_int; 5] = [
    crate::turbojpeg_h::TJPF_RGBX as libc::c_int,
    crate::turbojpeg_h::TJPF_BGRX as libc::c_int,
    crate::turbojpeg_h::TJPF_XBGR as libc::c_int,
    crate::turbojpeg_h::TJPF_XRGB as libc::c_int,
    crate::turbojpeg_h::TJPF_CMYK as libc::c_int,
];
#[no_mangle]

pub static mut _onlyGray: [libc::c_int; 1] = [crate::turbojpeg_h::TJPF_GRAY as libc::c_int];
#[no_mangle]

pub static mut _onlyRGB: [libc::c_int; 1] = [crate::turbojpeg_h::TJPF_RGB as libc::c_int];
#[no_mangle]

pub static mut doYUV: libc::c_int = 0 as libc::c_int;
#[no_mangle]

pub static mut alloc: libc::c_int = 0 as libc::c_int;
#[no_mangle]

pub static mut pad: libc::c_int = 4 as libc::c_int;
#[no_mangle]

pub static mut exitStatus: libc::c_int = 0 as libc::c_int;
#[no_mangle]

pub unsafe extern "C" fn initBuf(
    mut buf: *mut libc::c_uchar,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut pf: libc::c_int,
    mut flags: libc::c_int,
) {
    let mut roffset: libc::c_int = tjRedOffset[pf as usize];
    let mut goffset: libc::c_int = tjGreenOffset[pf as usize];
    let mut boffset: libc::c_int = tjBlueOffset[pf as usize];
    let mut ps: libc::c_int = tjPixelSize[pf as usize];
    let mut index: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut halfway: libc::c_int = 16 as libc::c_int;
    if pf == crate::turbojpeg_h::TJPF_GRAY as libc::c_int {
        crate::stdlib::memset(
            buf as *mut libc::c_void,
            0 as libc::c_int,
            (w * h * ps) as libc::c_ulong,
        );
        row = 0 as libc::c_int;
        while row < h {
            col = 0 as libc::c_int;
            while col < w {
                if flags & crate::turbojpeg_h::TJFLAG_BOTTOMUP != 0 {
                    index = (h - row - 1 as libc::c_int) * w + col
                } else {
                    index = row * w + col
                }
                if (row / 8 as libc::c_int + col / 8 as libc::c_int) % 2 as libc::c_int
                    == 0 as libc::c_int
                {
                    *buf.offset(index as isize) = if row < halfway {
                        255 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } as libc::c_uchar
                } else {
                    *buf.offset(index as isize) = if row < halfway {
                        76 as libc::c_int
                    } else {
                        226 as libc::c_int
                    } as libc::c_uchar
                }
                col += 1
            }
            row += 1
        }
    } else if pf == crate::turbojpeg_h::TJPF_CMYK as libc::c_int {
        crate::stdlib::memset(
            buf as *mut libc::c_void,
            255 as libc::c_int,
            (w * h * ps) as libc::c_ulong,
        );
        row = 0 as libc::c_int;
        while row < h {
            col = 0 as libc::c_int;
            while col < w {
                if flags & crate::turbojpeg_h::TJFLAG_BOTTOMUP != 0 {
                    index = (h - row - 1 as libc::c_int) * w + col
                } else {
                    index = row * w + col
                }
                if (row / 8 as libc::c_int + col / 8 as libc::c_int) % 2 as libc::c_int
                    == 0 as libc::c_int
                {
                    if row >= halfway {
                        *buf.offset((index * ps + 3 as libc::c_int) as isize) =
                            0 as libc::c_int as libc::c_uchar
                    }
                } else {
                    *buf.offset((index * ps + 2 as libc::c_int) as isize) =
                        0 as libc::c_int as libc::c_uchar;
                    if row < halfway {
                        *buf.offset((index * ps + 1 as libc::c_int) as isize) =
                            0 as libc::c_int as libc::c_uchar
                    }
                }
                col += 1
            }
            row += 1
        }
    } else {
        crate::stdlib::memset(
            buf as *mut libc::c_void,
            0 as libc::c_int,
            (w * h * ps) as libc::c_ulong,
        );
        row = 0 as libc::c_int;
        while row < h {
            col = 0 as libc::c_int;
            while col < w {
                if flags & crate::turbojpeg_h::TJFLAG_BOTTOMUP != 0 {
                    index = (h - row - 1 as libc::c_int) * w + col
                } else {
                    index = row * w + col
                }
                if (row / 8 as libc::c_int + col / 8 as libc::c_int) % 2 as libc::c_int
                    == 0 as libc::c_int
                {
                    if row < halfway {
                        *buf.offset((index * ps + roffset) as isize) =
                            255 as libc::c_int as libc::c_uchar;
                        *buf.offset((index * ps + goffset) as isize) =
                            255 as libc::c_int as libc::c_uchar;
                        *buf.offset((index * ps + boffset) as isize) =
                            255 as libc::c_int as libc::c_uchar
                    }
                } else {
                    *buf.offset((index * ps + roffset) as isize) =
                        255 as libc::c_int as libc::c_uchar;
                    if row >= halfway {
                        *buf.offset((index * ps + goffset) as isize) =
                            255 as libc::c_int as libc::c_uchar
                    }
                }
                col += 1
            }
            row += 1
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn checkBuf(
    mut buf: *mut libc::c_uchar,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut pf: libc::c_int,
    mut subsamp: libc::c_int,
    mut sf: crate::turbojpeg_h::tjscalingfactor,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut roffset: libc::c_int = tjRedOffset[pf as usize];
    let mut goffset: libc::c_int = tjGreenOffset[pf as usize];
    let mut boffset: libc::c_int = tjBlueOffset[pf as usize];
    let mut aoffset: libc::c_int = tjAlphaOffset[pf as usize];
    let mut ps: libc::c_int = tjPixelSize[pf as usize];
    let mut index: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut retval: libc::c_int = 1 as libc::c_int;
    let mut halfway: libc::c_int = 16 as libc::c_int * sf.num / sf.denom;
    let mut blocksize: libc::c_int = 8 as libc::c_int * sf.num / sf.denom;
    if pf == crate::turbojpeg_h::TJPF_GRAY as libc::c_int {
        boffset = 0 as libc::c_int;
        goffset = boffset;
        roffset = goffset
    }
    if pf == crate::turbojpeg_h::TJPF_CMYK as libc::c_int {
        row = 0 as libc::c_int;
        's_40: loop {
            if !(row < h) {
                current_block = 15514718523126015390;
                break;
            }
            col = 0 as libc::c_int;
            while col < w {
                let mut c: libc::c_uchar = 0;
                let mut m: libc::c_uchar = 0;
                let mut y: libc::c_uchar = 0;
                let mut k: libc::c_uchar = 0;
                if flags & crate::turbojpeg_h::TJFLAG_BOTTOMUP != 0 {
                    index = (h - row - 1 as libc::c_int) * w + col
                } else {
                    index = row * w + col
                }
                c = *buf.offset((index * ps) as isize);
                m = *buf.offset((index * ps + 1 as libc::c_int) as isize);
                y = *buf.offset((index * ps + 2 as libc::c_int) as isize);
                k = *buf.offset((index * ps + 3 as libc::c_int) as isize);
                if (row / blocksize + col / blocksize) % 2 as libc::c_int == 0 as libc::c_int {
                    if (c as libc::c_int) < 254 as libc::c_int {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"c\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            c as libc::c_int,
                        );
                        retval = 0 as libc::c_int;
                        exitStatus = -(1 as libc::c_int);
                        current_block = 10090817204669828264;
                        break 's_40;
                    } else if (m as libc::c_int) < 254 as libc::c_int {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"m\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            m as libc::c_int,
                        );
                        retval = 0 as libc::c_int;
                        exitStatus = -(1 as libc::c_int);
                        current_block = 10090817204669828264;
                        break 's_40;
                    } else if (y as libc::c_int) < 254 as libc::c_int {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"y\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            y as libc::c_int,
                        );
                        retval = 0 as libc::c_int;
                        exitStatus = -(1 as libc::c_int);
                        current_block = 10090817204669828264;
                        break 's_40;
                    } else if row < halfway {
                        if (k as libc::c_int) < 254 as libc::c_int {
                            crate::stdlib::printf(
                                b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                                    as *const libc::c_char,
                                b"k\x00" as *const u8 as *const libc::c_char,
                                row,
                                col,
                                k as libc::c_int,
                            );
                            retval = 0 as libc::c_int;
                            exitStatus = -(1 as libc::c_int);
                            current_block = 10090817204669828264;
                            break 's_40;
                        }
                    } else if k as libc::c_int > 1 as libc::c_int {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"k\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            k as libc::c_int,
                        );
                        retval = 0 as libc::c_int;
                        exitStatus = -(1 as libc::c_int);
                        current_block = 10090817204669828264;
                        break 's_40;
                    }
                } else if (c as libc::c_int) < 254 as libc::c_int {
                    crate::stdlib::printf(
                        b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                            as *const libc::c_char,
                        b"c\x00" as *const u8 as *const libc::c_char,
                        row,
                        col,
                        c as libc::c_int,
                    );
                    retval = 0 as libc::c_int;
                    exitStatus = -(1 as libc::c_int);
                    current_block = 10090817204669828264;
                    break 's_40;
                } else if y as libc::c_int > 1 as libc::c_int {
                    crate::stdlib::printf(
                        b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                            as *const libc::c_char,
                        b"y\x00" as *const u8 as *const libc::c_char,
                        row,
                        col,
                        y as libc::c_int,
                    );
                    retval = 0 as libc::c_int;
                    exitStatus = -(1 as libc::c_int);
                    current_block = 10090817204669828264;
                    break 's_40;
                } else if (k as libc::c_int) < 254 as libc::c_int {
                    crate::stdlib::printf(
                        b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                            as *const libc::c_char,
                        b"k\x00" as *const u8 as *const libc::c_char,
                        row,
                        col,
                        k as libc::c_int,
                    );
                    retval = 0 as libc::c_int;
                    exitStatus = -(1 as libc::c_int);
                    current_block = 10090817204669828264;
                    break 's_40;
                } else if row < halfway {
                    if m as libc::c_int > 1 as libc::c_int {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"m\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            m as libc::c_int,
                        );
                        retval = 0 as libc::c_int;
                        exitStatus = -(1 as libc::c_int);
                        current_block = 10090817204669828264;
                        break 's_40;
                    }
                } else if (m as libc::c_int) < 254 as libc::c_int {
                    crate::stdlib::printf(
                        b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                            as *const libc::c_char,
                        b"m\x00" as *const u8 as *const libc::c_char,
                        row,
                        col,
                        m as libc::c_int,
                    );
                    retval = 0 as libc::c_int;
                    exitStatus = -(1 as libc::c_int);
                    current_block = 10090817204669828264;
                    break 's_40;
                }
                col += 1
            }
            row += 1
        }
        match current_block {
            10090817204669828264 => {}
            _ => return 1 as libc::c_int,
        }
    } else {
        row = 0 as libc::c_int;
        's_400: while row < h {
            col = 0 as libc::c_int;
            while col < w {
                let mut r: libc::c_uchar = 0;
                let mut g: libc::c_uchar = 0;
                let mut b: libc::c_uchar = 0;
                let mut a: libc::c_uchar = 0;
                if flags & crate::turbojpeg_h::TJFLAG_BOTTOMUP != 0 {
                    index = (h - row - 1 as libc::c_int) * w + col
                } else {
                    index = row * w + col
                }
                r = *buf.offset((index * ps + roffset) as isize);
                g = *buf.offset((index * ps + goffset) as isize);
                b = *buf.offset((index * ps + boffset) as isize);
                a = if aoffset >= 0 as libc::c_int {
                    *buf.offset((index * ps + aoffset) as isize) as libc::c_int
                } else {
                    0xff as libc::c_int
                } as libc::c_uchar;
                if (row / blocksize + col / blocksize) % 2 as libc::c_int == 0 as libc::c_int {
                    if row < halfway {
                        if (r as libc::c_int) < 254 as libc::c_int {
                            crate::stdlib::printf(
                                b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                                    as *const libc::c_char,
                                b"r\x00" as *const u8 as *const libc::c_char,
                                row,
                                col,
                                r as libc::c_int,
                            );
                            retval = 0 as libc::c_int;
                            exitStatus = -(1 as libc::c_int);
                            break 's_400;
                        } else if (g as libc::c_int) < 254 as libc::c_int {
                            crate::stdlib::printf(
                                b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                                    as *const libc::c_char,
                                b"g\x00" as *const u8 as *const libc::c_char,
                                row,
                                col,
                                g as libc::c_int,
                            );
                            retval = 0 as libc::c_int;
                            exitStatus = -(1 as libc::c_int);
                            break 's_400;
                        } else if (b as libc::c_int) < 254 as libc::c_int {
                            crate::stdlib::printf(
                                b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                                    as *const libc::c_char,
                                b"b\x00" as *const u8 as *const libc::c_char,
                                row,
                                col,
                                b as libc::c_int,
                            );
                            retval = 0 as libc::c_int;
                            exitStatus = -(1 as libc::c_int);
                            break 's_400;
                        }
                    } else if r as libc::c_int > 1 as libc::c_int {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"r\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            r as libc::c_int,
                        );
                        retval = 0 as libc::c_int;
                        exitStatus = -(1 as libc::c_int);
                        break 's_400;
                    } else if g as libc::c_int > 1 as libc::c_int {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"g\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            g as libc::c_int,
                        );
                        retval = 0 as libc::c_int;
                        exitStatus = -(1 as libc::c_int);
                        break 's_400;
                    } else if b as libc::c_int > 1 as libc::c_int {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"b\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            b as libc::c_int,
                        );
                        retval = 0 as libc::c_int;
                        exitStatus = -(1 as libc::c_int);
                        break 's_400;
                    }
                } else if subsamp == crate::turbojpeg_h::TJSAMP_GRAY as libc::c_int {
                    if row < halfway {
                        if (r as libc::c_int) < 76 as libc::c_int - 1 as libc::c_int
                            || r as libc::c_int > 76 as libc::c_int + 1 as libc::c_int
                        {
                            crate::stdlib::printf(
                                b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                                    as *const libc::c_char,
                                b"r\x00" as *const u8 as *const libc::c_char,
                                row,
                                col,
                                76 as libc::c_int,
                                r as libc::c_int,
                            );
                            retval = 0 as libc::c_int;
                            exitStatus = -(1 as libc::c_int);
                            break 's_400;
                        } else if (g as libc::c_int) < 76 as libc::c_int - 1 as libc::c_int
                            || g as libc::c_int > 76 as libc::c_int + 1 as libc::c_int
                        {
                            crate::stdlib::printf(
                                b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                                    as *const libc::c_char,
                                b"g\x00" as *const u8 as *const libc::c_char,
                                row,
                                col,
                                76 as libc::c_int,
                                g as libc::c_int,
                            );
                            retval = 0 as libc::c_int;
                            exitStatus = -(1 as libc::c_int);
                            break 's_400;
                        } else if (b as libc::c_int) < 76 as libc::c_int - 1 as libc::c_int
                            || b as libc::c_int > 76 as libc::c_int + 1 as libc::c_int
                        {
                            crate::stdlib::printf(
                                b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                                    as *const libc::c_char,
                                b"b\x00" as *const u8 as *const libc::c_char,
                                row,
                                col,
                                76 as libc::c_int,
                                b as libc::c_int,
                            );
                            retval = 0 as libc::c_int;
                            exitStatus = -(1 as libc::c_int);
                            break 's_400;
                        }
                    } else if (r as libc::c_int) < 226 as libc::c_int - 1 as libc::c_int
                        || r as libc::c_int > 226 as libc::c_int + 1 as libc::c_int
                    {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"r\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            226 as libc::c_int,
                            r as libc::c_int,
                        );
                        retval = 0 as libc::c_int;
                        exitStatus = -(1 as libc::c_int);
                        break 's_400;
                    } else if (g as libc::c_int) < 226 as libc::c_int - 1 as libc::c_int
                        || g as libc::c_int > 226 as libc::c_int + 1 as libc::c_int
                    {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"g\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            226 as libc::c_int,
                            g as libc::c_int,
                        );
                        retval = 0 as libc::c_int;
                        exitStatus = -(1 as libc::c_int);
                        break 's_400;
                    } else if (b as libc::c_int) < 226 as libc::c_int - 1 as libc::c_int
                        || b as libc::c_int > 226 as libc::c_int + 1 as libc::c_int
                    {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"b\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            226 as libc::c_int,
                            b as libc::c_int,
                        );
                        retval = 0 as libc::c_int;
                        exitStatus = -(1 as libc::c_int);
                        break 's_400;
                    }
                } else if row < halfway {
                    if (r as libc::c_int) < 254 as libc::c_int {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"r\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            r as libc::c_int,
                        );
                        retval = 0 as libc::c_int;
                        exitStatus = -(1 as libc::c_int);
                        break 's_400;
                    } else if g as libc::c_int > 1 as libc::c_int {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"g\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            g as libc::c_int,
                        );
                        retval = 0 as libc::c_int;
                        exitStatus = -(1 as libc::c_int);
                        break 's_400;
                    } else if b as libc::c_int > 1 as libc::c_int {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"b\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            b as libc::c_int,
                        );
                        retval = 0 as libc::c_int;
                        exitStatus = -(1 as libc::c_int);
                        break 's_400;
                    }
                } else if (r as libc::c_int) < 254 as libc::c_int {
                    crate::stdlib::printf(
                        b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                            as *const libc::c_char,
                        b"r\x00" as *const u8 as *const libc::c_char,
                        row,
                        col,
                        r as libc::c_int,
                    );
                    retval = 0 as libc::c_int;
                    exitStatus = -(1 as libc::c_int);
                    break 's_400;
                } else if (g as libc::c_int) < 254 as libc::c_int {
                    crate::stdlib::printf(
                        b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                            as *const libc::c_char,
                        b"g\x00" as *const u8 as *const libc::c_char,
                        row,
                        col,
                        g as libc::c_int,
                    );
                    retval = 0 as libc::c_int;
                    exitStatus = -(1 as libc::c_int);
                    break 's_400;
                } else if b as libc::c_int > 1 as libc::c_int {
                    crate::stdlib::printf(
                        b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                            as *const libc::c_char,
                        b"b\x00" as *const u8 as *const libc::c_char,
                        row,
                        col,
                        b as libc::c_int,
                    );
                    retval = 0 as libc::c_int;
                    exitStatus = -(1 as libc::c_int);
                    break 's_400;
                }
                if (a as libc::c_int) < 254 as libc::c_int {
                    crate::stdlib::printf(
                        b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                            as *const libc::c_char,
                        b"a\x00" as *const u8 as *const libc::c_char,
                        row,
                        col,
                        a as libc::c_int,
                    );
                    retval = 0 as libc::c_int;
                    exitStatus = -(1 as libc::c_int);
                    break 's_400;
                } else {
                    col += 1
                }
            }
            row += 1
        }
    }
    if retval == 0 as libc::c_int {
        row = 0 as libc::c_int;
        while row < h {
            col = 0 as libc::c_int;
            while col < w {
                if pf == crate::turbojpeg_h::TJPF_CMYK as libc::c_int {
                    crate::stdlib::printf(
                        b"%.3d/%.3d/%.3d/%.3d \x00" as *const u8 as *const libc::c_char,
                        *buf.offset(((row * w + col) * ps) as isize) as libc::c_int,
                        *buf.offset(((row * w + col) * ps + 1 as libc::c_int) as isize)
                            as libc::c_int,
                        *buf.offset(((row * w + col) * ps + 2 as libc::c_int) as isize)
                            as libc::c_int,
                        *buf.offset(((row * w + col) * ps + 3 as libc::c_int) as isize)
                            as libc::c_int,
                    );
                } else {
                    crate::stdlib::printf(
                        b"%.3d/%.3d/%.3d \x00" as *const u8 as *const libc::c_char,
                        *buf.offset(((row * w + col) * ps + roffset) as isize) as libc::c_int,
                        *buf.offset(((row * w + col) * ps + goffset) as isize) as libc::c_int,
                        *buf.offset(((row * w + col) * ps + boffset) as isize) as libc::c_int,
                    );
                }
                col += 1
            }
            crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
            row += 1
        }
    }
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn checkBufYUV(
    mut buf: *mut libc::c_uchar,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut subsamp: libc::c_int,
    mut sf: crate::turbojpeg_h::tjscalingfactor,
) -> libc::c_int {
    let mut current_block: u64;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut hsf: libc::c_int = tjMCUWidth[subsamp as usize] / 8 as libc::c_int;
    let mut vsf: libc::c_int = tjMCUHeight[subsamp as usize] / 8 as libc::c_int;
    let mut pw: libc::c_int = w + hsf - 1 as libc::c_int & !(hsf - 1 as libc::c_int);
    let mut ph: libc::c_int = h + vsf - 1 as libc::c_int & !(vsf - 1 as libc::c_int);
    let mut cw: libc::c_int = pw / hsf;
    let mut ch: libc::c_int = ph / vsf;
    let mut ypitch: libc::c_int = pw + pad - 1 as libc::c_int & !(pad - 1 as libc::c_int);
    let mut uvpitch: libc::c_int = cw + pad - 1 as libc::c_int & !(pad - 1 as libc::c_int);
    let mut retval: libc::c_int = 1 as libc::c_int;
    let mut halfway: libc::c_int = 16 as libc::c_int * sf.num / sf.denom;
    let mut blocksize: libc::c_int = 8 as libc::c_int * sf.num / sf.denom;
    row = 0 as libc::c_int;
    's_27: loop {
        if !(row < ph) {
            current_block = 1836292691772056875;
            break;
        }
        col = 0 as libc::c_int;
        while col < pw {
            let mut y: libc::c_uchar = *buf.offset((ypitch * row + col) as isize);
            if (row / blocksize + col / blocksize) % 2 as libc::c_int == 0 as libc::c_int {
                if row < halfway {
                    if (y as libc::c_int) < 254 as libc::c_int {
                        crate::stdlib::printf(
                            b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"y\x00" as *const u8 as *const libc::c_char,
                            row,
                            col,
                            y as libc::c_int,
                        );
                        retval = 0 as libc::c_int;
                        exitStatus = -(1 as libc::c_int);
                        current_block = 17379669344980314800;
                        break 's_27;
                    }
                } else if y as libc::c_int > 1 as libc::c_int {
                    crate::stdlib::printf(
                        b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                            as *const libc::c_char,
                        b"y\x00" as *const u8 as *const libc::c_char,
                        row,
                        col,
                        y as libc::c_int,
                    );
                    retval = 0 as libc::c_int;
                    exitStatus = -(1 as libc::c_int);
                    current_block = 17379669344980314800;
                    break 's_27;
                }
            } else if row < halfway {
                if (y as libc::c_int) < 76 as libc::c_int - 1 as libc::c_int
                    || y as libc::c_int > 76 as libc::c_int + 1 as libc::c_int
                {
                    crate::stdlib::printf(
                        b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                            as *const libc::c_char,
                        b"y\x00" as *const u8 as *const libc::c_char,
                        row,
                        col,
                        76 as libc::c_int,
                        y as libc::c_int,
                    );
                    retval = 0 as libc::c_int;
                    exitStatus = -(1 as libc::c_int);
                    current_block = 17379669344980314800;
                    break 's_27;
                }
            } else if (y as libc::c_int) < 226 as libc::c_int - 1 as libc::c_int
                || y as libc::c_int > 226 as libc::c_int + 1 as libc::c_int
            {
                crate::stdlib::printf(
                    b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                        as *const libc::c_char,
                    b"y\x00" as *const u8 as *const libc::c_char,
                    row,
                    col,
                    226 as libc::c_int,
                    y as libc::c_int,
                );
                retval = 0 as libc::c_int;
                exitStatus = -(1 as libc::c_int);
                current_block = 17379669344980314800;
                break 's_27;
            }
            col += 1
        }
        row += 1
    }
    match current_block {
        1836292691772056875 => {
            if subsamp != crate::turbojpeg_h::TJSAMP_GRAY as libc::c_int {
                let mut halfway_0: libc::c_int = 16 as libc::c_int / vsf * sf.num / sf.denom;
                row = 0 as libc::c_int;
                's_193: while row < ch {
                    col = 0 as libc::c_int;
                    while col < cw {
                        let mut u: libc::c_uchar =
                            *buf.offset((ypitch * ph + (uvpitch * row + col)) as isize);
                        let mut v: libc::c_uchar = *buf
                            .offset((ypitch * ph + uvpitch * ch + (uvpitch * row + col)) as isize);
                        if (row * vsf / blocksize + col * hsf / blocksize) % 2 as libc::c_int
                            == 0 as libc::c_int
                        {
                            if (u as libc::c_int) < 128 as libc::c_int - 1 as libc::c_int
                                || u as libc::c_int > 128 as libc::c_int + 1 as libc::c_int
                            {
                                crate::stdlib::printf(
                                    b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                                        as *const libc::c_char,
                                    b"u\x00" as *const u8 as *const libc::c_char,
                                    row,
                                    col,
                                    128 as libc::c_int,
                                    u as libc::c_int,
                                );
                                retval = 0 as libc::c_int;
                                exitStatus = -(1 as libc::c_int);
                                break 's_193;
                            } else if (v as libc::c_int) < 128 as libc::c_int - 1 as libc::c_int
                                || v as libc::c_int > 128 as libc::c_int + 1 as libc::c_int
                            {
                                crate::stdlib::printf(
                                    b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                                        as *const libc::c_char,
                                    b"v\x00" as *const u8 as *const libc::c_char,
                                    row,
                                    col,
                                    128 as libc::c_int,
                                    v as libc::c_int,
                                );
                                retval = 0 as libc::c_int;
                                exitStatus = -(1 as libc::c_int);
                                break 's_193;
                            }
                        } else if row < halfway_0 {
                            if (u as libc::c_int) < 85 as libc::c_int - 1 as libc::c_int
                                || u as libc::c_int > 85 as libc::c_int + 1 as libc::c_int
                            {
                                crate::stdlib::printf(
                                    b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                                        as *const libc::c_char,
                                    b"u\x00" as *const u8 as *const libc::c_char,
                                    row,
                                    col,
                                    85 as libc::c_int,
                                    u as libc::c_int,
                                );
                                retval = 0 as libc::c_int;
                                exitStatus = -(1 as libc::c_int);
                                break 's_193;
                            } else if (v as libc::c_int) < 254 as libc::c_int {
                                crate::stdlib::printf(
                                    b"\nComp. %s at %d,%d should be 255, not %d\n\x00" as *const u8
                                        as *const libc::c_char,
                                    b"v\x00" as *const u8 as *const libc::c_char,
                                    row,
                                    col,
                                    v as libc::c_int,
                                );
                                retval = 0 as libc::c_int;
                                exitStatus = -(1 as libc::c_int);
                                break 's_193;
                            }
                        } else if u as libc::c_int > 1 as libc::c_int {
                            crate::stdlib::printf(
                                b"\nComp. %s at %d,%d should be 0, not %d\n\x00" as *const u8
                                    as *const libc::c_char,
                                b"u\x00" as *const u8 as *const libc::c_char,
                                row,
                                col,
                                u as libc::c_int,
                            );
                            retval = 0 as libc::c_int;
                            exitStatus = -(1 as libc::c_int);
                            break 's_193;
                        } else if (v as libc::c_int) < 149 as libc::c_int - 1 as libc::c_int
                            || v as libc::c_int > 149 as libc::c_int + 1 as libc::c_int
                        {
                            crate::stdlib::printf(
                                b"\nComp. %s at %d,%d should be %d, not %d\n\x00" as *const u8
                                    as *const libc::c_char,
                                b"v\x00" as *const u8 as *const libc::c_char,
                                row,
                                col,
                                149 as libc::c_int,
                                v as libc::c_int,
                            );
                            retval = 0 as libc::c_int;
                            exitStatus = -(1 as libc::c_int);
                            break 's_193;
                        }
                        col += 1
                    }
                    row += 1
                }
            }
        }
        _ => {}
    }
    if retval == 0 as libc::c_int {
        row = 0 as libc::c_int;
        while row < ph {
            col = 0 as libc::c_int;
            while col < pw {
                crate::stdlib::printf(
                    b"%.3d \x00" as *const u8 as *const libc::c_char,
                    *buf.offset((ypitch * row + col) as isize) as libc::c_int,
                );
                col += 1
            }
            crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
            row += 1
        }
        crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
        row = 0 as libc::c_int;
        while row < ch {
            col = 0 as libc::c_int;
            while col < cw {
                crate::stdlib::printf(
                    b"%.3d \x00" as *const u8 as *const libc::c_char,
                    *buf.offset((ypitch * ph + (uvpitch * row + col)) as isize) as libc::c_int,
                );
                col += 1
            }
            crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
            row += 1
        }
        crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
        row = 0 as libc::c_int;
        while row < ch {
            col = 0 as libc::c_int;
            while col < cw {
                crate::stdlib::printf(
                    b"%.3d \x00" as *const u8 as *const libc::c_char,
                    *buf.offset((ypitch * ph + uvpitch * ch + (uvpitch * row + col)) as isize)
                        as libc::c_int,
                );
                col += 1
            }
            crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
            row += 1
        }
    }
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn writeJPEG(
    mut jpegBuf: *mut libc::c_uchar,
    mut jpegSize: libc::c_ulong,
    mut filename: *mut libc::c_char,
) {
    let mut file: *mut crate::stdlib::FILE =
        crate::stdlib::fopen(filename, b"wb\x00" as *const u8 as *const libc::c_char);
    if file.is_null()
        || crate::stdlib::fwrite(
            jpegBuf as *const libc::c_void,
            jpegSize,
            1 as libc::c_int as libc::c_ulong,
            file,
        ) != 1 as libc::c_int as libc::c_ulong
    {
        crate::stdlib::printf(
            b"ERROR: Could not write to %s.\n%s\n\x00" as *const u8 as *const libc::c_char,
            filename,
            crate::stdlib::strerror(*crate::stdlib::__errno_location()),
        );
        exitStatus = -(1 as libc::c_int)
    }
    if !file.is_null() {
        crate::stdlib::fclose(file);
    };
}
#[no_mangle]

pub unsafe extern "C" fn compTest(
    mut handle: crate::turbojpeg_h::tjhandle,
    mut dstBuf: *mut *mut libc::c_uchar,
    mut dstSize: *mut libc::c_ulong,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut pf: libc::c_int,
    mut basename: *mut libc::c_char,
    mut subsamp: libc::c_int,
    mut jpegQual: libc::c_int,
    mut flags: libc::c_int,
) {
    let mut current_block: u64;
    let mut tempStr: [libc::c_char; 1024] = [0; 1024];
    let mut srcBuf: *mut libc::c_uchar = crate::stddef_h::NULL as *mut libc::c_uchar;
    let mut yuvBuf: *mut libc::c_uchar = crate::stddef_h::NULL as *mut libc::c_uchar;
    let mut pfStr: *const libc::c_char = pixFormatStr[pf as usize];
    let mut buStrLong: *const libc::c_char = if flags & crate::turbojpeg_h::TJFLAG_BOTTOMUP != 0 {
        b"Bottom-Up\x00" as *const u8 as *const libc::c_char
    } else {
        b"Top-Down \x00" as *const u8 as *const libc::c_char
    };
    let mut buStr: *const libc::c_char = if flags & crate::turbojpeg_h::TJFLAG_BOTTOMUP != 0 {
        b"BU\x00" as *const u8 as *const libc::c_char
    } else {
        b"TD\x00" as *const u8 as *const libc::c_char
    };
    srcBuf = crate::stdlib::malloc((w * h * tjPixelSize[pf as usize]) as libc::c_ulong)
        as *mut libc::c_uchar;
    if srcBuf.is_null() {
        crate::stdlib::printf(
            b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
            b"Memory allocation failure\x00" as *const u8 as *const libc::c_char,
        );
        exitStatus = -(1 as libc::c_int)
    } else {
        initBuf(srcBuf, w, h, pf, flags);
        if !(*dstBuf).is_null() && *dstSize > 0 as libc::c_int as libc::c_ulong {
            crate::stdlib::memset(*dstBuf as *mut libc::c_void, 0 as libc::c_int, *dstSize);
        }
        if alloc == 0 {
            flags |= crate::turbojpeg_h::TJFLAG_NOREALLOC
        }
        if doYUV != 0 {
            let mut yuvSize: libc::c_ulong = crate::turbojpeg_h::tjBufSizeYUV2(w, pad, h, subsamp);
            let mut sf: crate::turbojpeg_h::tjscalingfactor = {
                let mut init = crate::turbojpeg_h::tjscalingfactor {
                    num: 1 as libc::c_int,
                    denom: 1 as libc::c_int,
                };
                init
            };
            let mut handle2: crate::turbojpeg_h::tjhandle = crate::turbojpeg_h::tjInitCompress();
            if handle2.is_null() {
                crate::stdlib::printf(
                    b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                    crate::turbojpeg_h::tjGetErrorStr(),
                );
                exitStatus = -(1 as libc::c_int);
                current_block = 860601949763470011;
            } else {
                yuvBuf = crate::stdlib::malloc(yuvSize) as *mut libc::c_uchar;
                if yuvBuf.is_null() {
                    crate::stdlib::printf(
                        b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
                        b"Memory allocation failure\x00" as *const u8 as *const libc::c_char,
                    );
                    exitStatus = -(1 as libc::c_int);
                    current_block = 860601949763470011;
                } else {
                    crate::stdlib::memset(yuvBuf as *mut libc::c_void, 0 as libc::c_int, yuvSize);
                    crate::stdlib::printf(
                        b"%s %s -> YUV %s ... \x00" as *const u8 as *const libc::c_char,
                        pfStr,
                        buStrLong,
                        subNameLong[subsamp as usize],
                    );
                    if crate::turbojpeg_h::tjEncodeYUV3(
                        handle2,
                        srcBuf,
                        w,
                        0 as libc::c_int,
                        h,
                        pf,
                        yuvBuf,
                        pad,
                        subsamp,
                        flags,
                    ) == -(1 as libc::c_int)
                    {
                        crate::stdlib::printf(
                            b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                            crate::turbojpeg_h::tjGetErrorStr(),
                        );
                        exitStatus = -(1 as libc::c_int);
                        current_block = 860601949763470011;
                    } else {
                        crate::turbojpeg_h::tjDestroy(handle2);
                        if checkBufYUV(yuvBuf, w, h, subsamp, sf) != 0 {
                            crate::stdlib::printf(
                                b"Passed.\n\x00" as *const u8 as *const libc::c_char,
                            );
                        } else {
                            crate::stdlib::printf(
                                b"FAILED!\n\x00" as *const u8 as *const libc::c_char,
                            );
                        }
                        crate::stdlib::printf(
                            b"YUV %s %s -> JPEG Q%d ... \x00" as *const u8 as *const libc::c_char,
                            subNameLong[subsamp as usize],
                            buStrLong,
                            jpegQual,
                        );
                        if crate::turbojpeg_h::tjCompressFromYUV(
                            handle, yuvBuf, w, pad, h, subsamp, dstBuf, dstSize, jpegQual, flags,
                        ) == -(1 as libc::c_int)
                        {
                            crate::stdlib::printf(
                                b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                                crate::turbojpeg_h::tjGetErrorStr(),
                            );
                            exitStatus = -(1 as libc::c_int);
                            current_block = 860601949763470011;
                        } else {
                            current_block = 10095721787123848864;
                        }
                    }
                }
            }
        } else {
            crate::stdlib::printf(
                b"%s %s -> %s Q%d ... \x00" as *const u8 as *const libc::c_char,
                pfStr,
                buStrLong,
                subNameLong[subsamp as usize],
                jpegQual,
            );
            if crate::turbojpeg_h::tjCompress2(
                handle,
                srcBuf,
                w,
                0 as libc::c_int,
                h,
                pf,
                dstBuf,
                dstSize,
                subsamp,
                jpegQual,
                flags,
            ) == -(1 as libc::c_int)
            {
                crate::stdlib::printf(
                    b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                    crate::turbojpeg_h::tjGetErrorStr(),
                );
                exitStatus = -(1 as libc::c_int);
                current_block = 860601949763470011;
            } else {
                current_block = 10095721787123848864;
            }
        }
        match current_block {
            860601949763470011 => {}
            _ => {
                crate::stdlib::snprintf(
                    tempStr.as_mut_ptr(),
                    1024 as libc::c_int as libc::c_ulong,
                    b"%s_enc_%s_%s_%s_Q%d.jpg\x00" as *const u8 as *const libc::c_char,
                    basename,
                    pfStr,
                    buStr,
                    subName[subsamp as usize],
                    jpegQual,
                );
                writeJPEG(*dstBuf, *dstSize, tempStr.as_mut_ptr());
                crate::stdlib::printf(
                    b"Done.\n  Result in %s\n\x00" as *const u8 as *const libc::c_char,
                    tempStr.as_mut_ptr(),
                );
            }
        }
    }
    if !yuvBuf.is_null() {
        crate::stdlib::free(yuvBuf as *mut libc::c_void);
    }
    if !srcBuf.is_null() {
        crate::stdlib::free(srcBuf as *mut libc::c_void);
    };
}
#[no_mangle]

pub unsafe extern "C" fn _decompTest(
    mut handle: crate::turbojpeg_h::tjhandle,
    mut jpegBuf: *mut libc::c_uchar,
    mut jpegSize: libc::c_ulong,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut pf: libc::c_int,
    mut basename: *mut libc::c_char,
    mut subsamp: libc::c_int,
    mut flags: libc::c_int,
    mut sf: crate::turbojpeg_h::tjscalingfactor,
) {
    let mut current_block: u64;
    let mut dstBuf: *mut libc::c_uchar = crate::stddef_h::NULL as *mut libc::c_uchar;
    let mut yuvBuf: *mut libc::c_uchar = crate::stddef_h::NULL as *mut libc::c_uchar;
    let mut _hdrw: libc::c_int = 0 as libc::c_int;
    let mut _hdrh: libc::c_int = 0 as libc::c_int;
    let mut _hdrsubsamp: libc::c_int = -(1 as libc::c_int);
    let mut scaledWidth: libc::c_int = (w * sf.num + sf.denom - 1 as libc::c_int) / sf.denom;
    let mut scaledHeight: libc::c_int = (h * sf.num + sf.denom - 1 as libc::c_int) / sf.denom;
    let mut dstSize: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    if crate::turbojpeg_h::tjDecompressHeader2(
        handle,
        jpegBuf,
        jpegSize,
        &mut _hdrw,
        &mut _hdrh,
        &mut _hdrsubsamp,
    ) == -(1 as libc::c_int)
    {
        crate::stdlib::printf(
            b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
            crate::turbojpeg_h::tjGetErrorStr(),
        );
        exitStatus = -(1 as libc::c_int)
    } else if _hdrw != w || _hdrh != h || _hdrsubsamp != subsamp {
        crate::stdlib::printf(
            b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
            b"Incorrect JPEG header\x00" as *const u8 as *const libc::c_char,
        );
        exitStatus = -(1 as libc::c_int)
    } else {
        dstSize = (scaledWidth * scaledHeight * tjPixelSize[pf as usize]) as libc::c_ulong;
        dstBuf = crate::stdlib::malloc(dstSize) as *mut libc::c_uchar;
        if dstBuf.is_null() {
            crate::stdlib::printf(
                b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
                b"Memory allocation failure\x00" as *const u8 as *const libc::c_char,
            );
            exitStatus = -(1 as libc::c_int)
        } else {
            crate::stdlib::memset(dstBuf as *mut libc::c_void, 0 as libc::c_int, dstSize);
            if doYUV != 0 {
                let mut yuvSize: libc::c_ulong =
                    crate::turbojpeg_h::tjBufSizeYUV2(scaledWidth, pad, scaledHeight, subsamp);
                let mut handle2: crate::turbojpeg_h::tjhandle =
                    crate::turbojpeg_h::tjInitDecompress();
                if handle2.is_null() {
                    crate::stdlib::printf(
                        b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                        crate::turbojpeg_h::tjGetErrorStr(),
                    );
                    exitStatus = -(1 as libc::c_int);
                    current_block = 14297773496329963019;
                } else {
                    yuvBuf = crate::stdlib::malloc(yuvSize) as *mut libc::c_uchar;
                    if yuvBuf.is_null() {
                        crate::stdlib::printf(
                            b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
                            b"Memory allocation failure\x00" as *const u8 as *const libc::c_char,
                        );
                        exitStatus = -(1 as libc::c_int);
                        current_block = 14297773496329963019;
                    } else {
                        crate::stdlib::memset(
                            yuvBuf as *mut libc::c_void,
                            0 as libc::c_int,
                            yuvSize,
                        );
                        crate::stdlib::printf(
                            b"JPEG -> YUV %s \x00" as *const u8 as *const libc::c_char,
                            subNameLong[subsamp as usize],
                        );
                        if sf.num != 1 as libc::c_int || sf.denom != 1 as libc::c_int {
                            crate::stdlib::printf(
                                b"%d/%d ... \x00" as *const u8 as *const libc::c_char,
                                sf.num,
                                sf.denom,
                            );
                        } else {
                            crate::stdlib::printf(b"... \x00" as *const u8 as *const libc::c_char);
                        }
                        if crate::turbojpeg_h::tjDecompressToYUV2(
                            handle,
                            jpegBuf,
                            jpegSize,
                            yuvBuf,
                            scaledWidth,
                            pad,
                            scaledHeight,
                            flags,
                        ) == -(1 as libc::c_int)
                        {
                            crate::stdlib::printf(
                                b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                                crate::turbojpeg_h::tjGetErrorStr(),
                            );
                            exitStatus = -(1 as libc::c_int);
                            current_block = 14297773496329963019;
                        } else {
                            if checkBufYUV(yuvBuf, scaledWidth, scaledHeight, subsamp, sf) != 0 {
                                crate::stdlib::printf(
                                    b"Passed.\n\x00" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                crate::stdlib::printf(
                                    b"FAILED!\n\x00" as *const u8 as *const libc::c_char,
                                );
                            }
                            crate::stdlib::printf(
                                b"YUV %s -> %s %s ... \x00" as *const u8 as *const libc::c_char,
                                subNameLong[subsamp as usize],
                                pixFormatStr[pf as usize],
                                if flags & crate::turbojpeg_h::TJFLAG_BOTTOMUP != 0 {
                                    b"Bottom-Up\x00" as *const u8 as *const libc::c_char
                                } else {
                                    b"Top-Down \x00" as *const u8 as *const libc::c_char
                                },
                            );
                            if crate::turbojpeg_h::tjDecodeYUV(
                                handle2,
                                yuvBuf,
                                pad,
                                subsamp,
                                dstBuf,
                                scaledWidth,
                                0 as libc::c_int,
                                scaledHeight,
                                pf,
                                flags,
                            ) == -(1 as libc::c_int)
                            {
                                crate::stdlib::printf(
                                    b"TurboJPEG ERROR:\n%s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::turbojpeg_h::tjGetErrorStr(),
                                );
                                exitStatus = -(1 as libc::c_int);
                                current_block = 14297773496329963019;
                            } else {
                                crate::turbojpeg_h::tjDestroy(handle2);
                                current_block = 15594839951440953787;
                            }
                        }
                    }
                }
            } else {
                crate::stdlib::printf(
                    b"JPEG -> %s %s \x00" as *const u8 as *const libc::c_char,
                    pixFormatStr[pf as usize],
                    if flags & crate::turbojpeg_h::TJFLAG_BOTTOMUP != 0 {
                        b"Bottom-Up\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"Top-Down \x00" as *const u8 as *const libc::c_char
                    },
                );
                if sf.num != 1 as libc::c_int || sf.denom != 1 as libc::c_int {
                    crate::stdlib::printf(
                        b"%d/%d ... \x00" as *const u8 as *const libc::c_char,
                        sf.num,
                        sf.denom,
                    );
                } else {
                    crate::stdlib::printf(b"... \x00" as *const u8 as *const libc::c_char);
                }
                if crate::turbojpeg_h::tjDecompress2(
                    handle,
                    jpegBuf,
                    jpegSize,
                    dstBuf,
                    scaledWidth,
                    0 as libc::c_int,
                    scaledHeight,
                    pf,
                    flags,
                ) == -(1 as libc::c_int)
                {
                    crate::stdlib::printf(
                        b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                        crate::turbojpeg_h::tjGetErrorStr(),
                    );
                    exitStatus = -(1 as libc::c_int);
                    current_block = 14297773496329963019;
                } else {
                    current_block = 15594839951440953787;
                }
            }
            match current_block {
                14297773496329963019 => {}
                _ => {
                    if checkBuf(dstBuf, scaledWidth, scaledHeight, pf, subsamp, sf, flags) != 0 {
                        crate::stdlib::printf(b"Passed.\x00" as *const u8 as *const libc::c_char);
                    } else {
                        crate::stdlib::printf(b"FAILED!\x00" as *const u8 as *const libc::c_char);
                    }
                    crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                }
            }
        }
    }
    if !yuvBuf.is_null() {
        crate::stdlib::free(yuvBuf as *mut libc::c_void);
    }
    if !dstBuf.is_null() {
        crate::stdlib::free(dstBuf as *mut libc::c_void);
    };
}
#[no_mangle]

pub unsafe extern "C" fn decompTest(
    mut handle: crate::turbojpeg_h::tjhandle,
    mut jpegBuf: *mut libc::c_uchar,
    mut jpegSize: libc::c_ulong,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut pf: libc::c_int,
    mut basename: *mut libc::c_char,
    mut subsamp: libc::c_int,
    mut flags: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut sf: *mut crate::turbojpeg_h::tjscalingfactor =
        crate::turbojpeg_h::tjGetScalingFactors(&mut n);
    if sf.is_null() || n == 0 {
        crate::stdlib::printf(
            b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
            crate::turbojpeg_h::tjGetErrorStr(),
        );
        exitStatus = -(1 as libc::c_int)
    } else {
        i = 0 as libc::c_int;
        while i < n {
            if subsamp == crate::turbojpeg_h::TJSAMP_444 as libc::c_int
                || subsamp == crate::turbojpeg_h::TJSAMP_GRAY as libc::c_int
                || subsamp == crate::turbojpeg_h::TJSAMP_411 as libc::c_int
                    && (*sf.offset(i as isize)).num == 1 as libc::c_int
                    && ((*sf.offset(i as isize)).denom == 2 as libc::c_int
                        || (*sf.offset(i as isize)).denom == 1 as libc::c_int)
                || subsamp != crate::turbojpeg_h::TJSAMP_411 as libc::c_int
                    && (*sf.offset(i as isize)).num == 1 as libc::c_int
                    && ((*sf.offset(i as isize)).denom == 4 as libc::c_int
                        || (*sf.offset(i as isize)).denom == 2 as libc::c_int
                        || (*sf.offset(i as isize)).denom == 1 as libc::c_int)
            {
                _decompTest(
                    handle,
                    jpegBuf,
                    jpegSize,
                    w,
                    h,
                    pf,
                    basename,
                    subsamp,
                    flags,
                    *sf.offset(i as isize),
                );
            }
            i += 1
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn doTest(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut formats: *const libc::c_int,
    mut nformats: libc::c_int,
    mut subsamp: libc::c_int,
    mut basename: *mut libc::c_char,
) {
    let mut current_block: u64;
    let mut chandle: crate::turbojpeg_h::tjhandle = crate::stddef_h::NULL as *mut libc::c_void;
    let mut dhandle: crate::turbojpeg_h::tjhandle = crate::stddef_h::NULL as *mut libc::c_void;
    let mut dstBuf: *mut libc::c_uchar = crate::stddef_h::NULL as *mut libc::c_uchar;
    let mut size: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut pfi: libc::c_int = 0;
    let mut pf: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if alloc == 0 {
        size = crate::turbojpeg_h::tjBufSize(w, h, subsamp)
    }
    if size != 0 as libc::c_int as libc::c_ulong {
        dstBuf = crate::turbojpeg_h::tjAlloc(size as libc::c_int);
        if dstBuf.is_null() {
            crate::stdlib::printf(
                b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
                b"Memory allocation failure.\x00" as *const u8 as *const libc::c_char,
            );
            exitStatus = -(1 as libc::c_int);
            current_block = 15679700284600549218;
        } else {
            current_block = 1394248824506584008;
        }
    } else {
        current_block = 1394248824506584008;
    }
    match current_block {
        1394248824506584008 => {
            chandle = crate::turbojpeg_h::tjInitCompress();
            if chandle.is_null() || {
                dhandle = crate::turbojpeg_h::tjInitDecompress();
                dhandle.is_null()
            } {
                crate::stdlib::printf(
                    b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                    crate::turbojpeg_h::tjGetErrorStr(),
                );
                exitStatus = -(1 as libc::c_int)
            } else {
                pfi = 0 as libc::c_int;
                while pfi < nformats {
                    i = 0 as libc::c_int;
                    while i < 2 as libc::c_int {
                        let mut flags: libc::c_int = 0 as libc::c_int;
                        if subsamp == crate::turbojpeg_h::TJSAMP_422 as libc::c_int
                            || subsamp == crate::turbojpeg_h::TJSAMP_420 as libc::c_int
                            || subsamp == crate::turbojpeg_h::TJSAMP_440 as libc::c_int
                            || subsamp == crate::turbojpeg_h::TJSAMP_411 as libc::c_int
                        {
                            flags |= crate::turbojpeg_h::TJFLAG_FASTUPSAMPLE
                        }
                        if i == 1 as libc::c_int {
                            flags |= crate::turbojpeg_h::TJFLAG_BOTTOMUP
                        }
                        pf = *formats.offset(pfi as isize);
                        compTest(
                            chandle,
                            &mut dstBuf,
                            &mut size,
                            w,
                            h,
                            pf,
                            basename,
                            subsamp,
                            100 as libc::c_int,
                            flags,
                        );
                        decompTest(dhandle, dstBuf, size, w, h, pf, basename, subsamp, flags);
                        if pf >= crate::turbojpeg_h::TJPF_RGBX as libc::c_int
                            && pf <= crate::turbojpeg_h::TJPF_XRGB as libc::c_int
                        {
                            crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                            decompTest(
                                dhandle,
                                dstBuf,
                                size,
                                w,
                                h,
                                pf + (crate::turbojpeg_h::TJPF_RGBA as libc::c_int
                                    - crate::turbojpeg_h::TJPF_RGBX as libc::c_int),
                                basename,
                                subsamp,
                                flags,
                            );
                        }
                        crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                        i += 1
                    }
                    pfi += 1
                }
                crate::stdlib::printf(
                    b"--------------------\n\n\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
        _ => {}
    }
    if !chandle.is_null() {
        crate::turbojpeg_h::tjDestroy(chandle);
    }
    if !dhandle.is_null() {
        crate::turbojpeg_h::tjDestroy(dhandle);
    }
    if !dstBuf.is_null() {
        crate::turbojpeg_h::tjFree(dstBuf);
    };
}
#[no_mangle]

pub unsafe extern "C" fn bufSizeTest() {
    let mut current_block: u64;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut subsamp: libc::c_int = 0;
    let mut srcBuf: *mut libc::c_uchar = crate::stddef_h::NULL as *mut libc::c_uchar;
    let mut dstBuf: *mut libc::c_uchar = crate::stddef_h::NULL as *mut libc::c_uchar;
    let mut handle: crate::turbojpeg_h::tjhandle = crate::stddef_h::NULL as *mut libc::c_void;
    let mut dstSize: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    handle = crate::turbojpeg_h::tjInitCompress();
    if handle.is_null() {
        crate::stdlib::printf(
            b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
            crate::turbojpeg_h::tjGetErrorStr(),
        );
        exitStatus = -(1 as libc::c_int)
    } else {
        crate::stdlib::printf(
            b"Buffer size regression test\n\x00" as *const u8 as *const libc::c_char,
        );
        subsamp = 0 as libc::c_int;
        's_43: loop {
            if !(subsamp < crate::turbojpeg_h::TJ_NUMSAMP) {
                current_block = 6040267449472925966;
                break;
            }
            w = 1 as libc::c_int;
            while w < 48 as libc::c_int {
                let mut maxh: libc::c_int = if w == 1 as libc::c_int {
                    2048 as libc::c_int
                } else {
                    48 as libc::c_int
                };
                h = 1 as libc::c_int;
                while h < maxh {
                    if h % 100 as libc::c_int == 0 as libc::c_int {
                        crate::stdlib::printf(
                            b"%.4d x %.4d\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x00"
                                as *const u8 as *const libc::c_char,
                            w,
                            h,
                        );
                    }
                    srcBuf = crate::stdlib::malloc((w * h * 4 as libc::c_int) as libc::c_ulong)
                        as *mut libc::c_uchar;
                    if srcBuf.is_null() {
                        crate::stdlib::printf(
                            b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
                            b"Memory allocation failure\x00" as *const u8 as *const libc::c_char,
                        );
                        exitStatus = -(1 as libc::c_int);
                        current_block = 17868673386502678986;
                        break 's_43;
                    } else {
                        if alloc == 0 || doYUV != 0 {
                            if doYUV != 0 {
                                dstSize = crate::turbojpeg_h::tjBufSizeYUV2(w, pad, h, subsamp)
                            } else {
                                dstSize = crate::turbojpeg_h::tjBufSize(w, h, subsamp)
                            }
                            dstBuf = crate::turbojpeg_h::tjAlloc(dstSize as libc::c_int);
                            if dstBuf.is_null() {
                                crate::stdlib::printf(
                                    b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
                                    b"Memory allocation failure\x00" as *const u8
                                        as *const libc::c_char,
                                );
                                exitStatus = -(1 as libc::c_int);
                                current_block = 17868673386502678986;
                                break 's_43;
                            }
                        }
                        i = 0 as libc::c_int;
                        while i < w * h * 4 as libc::c_int {
                            if crate::stdlib::random()
                                < (crate::stdlib::RAND_MAX / 2 as libc::c_int) as libc::c_long
                            {
                                *srcBuf.offset(i as isize) = 0 as libc::c_int as libc::c_uchar
                            } else {
                                *srcBuf.offset(i as isize) = 255 as libc::c_int as libc::c_uchar
                            }
                            i += 1
                        }
                        if doYUV != 0 {
                            if crate::turbojpeg_h::tjEncodeYUV3(
                                handle,
                                srcBuf,
                                w,
                                0 as libc::c_int,
                                h,
                                crate::turbojpeg_h::TJPF_BGRX as libc::c_int,
                                dstBuf,
                                pad,
                                subsamp,
                                0 as libc::c_int,
                            ) == -(1 as libc::c_int)
                            {
                                crate::stdlib::printf(
                                    b"TurboJPEG ERROR:\n%s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::turbojpeg_h::tjGetErrorStr(),
                                );
                                exitStatus = -(1 as libc::c_int);
                                current_block = 17868673386502678986;
                                break 's_43;
                            }
                        } else if crate::turbojpeg_h::tjCompress2(
                            handle,
                            srcBuf,
                            w,
                            0 as libc::c_int,
                            h,
                            crate::turbojpeg_h::TJPF_BGRX as libc::c_int,
                            &mut dstBuf,
                            &mut dstSize,
                            subsamp,
                            100 as libc::c_int,
                            (if alloc != 0 {
                                0 as libc::c_int
                            } else {
                                1024 as libc::c_int
                            }),
                        ) == -(1 as libc::c_int)
                        {
                            crate::stdlib::printf(
                                b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                                crate::turbojpeg_h::tjGetErrorStr(),
                            );
                            exitStatus = -(1 as libc::c_int);
                            current_block = 17868673386502678986;
                            break 's_43;
                        }
                        crate::stdlib::free(srcBuf as *mut libc::c_void);
                        srcBuf = crate::stddef_h::NULL as *mut libc::c_uchar;
                        if alloc == 0 || doYUV != 0 {
                            crate::turbojpeg_h::tjFree(dstBuf);
                            dstBuf = crate::stddef_h::NULL as *mut libc::c_uchar
                        }
                        srcBuf = crate::stdlib::malloc((h * w * 4 as libc::c_int) as libc::c_ulong)
                            as *mut libc::c_uchar;
                        if srcBuf.is_null() {
                            crate::stdlib::printf(
                                b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
                                b"Memory allocation failure\x00" as *const u8
                                    as *const libc::c_char,
                            );
                            exitStatus = -(1 as libc::c_int);
                            current_block = 17868673386502678986;
                            break 's_43;
                        } else {
                            if alloc == 0 || doYUV != 0 {
                                if doYUV != 0 {
                                    dstSize = crate::turbojpeg_h::tjBufSizeYUV2(h, pad, w, subsamp)
                                } else {
                                    dstSize = crate::turbojpeg_h::tjBufSize(h, w, subsamp)
                                }
                                dstBuf = crate::turbojpeg_h::tjAlloc(dstSize as libc::c_int);
                                if dstBuf.is_null() {
                                    crate::stdlib::printf(
                                        b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
                                        b"Memory allocation failure\x00" as *const u8
                                            as *const libc::c_char,
                                    );
                                    exitStatus = -(1 as libc::c_int);
                                    current_block = 17868673386502678986;
                                    break 's_43;
                                }
                            }
                            i = 0 as libc::c_int;
                            while i < h * w * 4 as libc::c_int {
                                if crate::stdlib::random()
                                    < (crate::stdlib::RAND_MAX / 2 as libc::c_int) as libc::c_long
                                {
                                    *srcBuf.offset(i as isize) = 0 as libc::c_int as libc::c_uchar
                                } else {
                                    *srcBuf.offset(i as isize) = 255 as libc::c_int as libc::c_uchar
                                }
                                i += 1
                            }
                            if doYUV != 0 {
                                if crate::turbojpeg_h::tjEncodeYUV3(
                                    handle,
                                    srcBuf,
                                    h,
                                    0 as libc::c_int,
                                    w,
                                    crate::turbojpeg_h::TJPF_BGRX as libc::c_int,
                                    dstBuf,
                                    pad,
                                    subsamp,
                                    0 as libc::c_int,
                                ) == -(1 as libc::c_int)
                                {
                                    crate::stdlib::printf(
                                        b"TurboJPEG ERROR:\n%s\n\x00" as *const u8
                                            as *const libc::c_char,
                                        crate::turbojpeg_h::tjGetErrorStr(),
                                    );
                                    exitStatus = -(1 as libc::c_int);
                                    current_block = 17868673386502678986;
                                    break 's_43;
                                }
                            } else if crate::turbojpeg_h::tjCompress2(
                                handle,
                                srcBuf,
                                h,
                                0 as libc::c_int,
                                w,
                                crate::turbojpeg_h::TJPF_BGRX as libc::c_int,
                                &mut dstBuf,
                                &mut dstSize,
                                subsamp,
                                100 as libc::c_int,
                                (if alloc != 0 {
                                    0 as libc::c_int
                                } else {
                                    1024 as libc::c_int
                                }),
                            ) == -(1 as libc::c_int)
                            {
                                crate::stdlib::printf(
                                    b"TurboJPEG ERROR:\n%s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::turbojpeg_h::tjGetErrorStr(),
                                );
                                exitStatus = -(1 as libc::c_int);
                                current_block = 17868673386502678986;
                                break 's_43;
                            }
                            crate::stdlib::free(srcBuf as *mut libc::c_void);
                            srcBuf = crate::stddef_h::NULL as *mut libc::c_uchar;
                            if alloc == 0 || doYUV != 0 {
                                crate::turbojpeg_h::tjFree(dstBuf);
                                dstBuf = crate::stddef_h::NULL as *mut libc::c_uchar
                            }
                            h += 1
                        }
                    }
                }
                w += 1
            }
            subsamp += 1
        }
        match current_block {
            17868673386502678986 => {}
            _ => {
                crate::stdlib::printf(b"Done.      \n\x00" as *const u8 as *const libc::c_char);
            }
        }
    }
    if !srcBuf.is_null() {
        crate::stdlib::free(srcBuf as *mut libc::c_void);
    }
    if !dstBuf.is_null() {
        crate::turbojpeg_h::tjFree(dstBuf);
    }
    if !handle.is_null() {
        crate::turbojpeg_h::tjDestroy(handle);
    };
}
#[no_mangle]

pub unsafe extern "C" fn initBitmap(
    mut buf: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut pitch: libc::c_int,
    mut height: libc::c_int,
    mut pf: libc::c_int,
    mut flags: libc::c_int,
) {
    let mut roffset: libc::c_int = tjRedOffset[pf as usize];
    let mut goffset: libc::c_int = tjGreenOffset[pf as usize];
    let mut boffset: libc::c_int = tjBlueOffset[pf as usize];
    let mut ps: libc::c_int = tjPixelSize[pf as usize];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < height {
        let mut row: libc::c_int = if flags & crate::turbojpeg_h::TJFLAG_BOTTOMUP != 0 {
            (height - j) - 1 as libc::c_int
        } else {
            j
        };
        i = 0 as libc::c_int;
        while i < width {
            let mut r: libc::c_uchar =
                (i * 256 as libc::c_int / width % 256 as libc::c_int) as libc::c_uchar;
            let mut g: libc::c_uchar =
                (j * 256 as libc::c_int / height % 256 as libc::c_int) as libc::c_uchar;
            let mut b: libc::c_uchar = ((j * 256 as libc::c_int / height
                + i * 256 as libc::c_int / width)
                % 256 as libc::c_int) as libc::c_uchar;
            crate::stdlib::memset(
                &mut *buf.offset((row * pitch + i * ps) as isize) as *mut libc::c_uchar
                    as *mut libc::c_void,
                0 as libc::c_int,
                ps as libc::c_ulong,
            );
            if pf == crate::turbojpeg_h::TJPF_GRAY as libc::c_int {
                *buf.offset((row * pitch + i * ps) as isize) = b
            } else if pf == crate::turbojpeg_h::TJPF_CMYK as libc::c_int {
                rgb_to_cmyk(
                    r,
                    g,
                    b,
                    &mut *buf.offset((row * pitch + i * ps + 0 as libc::c_int) as isize),
                    &mut *buf.offset((row * pitch + i * ps + 1 as libc::c_int) as isize),
                    &mut *buf.offset((row * pitch + i * ps + 2 as libc::c_int) as isize),
                    &mut *buf.offset((row * pitch + i * ps + 3 as libc::c_int) as isize),
                );
            } else {
                *buf.offset((row * pitch + i * ps + roffset) as isize) = r;
                *buf.offset((row * pitch + i * ps + goffset) as isize) = g;
                *buf.offset((row * pitch + i * ps + boffset) as isize) = b
            }
            i += 1
        }
        j += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn cmpBitmap(
    mut buf: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut pitch: libc::c_int,
    mut height: libc::c_int,
    mut pf: libc::c_int,
    mut flags: libc::c_int,
    mut gray2rgb: libc::c_int,
) -> libc::c_int {
    let mut roffset: libc::c_int = tjRedOffset[pf as usize];
    let mut goffset: libc::c_int = tjGreenOffset[pf as usize];
    let mut boffset: libc::c_int = tjBlueOffset[pf as usize];
    let mut aoffset: libc::c_int = tjAlphaOffset[pf as usize];
    let mut ps: libc::c_int = tjPixelSize[pf as usize];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < height {
        let mut row: libc::c_int = if flags & crate::turbojpeg_h::TJFLAG_BOTTOMUP != 0 {
            (height - j) - 1 as libc::c_int
        } else {
            j
        };
        i = 0 as libc::c_int;
        while i < width {
            let mut r: libc::c_uchar =
                (i * 256 as libc::c_int / width % 256 as libc::c_int) as libc::c_uchar;
            let mut g: libc::c_uchar =
                (j * 256 as libc::c_int / height % 256 as libc::c_int) as libc::c_uchar;
            let mut b: libc::c_uchar = ((j * 256 as libc::c_int / height
                + i * 256 as libc::c_int / width)
                % 256 as libc::c_int) as libc::c_uchar;
            if pf == crate::turbojpeg_h::TJPF_GRAY as libc::c_int {
                if *buf.offset((row * pitch + i * ps) as isize) as libc::c_int != b as libc::c_int {
                    return 0 as libc::c_int;
                }
            } else if pf == crate::turbojpeg_h::TJPF_CMYK as libc::c_int {
                let mut rf: libc::c_uchar = 0;
                let mut gf: libc::c_uchar = 0;
                let mut bf: libc::c_uchar = 0;
                cmyk_to_rgb(
                    *buf.offset((row * pitch + i * ps + 0 as libc::c_int) as isize),
                    *buf.offset((row * pitch + i * ps + 1 as libc::c_int) as isize),
                    *buf.offset((row * pitch + i * ps + 2 as libc::c_int) as isize),
                    *buf.offset((row * pitch + i * ps + 3 as libc::c_int) as isize),
                    &mut rf,
                    &mut gf,
                    &mut bf,
                );
                if gray2rgb != 0 {
                    if rf as libc::c_int != b as libc::c_int
                        || gf as libc::c_int != b as libc::c_int
                        || bf as libc::c_int != b as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                } else if rf as libc::c_int != r as libc::c_int
                    || gf as libc::c_int != g as libc::c_int
                    || bf as libc::c_int != b as libc::c_int
                {
                    return 0 as libc::c_int;
                }
            } else {
                if gray2rgb != 0 {
                    if *buf.offset((row * pitch + i * ps + roffset) as isize) as libc::c_int
                        != b as libc::c_int
                        || *buf.offset((row * pitch + i * ps + goffset) as isize) as libc::c_int
                            != b as libc::c_int
                        || *buf.offset((row * pitch + i * ps + boffset) as isize) as libc::c_int
                            != b as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                } else if *buf.offset((row * pitch + i * ps + roffset) as isize) as libc::c_int
                    != r as libc::c_int
                    || *buf.offset((row * pitch + i * ps + goffset) as isize) as libc::c_int
                        != g as libc::c_int
                    || *buf.offset((row * pitch + i * ps + boffset) as isize) as libc::c_int
                        != b as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                if aoffset >= 0 as libc::c_int
                    && *buf.offset((row * pitch + i * ps + aoffset) as isize) as libc::c_int
                        != 0xff as libc::c_int
                {
                    return 0 as libc::c_int;
                }
            }
            i += 1
        }
        j += 1
    }
    return 1 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn doBmpTest(
    mut ext: *const libc::c_char,
    mut width: libc::c_int,
    mut align: libc::c_int,
    mut height: libc::c_int,
    mut pf: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut filename: [libc::c_char; 80] = [0; 80];
    let mut md5sum: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut md5buf: [libc::c_char; 65] = [0; 65];
    let mut ps: libc::c_int = tjPixelSize[pf as usize];
    let mut pitch: libc::c_int =
        width * ps + align - 1 as libc::c_int & !(align - 1 as libc::c_int);
    let mut loadWidth: libc::c_int = 0 as libc::c_int;
    let mut loadHeight: libc::c_int = 0 as libc::c_int;
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut pixelFormat: libc::c_int = pf;
    let mut buf: *mut libc::c_uchar = crate::stddef_h::NULL as *mut libc::c_uchar;
    let mut md5ref: *mut libc::c_char = 0 as *mut libc::c_char;
    if pf == crate::turbojpeg_h::TJPF_GRAY as libc::c_int {
        md5ref =
            if crate::stdlib::strcasecmp(ext, b"ppm\x00" as *const u8 as *const libc::c_char) == 0 {
                b"112c682e82ce5de1cca089e20d60000b\x00" as *const u8 as *const libc::c_char
            } else {
                b"51976530acf75f02beddf5d21149101d\x00" as *const u8 as *const libc::c_char
            } as *mut libc::c_char
    } else {
        md5ref =
            if crate::stdlib::strcasecmp(ext, b"ppm\x00" as *const u8 as *const libc::c_char) == 0 {
                b"c0c9f772b464d1896326883a5c79c545\x00" as *const u8 as *const libc::c_char
            } else {
                b"6d659071b9bfcdee2def22cb58ddadca\x00" as *const u8 as *const libc::c_char
            } as *mut libc::c_char
    }
    buf = crate::turbojpeg_h::tjAlloc(pitch * height);
    if buf.is_null() {
        crate::stdlib::printf(
            b"ERROR: %s\n\x00" as *const u8 as *const libc::c_char,
            b"Could not allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        exitStatus = -(1 as libc::c_int)
    } else {
        initBitmap(buf, width, pitch, height, pf, flags);
        crate::stdlib::snprintf(
            filename.as_mut_ptr(),
            80 as libc::c_int as libc::c_ulong,
            b"test_bmp_%s_%d_%s.%s\x00" as *const u8 as *const libc::c_char,
            pixFormatStr[pf as usize],
            align,
            if flags & crate::turbojpeg_h::TJFLAG_BOTTOMUP != 0 {
                b"bu\x00" as *const u8 as *const libc::c_char
            } else {
                b"td\x00" as *const u8 as *const libc::c_char
            },
            ext,
        );
        if crate::turbojpeg_h::tjSaveImage(
            filename.as_mut_ptr(),
            buf,
            width,
            pitch,
            height,
            pf,
            flags,
        ) == -(1 as libc::c_int)
        {
            crate::stdlib::printf(
                b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                crate::turbojpeg_h::tjGetErrorStr(),
            );
            exitStatus = -(1 as libc::c_int)
        } else {
            md5sum = crate::md5_h::MD5File(filename.as_mut_ptr(), md5buf.as_mut_ptr());
            if crate::stdlib::strcasecmp(md5sum, md5ref) != 0 {
                crate::stdlib::printf(
                    b"\n%s has an MD5 sum of %s.\n   Should be %s.\n\x00" as *const u8
                        as *const libc::c_char,
                    filename.as_mut_ptr(),
                    md5sum,
                    md5ref,
                );
                exitStatus = -(1 as libc::c_int)
            } else {
                crate::turbojpeg_h::tjFree(buf);
                buf = crate::stddef_h::NULL as *mut libc::c_uchar;
                buf = crate::turbojpeg_h::tjLoadImage(
                    filename.as_mut_ptr(),
                    &mut loadWidth,
                    align,
                    &mut loadHeight,
                    &mut pf,
                    flags,
                );
                if buf.is_null() {
                    crate::stdlib::printf(
                        b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                        crate::turbojpeg_h::tjGetErrorStr(),
                    );
                    exitStatus = -(1 as libc::c_int)
                } else if width != loadWidth || height != loadHeight {
                    crate::stdlib::printf(
                        b"\n   Image dimensions of %s are bogus\n\x00" as *const u8
                            as *const libc::c_char,
                        filename.as_mut_ptr(),
                    );
                    retval = -(1 as libc::c_int)
                } else if cmpBitmap(buf, width, pitch, height, pf, flags, 0 as libc::c_int) == 0 {
                    crate::stdlib::printf(
                        b"\n   Pixel data in %s is bogus\n\x00" as *const u8 as *const libc::c_char,
                        filename.as_mut_ptr(),
                    );
                    retval = -(1 as libc::c_int)
                } else {
                    if pf == crate::turbojpeg_h::TJPF_GRAY as libc::c_int {
                        crate::turbojpeg_h::tjFree(buf);
                        buf = crate::stddef_h::NULL as *mut libc::c_uchar;
                        pf = crate::turbojpeg_h::TJPF_XBGR as libc::c_int;
                        buf = crate::turbojpeg_h::tjLoadImage(
                            filename.as_mut_ptr(),
                            &mut loadWidth,
                            align,
                            &mut loadHeight,
                            &mut pf,
                            flags,
                        );
                        if buf.is_null() {
                            crate::stdlib::printf(
                                b"TurboJPEG ERROR:\n%s\n\x00" as *const u8 as *const libc::c_char,
                                crate::turbojpeg_h::tjGetErrorStr(),
                            );
                            exitStatus = -(1 as libc::c_int);
                            current_block = 14417489546151714667;
                        } else {
                            pitch = width * tjPixelSize[pf as usize] + align - 1 as libc::c_int
                                & !(align - 1 as libc::c_int);
                            if cmpBitmap(buf, width, pitch, height, pf, flags, 1 as libc::c_int)
                                == 0
                            {
                                crate::stdlib::printf(
                                    b"\n   Converting %s to RGB failed\n\x00" as *const u8
                                        as *const libc::c_char,
                                    filename.as_mut_ptr(),
                                );
                                retval = -(1 as libc::c_int);
                                current_block = 14417489546151714667;
                            } else {
                                crate::turbojpeg_h::tjFree(buf);
                                buf = crate::stddef_h::NULL as *mut libc::c_uchar;
                                pf = crate::turbojpeg_h::TJPF_CMYK as libc::c_int;
                                buf = crate::turbojpeg_h::tjLoadImage(
                                    filename.as_mut_ptr(),
                                    &mut loadWidth,
                                    align,
                                    &mut loadHeight,
                                    &mut pf,
                                    flags,
                                );
                                if buf.is_null() {
                                    crate::stdlib::printf(
                                        b"TurboJPEG ERROR:\n%s\n\x00" as *const u8
                                            as *const libc::c_char,
                                        crate::turbojpeg_h::tjGetErrorStr(),
                                    );
                                    exitStatus = -(1 as libc::c_int);
                                    current_block = 14417489546151714667;
                                } else {
                                    pitch = width * tjPixelSize[pf as usize] + align
                                        - 1 as libc::c_int
                                        & !(align - 1 as libc::c_int);
                                    if cmpBitmap(
                                        buf,
                                        width,
                                        pitch,
                                        height,
                                        pf,
                                        flags,
                                        1 as libc::c_int,
                                    ) == 0
                                    {
                                        crate::stdlib::printf(
                                            b"\n   Converting %s to CMYK failed\n\x00" as *const u8
                                                as *const libc::c_char,
                                            filename.as_mut_ptr(),
                                        );
                                        retval = -(1 as libc::c_int);
                                        current_block = 14417489546151714667;
                                    } else {
                                        current_block = 12930649117290160518;
                                    }
                                }
                            }
                        }
                    } else {
                        current_block = 12930649117290160518;
                    }
                    match current_block {
                        14417489546151714667 => {}
                        _ => {
                            /* Verify that tjLoadImage() returns the proper "preferred" pixel format for
                            the file type. */
                            crate::turbojpeg_h::tjFree(buf);
                            buf = crate::stddef_h::NULL as *mut libc::c_uchar;
                            pf = pixelFormat;
                            pixelFormat = crate::turbojpeg_h::TJPF_UNKNOWN as libc::c_int;
                            buf = crate::turbojpeg_h::tjLoadImage(
                                filename.as_mut_ptr(),
                                &mut loadWidth,
                                align,
                                &mut loadHeight,
                                &mut pixelFormat,
                                flags,
                            );
                            if buf.is_null() {
                                crate::stdlib::printf(
                                    b"TurboJPEG ERROR:\n%s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::turbojpeg_h::tjGetErrorStr(),
                                );
                                exitStatus = -(1 as libc::c_int)
                            } else {
                                if pf == crate::turbojpeg_h::TJPF_GRAY as libc::c_int
                                    && pixelFormat != crate::turbojpeg_h::TJPF_GRAY as libc::c_int
                                    || pf != crate::turbojpeg_h::TJPF_GRAY as libc::c_int
                                        && crate::stdlib::strcasecmp(
                                            ext,
                                            b"bmp\x00" as *const u8 as *const libc::c_char,
                                        ) == 0
                                        && pixelFormat
                                            != crate::turbojpeg_h::TJPF_BGR as libc::c_int
                                    || pf != crate::turbojpeg_h::TJPF_GRAY as libc::c_int
                                        && crate::stdlib::strcasecmp(
                                            ext,
                                            b"ppm\x00" as *const u8 as *const libc::c_char,
                                        ) == 0
                                        && pixelFormat
                                            != crate::turbojpeg_h::TJPF_RGB as libc::c_int
                                {
                                    crate::stdlib::printf(b"\n   tjLoadImage() returned unexpected pixel format: %s\n\x00"
                                               as *const u8 as
                                               *const libc::c_char,
                                           pixFormatStr[pixelFormat as
                                                            usize]);
                                    retval = -(1 as libc::c_int)
                                }
                                crate::stdlib::unlink(filename.as_mut_ptr());
                            }
                        }
                    }
                }
            }
        }
    }
    if !buf.is_null() {
        crate::turbojpeg_h::tjFree(buf);
    }
    if exitStatus < 0 as libc::c_int {
        return exitStatus;
    }
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn bmpTest() -> libc::c_int {
    let mut align: libc::c_int = 0;
    let mut width: libc::c_int = 35 as libc::c_int;
    let mut height: libc::c_int = 39 as libc::c_int;
    let mut format: libc::c_int = 0;
    align = 1 as libc::c_int;
    while align <= 8 as libc::c_int {
        format = 0 as libc::c_int;
        while format < crate::turbojpeg_h::TJ_NUMPF {
            crate::stdlib::printf(
                b"%s Top-Down BMP (row alignment = %d bytes)  ...  \x00" as *const u8
                    as *const libc::c_char,
                pixFormatStr[format as usize],
                align,
            );
            if doBmpTest(
                b"bmp\x00" as *const u8 as *const libc::c_char,
                width,
                align,
                height,
                format,
                0 as libc::c_int,
            ) == -(1 as libc::c_int)
            {
                return -(1 as libc::c_int);
            }
            crate::stdlib::printf(b"OK.\n\x00" as *const u8 as *const libc::c_char);
            crate::stdlib::printf(
                b"%s Top-Down PPM (row alignment = %d bytes)  ...  \x00" as *const u8
                    as *const libc::c_char,
                pixFormatStr[format as usize],
                align,
            );
            if doBmpTest(
                b"ppm\x00" as *const u8 as *const libc::c_char,
                width,
                align,
                height,
                format,
                crate::turbojpeg_h::TJFLAG_BOTTOMUP,
            ) == -(1 as libc::c_int)
            {
                return -(1 as libc::c_int);
            }
            crate::stdlib::printf(b"OK.\n\x00" as *const u8 as *const libc::c_char);
            crate::stdlib::printf(
                b"%s Bottom-Up BMP (row alignment = %d bytes)  ...  \x00" as *const u8
                    as *const libc::c_char,
                pixFormatStr[format as usize],
                align,
            );
            if doBmpTest(
                b"bmp\x00" as *const u8 as *const libc::c_char,
                width,
                align,
                height,
                format,
                0 as libc::c_int,
            ) == -(1 as libc::c_int)
            {
                return -(1 as libc::c_int);
            }
            crate::stdlib::printf(b"OK.\n\x00" as *const u8 as *const libc::c_char);
            crate::stdlib::printf(
                b"%s Bottom-Up PPM (row alignment = %d bytes)  ...  \x00" as *const u8
                    as *const libc::c_char,
                pixFormatStr[format as usize],
                align,
            );
            if doBmpTest(
                b"ppm\x00" as *const u8 as *const libc::c_char,
                width,
                align,
                height,
                format,
                crate::turbojpeg_h::TJFLAG_BOTTOMUP,
            ) == -(1 as libc::c_int)
            {
                return -(1 as libc::c_int);
            }
            crate::stdlib::printf(b"OK.\n\x00" as *const u8 as *const libc::c_char);
            format += 1
        }
        align *= 2 as libc::c_int
    }
    return 0 as libc::c_int;
}

unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut num4bf: libc::c_int = 5 as libc::c_int;
    if argc > 1 as libc::c_int {
        i = 1 as libc::c_int;
        while i < argc {
            if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-yuv\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                doYUV = 1 as libc::c_int
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-noyuvpad\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                pad = 1 as libc::c_int
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-alloc\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                alloc = 1 as libc::c_int
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-bmp\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                return bmpTest();
            } else {
                usage(*argv.offset(0 as libc::c_int as isize));
            }
            i += 1
        }
    }
    if alloc != 0 {
        crate::stdlib::printf(
            b"Testing automatic buffer allocation\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    if doYUV != 0 {
        num4bf = 4 as libc::c_int
    }
    doTest(
        35 as libc::c_int,
        39 as libc::c_int,
        _3byteFormats.as_ptr(),
        2 as libc::c_int,
        crate::turbojpeg_h::TJSAMP_444 as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        39 as libc::c_int,
        41 as libc::c_int,
        _4byteFormats.as_ptr(),
        num4bf,
        crate::turbojpeg_h::TJSAMP_444 as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        41 as libc::c_int,
        35 as libc::c_int,
        _3byteFormats.as_ptr(),
        2 as libc::c_int,
        crate::turbojpeg_h::TJSAMP_422 as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        35 as libc::c_int,
        39 as libc::c_int,
        _4byteFormats.as_ptr(),
        num4bf,
        crate::turbojpeg_h::TJSAMP_422 as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        39 as libc::c_int,
        41 as libc::c_int,
        _3byteFormats.as_ptr(),
        2 as libc::c_int,
        crate::turbojpeg_h::TJSAMP_420 as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        41 as libc::c_int,
        35 as libc::c_int,
        _4byteFormats.as_ptr(),
        num4bf,
        crate::turbojpeg_h::TJSAMP_420 as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        35 as libc::c_int,
        39 as libc::c_int,
        _3byteFormats.as_ptr(),
        2 as libc::c_int,
        crate::turbojpeg_h::TJSAMP_440 as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        39 as libc::c_int,
        41 as libc::c_int,
        _4byteFormats.as_ptr(),
        num4bf,
        crate::turbojpeg_h::TJSAMP_440 as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        41 as libc::c_int,
        35 as libc::c_int,
        _3byteFormats.as_ptr(),
        2 as libc::c_int,
        crate::turbojpeg_h::TJSAMP_411 as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        35 as libc::c_int,
        39 as libc::c_int,
        _4byteFormats.as_ptr(),
        num4bf,
        crate::turbojpeg_h::TJSAMP_411 as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        39 as libc::c_int,
        41 as libc::c_int,
        _onlyGray.as_ptr(),
        1 as libc::c_int,
        crate::turbojpeg_h::TJSAMP_GRAY as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        41 as libc::c_int,
        35 as libc::c_int,
        _3byteFormats.as_ptr(),
        2 as libc::c_int,
        crate::turbojpeg_h::TJSAMP_GRAY as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    doTest(
        35 as libc::c_int,
        39 as libc::c_int,
        _4byteFormats.as_ptr(),
        4 as libc::c_int,
        crate::turbojpeg_h::TJSAMP_GRAY as libc::c_int,
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    bufSizeTest();
    if doYUV != 0 {
        crate::stdlib::printf(
            b"\n--------------------\n\n\x00" as *const u8 as *const libc::c_char,
        );
        doTest(
            48 as libc::c_int,
            48 as libc::c_int,
            _onlyRGB.as_ptr(),
            1 as libc::c_int,
            crate::turbojpeg_h::TJSAMP_444 as libc::c_int,
            b"test_yuv0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        doTest(
            48 as libc::c_int,
            48 as libc::c_int,
            _onlyRGB.as_ptr(),
            1 as libc::c_int,
            crate::turbojpeg_h::TJSAMP_422 as libc::c_int,
            b"test_yuv0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        doTest(
            48 as libc::c_int,
            48 as libc::c_int,
            _onlyRGB.as_ptr(),
            1 as libc::c_int,
            crate::turbojpeg_h::TJSAMP_420 as libc::c_int,
            b"test_yuv0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        doTest(
            48 as libc::c_int,
            48 as libc::c_int,
            _onlyRGB.as_ptr(),
            1 as libc::c_int,
            crate::turbojpeg_h::TJSAMP_440 as libc::c_int,
            b"test_yuv0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        doTest(
            48 as libc::c_int,
            48 as libc::c_int,
            _onlyRGB.as_ptr(),
            1 as libc::c_int,
            crate::turbojpeg_h::TJSAMP_411 as libc::c_int,
            b"test_yuv0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        doTest(
            48 as libc::c_int,
            48 as libc::c_int,
            _onlyRGB.as_ptr(),
            1 as libc::c_int,
            crate::turbojpeg_h::TJSAMP_GRAY as libc::c_int,
            b"test_yuv0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        doTest(
            48 as libc::c_int,
            48 as libc::c_int,
            _onlyGray.as_ptr(),
            1 as libc::c_int,
            crate::turbojpeg_h::TJSAMP_GRAY as libc::c_int,
            b"test_yuv0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return exitStatus;
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
