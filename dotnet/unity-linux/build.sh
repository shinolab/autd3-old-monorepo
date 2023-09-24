#!/bin/bash

CMDNAME=`basename $0`
script_dir="$( dirname -- "$( readlink -f -- "$0"; )"; )"

pushd $script_dir

cd ../..

cp -rf ./dotnet/unity/Assets/Models ./dotnet/unity-linux/Assets/
cp -rf ./dotnet/unity/Assets/Samples ./dotnet/unity-linux/Assets/
cp -rf ./dotnet/unity/Assets/Scripts ./dotnet/unity-linux/Assets/

sourceDirectory="./dotnet/cs/src"
destinationDirectory="./dotnet/unity-linux/Assets/Scripts"
find "$sourceDirectory" -type f -name "*.cs" -print | while IFS= read -r sourceFilePath; do
  relativePath="${sourceFilePath#$sourceDirectory}"
  destinationFilePath="$destinationDirectory$relativePath"
  destinationFileDirectory=$(dirname "$destinationFilePath")
  mkdir -p "$destinationFileDirectory"
  if [[ "$sourceFilePath" != *NativeMethods* ]];then
    cp "$sourceFilePath" "$destinationFilePath"
  fi
done
rm -rf ./dotnet/unity-linux/Assets/Scripts/Utils
rm -rf ./dotnet/unity-linux/Assets/Scripts/obj

cd capi
cargo build --release --all --features "single_float use_meter"
cd ..
for binfile in `ls ./capi/target/release/*.so`; do
  cp -f $binfile ./dotnet/unity-linux/Assets/Plugins/x86_64/
done

cp -f LICENSE ./dotnet/unity-linux/Assets/LICENSE.md
echo "" >> ./dotnet/unity-linux/Assets/LICENSE.md
echo "=========================================================" >> ./dotnet/unity-linux/Assets/LICENSE.md
echo "" >> ./dotnet/unity-linux/Assets/LICENSE.md
cat ./capi/ThirdPartyNotice.txt >> ./dotnet/unity-linux/Assets/LICENSE.md
cp -f CHANGELOG.md ./dotnet/unity-linux/Assets/CHANGELOG.md

popd
