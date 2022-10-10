# File: bessel.nim
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

proc bessel_test*(cnt: Controller) =
    let config = initSilencerConfig()
    cnt.send(config)

    let f = initBesselBeam([90.0, 80.0, 0.0], [0.0, 0.0, 1.0], 13.0 / 180.0 * PI)
    let m = initSine(150)

    cnt.send(m, f)
