#!/bin/bash -ex
targets=(x86_64-unknown-linux-gnu
    i686-unknown-linux-gnu
    x86_64-pc-windows-gnu
    i686-pc-windows-gnu
    aarch64-linux-android
    arm-linux-androideabi
    i686-linux-android)
for target in ${targets[@]}; do
  cross build --release --target $target
done
