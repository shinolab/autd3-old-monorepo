# File: point_stm.nim
# Project: tests
# Created Date: 13/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 13/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
#


import autd3
import math

proc point_stm_test*(cnt: Controller) =
    let config = SilencerConfig.none()
    cnt.send(config)

    let m = initStatic(1.0)

    let stm = initPointSTM()
    let radius = 30.0
    let size = 200
    let x = 90.0
    let y = 80.0
    let z = 150.0
    for i in 0..<size:
        let theta = 2.0 * PI * cast[float64](i) / cast[float64](size)
        let p = [x + radius * theta.cos, y + radius * theta.sin, z]
        stm.add(p)

    stm.frequency = 1.0

    cnt.send(m, stm)
