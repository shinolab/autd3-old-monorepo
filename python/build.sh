#!/bin/bash

CMDNAME=`basename $0`
script_dir="$( dirname -- "$( readlink -f -- "$0"; )"; )"

pushd $script_dir
cd ..

cd capi
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  if ! [ -x "$(command -v nvcc)" ]; then    
    cargo build --release --all --exclude autd3capi-backend-cuda
  else 
    cargo build --release --all
  fi
  cd ..
  mkdir -p python/pyautd3/bin
  cp ./capi/target/release/*.so python/pyautd3/bin
  cd python
  python -m build -w -C="--build-option=--plat-name" -C="--build-option=manylinux1-x86_64"
elif [[ "$OSTYPE" == "darwin"* ]]; then
  cargo build --release --all --exclude autd3capi-backend-cuda --target=x86_64-apple-darwin
  cargo build --release --all --exclude autd3capi-backend-cuda --target=aarch64-apple-darwin
  cd ..
  mkdir -p python/pyautd3/bin
  for x64_file in `ls ./capi/target/x86_64-apple-darwin/release/*.dylib`; do
    file_basename=`basename $x64_file`
    lipo -create $x64_file ./capi/target/aarch64-apple-darwin/release/$file_basename -output python/pyautd3/bin/$file_basename
  done
  cd python
  python -m build -w -C="--build-option=--plat-name" -C="--build-option=macosx-10-13-x86_64"
  python -m build -w -C="--build-option=--plat-name" -C="--build-option=macosx-11-0-arm64"
fi
popd
