#!/bin/bash -ex
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

cargo install cross --force
if [[ $TRAVIS_OS_NAME == "osx" ]]; then
  # NOTE: We don't need to add x86_64-apple-darwin because it's installed by default.
  rustup target add i686-apple-darwin
  rustup target add aarch64-apple-ios
  rustup target add armv7-apple-ios
  rustup target add armv7s-apple-ios
  rustup target add i386-apple-ios
  rustup target add x86_64-apple-ios
  cargo install cargo-lipo --force
fi

$DIR/build_deploy_${TRAVIS_OS_NAME}.sh

libs=(librzero.so librzero.a librzero.dylib rzero.dll)
target_dir="$DIR/../target"
deploy_dir="$DIR/../deploy"
mkdir -p $deploy_dir
cd $deploy_dir
for target_subdir in $target_dir/*/; do
  target_subdir=${target_subdir%*/}
  target_name=${target_subdir##*/}
  if [[ $target_name == "release" ]] || [[ $target_name == "debug" ]]; then
    # The "release" and "debug" directories contain whatever happened to be last built, so they
    # should be ignored.
    continue
  fi
  if [[ $target_name == "universal" ]]; then
    # cargo-lipo outputs the generated universal iOS library in a directory called "universal",
    # which isn't as descriptive as we would like.
    target_name="universal-apple-ios"
  fi
  mkdir -p $target_name
  for lib in ${libs[@]}; do
    lib_file=$target_subdir/release/$lib
    [[ -e $lib_file ]] && cp -f $lib_file $target_name
  done
  zip -r $target_name.zip $target_name
done
