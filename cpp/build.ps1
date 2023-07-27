# File: build.ps1
# Project: cpp
# Created Date: 28/05/2023
# Author: Shun Suzuki
# -----
# Last Modified: 27/07/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2023 Shun Suzuki. All rights reserved.
# 

Param(
  [switch]$debug
)

pushd $PSScriptRoot

cd ..
cd capi
$target_dir = ""
If($debug){
  cargo build --all
  $target_dir = "debug"
} Else{
  cargo build --release --all
  $target_dir = "release"
}
cd ..
mkdir cpp/lib > NUL 2>&1
mkdir cpp/bin > NUL 2>&1
foreach($dll in Get-ChildItem -Path capi/target/$target_dir | Where {$_.FullName -match ".*dll\.lib$"}){
    Copy-Item -Path $dll -Destination cpp/lib
}
foreach($dll in Get-ChildItem -Path capi/target/$target_dir | Where {$_.FullName -match ".*\.dll$"}){
    Copy-Item -Path $dll -Destination cpp/bin
}
Copy-Item -Path capi/ThirdPartyNotice.txt -Destination cpp/ThirdPartyNotice.txt -Force
popd
