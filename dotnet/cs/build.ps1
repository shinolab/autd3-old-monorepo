# File: build.ps1
# Project: cs
# Created Date: 28/05/2023
# Author: Shun Suzuki
# -----
# Last Modified: 11/10/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2023 Shun Suzuki. All rights reserved.
# 

pushd $PSScriptRoot

cd ../..
cd capi
cargo build --release --all
cd ..
mkdir dotnet/cs/src/native/windows/x64 > NUL 2>&1
foreach($dll in Get-ChildItem -Path capi/target/release | Where {$_.extension -like ".dll"}){
    Copy-Item -Path $dll -Destination dotnet/cs/src/native/windows/x64
    Copy-Item -Path $dll -Destination dotnet/cs/tests
}
Copy-Item -Path LICENSE -Destination dotnet/cs/src/LICENSE.txt -Force

popd
