#!/bin/bash

CMDNAME=`basename $0`
script_dir="$( dirname -- "$( readlink -f -- "$0"; )"; )"

pushd $script_dir

cd ../..

cp -rf ./dotnet/unity/Assets/Models ./dotnet/unity-mac/Assets/
cp -rf ./dotnet/unity/Assets/Samples ./dotnet/unity-mac/Assets/
cp -rf ./dotnet/unity/Assets/Scripts ./dotnet/unity-mac/Assets/

sourceDirectory="./dotnet/cs/src"
destinationDirectory="./dotnet/unity-mac/Assets/Scripts"
find "$sourceDirectory" -type f -name "*.cs" -print | while IFS= read -r sourceFilePath; do
  relativePath="${sourceFilePath#$sourceDirectory}"
  destinationFilePath="$destinationDirectory$relativePath"
  destinationFileDirectory=$(dirname "$destinationFilePath")
  mkdir -p "$destinationFileDirectory"
  if [[ "$sourceFilePath" != *NativeMethods* ]];then
    cp "$sourceFilePath" "$destinationFilePath"
  fi
done
rm -rf ./dotnet/unity-mac/Assets/Scripts/Utils
rm -rf ./dotnet/unity-mac/Assets/Scripts/obj
rm -rf ./dotnet/unity-mac/Assets/Scripts/Gain/BackendCUDA.cs
rm -rf ./dotnet/unity-mac/Assets/Scripts/Gain/BackendCUDA.cs.meta

cd capi
cargo build --release --all --exclude autd3capi-backend-cuda --features "single_float use_meter" --target=x86_64-apple-darwin
cargo build --release --all --exclude autd3capi-backend-cuda --features "single_float use_meter" --target=aarch64-apple-darwin
cd ..
for binfile in `ls ./capi/target/x86_64-apple-darwin/release/*.dylib`; do
  cp -f $binfile ./dotnet/unity-mac/Assets/Plugins/x86_64/
done
for binfile in `ls ./capi/target/aarch64-apple-darwin/release/*.dylib`; do
  cp -f $binfile ./dotnet/unity-mac/Assets/Plugins/aarch64/
done

cp -f LICENSE ./dotnet/unity-mac/Assets/LICENSE.md
echo "" >> ./dotnet/unity-mac/Assets/LICENSE.md
echo "=========================================================" >> ./dotnet/unity-mac/Assets/LICENSE.md
echo "" >> ./dotnet/unity-mac/Assets/LICENSE.md
cat ./capi/ThirdPartyNotice.txt >> ./dotnet/unity-mac/Assets/LICENSE.md
cp -f CHANGELOG.md ./dotnet/unity-mac/Assets/CHANGELOG.md

popd
