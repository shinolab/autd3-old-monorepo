#!/bin/bash

CMDNAME=`basename $0`
script_dir="$( dirname -- "$( readlink -f -- "$0"; )"; )"

pushd $script_dir
cd ..
mkdir -p cpp/bin
cd capi

cargo build --all --release  --target=aarch64-unknown-linux-gnu --exclude autd3capi-backend-cuda
cd ..
cp ./capi/target/aarch64-unknown-linux-gnu/release/*.so cpp/bin

cp -f ./capi/ThirdPartyNotice.txt ./cpp/ThirdPartyNotice.txt

popd
