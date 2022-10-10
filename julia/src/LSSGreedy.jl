# File: LSSGreedy.jl
# Project: src
# Created Date: 08/08/2022
# Author: Shun Suzuki
# -----
# Last Modified: 08/08/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 


mutable struct LSSGreedy
    _holo::Holo
    _body_ptr
    add
    set_constraint
    function LSSGreedy(backend; div=16)
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi_gain_holo.autd_gain_holo_lss_greedy(chandle, backend._backend_ptr, Int32(div))
        h = Holo(chandle[])
        g = new(h, chandle[])
        g.add = h.add
        g.set_constraint = h.set_constraint
        g
    end
end
