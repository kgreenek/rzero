#!/bin/bash

# Linux
cross build --target x86_64-unknown-linux-gnu
cross build --release --target x86_64-unknown-linux-gnu

cross build --target i686-unknown-linux-gnu
cross build --release --target i686-unknown-linux-gnu

# Windows
cross build --target x86_64-pc-windows-gnu
cross build --release --target x86_64-pc-windows-gnu

cross build --target i686-pc-windows-gnu
cross build --release --target i686-pc-windows-gnu

# Android
cross build --target aarch64-linux-android
cross build --release --target aarch64-linux-android

cross build --target arm-linux-androideabi
cross build --release --target arm-linux-androideabi

cross build --target i686-linux-android
cross build --release --target i686-linux-android
