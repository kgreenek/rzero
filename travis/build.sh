#!/bin/bash -ex
if [[ $RZERO_TARGET == "universal-apple-ios" ]]; then
  cargo lipo
  cargo lipo --release
else
  cross build --target $RZERO_TARGET
  cross build --release --target $RZERO_TARGET
fi
