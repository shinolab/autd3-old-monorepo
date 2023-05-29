#!/bin/bash

CMDNAME=`basename $0`
script_dir="$( dirname -- "$( readlink -f -- "$0"; )"; )"

pushd $script_dir
cd ..

mkdir python/pyautd3/bin/win_x64 > NUL 2>&1

cd capi
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  cargo build --release --all
  mkdir -p python/pyautd3/bin/linux_x64
  cp ./capi/target/release/*.so python/pyautd3/bin/linux_x64
elif [[ "$OSTYPE" == "darwin"* ]]; then
  cargo build --release --all --target=x86_64-apple-darwin
  cargo build --release --all --target=aarch64-apple-darwin
  cd ..
  mkdir -p python/pyautd3/bin/macos_universal
  for x64_file in `ls ./capi/target/x86_64-apple-darwin/release/*.dylib`; do
    file_basename=`basename $x64_file`
    lipo -create $x64_file ./capi/target/aarch64-apple-darwin/release/$file_basename -output python/pyautd3/bin/macos_universal/$file_basename
  done
fi

popd
