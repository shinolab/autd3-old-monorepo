# File: focus.jl
# Project: tests
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 14/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

function focus(cnt::Controller)
    config = SilencerConfig()
    cnt.send(config)

    g = Focus(SVector(90.0, 80.0, 150.0))
    m = Sine(150)

    cnt.send(m, g)
end
