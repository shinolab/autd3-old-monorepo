# File: Grouped.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 14/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 


mutable struct Grouped
    _gain::Gain
    _body_ptr
    add
    function Grouped(cnt::Controller)
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_gain_grouped(chandle, cnt._ptr)
        g = Gain(chandle[])
        gg = new(g, chandle[])
        gg.add = (dev_id, gain) => autd3capi.autd_gain_grouped_add(gg._body_ptr, Int32(dev_id), gain._body_ptr)
        gg
    end
end
