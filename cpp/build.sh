#!/bin/bash

CMDNAME=`basename $0`
script_dir="$( dirname -- "$( readlink -f -- "$0"; )"; )"

pushd $script_dir
cd ..
mkdir -p cpp/lib
cd capi
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  cargo build --release --all
  cd ..
  cp ./capi/target/release/*.so cpp/lib
elif [[ "$OSTYPE" == "darwin"* ]]; then
  cargo build --release --all --target=x86_64-apple-darwin
  cargo build --release --all --target=aarch64-apple-darwin
  cd ..
  for x64_file in `ls ./capi/target/x86_64-apple-darwin/release/*.dylib`; do
    file_basename=`basename $x64_file`
    lipo -create $x64_file ./capi/target/aarch64-apple-darwin/release/$file_basename -output cpp/lib/$file_basename
  done
fi

popd
