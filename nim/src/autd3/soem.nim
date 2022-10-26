# File: autd3.nim
# Project: src
# Created Date: 11/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 26/10/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
#


import strutils

import native_methods/autd3capi_link_soem
import link

type Callback* = proc(a: cstring)

type SOEM* = object of RootObj
  ifname: cstring
  sync0Cycle: uint16
  sendCycle: uint16
  highPrecision: bool
  onLost: Callback
  freerun: bool

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
  result.sync0Cycle = 2
  result.sendCycle = 2
  result.highPrecision = false
  result.onLost = nil
  result.freerun = false

proc ifname*(cnt: var SOEM, ifname: string): var SOEM =
  cnt.ifname = cast[cstring](ifname)
  result = cnt

proc sync0Cycle*(cnt: var SOEM, sync0Cycle: uint16): var SOEM =
  cnt.sync0Cycle = sync0Cycle
  result = cnt

proc sendCycle*(cnt: var SOEM, sync0Cycle: uint16): var SOEM =
  cnt.sync0Cycle = sync0Cycle
  result = cnt

proc highPrecision*(cnt: var SOEM, highPrecision: bool): var SOEM =
  cnt.highPrecision = highPrecision
  result = cnt

proc freerun*(cnt: var SOEM, freerun: bool): var SOEM =
  cnt.freerun = freerun
  result = cnt

proc onLost*(cnt: var SOEM, onLost: Callback): var SOEM =
  cnt.onLost = onLost
  result = cnt

func build*(cnt: SOEM): Link {.discardable.} =
  let p = rawProc(cnt.onLost)
  AUTDLinkSOEM(result.p.addr, cnt.ifname, cnt.sync0Cycle, cnt.sendCycle,
      cnt.freerun, p, cnt.highPrecision)
