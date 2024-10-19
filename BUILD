#!/usr/bin/env bash
export RUSTFLAGS="-C target-cpu=native"
cargo build --release -Z build-std-features="optimize_for_size" -Z build-std=core,alloc,proc_macro,std --target x86_64-unknown-linux-gnu
