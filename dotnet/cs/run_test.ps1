# File: run_test.ps1
# Project: cs
# Created Date: 21/09/2023
# Author: Shun Suzuki
# -----
# Last Modified: 25/09/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2023 Shun Suzuki. All rights reserved.
# 

pushd $PSScriptRoot

cd ../..
cd capi
cargo build
cd ..
foreach($dll in Get-ChildItem -Path capi/target/debug | Where {$_.extension -like ".dll"}){
    Copy-Item -Path $dll -Destination dotnet/cs/tests -Force
}
foreach($dll in Get-ChildItem -Path capi/target/debug | Where {$_.extension -like ".pdb"}){
    Copy-Item -Path $dll -Destination dotnet/cs/tests -Force
}

cd dotnet/cs/tests

dotnet test

popd
