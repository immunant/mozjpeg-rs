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
pub mod stddef_h {
    pub type size_t = libc::c_ulong;

    pub const NULL_0: libc::c_int = 0 as libc::c_int;

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
        pub fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;

        #[no_mangle]
        pub fn fread(
            _: *mut libc::c_void,
            _: libc::c_ulong,
            _: libc::c_ulong,
            _: *mut crate::stdlib::FILE,
        ) -> libc::c_ulong;

        #[no_mangle]
        pub fn fwrite(
            _: *const libc::c_void,
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
        pub fn strtol(
            _: *const libc::c_char,
            _: *mut *mut libc::c_char,
            _: libc::c_int,
        ) -> libc::c_long;

        #[no_mangle]
        pub fn exit(_: libc::c_int) -> !;
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

        #[no_mangle]
        pub fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;

        #[no_mangle]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn strncasecmp(
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: libc::c_ulong,
        ) -> libc::c_int;

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
    pub type __off_t = libc::c_long;

    pub type __off64_t = libc::c_long;
}
use ::mozjpeg::*;

#[c2rust::header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/turbojpeg.h:39"]
pub mod turbojpeg_h {
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
            flags: libc::c_int,
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

    /* *
     * The uncompressed source/destination image is stored in bottom-up (Windows,
     * OpenGL) order, not top-down (X11) order.
     */
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
     * @}
     */
}

#[c2rust::header_src = "/usr/include/stdlib.h:36"]
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

pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
pub use crate::stddef_h::NULL_0;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

use crate::stdlib::__errno_location;
pub use crate::stdlib::exit;
pub use crate::stdlib::fclose;
pub use crate::stdlib::fopen;
pub use crate::stdlib::fread;
pub use crate::stdlib::fseek;
pub use crate::stdlib::ftell;
pub use crate::stdlib::fwrite;
use crate::stdlib::memset;
pub use crate::stdlib::printf;
pub use crate::stdlib::sscanf;
use crate::stdlib::strcasecmp;
use crate::stdlib::strerror;
use crate::stdlib::strlen;
use crate::stdlib::strncasecmp;
use crate::stdlib::strrchr;
pub use crate::stdlib::strtol;
pub use crate::stdlib::SEEK_END;
pub use crate::stdlib::SEEK_SET;
pub use crate::stdlib_h::atoi;
pub use crate::turbojpeg_h::tjAlloc;
pub use crate::turbojpeg_h::tjCompress2;
pub use crate::turbojpeg_h::tjDecompress2;
pub use crate::turbojpeg_h::tjDecompressHeader3;
pub use crate::turbojpeg_h::tjDestroy;
pub use crate::turbojpeg_h::tjFree;
pub use crate::turbojpeg_h::tjGetErrorStr2;
pub use crate::turbojpeg_h::tjGetScalingFactors;
pub use crate::turbojpeg_h::tjInitCompress;
pub use crate::turbojpeg_h::tjInitDecompress;
pub use crate::turbojpeg_h::tjInitTransform;
pub use crate::turbojpeg_h::tjLoadImage;
pub use crate::turbojpeg_h::tjPixelSize;
pub use crate::turbojpeg_h::tjSaveImage;
pub use crate::turbojpeg_h::tjTransform;
pub use crate::turbojpeg_h::tjhandle;
pub use crate::turbojpeg_h::tjregion;
pub use crate::turbojpeg_h::tjscalingfactor;
pub use crate::turbojpeg_h::tjtransform;
pub use crate::turbojpeg_h::TJFLAG_ACCURATEDCT;
pub use crate::turbojpeg_h::TJFLAG_FASTDCT;
pub use crate::turbojpeg_h::TJFLAG_FASTUPSAMPLE;
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
pub use crate::turbojpeg_h::TJXOPT_CROP;
pub use crate::turbojpeg_h::TJXOPT_GRAY;
pub use crate::turbojpeg_h::TJXOPT_TRIM;
pub use crate::turbojpeg_h::TJXOP_HFLIP;
pub use crate::turbojpeg_h::TJXOP_NONE;
pub use crate::turbojpeg_h::TJXOP_ROT180;
pub use crate::turbojpeg_h::TJXOP_ROT270;
pub use crate::turbojpeg_h::TJXOP_ROT90;
pub use crate::turbojpeg_h::TJXOP_TRANSPOSE;
pub use crate::turbojpeg_h::TJXOP_TRANSVERSE;
pub use crate::turbojpeg_h::TJXOP_VFLIP;
/*
 * Copyright (C)2011-2012, 2014-2015, 2017 D. R. Commander.
 *                                         All Rights Reserved.
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
 * This program demonstrates how to compress, decompress, and transform JPEG
 * images using the TurboJPEG C API
 */

pub const DEFAULT_SUBSAMP: libc::c_int = crate::turbojpeg_h::TJSAMP_444 as libc::c_int;

pub const DEFAULT_QUALITY: libc::c_int = 95 as libc::c_int;
#[no_mangle]

pub static mut subsampName: [*const libc::c_char; 6] = [
    b"4:4:4\x00" as *const u8 as *const libc::c_char,
    b"4:2:2\x00" as *const u8 as *const libc::c_char,
    b"4:2:0\x00" as *const u8 as *const libc::c_char,
    b"Grayscale\x00" as *const u8 as *const libc::c_char,
    b"4:4:0\x00" as *const u8 as *const libc::c_char,
    b"4:1:1\x00" as *const u8 as *const libc::c_char,
];
#[no_mangle]

pub static mut colorspaceName: [*const libc::c_char; 5] = [
    b"RGB\x00" as *const u8 as *const libc::c_char,
    b"YCbCr\x00" as *const u8 as *const libc::c_char,
    b"GRAY\x00" as *const u8 as *const libc::c_char,
    b"CMYK\x00" as *const u8 as *const libc::c_char,
    b"YCCK\x00" as *const u8 as *const libc::c_char,
];
#[no_mangle]

pub static mut scalingFactors: *mut crate::turbojpeg_h::tjscalingfactor =
    crate::stddef_h::NULL_0 as *mut crate::turbojpeg_h::tjscalingfactor;
#[no_mangle]

pub static mut numScalingFactors: libc::c_int = 0 as libc::c_int;
/* DCT filter example.  This produces a negative of the image. */
#[no_mangle]

pub unsafe extern "C" fn customFilter(
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
#[no_mangle]

pub unsafe extern "C" fn usage(mut programName: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    crate::stdlib::printf(
        b"\nUSAGE: %s <Input image> <Output image> [options]\n\n\x00" as *const u8
            as *const libc::c_char,
        programName,
    );
    crate::stdlib::printf(
        b"Input and output images can be in Windows BMP or PBMPLUS (PPM/PGM) format.  If\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"either filename ends in a .jpg extension, then the TurboJPEG API will be used\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"to compress or decompress the image.\n\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"Compression Options (used if the output image is a JPEG image)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"--------------------------------------------------------------\n\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-subsamp <444|422|420|gray> = Apply this level of chrominance subsampling when\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     compressing the output image.  The default is to use the same level of\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     subsampling as in the input image, if the input image is also a JPEG\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     image, or to use grayscale if the input image is a grayscale non-JPEG\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     image, or to use %s subsampling otherwise.\n\n\x00" as *const u8
            as *const libc::c_char,
        subsampName[DEFAULT_SUBSAMP as usize],
    );
    crate::stdlib::printf(
        b"-q <1-100> = Compress the output image with this JPEG quality level\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     (default = %d).\n\n\x00" as *const u8 as *const libc::c_char,
        DEFAULT_QUALITY,
    );
    crate::stdlib::printf(
        b"Decompression Options (used if the input image is a JPEG image)\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"---------------------------------------------------------------\n\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-scale M/N = Scale the input image by a factor of M/N when decompressing it.\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"(M/N = \x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < numScalingFactors {
        crate::stdlib::printf(
            b"%d/%d\x00" as *const u8 as *const libc::c_char,
            (*scalingFactors.offset(i as isize)).num,
            (*scalingFactors.offset(i as isize)).denom,
        );
        if numScalingFactors == 2 as libc::c_int && i != numScalingFactors - 1 as libc::c_int {
            crate::stdlib::printf(b" or \x00" as *const u8 as *const libc::c_char);
        } else if numScalingFactors > 2 as libc::c_int {
            if i != numScalingFactors - 1 as libc::c_int {
                crate::stdlib::printf(b", \x00" as *const u8 as *const libc::c_char);
            }
            if i == numScalingFactors - 2 as libc::c_int {
                crate::stdlib::printf(b"or \x00" as *const u8 as *const libc::c_char);
            }
        }
        i += 1
    }
    crate::stdlib::printf(b")\n\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-hflip, -vflip, -transpose, -transverse, -rot90, -rot180, -rot270 =\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     Perform one of these lossless transform operations on the input image\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     prior to decompressing it (these options are mutually exclusive.)\n\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-grayscale = Perform lossless grayscale conversion on the input image prior\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     to decompressing it (can be combined with the other transform operations\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"     above.)\n\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-crop WxH+X+Y = Perform lossless cropping on the input image prior to\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     decompressing it.  X and Y specify the upper left corner of the cropping\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     region, and W and H specify the width and height of the cropping region.\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     X and Y must be evenly divible by the MCU block size (8x8 if the input\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     image was compressed using no subsampling or grayscale, 16x8 if it was\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     compressed using 4:2:2 subsampling, or 16x16 if it was compressed using\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"     4:2:0 subsampling.)\n\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(b"General Options\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(b"---------------\n\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-fastupsample = Use the fastest chrominance upsampling algorithm available in\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"     the underlying codec.\n\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(
        b"-fastdct = Use the fastest DCT/IDCT algorithms available in the underlying\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"     codec.\n\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::printf(
        b"-accuratedct = Use the most accurate DCT/IDCT algorithms available in the\n\x00"
            as *const u8 as *const libc::c_char,
    );
    crate::stdlib::printf(b"     underlying codec.\n\n\x00" as *const u8 as *const libc::c_char);
    crate::stdlib::exit(1 as libc::c_int);
}

unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut current_block: u64;
    let mut scalingFactor: crate::turbojpeg_h::tjscalingfactor = {
        let mut init = crate::turbojpeg_h::tjscalingfactor {
            num: 1 as libc::c_int,
            denom: 1 as libc::c_int,
        };
        init
    };
    let mut outSubsamp: libc::c_int = -(1 as libc::c_int);
    let mut outQual: libc::c_int = -(1 as libc::c_int);
    let mut xform: crate::turbojpeg_h::tjtransform = crate::turbojpeg_h::tjtransform {
        r: crate::turbojpeg_h::tjregion {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
        },
        op: 0,
        options: 0,
        data: 0 as *mut libc::c_void,
        customFilter: None,
    };
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut inFormat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outFormat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut jpegFile: *mut crate::stdlib::FILE =
        crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE;
    let mut imgBuf: *mut libc::c_uchar = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
    let mut jpegBuf: *mut libc::c_uchar = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut pixelFormat: libc::c_int = crate::turbojpeg_h::TJPF_UNKNOWN as libc::c_int;
    let mut tjInstance: crate::turbojpeg_h::tjhandle = crate::stddef_h::NULL_0 as *mut libc::c_void;
    scalingFactors = crate::turbojpeg_h::tjGetScalingFactors(&mut numScalingFactors);
    if scalingFactors.is_null() {
        crate::stdlib::printf(
            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            b"getting scaling factors\x00" as *const u8 as *const libc::c_char,
            crate::turbojpeg_h::tjGetErrorStr2(tjInstance),
        );
        retval = -(1 as libc::c_int)
    } else {
        crate::stdlib::memset(
            &mut xform as *mut crate::turbojpeg_h::tjtransform as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::turbojpeg_h::tjtransform>() as libc::c_ulong,
        );
        if argc < 3 as libc::c_int {
            usage(*argv.offset(0 as libc::c_int as isize));
        }
        /* Parse arguments. */
        i = 3 as libc::c_int;
        while i < argc {
            if crate::stdlib::strncasecmp(
                *argv.offset(i as isize),
                b"-sc\x00" as *const u8 as *const libc::c_char,
                3 as libc::c_int as libc::c_ulong,
            ) == 0
                && i < argc - 1 as libc::c_int
            {
                let mut match_0: libc::c_int = 0 as libc::c_int;
                let mut temp1: libc::c_int = 0 as libc::c_int;
                let mut temp2: libc::c_int = 0 as libc::c_int;
                let mut j: libc::c_int = 0;
                i += 1;
                if crate::stdlib::sscanf(
                    *argv.offset(i as isize),
                    b"%d/%d\x00" as *const u8 as *const libc::c_char,
                    &mut temp1 as *mut libc::c_int,
                    &mut temp2 as *mut libc::c_int,
                ) < 2 as libc::c_int
                {
                    usage(*argv.offset(0 as libc::c_int as isize));
                }
                j = 0 as libc::c_int;
                while j < numScalingFactors {
                    if temp1 as libc::c_double / temp2 as libc::c_double
                        == (*scalingFactors.offset(j as isize)).num as libc::c_double
                            / (*scalingFactors.offset(j as isize)).denom as libc::c_double
                    {
                        scalingFactor = *scalingFactors.offset(j as isize);
                        match_0 = 1 as libc::c_int;
                        break;
                    } else {
                        j += 1
                    }
                }
                if match_0 != 1 as libc::c_int {
                    usage(*argv.offset(0 as libc::c_int as isize));
                }
            } else if crate::stdlib::strncasecmp(
                *argv.offset(i as isize),
                b"-su\x00" as *const u8 as *const libc::c_char,
                3 as libc::c_int as libc::c_ulong,
            ) == 0
                && i < argc - 1 as libc::c_int
            {
                i += 1;
                if crate::stdlib::strncasecmp(
                    *argv.offset(i as isize),
                    b"g\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    outSubsamp = crate::turbojpeg_h::TJSAMP_GRAY as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"444\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    outSubsamp = crate::turbojpeg_h::TJSAMP_444 as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"422\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    outSubsamp = crate::turbojpeg_h::TJSAMP_422 as libc::c_int
                } else if crate::stdlib::strcasecmp(
                    *argv.offset(i as isize),
                    b"420\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    outSubsamp = crate::turbojpeg_h::TJSAMP_420 as libc::c_int
                } else {
                    usage(*argv.offset(0 as libc::c_int as isize));
                }
            } else if crate::stdlib::strncasecmp(
                *argv.offset(i as isize),
                b"-q\x00" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            ) == 0
                && i < argc - 1 as libc::c_int
            {
                i += 1;
                outQual = atoi(*argv.offset(i as isize));
                if outQual < 1 as libc::c_int || outQual > 100 as libc::c_int {
                    usage(*argv.offset(0 as libc::c_int as isize));
                }
            } else if crate::stdlib::strncasecmp(
                *argv.offset(i as isize),
                b"-g\x00" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                xform.options |= crate::turbojpeg_h::TJXOPT_GRAY
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-hflip\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                xform.op = crate::turbojpeg_h::TJXOP_HFLIP as libc::c_int
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-vflip\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                xform.op = crate::turbojpeg_h::TJXOP_VFLIP as libc::c_int
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-transpose\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                xform.op = crate::turbojpeg_h::TJXOP_TRANSPOSE as libc::c_int
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-transverse\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                xform.op = crate::turbojpeg_h::TJXOP_TRANSVERSE as libc::c_int
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-rot90\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                xform.op = crate::turbojpeg_h::TJXOP_ROT90 as libc::c_int
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-rot180\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                xform.op = crate::turbojpeg_h::TJXOP_ROT180 as libc::c_int
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-rot270\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                xform.op = crate::turbojpeg_h::TJXOP_ROT270 as libc::c_int
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-custom\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                xform.customFilter = Some(
                    customFilter
                        as unsafe extern "C" fn(
                            _: *mut libc::c_short,
                            _: crate::turbojpeg_h::tjregion,
                            _: crate::turbojpeg_h::tjregion,
                            _: libc::c_int,
                            _: libc::c_int,
                            _: *mut crate::turbojpeg_h::tjtransform,
                        ) -> libc::c_int,
                )
            } else if crate::stdlib::strncasecmp(
                *argv.offset(i as isize),
                b"-c\x00" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            ) == 0
                && i < argc - 1 as libc::c_int
            {
                i += 1;
                if crate::stdlib::sscanf(
                    *argv.offset(i as isize),
                    b"%dx%d+%d+%d\x00" as *const u8 as *const libc::c_char,
                    &mut xform.r.w as *mut libc::c_int,
                    &mut xform.r.h as *mut libc::c_int,
                    &mut xform.r.x as *mut libc::c_int,
                    &mut xform.r.y as *mut libc::c_int,
                ) < 4 as libc::c_int
                    || xform.r.x < 0 as libc::c_int
                    || xform.r.y < 0 as libc::c_int
                    || xform.r.w < 1 as libc::c_int
                    || xform.r.h < 1 as libc::c_int
                {
                    usage(*argv.offset(0 as libc::c_int as isize));
                }
                xform.options |= crate::turbojpeg_h::TJXOPT_CROP
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-fastupsample\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                crate::stdlib::printf(
                    b"Using fast upsampling code\n\x00" as *const u8 as *const libc::c_char,
                );
                flags |= crate::turbojpeg_h::TJFLAG_FASTUPSAMPLE
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-fastdct\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                crate::stdlib::printf(
                    b"Using fastest DCT/IDCT algorithm\n\x00" as *const u8 as *const libc::c_char,
                );
                flags |= crate::turbojpeg_h::TJFLAG_FASTDCT
            } else if crate::stdlib::strcasecmp(
                *argv.offset(i as isize),
                b"-accuratedct\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                crate::stdlib::printf(
                    b"Using most accurate DCT/IDCT algorithm\n\x00" as *const u8
                        as *const libc::c_char,
                );
                flags |= crate::turbojpeg_h::TJFLAG_ACCURATEDCT
            } else {
                usage(*argv.offset(0 as libc::c_int as isize));
            }
            i += 1
        }
        /* Determine input and output image formats based on file extensions. */
        inFormat = crate::stdlib::strrchr(*argv.offset(1 as libc::c_int as isize), '.' as i32);
        outFormat = crate::stdlib::strrchr(*argv.offset(2 as libc::c_int as isize), '.' as i32);
        if inFormat.is_null()
            || outFormat.is_null()
            || crate::stdlib::strlen(inFormat) < 2 as libc::c_int as libc::c_ulong
            || crate::stdlib::strlen(outFormat) < 2 as libc::c_int as libc::c_ulong
        {
            usage(*argv.offset(0 as libc::c_int as isize));
        }
        inFormat = &mut *inFormat.offset(1 as libc::c_int as isize) as *mut libc::c_char;
        outFormat = &mut *outFormat.offset(1 as libc::c_int as isize) as *mut libc::c_char;
        if crate::stdlib::strcasecmp(inFormat, b"jpg\x00" as *const u8 as *const libc::c_char) == 0
        {
            /* Input image is a JPEG image.  Decompress and/or transform it. */
            let mut size: libc::c_long = 0;
            let mut inSubsamp: libc::c_int = 0;
            let mut inColorspace: libc::c_int = 0;
            let mut doTransform: libc::c_int =
                (xform.op != crate::turbojpeg_h::TJXOP_NONE as libc::c_int
                    || xform.options != 0 as libc::c_int
                    || xform.customFilter.is_some()) as libc::c_int;
            let mut jpegSize: libc::c_ulong = 0;
            /* Read the JPEG file into memory. */
            jpegFile = crate::stdlib::fopen(
                *argv.offset(1 as libc::c_int as isize),
                b"rb\x00" as *const u8 as *const libc::c_char,
            );
            if jpegFile.is_null() {
                crate::stdlib::printf(
                    b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                    269 as libc::c_int,
                    b"opening input file\x00" as *const u8 as *const libc::c_char,
                    crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                );
                retval = -(1 as libc::c_int);
                current_block = 16288987300638808654;
            } else if crate::stdlib::fseek(
                jpegFile,
                0 as libc::c_int as libc::c_long,
                crate::stdlib::SEEK_END,
            ) < 0 as libc::c_int
                || {
                    size = crate::stdlib::ftell(jpegFile);
                    (size) < 0 as libc::c_int as libc::c_long
                }
                || crate::stdlib::fseek(
                    jpegFile,
                    0 as libc::c_int as libc::c_long,
                    crate::stdlib::SEEK_SET,
                ) < 0 as libc::c_int
            {
                crate::stdlib::printf(
                    b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                    272 as libc::c_int,
                    b"determining input file size\x00" as *const u8 as *const libc::c_char,
                    crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                );
                retval = -(1 as libc::c_int);
                current_block = 16288987300638808654;
            } else if size == 0 as libc::c_int as libc::c_long {
                crate::stdlib::printf(
                    b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                    274 as libc::c_int,
                    b"determining input file size\x00" as *const u8 as *const libc::c_char,
                    b"Input file contains no data\x00" as *const u8 as *const libc::c_char,
                );
                retval = -(1 as libc::c_int);
                current_block = 16288987300638808654;
            } else {
                jpegSize = size as libc::c_ulong;
                jpegBuf = crate::turbojpeg_h::tjAlloc(jpegSize as libc::c_int);
                if jpegBuf.is_null() {
                    crate::stdlib::printf(
                        b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                        277 as libc::c_int,
                        b"allocating JPEG buffer\x00" as *const u8 as *const libc::c_char,
                        crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                    );
                    retval = -(1 as libc::c_int);
                    current_block = 16288987300638808654;
                } else if crate::stdlib::fread(
                    jpegBuf as *mut libc::c_void,
                    jpegSize,
                    1 as libc::c_int as libc::c_ulong,
                    jpegFile,
                ) < 1 as libc::c_int as libc::c_ulong
                {
                    crate::stdlib::printf(
                        b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                        279 as libc::c_int,
                        b"reading input file\x00" as *const u8 as *const libc::c_char,
                        crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                    );
                    retval = -(1 as libc::c_int);
                    current_block = 16288987300638808654;
                } else {
                    crate::stdlib::fclose(jpegFile);
                    jpegFile = crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE;
                    if doTransform != 0 {
                        /* Transform it. */
                        let mut dstBuf: *mut libc::c_uchar =
                            crate::stddef_h::NULL_0 as *mut libc::c_uchar; /* Dynamically allocate the JPEG buffer */
                        let mut dstSize: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
                        tjInstance = crate::turbojpeg_h::tjInitTransform();
                        if tjInstance.is_null() {
                            crate::stdlib::printf(
                                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                288 as libc::c_int,
                                b"initializing transformer\x00" as *const u8 as *const libc::c_char,
                                crate::turbojpeg_h::tjGetErrorStr2(tjInstance),
                            );
                            retval = -(1 as libc::c_int);
                            current_block = 16288987300638808654;
                        } else {
                            xform.options |= crate::turbojpeg_h::TJXOPT_TRIM;
                            if crate::turbojpeg_h::tjTransform(
                                tjInstance,
                                jpegBuf,
                                jpegSize,
                                1 as libc::c_int,
                                &mut dstBuf,
                                &mut dstSize,
                                &mut xform,
                                flags,
                            ) < 0 as libc::c_int
                            {
                                crate::stdlib::printf(
                                    b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    292 as libc::c_int,
                                    b"transforming input image\x00" as *const u8
                                        as *const libc::c_char,
                                    crate::turbojpeg_h::tjGetErrorStr2(tjInstance),
                                );
                                retval = -(1 as libc::c_int);
                                current_block = 16288987300638808654;
                            } else {
                                crate::turbojpeg_h::tjFree(jpegBuf);
                                jpegBuf = dstBuf;
                                jpegSize = dstSize;
                                current_block = 7244994750255146185;
                            }
                        }
                    } else {
                        tjInstance = crate::turbojpeg_h::tjInitDecompress();
                        if tjInstance.is_null() {
                            crate::stdlib::printf(
                                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                298 as libc::c_int,
                                b"initializing decompressor\x00" as *const u8
                                    as *const libc::c_char,
                                crate::turbojpeg_h::tjGetErrorStr2(tjInstance),
                            );
                            retval = -(1 as libc::c_int);
                            current_block = 16288987300638808654;
                        } else {
                            current_block = 7244994750255146185;
                        }
                    }
                    match current_block {
                        16288987300638808654 => {}
                        _ => {
                            if crate::turbojpeg_h::tjDecompressHeader3(
                                tjInstance,
                                jpegBuf,
                                jpegSize,
                                &mut width,
                                &mut height,
                                &mut inSubsamp,
                                &mut inColorspace,
                            ) < 0 as libc::c_int
                            {
                                crate::stdlib::printf(
                                    b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                        as *const libc::c_char,
                                    303 as libc::c_int,
                                    b"reading JPEG header\x00" as *const u8 as *const libc::c_char,
                                    crate::turbojpeg_h::tjGetErrorStr2(tjInstance),
                                );
                                retval = -(1 as libc::c_int);
                                current_block = 16288987300638808654;
                            } else {
                                crate::stdlib::printf(b"%s Image:  %d x %d pixels, %s subsampling, %s colorspace\n\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       if doTransform != 0 {
                                           b"Transformed\x00" as *const u8 as
                                               *const libc::c_char
                                       } else {
                                           b"Input\x00" as *const u8 as
                                               *const libc::c_char
                                       }, width, height,
                                       subsampName[inSubsamp as usize],
                                       colorspaceName[inColorspace as usize]);
                                if crate::stdlib::strcasecmp(
                                    outFormat,
                                    b"jpg\x00" as *const u8 as *const libc::c_char,
                                ) == 0
                                    && doTransform != 0
                                    && scalingFactor.num == 1 as libc::c_int
                                    && scalingFactor.denom == 1 as libc::c_int
                                    && outSubsamp < 0 as libc::c_int
                                    && outQual < 0 as libc::c_int
                                {
                                    /* Input image has been transformed, and no re-compression options
                                    have been selected.  Write the transformed image to disk and exit. */
                                    jpegFile = crate::stdlib::fopen(
                                        *argv.offset(2 as libc::c_int as isize),
                                        b"wb\x00" as *const u8 as *const libc::c_char,
                                    );
                                    if jpegFile.is_null() {
                                        crate::stdlib::printf(
                                            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                                as *const libc::c_char,
                                            315 as libc::c_int,
                                            b"opening output file\x00" as *const u8
                                                as *const libc::c_char,
                                            crate::stdlib::strerror(
                                                *crate::stdlib::__errno_location(),
                                            ),
                                        );
                                        retval = -(1 as libc::c_int)
                                    } else if crate::stdlib::fwrite(
                                        jpegBuf as *const libc::c_void,
                                        jpegSize,
                                        1 as libc::c_int as libc::c_ulong,
                                        jpegFile,
                                    ) < 1 as libc::c_int as libc::c_ulong
                                    {
                                        crate::stdlib::printf(
                                            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                                as *const libc::c_char,
                                            317 as libc::c_int,
                                            b"writing output file\x00" as *const u8
                                                as *const libc::c_char,
                                            crate::stdlib::strerror(
                                                *crate::stdlib::__errno_location(),
                                            ),
                                        );
                                        retval = -(1 as libc::c_int)
                                    } else {
                                        crate::stdlib::fclose(jpegFile);
                                        jpegFile =
                                            crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE
                                    }
                                    current_block = 16288987300638808654;
                                } else {
                                    /* Scaling and/or a non-JPEG output image format and/or compression options
                                    have been selected, so we need to decompress the input/transformed
                                    image. */
                                    width = (width * scalingFactor.num + scalingFactor.denom
                                        - 1 as libc::c_int)
                                        / scalingFactor.denom;
                                    height = (height * scalingFactor.num + scalingFactor.denom
                                        - 1 as libc::c_int)
                                        / scalingFactor.denom;
                                    if outSubsamp < 0 as libc::c_int {
                                        outSubsamp = inSubsamp
                                    }
                                    pixelFormat = crate::turbojpeg_h::TJPF_BGRX as libc::c_int;
                                    imgBuf = crate::turbojpeg_h::tjAlloc(
                                        width * height * tjPixelSize[pixelFormat as usize],
                                    );
                                    if imgBuf.is_null() {
                                        crate::stdlib::printf(
                                            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                                as *const libc::c_char,
                                            333 as libc::c_int,
                                            b"allocating uncompressed image buffer\x00" as *const u8
                                                as *const libc::c_char,
                                            crate::stdlib::strerror(
                                                *crate::stdlib::__errno_location(),
                                            ),
                                        );
                                        retval = -(1 as libc::c_int);
                                        current_block = 16288987300638808654;
                                    } else if crate::turbojpeg_h::tjDecompress2(
                                        tjInstance,
                                        jpegBuf,
                                        jpegSize,
                                        imgBuf,
                                        width,
                                        0 as libc::c_int,
                                        height,
                                        pixelFormat,
                                        flags,
                                    ) < 0 as libc::c_int
                                    {
                                        crate::stdlib::printf(
                                            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                                as *const libc::c_char,
                                            337 as libc::c_int,
                                            b"decompressing JPEG image\x00" as *const u8
                                                as *const libc::c_char,
                                            crate::turbojpeg_h::tjGetErrorStr2(tjInstance),
                                        );
                                        retval = -(1 as libc::c_int);
                                        current_block = 16288987300638808654;
                                    } else {
                                        crate::turbojpeg_h::tjFree(jpegBuf);
                                        jpegBuf = crate::stddef_h::NULL_0 as *mut libc::c_uchar;
                                        crate::turbojpeg_h::tjDestroy(tjInstance);
                                        tjInstance = crate::stddef_h::NULL_0 as *mut libc::c_void;
                                        current_block = 3098209481605707636;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            /* Input image is not a JPEG image.  Load it into memory. */
            imgBuf = crate::turbojpeg_h::tjLoadImage(
                *argv.offset(1 as libc::c_int as isize),
                &mut width,
                1 as libc::c_int,
                &mut height,
                &mut pixelFormat,
                0 as libc::c_int,
            );
            if imgBuf.is_null() {
                crate::stdlib::printf(
                    b"ERROR in line %d while %s:\n%s\n\x00" as *const u8 as *const libc::c_char,
                    344 as libc::c_int,
                    b"loading input image\x00" as *const u8 as *const libc::c_char,
                    crate::turbojpeg_h::tjGetErrorStr2(tjInstance),
                );
                retval = -(1 as libc::c_int);
                current_block = 16288987300638808654;
            } else {
                if outSubsamp < 0 as libc::c_int {
                    if pixelFormat == crate::turbojpeg_h::TJPF_GRAY as libc::c_int {
                        outSubsamp = crate::turbojpeg_h::TJSAMP_GRAY as libc::c_int
                    } else {
                        outSubsamp = crate::turbojpeg_h::TJSAMP_444 as libc::c_int
                    }
                }
                crate::stdlib::printf(
                    b"Input Image:  %d x %d pixels\n\x00" as *const u8 as *const libc::c_char,
                    width,
                    height,
                );
                current_block = 3098209481605707636;
            }
        }
        match current_block {
            16288987300638808654 => {}
            _ => {
                crate::stdlib::printf(
                    b"Output Image (%s):  %d x %d pixels\x00" as *const u8 as *const libc::c_char,
                    outFormat,
                    width,
                    height,
                );
                if crate::stdlib::strcasecmp(
                    outFormat,
                    b"jpg\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    /* Output image format is JPEG.  Compress the uncompressed image. */
                    let mut jpegBuf_0: *mut libc::c_uchar =
                        crate::stddef_h::NULL_0 as *mut libc::c_uchar; /* Dynamically allocate the JPEG buffer */
                    let mut jpegSize_0: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
                    if outQual < 0 as libc::c_int {
                        outQual = DEFAULT_QUALITY
                    }
                    crate::stdlib::printf(
                        b", %s subsampling, quality = %d\n\x00" as *const u8 as *const libc::c_char,
                        subsampName[outSubsamp as usize],
                        outQual,
                    );
                    tjInstance = crate::turbojpeg_h::tjInitCompress();
                    if tjInstance.is_null() {
                        crate::stdlib::printf(
                            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                as *const libc::c_char,
                            367 as libc::c_int,
                            b"initializing compressor\x00" as *const u8 as *const libc::c_char,
                            crate::turbojpeg_h::tjGetErrorStr2(tjInstance),
                        );
                        retval = -(1 as libc::c_int)
                    } else if crate::turbojpeg_h::tjCompress2(
                        tjInstance,
                        imgBuf,
                        width,
                        0 as libc::c_int,
                        height,
                        pixelFormat,
                        &mut jpegBuf_0,
                        &mut jpegSize_0,
                        outSubsamp,
                        outQual,
                        flags,
                    ) < 0 as libc::c_int
                    {
                        crate::stdlib::printf(
                            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                as *const libc::c_char,
                            370 as libc::c_int,
                            b"compressing image\x00" as *const u8 as *const libc::c_char,
                            crate::turbojpeg_h::tjGetErrorStr2(tjInstance),
                        );
                        retval = -(1 as libc::c_int)
                    } else {
                        crate::turbojpeg_h::tjDestroy(tjInstance);
                        tjInstance = crate::stddef_h::NULL_0 as *mut libc::c_void;
                        /* Write the JPEG image to disk. */
                        jpegFile = crate::stdlib::fopen(
                            *argv.offset(2 as libc::c_int as isize),
                            b"wb\x00" as *const u8 as *const libc::c_char,
                        );
                        if jpegFile.is_null() {
                            crate::stdlib::printf(
                                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                375 as libc::c_int,
                                b"opening output file\x00" as *const u8 as *const libc::c_char,
                                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                            );
                            retval = -(1 as libc::c_int)
                        } else if crate::stdlib::fwrite(
                            jpegBuf_0 as *const libc::c_void,
                            jpegSize_0,
                            1 as libc::c_int as libc::c_ulong,
                            jpegFile,
                        ) < 1 as libc::c_int as libc::c_ulong
                        {
                            crate::stdlib::printf(
                                b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                    as *const libc::c_char,
                                377 as libc::c_int,
                                b"writing output file\x00" as *const u8 as *const libc::c_char,
                                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                            );
                            retval = -(1 as libc::c_int)
                        } else {
                            crate::turbojpeg_h::tjDestroy(tjInstance);
                            tjInstance = crate::stddef_h::NULL_0 as *mut libc::c_void;
                            crate::stdlib::fclose(jpegFile);
                            jpegFile = crate::stddef_h::NULL_0 as *mut crate::stdlib::FILE;
                            crate::turbojpeg_h::tjFree(jpegBuf_0);
                            jpegBuf_0 = crate::stddef_h::NULL_0 as *mut libc::c_uchar
                        }
                    }
                } else {
                    /* Output image format is not JPEG.  Save the uncompressed image
                    directly to disk. */
                    crate::stdlib::printf(b"\n\x00" as *const u8 as *const libc::c_char);
                    if crate::turbojpeg_h::tjSaveImage(
                        *argv.offset(2 as libc::c_int as isize),
                        imgBuf,
                        width,
                        0 as libc::c_int,
                        height,
                        pixelFormat,
                        0 as libc::c_int,
                    ) < 0 as libc::c_int
                    {
                        crate::stdlib::printf(
                            b"ERROR in line %d while %s:\n%s\n\x00" as *const u8
                                as *const libc::c_char,
                            386 as libc::c_int,
                            b"saving output image\x00" as *const u8 as *const libc::c_char,
                            crate::turbojpeg_h::tjGetErrorStr2(tjInstance),
                        );
                        retval = -(1 as libc::c_int)
                    }
                }
            }
        }
    }
    if !imgBuf.is_null() {
        crate::turbojpeg_h::tjFree(imgBuf);
    }
    if !tjInstance.is_null() {
        crate::turbojpeg_h::tjDestroy(tjInstance);
    }
    if !jpegBuf.is_null() {
        crate::turbojpeg_h::tjFree(jpegBuf);
    }
    if !jpegFile.is_null() {
        crate::stdlib::fclose(jpegFile);
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
