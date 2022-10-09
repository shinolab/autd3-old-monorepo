# File: build.ps1
# Project: python
# Created Date: 09/10/2022
# Author: Shun Suzuki
# -----
# Last Modified: 09/10/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

pushd $PSScriptRoot

cd ..
mkdir build > NUL 2>&1
cd build
cmake .. -DBUILD_ALL=ON
cmake --build . --config Release --parallel 8
cd ..
mkdir python/pyautd3/bin/win_x64 > NUL 2>&1
foreach($dll in Get-ChildItem -Path build/bin/Release | Where {$_.extension -like ".dll"}){
    Copy-Item -Path $dll -Destination python/pyautd3/bin/win_x64
}
popd
