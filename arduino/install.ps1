# File: install.ps1
# Project: arduino
# Created Date: 12/10/2022
# Author: Shun Suzuki
# -----
# Last Modified: 12/10/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

pushd $PSScriptRoot

cd ..

git submodule update --init -- 3rdparty/SOEM4Arduino

Compress-Archive -Path 3rdparty/SOEM4Arduino -DestinationPath $PSScriptRoot/SOEM4Arduino.zip

popd
