#!/bin/bash -ex
# NOTE: We don't need to add x86_64-apple-darwin because it's installed by default.
rustup target add i686-apple-darwin
rustup target add aarch64-apple-ios
rustup target add armv7-apple-ios
rustup target add armv7s-apple-ios
rustup target add i386-apple-ios
rustup target add x86_64-apple-ios
cargo install cargo-lipo --force
