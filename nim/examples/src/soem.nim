# File: soem.nim
# Project: src
# Created Date: 11/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 20/11/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
#

import autd3
import autd3/soem

import tests/runner

proc onLost(msg: cstring) =
    echo msg
    quit(-1)

when isMainModule:
    try:
        var cnt = initController()
        cnt.addDevice([0.0, 0.0, 0.0], [0.0, 0.0, 0.0])

        var soem = initSOEM()
        let link = soem.highPrecision(true).onLost(onLost).build()
        if not cnt.open(link):
            echo "Failed to open Controller."
            quit(-1)

        cnt.ackCheckTimeoutMs = 20

        run(cnt)

    except:
        let
            e = getCurrentException()
            msg = getCurrentExceptionMsg()
        echo "Got exception ", repr(e), " with message ", msg
