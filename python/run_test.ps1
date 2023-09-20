# File: run_test.ps1
# Project: python
# Created Date: 20/09/2023
# Author: Shun Suzuki
# -----
# Last Modified: 20/09/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2023 Shun Suzuki. All rights reserved.
# 


pushd $PSScriptRoot

cd ..
cd capi
cargo build
cd ..
mkdir python/pyautd3/bin > NUL 2>&1
foreach($dll in Get-ChildItem -Path capi/target/debug | Where {$_.extension -like ".dll"}){
    Copy-Item -Path $dll -Destination python/pyautd3/bin
}
cd python
python3 -m pytest

popd
