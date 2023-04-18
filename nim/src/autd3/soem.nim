# File: autd3.nim
# Project: src
# Created Date: 11/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 18/04/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
#


import strutils

import native_methods/autd3capi_link_soem
import link

type Callback* = proc(a: cstring)
type LogOut* = proc(a: cstring)
type LogFlush* = proc()


type SyncMode* {.pure.} = enum
    DC = 0
    FreeRun = 1

 
type TimerStrategy* {.pure.} = enum
    Sleep = 0
    BusyWait = 1
    NativeTimer = 2


type SOEM* = object of RootObj
  soem: pointer
  # ifname: cstring
  # bufSize: uint64
  # sync0Cycle: uint16
  # sendCycle: uint16
  # timerStrategy: TimerStrategy
  # onLost: Callback
  # syncMode: SyncMode
  # check_interval: uint64
  # debug_level: int32
  # debug_log_out: LogOut
  # debug_log_flush: LogFlush

type Adapter* = object
  name*: string
  desc*: string

func initAdapter(name: string, desc: string): Adapter =
  result.name = name
  result.desc = desc

proc enumerateAdapters*(_: typedesc[SOEM]): seq[Adapter] =
  var p = pointer(nil)
  let n = AUTDGetAdapterPointer(p.addr)
  var adapters: seq[Adapter] = @[]
  for i in 0..<n:
    var desc = cast[cstring]('\0'.repeat(128))
    var name = cast[cstring]('\0'.repeat(128))
    AUTDGetAdapter(p, i, desc, name)
    adapters.add(initAdapter($name, $desc))
  AUTDFreeAdapterPointer(p)
  adapters

func initSOEM*(): SOEM =
  AUTDLinkSOEM(result.soem.addr)

proc ifname*(cnt: var SOEM, ifname: string): var SOEM =
  AUTDLinkSOEMIfname(cnt.soem, cast[cstring](ifname))
  result = cnt

proc bufSize*(cnt: var SOEM, size: uint64): var SOEM =
  AUTDLinkSOEMBufSize(cnt.soem, size)
  result = cnt

proc sync0Cycle*(cnt: var SOEM, sync0Cycle: uint16): var SOEM =
  AUTDLinkSOEMSync0Cycle(cnt.soem, sync0Cycle)
  result = cnt

proc sendCycle*(cnt: var SOEM, sendCycle: uint16): var SOEM =
  AUTDLinkSOEMSendCycle(cnt.soem, sendCycle)
  result = cnt

proc timerStrategy*(cnt: var SOEM, strategy: TimerStrategy): var SOEM =
  AUTDLinkSOEMTimerStrategy(cnt.soem, cast[uint8](strategy))
  result = cnt

proc syncMode*(cnt: var SOEM, mode: SyncMode): var SOEM =
  AUTDLinkSOEMFreerun(cnt.soem, mode == SyncMode.FreeRun)
  result = cnt

proc onLost*(cnt: var SOEM, onLost: Callback): var SOEM =
  let p = rawProc(onLost)
  AUTDLinkSOEMOnLost(cnt.soem, p)
  result = cnt

proc stateCheckInterval*(cnt: var SOEM, intervalMs: uint64): var SOEM =
  AUTDLinkSOEMStateCheckInterval(cnt.soem, intervalMs)
  result = cnt

proc debugLevel*(cnt: var SOEM, level: int32): var SOEM =
  AUTDLinkSOEMLogLevel(cnt.soem, level)
  result = cnt
  
proc debugLogFunc*(cnt: var SOEM, logOut: LogOut, logFlush: LogFlush): var SOEM =
  let pLogOut = rawProc(logOut)
  let pLogFlush = rawProc(logFlush)
  AUTDLinkSOEMLogFunc(cnt.soem, pLogOut, pLogFlush)
  result = cnt

proc timeout*(cnt: var SOEM, timeout: uint64): var SOEM =
  AUTDLinkSOEMTimeout(cnt.soem, timeout)
  result = cnt

func build*(cnt: SOEM): Link =
  AUTDLinkSOEMBuild(result.p.addr, cnt.soem)
  AUTDLinkSOEMDelete(cnt.soem)
