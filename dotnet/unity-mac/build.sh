#!/bin/bash

CMDNAME=`basename $0`
script_dir="$( dirname -- "$( readlink -f -- "$0"; )"; )"

pushd $script_dir

cd ../..
cd capi
cargo build --release --all --exclude autd3capi-backend-cuda --features "single_float left_handed use_meter" --target=x86_64-apple-darwin
cargo build --release --all --exclude autd3capi-backend-cuda --features "single_float left_handed use_meter" --target=aarch64-apple-darwin

cd ..
for binfile in `ls ./capi/target/x86_64-apple-darwin/release/*.dylib`; do
  cp -f $binfile ./dotnet/unity-mac/Assets/autd3/Plugins/x86_64/
done
for binfile in `ls ./capi/target/aarch64-apple-darwin/release/*.dylib`; do
  cp -f $binfile ./dotnet/unity-mac/Assets/autd3/Plugins/aarch64/
done

for cssrc in `ls ./dotnet/cs/src/*.cs`; do
  if [[ "$cssrc" != *CUDA* ]];then
      cp -f $cssrc ./dotnet/unity-mac/Assets/autd3/Scripts
  fi
done

cp -f LICENSE ./dotnet/unity-mac/Assets/autd3/LICENSE.md
echo "" >> ./dotnet/unity-mac/Assets/autd3/LICENSE.md
echo "=========================================================" >> ./dotnet/unity-mac/Assets/autd3/LICENSE.md
echo "" >> ./dotnet/unity-mac/Assets/autd3/LICENSE.md
cat ./capi/ThirdPartyNotice.txt >> ./dotnet/unity-mac/Assets/autd3/LICENSE.md
cp -f CHANGELOG.md ./dotnet/unity-mac/Assets/autd3/CHANGELOG.md

popd
