# File: simulator.nim
# Project: autd3
# Created Date: 08/08/2022
# Author: Shun Suzuki
# -----
# Last Modified: 10/10/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
#



import native_methods/autd3capi_link_simulator
import link

type Simulator* = object of RootObj
  port: uint16
  ipAddr: cstring

proc initSimulator*(): Simulator =
  result.port = 50632
  result.ipAddr = cast[cstring](pointer(nil))


func port*(cnt: var Simulator, port: uint16): Simulator =
  cnt.port = port
  result = cnt

func build*(cnt: Simulator): Link =
  AUTDLinkSimulator(result.p.addr, cnt.port, cnt.ipAddr)
