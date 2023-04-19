#!/bin/sh
set -e

build_for() {
    TOOLCHAIN=$1
    EXE=$2

    rustup target add $TOOLCHAIN
    cargo build --target "$TOOLCHAIN" --release

    rm $1.zip
    7z a $1.zip ./target/$TOOLCHAIN/release/$EXE ./assets
}

rustup default stable
build_for "x86_64-pc-windows-gnu" "test-generator.exe"
build_for "x86_64-unknown-linux-gnu" "test-generator"
