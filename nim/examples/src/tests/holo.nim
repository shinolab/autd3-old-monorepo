# File: holo.nim
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
import autd3/holo

proc holo_test*(cnt: Controller) =
    let config = initSilencerConfig()
    cnt.send(config)

    let backend = initBackendEigen()

    let g = initGSPAT(backend)
    g.add([120.0, 80.0, 150.0], 1.0)
    g.add([60.0, 80.0, 150.0], 1.0)

    let m = initSine(150)

    cnt.send(m, g)
