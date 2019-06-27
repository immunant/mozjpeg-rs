Rust Port of the Mozilla JPEG Encoder Project
=============================================

This project attempts to be a drop-in compatible Rust port of [MozJPEG](https://github.com/mozilla/mozjpeg). The initial Rust sources transpiled using [C2Rust](https://github.com/immunant/c2rust) and are in the process of being incrementally rewritten and refactored into idiomatic Rust.

The original MozJPEG sources this code is based on can be found in the `mozjpeg-c` directory as a git submodule.

## Compiling

    cargo build

MozJPEG-rs is currently supported only on x86_64 Linux.

## MozJPEG README

MozJPEG reduces file sizes of JPEG images while retaining quality and compatibility with the vast majority of the world's deployed decoders.

MozJPEG is based on [libjpeg-turbo](https://github.com/libjpeg-turbo/libjpeg-turbo). **Please send pull requests to libjpeg-turbo** if the changes aren't specific to newly-added MozJPEG-only compression code. This project aims to keep differences with libjpeg-turbo minimal, so whenever possible, improvements and bug fixes should go there first.

It's compatible with libjpeg API and ABI, and can be used as a drop-in replacement for libjpeg. MozJPEG makes tradeoffs that are intended to benefit Web use cases and focuses solely on improving encoding, so it's best used as part of a Web encoding workflow.

MozJPEG is meant to be used as a library in graphics programs and image processing tools. We include a demo `cjpeg` tool, but it's not intended for serious use. We encourage authors of graphics programs to use MozJPEG's [C API](libjpeg.txt) instead.

### Features

* Progressive encoding with "jpegrescan" optimization. It can be applied to any JPEG file (with `jpegtran`) to losslessly reduce file size.
* Trellis quantization. When converting other formats to JPEG it maximizes quality/filesize ratio.
* Comes with new quantization table presets, e.g. tuned for high-resolution displays.
* Fully compatible with all web browsers.
* Can be seamlessly integrated into any program using libjpeg.
