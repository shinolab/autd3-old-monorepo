# File: build.ps1
# Project: python
# Created Date: 28/05/2023
# Author: Shun Suzuki
# -----
# Last Modified: 09/09/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2023 Shun Suzuki. All rights reserved.
# 


pushd $PSScriptRoot

cd ..
cd capi
cargo build --release --all
cd ..
mkdir python/pyautd3/bin > NUL 2>&1
foreach($dll in Get-ChildItem -Path capi/target/release | Where {$_.extension -like ".dll"}){
    Copy-Item -Path $dll -Destination python/pyautd3/bin
}
cd python

Copy-Item -Path setup.cfg.win-amd64 -Destination setup.cfg -Force
python -m build -w

popd
