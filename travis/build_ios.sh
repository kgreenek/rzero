#!/bin/bash -ex
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd $DIR/..
mkdir -p target/universal-apple-ios/release
targets=(aarch64-apple-ios
    armv7-apple-ios
    armv7s-apple-ios
    i386-apple-ios
    x86_64-apple-ios)
for target in ${targets[@]}; do
  RUSTFLAGS="--emit=llvm-bc" cargo build --release --target $target
done
lipo -create -output target/universal-apple-ios/release/rzero.bc \
  target/aarch64-apple-ios/release/deps/rzero.bc \
  target/armv7-apple-ios/release/deps/rzero.bc \
  target/armv7s-apple-ios/release/deps/rzero.bc \
  target/i386-apple-ios/release/deps/rzero.bc \
  target/x86_64-apple-ios/release/deps/rzero.bc
