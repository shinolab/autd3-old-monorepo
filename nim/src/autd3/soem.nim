# File: autd3.nim
# Project: src
# Created Date: 11/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 20/03/2023
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
  ifname: cstring
  bufSize: uint64
  sync0Cycle: uint16
  sendCycle: uint16
  timerStrategy: TimerStrategy
  onLost: Callback
  syncMode: SyncMode
  check_interval: uint64
  debug_level: int32
  debug_log_out: LogOut
  debug_log_flush: LogFlush

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
  result.ifname = cast[cstring](pointer(nil))
  result.bufSize = 0
  result.sync0Cycle = 2
  result.sendCycle = 2
  result.timerStrategy = TimerStrategy.Sleep
  result.onLost = nil
  result.syncMode = SyncMode.FreeRun
  result.checkInterval = 500
  result.debugLevel = 2
  result.debugLogOut = nil
  result.debugLogFlush = nil

proc ifname*(cnt: var SOEM, ifname: string): var SOEM =
  cnt.ifname = cast[cstring](ifname)
  result = cnt

proc sync0Cycle*(cnt: var SOEM, sync0Cycle: uint16): var SOEM =
  cnt.sync0Cycle = sync0Cycle
  result = cnt

proc sendCycle*(cnt: var SOEM, sync0Cycle: uint16): var SOEM =
  cnt.sync0Cycle = sync0Cycle
  result = cnt

proc timerStrategy*(cnt: var SOEM, strategy: TimerStrategy): var SOEM =
  cnt.timerStrategy = strategy
  result = cnt

proc syncMode*(cnt: var SOEM, mode: SyncMode): var SOEM =
  cnt.syncMode = mode
  result = cnt

proc onLost*(cnt: var SOEM, onLost: Callback): var SOEM =
  cnt.onLost = onLost
  result = cnt

proc checkInterval*(cnt: var SOEM, interval: uint64): var SOEM =
  cnt.checkInterval = interval
  result = cnt

proc debugLevel*(cnt: var SOEM, level: int32): var SOEM =
  cnt.debugLevel = level
  result = cnt
  
proc debugLogFunc*(cnt: var SOEM, logOut: LogOut, logFlush: LogFlush): var SOEM =
  cnt.debugLogOut = logOut
  cnt.debugLogFlush = logFlush
  result = cnt

func build*(cnt: SOEM): Link {.discardable.} =
  let p = rawProc(cnt.onLost)
  let pLogOut = rawProc(cnt.debugLogOut)
  let pLogFlush = rawProc(cnt.debugLogFlush)
  AUTDLinkSOEM(result.p.addr, cnt.ifname, cnt.bufSize, cnt.sync0Cycle, cnt.sendCycle,
      cnt.syncMode == SyncMode.FreeRun, p, cast[uint8](cnt.timerStrategy), cnt.checkInterval, cnt.debugLevel, pLogOut, pLogFlush)
