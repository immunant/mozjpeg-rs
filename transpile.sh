#!/bin/bash

set -e

export RUSTFLAGS=-Awarnings

src_dir=`pwd`

tmp_dir=`mktemp -d`
echo "Transpiling in temporary directory $tmp_dir"
cd "$tmp_dir"

# Generate compile_commands.json
cmake -G"Unix Makefiles" -DCMAKE_EXPORT_COMPILE_COMMANDS=true -DENABLE_STATIC=false -DCMAKE_C_COMPILER=clang "$src_dir/mozjpeg-c/"

# Transpile from C to Rust
c2rust transpile --output-dir . --emit-build-files --translate-const-macros --overwrite-existing --reorganize-definitions compile_commands.json

# Move binary modules from src/ to bin/
binary_modules=`grep --recursive --files-with-matches 'pub fn main()' src/`
new_header="#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_transmute)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(ptr_wrapping_offset_from)]
#![feature(main)]

"
new_use="use mozjpeg::*;\n"

mkdir bin || true

for path in $binary_modules
do
    filename="${path##*/}"
    modname="${filename%.rs}"

    # Remove binary module from lib.rs
    sed --in-place -e "$!N;s|^#\[path = \"$path\"\]\npub mod $modname;$||;P;D" lib.rs
    sed --in-place -e "$!N;s|pub mod $modname;$||;P;D" lib.rs

    # Add header to binary module file
    sed --in-place "s|use libc;|$new_use|" "$path"

    # Prepend header and move into bin/
    echo "$new_header" | cat - "$path" > "bin/$filename"
    rm "$path"
done

cp -r src bin lib.rs rust-toolchain "$src_dir/"

rm -r "$tmp_dir"
