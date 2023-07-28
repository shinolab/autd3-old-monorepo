#!/bin/bash

CMDNAME=`basename $0`
script_dir="$( dirname -- "$( readlink -f -- "$0"; )"; )"

pushd $script_dir

cd ../..
cd capi
cargo build --release --all --features "single_float left_handed use_meter"

cd ..
for binfile in `ls ./capi/target/release/*.so`; do
  cp -f $binfile ./dotnet/unity-linux/Assets/autd3/Plugins/x86_64/
done

for cssrc in `ls ./dotnet/cs/src/*.cs`; do
  cp -f $cssrc ./dotnet/unity-linux/Assets/autd3/Scripts
done

cp -f LICENSE ./dotnet/unity-linux/Assets/autd3/LICENSE.md
echo "" >> ./dotnet/unity-linux/Assets/autd3/LICENSE.md
echo "=========================================================" >> ./dotnet/unity-linux/Assets/autd3/LICENSE.md
echo "" >> ./dotnet/unity-linux/Assets/autd3/LICENSE.md
cat ./capi/ThirdPartyNotice.txt >> ./dotnet/unity-linux/Assets/autd3/LICENSE.md
cp -f CHANGELOG.md ./dotnet/unity-linux/Assets/autd3/CHANGELOG.md

popd
