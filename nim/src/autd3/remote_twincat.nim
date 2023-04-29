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


import native_methods/autd3capi_link_remote_twincat
import link

type RemoteTwinCAT* = object of RootObj
  builder: pointer

func initRemoteTwinCAT*(remoteIP: string, serverAmsNetId: string): RemoteTwinCAT =
  AUTDLinkRemoteTwinCAT(result.builder.addr, cast[cstring](serverAmsNetId))

func serverIP*(cnt: var RemoteTwinCAT,
    serverIp: string): RemoteTwinCAT =
  AUTDLinkRemoteTwinCATServerIpAddr(result.builder.addr, cast[cstring](serverIp))
  result = cnt

func clientAmsNetId*(cnt: var RemoteTwinCAT,
    clientAmsNetId: string): RemoteTwinCAT =
  AUTDLinkRemoteTwinCATClientAmsNetId(result.builder.addr, cast[cstring](clientAmsNetId))
  result = cnt

proc logLevel*(cnt: var RemoteTwinCAT, level: int32): var RemoteTwinCAT =
  AUTDLinkRemoteTwinCATLogLevel(cnt.builder, level)
  result = cnt
  
proc logFunc*(cnt: var RemoteTwinCAT, logOut: LogOut, logFlush: LogFlush): var RemoteTwinCAT =
  let pLogOut = rawProc(logOut)
  let pLogFlush = rawProc(logFlush)
  AUTDLinkRemoteTwinCATLogFunc(cnt.builder, pLogOut, pLogFlush)
  result = cnt

proc timeout*(cnt: var RemoteTwinCAT, timeout: uint64): var RemoteTwinCAT =
  AUTDLinkRemoteTwinCATTimeout(cnt.builder, timeout)
  result = cnt

func build*(cnt: RemoteTwinCAT): Link =
  AUTDLinkRemoteTwinCATBuild(result.p.addr, cnt.builder)
