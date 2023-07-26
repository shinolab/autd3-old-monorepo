# File: build.ps1
# Project: cs
# Created Date: 28/05/2023
# Author: Shun Suzuki
# -----
# Last Modified: 27/07/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2023 Shun Suzuki. All rights reserved.
# 

pushd $PSScriptRoot

cd ../..
cd capi
cargo build --release --all --features "single_float left_handed use_meter"
cd ..
foreach($dll in Get-ChildItem -Path capi/target/release | Where {$_.extension -like ".dll"}){
    Copy-Item -Path $dll -Destination dotnet/unity/Assets/autd3/Plugins/x86_64
}
foreach($src in Get-ChildItem -Path ./dotnet/cs/src | Where {$_.extension -like ".cs"}){
    Copy-Item -Path $src -Destination ./dotnet/unity/Assets/autd3/Scripts
}
Copy-Item -Path LICENSE -Destination ./dotnet/unity/Assets/autd3/LICENSE.txt
Add-Content -Path ./dotnet/unity/Assets/autd3/LICENSE.txt -Value ""
Add-Content -Path ./dotnet/unity/Assets/autd3/LICENSE.txt -Value "========================================================="
Add-Content -Path ./dotnet/unity/Assets/autd3/LICENSE.txt -Value ""
Get-Content -Path ./capi/ThirdPartyNotice.txt | Add-Content -Path ./dotnet/unity/Assets/autd3/LICENSE.txt
Copy-Item -Path CHANGELOG.md -Destination ./dotnet/unity/Assets/autd3/CHANGELOG.md

cd server/simulator
cargo build --release --features "left_handed use_meter"
cd ../..
Copy-Item -Path server/simulator/target/release/simulator.exe -Destination ./dotnet/unity/Assets/autd3/Editor/autd_simulator.exe

Add-Content -Path ./dotnet/unity/Assets/autd3/LICENSE.txt -Value ""
Add-Content -Path ./dotnet/unity/Assets/autd3/LICENSE.txt -Value "========================================================="
Add-Content -Path ./dotnet/unity/Assets/autd3/LICENSE.txt -Value ""
Add-Content -Path ./dotnet/unity/Assets/autd3/LICENSE.txt -Value "AUTD SIMULATOR " -NoNewline
Get-Content -Path ./server/simulator/AUTD-Simulator-ThirdPartyNotice.txt | Add-Content -Path ./dotnet/unity/Assets/autd3/LICENSE.txt

popd
