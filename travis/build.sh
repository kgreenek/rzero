#!/bin/bash -ex
if [[ $RZERO_TARGET == "universal-apple-ios" ]]; then
  if [[ $1 == "release" ]]; then
    cargo lipo --release
  else
    cargo lipo
  fi
else
  if [[ $1 == "release" ]]; then
    cross build --release --target $RZERO_TARGET
  else
    cross build --target $RZERO_TARGET
  fi
fi
