# File: holo.jl
# Project: tests
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 08/03/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 


function holo(cnt::Controller)
    config = SilencerConfig()
    cnt.send(config; timeout_ns=UInt64(20 * 1000 * 1000))

    backend = BackendEigen()
    g = GSPAT(backend)
    g.add(SVector(120.0, 80.0, 150.0), 1.0)
    g.add(SVector(60.0, 80.0, 150.0), 1.0)
    m = Sine(150)

    cnt.send(m, g; timeout_ns=UInt64(20 * 1000 * 1000))
end
