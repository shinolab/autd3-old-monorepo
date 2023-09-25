#!/bin/bash

CMDNAME=`basename $0`
script_dir="$( dirname -- "$( readlink -f -- "$0"; )"; )"

pushd $script_dir
cd ../..

cd capi
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  cargo build
  cd ..
  cp ./capi/target/debug/*.so ./dotnet/cs/tests
elif [[ "$OSTYPE" == "darwin"* ]]; then
  cargo build --target=x86_64-apple-darwin
  cargo build --target=aarch64-apple-darwin
  cd ..
  for x64_file in `ls ./capi/target/x86_64-apple-darwin/debug/*.dylib`; do
    file_basename=`basename $x64_file`
    lipo -create $x64_file ./capi/target/aarch64-apple-darwin/debug/$file_basename -output ./dotnet/cs/tests/$file_basename
  done
fi

cd ./dotnet/cs/tests
dotnet test

popd
