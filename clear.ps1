# File: clear.ps1
# Project: autd3-library-firmware-fpga
# Created Date: 19/07/2021
# Author: Shun Suzuki
# -----
# Last Modified: 17/03/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2021 Hapis Lab. All rights reserved.
# 

Remove-Item *.jou
Remove-Item *.log
Remove-Item *.zip
Remove-Item *.prm
Remove-Item *.str
Remove-Item *.pb
Remove-Item *.xpr
Remove-Item .\autd3-fpga.ip_user_files -Recurse -ErrorAction SilentlyContinue
Remove-Item .\autd3-fpga.cache -Recurse -ErrorAction SilentlyContinue
Remove-Item .\autd3-fpga.gen -Recurse -ErrorAction SilentlyContinue
Remove-Item .\autd3-fpga.hw -Recurse -ErrorAction SilentlyContinue
Remove-Item .\autd3-fpga.runs -Recurse -ErrorAction SilentlyContinue
Remove-Item .\autd3-fpga.sim -Recurse -ErrorAction SilentlyContinue
Remove-Item .\autd3-fpga.srcs -Recurse -ErrorAction SilentlyContinue
Remove-Item .\.Xil -Recurse -ErrorAction SilentlyContinue
