# File: build.ps1
# Project: python
# Created Date: 28/05/2023
# Author: Shun Suzuki
# -----
# Last Modified: 28/05/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2023 Shun Suzuki. All rights reserved.
# 


pushd $PSScriptRoot

cd ..
cd capi
cargo build --release --all
cd ..
mkdir python/pyautd3/bin/win_x64 > NUL 2>&1
foreach($dll in Get-ChildItem -Path capi/target/release | Where {$_.extension -like ".dll"}){
    Copy-Item -Path $dll -Destination python/pyautd3/bin/win_x64
}
cd python
python setup.py sdist bdist_wheel True
popd
