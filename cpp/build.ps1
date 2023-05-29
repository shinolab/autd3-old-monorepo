# File: build.ps1
# Project: cpp
# Created Date: 28/05/2023
# Author: Shun Suzuki
# -----
# Last Modified: 29/05/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2023 Shun Suzuki. All rights reserved.
# 

pushd $PSScriptRoot

cd ..
cd capi
cargo build --release --all
cd ..
mkdir cpp/lib > NUL 2>&1
foreach($dll in Get-ChildItem -Path capi/target/release | Where {$_.FullName -match "^(?!.*\.dll).*\.lib$"}){
    Copy-Item -Path $dll -Destination cpp/lib
}
Copy-Item -Path ./src/autd3-link-soem/3rdparty/SOEM/oshw/win32/wpcap/Lib/x64  -Destination ./cpp/lib/wpcap -Recurse -Force
popd
