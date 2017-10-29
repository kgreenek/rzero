#!/bin/bash -ex
targets=(x86_64-apple-darwin i686-apple-darwin)
# Build iOS targets and universal iOS library.
cargo lipo --release
for target in ${targets[@]}; do
  cross build --release --target $target
done
