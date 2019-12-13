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
c2rust transpile --output-dir mozjpeg --emit-build-files --translate-const-macros --overwrite-existing --reorganize-definitions \
       --binary cjpeg \
       --binary djpeg \
       --binary jcstest \
       --binary jpegtran \
       --binary md5cmp \
       --binary rdjpgcom \
       --binary tjbench \
       --binary tjexample \
       --binary tjunittest \
       --binary wrjpgcom \
       compile_commands.json

# Move binary modules from src/ to bin/
binary_modules=`grep --recursive --files-with-matches 'pub fn main()' mozjpeg/src/`

mkdir mozjpeg/bin || true

for path in $binary_modules
do
    filename="${path##*/}"

    # Move into bin/
    mv "$path" "mozjpeg/bin/$filename"
done

cp -r mozjpeg/src mozjpeg/bin mozjpeg/lib.rs mozjpeg/rust-toolchain "$src_dir/"

rm -r "$tmp_dir"
