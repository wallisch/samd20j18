#!/bin/sh
set -e

rm -f build.rs
rm -f device.x
rm -rf src/*

svd2rust -i ATSAMD20J18.svd
form -i lib.rs -o src/ && rm lib.rs
cargo fmt
patch -p1 -i fix_register_group_naming.patch
