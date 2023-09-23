#!/bin/bash

CMDNAME=`basename $0`
script_dir="$( dirname -- "$( readlink -f -- "$0"; )"; )"

pushd $script_dir
cd ..
mkdir -p cpp/bin
cd capi

cargo build --all --release  --target=armv7-unknown-linux-gnueabihf --exclude autd3capi-backend-cuda
cd ..
cp ./capi/target/armv7-unknown-linux-gnueabihf/release/*.so cpp/bin

cp -f ./capi/ThirdPartyNotice.txt ./cpp/ThirdPartyNotice.txt

popd
