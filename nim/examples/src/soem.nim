# File: soem.nim
# Project: src
# Created Date: 11/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 02/02/2023
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
        var geometry = initGeometryBuilder().addDevice([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]).build()

        var soem = initSOEM()
        let link = soem.onLost(onLost).build()
        
        var autd = openController(geometry, link)
        
        run(autd)

    except:
        let
            e = getCurrentException()
            msg = getCurrentExceptionMsg()
        echo "Got exception ", repr(e), " with message ", msg
