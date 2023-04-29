# File: emulator.nim
# Project: autd3
# Created Date: 13/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 28/04/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
#


import native_methods/autd3capi_link_twincat
import link

type TwinCAT* = object of RootObj
  builder: pointer

proc initTwinCAT*(): TwinCAT =
  AUTDLinkTwinCAT(result.builder.addr)

proc logLevel*(cnt: var TwinCAT, level: int32): var TwinCAT =
  AUTDLinkTwinCATLogLevel(cnt.builder, level)
  result = cnt
  
proc logFunc*(cnt: var TwinCAT, logOut: LogOut, logFlush: LogFlush): var TwinCAT =
  let pLogOut = rawProc(logOut)
  let pLogFlush = rawProc(logFlush)
  AUTDLinkTwinCATLogFunc(cnt.builder, pLogOut, pLogFlush)
  result = cnt

proc timeout*(cnt: var TwinCAT, timeout: uint64): var TwinCAT =
  AUTDLinkTwinCATTimeout(cnt.builder, timeout)
  result = cnt

func build*(cnt: TwinCAT): Link =
  AUTDLinkTwinCATBuild(result.p.addr, cnt.builder)
