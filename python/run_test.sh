#!/bin/bash

CMDNAME=`basename $0`
script_dir="$( dirname -- "$( readlink -f -- "$0"; )"; )"

pushd $script_dir
cd ..

cd capi
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  cargo build
  cd ..
  mkdir -p python/pyautd3/bin
  cp ./capi/target/debug/*.so python/pyautd3/bin
  cd python
  python3 -m pytest
elif [[ "$OSTYPE" == "darwin"* ]]; then
  cargo build --target=x86_64-apple-darwin
  cargo build --target=aarch64-apple-darwin
  cd ..
  mkdir -p python/pyautd3/bin
  for x64_file in `ls ./capi/target/x86_64-apple-darwin/debug/*.dylib`; do
    file_basename=`basename $x64_file`
    lipo -create $x64_file ./capi/target/aarch64-apple-darwin/debug/$file_basename -output python/pyautd3/bin/$file_basename
  done
  cd python
  python3 -m pytest
fi
popd
