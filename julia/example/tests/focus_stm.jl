# File: focus_stm.jl
# Project: tests
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 31/12/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

function focus_stm(cnt::Controller)
    config = SilencerConfigNone()
    cnt.send(config)

    stm = FocusSTM()
    center = SVector(90.0, 80.0, 150.0)
    radius = 30.0
    size = 200
    for i = 1:size
        theta = 2.0 * pi * i / size
        p = radius * SVector(cos(theta), sin(theta), 0.0)
        stm.add(center + p)
    end
    stm.set_frequency(1.0)

    m = Sine(150)

    cnt.send(m, stm)
end
