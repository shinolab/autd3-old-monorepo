# File: build.ps1
# Project: cs
# Created Date: 28/05/2023
# Author: Shun Suzuki
# -----
# Last Modified: 20/08/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2023 Shun Suzuki. All rights reserved.
# 

pushd $PSScriptRoot

cd ../..

$sourceDirectory = Join-Path -Path $PWD -ChildPath "dotnet/cs/src"
$destinationDirectory = "./dotnet/unity/Assets/autd3/Scripts"
Get-ChildItem -Path $sourceDirectory -Recurse -File -Filter "*.cs" -Depth 1 | ForEach-Object {
    if ($_.Directory.Name -eq "NativeMethods") {
        return
    }
    $sourceFilePath = $_.FullName
    $relativePath = $sourceFilePath.Replace($sourceDirectory, "")
    $destinationFilePath = Join-Path -Path $destinationDirectory -ChildPath $relativePath
    $destinationFileDirectory = [System.IO.Path]::GetDirectoryName($destinationFilePath)
    if (-not (Test-Path -Path $destinationFileDirectory)) {
        New-Item -Path $destinationFileDirectory -ItemType Directory -Force
    }
    Copy-Item -Force -Path $sourceFilePath -Destination $destinationFilePath
}
Remove-Item -Force -Recurse -Path "./dotnet/unity/Assets/autd3/Scripts/Utils"

cd capi
cargo build --release --all --features "single_float left_handed use_meter"
cd ..
foreach($dll in Get-ChildItem -Path capi/target/release | Where {$_.extension -like ".dll"}){
    Copy-Item -Path $dll -Destination dotnet/unity/Assets/autd3/Plugins/x86_64
}

Copy-Item -Path LICENSE -Destination ./dotnet/unity/Assets/autd3/LICENSE.md
Add-Content -Path ./dotnet/unity/Assets/autd3/LICENSE.md -Value ""
Add-Content -Path ./dotnet/unity/Assets/autd3/LICENSE.md -Value "========================================================="
Add-Content -Path ./dotnet/unity/Assets/autd3/LICENSE.md -Value ""
Get-Content -Path ./capi/ThirdPartyNotice.txt | Add-Content -Path ./dotnet/unity/Assets/autd3/LICENSE.md
Copy-Item -Path CHANGELOG.md -Destination ./dotnet/unity/Assets/autd3/CHANGELOG.md

cd server/simulator
cargo build --release --features "left_handed use_meter"
cd ../..
Copy-Item -Path server/simulator/target/release/simulator.exe -Destination ./dotnet/unity/Assets/autd3/Editor/autd_simulator.exe

Add-Content -Path ./dotnet/unity/Assets/autd3/LICENSE.md -Value ""
Add-Content -Path ./dotnet/unity/Assets/autd3/LICENSE.md -Value "========================================================="
Add-Content -Path ./dotnet/unity/Assets/autd3/LICENSE.md -Value ""
Add-Content -Path ./dotnet/unity/Assets/autd3/LICENSE.md -Value "AUTD SIMULATOR " -NoNewline
Get-Content -Path ./server/simulator/ThirdPartyNotice.txt | Add-Content -Path ./dotnet/unity/Assets/autd3/LICENSE.md

popd
