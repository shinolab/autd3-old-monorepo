# File: install.ps1
# Project: arduino
# Created Date: 12/10/2022
# Author: Shun Suzuki
# -----
# Last Modified: 22/01/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

pushd $PSScriptRoot

cd ..

Compress-Archive -Path arduino/AUTD3 -DestinationPath $PSScriptRoot/AUTD3.zip -Force

popd
