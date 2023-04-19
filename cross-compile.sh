#!/bin/sh
set -e

die() {
    printf "\x1b[31;1mERROR\x1b[0m: $1\n"
    exit 1
}

build_for() {
    TOOLCHAIN=$1
    EXE=$2

    if [[ -z "$TOOLCHAIN" ]]; then
        die "no toolchain specified to build_for()"
    fi
    if [[ -z "$EXE" ]]; then
        die "no output file specified to build_for()"
    fi

    rustup target add $TOOLCHAIN
    cargo build --target "$TOOLCHAIN" --release

    rm $1.zip
    7z a $1.zip ./target/$TOOLCHAIN/release/$EXE ./assets
}

rustup default stable
build_for "x86_64-pc-windows-gnu" "test-generator.exe"
build_for "x86_64-unknown-linux-gnu" "test-generator"
