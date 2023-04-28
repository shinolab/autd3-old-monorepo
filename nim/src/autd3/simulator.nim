# File: simulator.nim
# Project: autd3
# Created Date: 08/08/2022
# Author: Shun Suzuki
# -----
# Last Modified: 28/04/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
#



import native_methods/autd3capi_link_simulator
import link

type Simulator* = object of RootObj
  builder: pointer

proc initSimulator*(): Simulator =
  AUTDLinkSimulator(result.builder.addr)

proc logLevel*(cnt: var Simulator, level: int32): var Simulator =
  AUTDLinkSimulatorLogLevel(cnt.builder, level)
  result = cnt
  
proc logFunc*(cnt: var Simulator, logOut: LogOut, logFlush: LogFlush): var Simulator =
  let pLogOut = rawProc(logOut)
  let pLogFlush = rawProc(logFlush)
  AUTDLinkSimulatorLogFunc(cnt.builder, pLogOut, pLogFlush)
  result = cnt

proc timeout*(cnt: var Simulator, timeout: uint64): var Simulator =
  AUTDLinkSimulatorTimeout(cnt.builder, timeout)
  result = cnt

func build*(cnt: Simulator): Link =
  AUTDLinkSimulatorBuild(result.p.addr, cnt.builder)
