# File: bessel.jl
# Project: tests
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 18/04/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

function bessel(cnt::Controller)
    config = SilencerConfig()
    cnt.send(config)

    g = BesselBeam(SVector(90.0, 80.0, 150.0), SVector(0.0, 0.0, 1.0), 13.0 / 180.0 * pi)
    m = Sine(150)

    cnt.send(m, g)
end
