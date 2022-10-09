#!/bin/bash

CMDNAME=`basename $0`
script_dir="$( dirname -- "$( readlink -f -- "$0"; )"; )"

pushd $script_dir
cd ..
mkdir -p build
cd build
cmake .. -DBUILD_ALL=ON
cmake --build . --parallel 8
cd ..
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  dst="python/pyautd3/bin/linux_x64"
  ext="so"
elif [[ "$OSTYPE" == "darwin"* ]]; then
  dst="python/pyautd3/bin/macos_universal"
  ext="dylib"
fi
mkdir -p $dst

cp build/bin/*.$ext $dst

popd
