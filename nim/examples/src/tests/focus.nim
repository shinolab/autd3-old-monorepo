# File: focus.nim
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

proc focus_test*(cnt: Controller) =
    let config = initSilencerConfig()
    cnt.send(config)

    let f = initFocus([90.0, 80.0, 150.0])
    let m = initSine(150)

    cnt.send(m, f)
