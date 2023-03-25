#!/bin/sh
set -e

cargo build --target x86_64-pc-windows-gnu --release
7z a win-build.zip ./target/x86_64-pc-windows-gnu/release/test-generator.exe ./assets
