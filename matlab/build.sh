#!/bin/bash

CMDNAME=`basename $0`
script_dir="$( dirname -- "$( readlink -f -- "$0"; )"; )"

pushd $script_dir
cd ..
mkdir -p build
cd build
cmake .. -DBUILD_ALL=ON -DCMAKE_BUILD_TYPE=Release
cmake --build . --parallel 8
cd ..
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  dst="matlab/bin/linux-x64"
  ext="so"
elif [[ "$OSTYPE" == "darwin"* ]]; then
  dst="matlab/bin/macos-universal"
  ext="dylib"
fi
mkdir -p $dst
cp build/bin/*.$ext $dst

pushd $dst
for i in *; do mv $i $(echo $i | sed 's/-/_/g'); done  
popd

popd
