#!/bin/bash

CMDNAME=`basename $0`
script_dir="$( dirname -- "$( readlink -f -- "$0"; )"; )"

pushd $script_dir
cd ../..
cd capi
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  cargo build --release --all
  cd ..
  cp ./capi/target/release/*.so ./dotnet/cs/src/native/linux/x64
  cp ./capi/target/release/*.so ./dotnet/cs/tests
elif [[ "$OSTYPE" == "darwin"* ]]; then
  cargo build --release --all --exclude autd3capi-backend-cuda --target=x86_64-apple-darwin
  cargo build --release --all --exclude autd3capi-backend-cuda --target=aarch64-apple-darwin
  cd ..
  for x64_file in `ls ./capi/target/x86_64-apple-darwin/release/*.dylib`; do
    file_basename=`basename $x64_file`
    lipo -create $x64_file ./capi/target/aarch64-apple-darwin/release/$file_basename -output ./dotnet/cs/src/native/osx/universal/$file_basename
    cp ./dotnet/cs/src/native/osx/universal/$file_basename ./dotnet/cs/tests
  done
fi

popd
