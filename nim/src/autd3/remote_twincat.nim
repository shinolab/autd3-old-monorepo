# File: emulator.nim
# Project: autd3
# Created Date: 13/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 14/08/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
#


import native_methods/autd3capi_link_remote_twincat
import link

type RemoteTwinCAT* = object of RootObj
  remoteIp: string
  remoteAmsNetId: string
  localAmsNetId: string

func initRemoteTwinCAT*(remoteIP: string, remoteAmsNetId: string,
    localAmsNetId: string): RemoteTwinCAT =
  result.remoteIp = remoteIP
  result.remoteAmsNetId = remoteAmsNetId
  result.localAmsNetId = ""

func localAmsNetId*(cnt: var RemoteTwinCAT,
    localAmsNetId: string): RemoteTwinCAT =
  cnt.localAmsNetId = localAmsNetId
  result = cnt

func build*(cnt: RemoteTwinCAT): Link =
  AUTDLinkRemoteTwinCAT(result.p.addr, cast[cstring](cnt.remoteIP), cast[
      cstring](cnt.remoteAmsNetId), cast[cstring](cnt.localAmsNetId))
