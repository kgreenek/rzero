#!/bin/bash -ex
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
$DIR/build_deploy_${TRAVIS_OS_NAME}.sh
target_dir="$DIR/../target"
deploy_dir="$DIR/../deploy"
mkdir -p $deploy_dir
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
  dest_dir="$deploy_dir/$target_name"
  mkdir -p $dest_dir/release
  cp -f $target_subdir/release/*rzero.* $dest_dir/release
  zip -r $deploy_dir/$target_name.zip $dest_dir
done
