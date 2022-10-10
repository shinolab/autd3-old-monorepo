# File: APO.jl
# Project: src
# Created Date: 08/08/2022
# Author: Shun Suzuki
# -----
# Last Modified: 08/08/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 


mutable struct APO
    _holo::Holo
    _body_ptr
    add
    set_constraint
    function APO(backend; eps::Float64=1e-8, lambda::Float64=1.0, k_max=200, line_search_max=100)
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi_gain_holo.autd_gain_holo_apo(chandle, backend._backend_ptr, eps, lambda, Int32(k_max), Int32(line_search_max))
        h = Holo(chandle[])
        g = new(h, chandle[])
        g.add = h.add
        g.set_constraint = h.set_constraint
        g
    end
end
