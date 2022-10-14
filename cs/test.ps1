# File: test.ps1
# Project: example
# Created Date: 13/10/2022
# Author: Shun Suzuki
# -----
# Last Modified: 13/10/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

pushd $PSScriptRoot

cd ..
mkdir build > NUL 2>&1
cd build
cmake .. -DBUILD_ALL=ON -DBUILD_SIMULATOR=ON -DBUILD_GEOMETRY_VIEWER=ON
cmake --build . --config Release --parallel 8
cd ..
foreach($dll in Get-ChildItem -Path build/bin/Release | Where {$_.extension -like ".dll"}){
    Copy-Item -Path $dll -Destination cs/src/native/windows/x64
}

cd cs/src
Remove-Item -Path bin -Recurse -Force
dotnet clean
dotnet build -c:Release /p:Platform=x64
cd ..
dotnet nuget add source $PSScriptRoot/src/bin/x64/Release -n autd3shapr_local

cd example
dotnet clean example.sln
dotnet nuget locals global-packages --clear
dotnet build example.sln -c:Release /p:Platform=x64

popd
