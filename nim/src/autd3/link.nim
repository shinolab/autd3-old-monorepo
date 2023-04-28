# File: link.nim
# Project: autd3
# Created Date: 12/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 28/04/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
#

type Link* = object of RootObj
  p*: pointer


type LogOut* = proc(a: cstring)
type LogFlush* = proc()
