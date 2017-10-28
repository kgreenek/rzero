#!/bin/bash -ex

# MacOS
cargo build --target x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin

cargo build --target i686-apple-darwin
cargo build --release --target i686-apple-darwin

# iOS
# This builds all ios architectures, and a universal library (under target/universal).
# NOTE: Only static libs are built.
cargo lipo
cargo lipo --release
