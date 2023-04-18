# File: simulator.nim
# Project: autd3
# Created Date: 08/08/2022
# Author: Shun Suzuki
# -----
# Last Modified: 18/04/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
#



import native_methods/autd3capi_link_simulator
import link

type Simulator* = object of RootObj
    timeout_ns: uint64

proc initSimulator*(): Simulator =
    result.timeout_ns = 20 * 1000 * 1000


proc timeout*(cnt: var Simulator, timeout: uint64): var Simulator =
  cnt.timeout_ns = timeout
  result = cnt


func build*(cnt: Simulator): Link =
    AUTDLinkSimulator(result.p.addr, cnt.timeout_ns)
