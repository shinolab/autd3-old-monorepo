# File: LM.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 14/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 


mutable struct LM
    _holo::Holo
    _body_ptr
    add
    set_constraint
    function LM(backend; eps_1::Float64=1e-8, eps_2::Float64=1e-8, tau::Float64=1e-3, k_max=5, initial::Array{Float64,1}=[])
        chandle = Ref(Ptr{Cvoid}(0))
        init = initial_len == 0 ? Ptr{Cvoid}(0) : initial
        autd3capi_gain_holo.autd_gain_holo_lm(chandle, backend._backend_ptr, eps_1, eps_2, tau, UInt64(k_max), init, Int32(length(initial)))
        h = Holo(chandle[])
        g = new(h, chandle[])
        g.add = h.add
        g.set_constraint = h.set_constraint
        g
    end
end
