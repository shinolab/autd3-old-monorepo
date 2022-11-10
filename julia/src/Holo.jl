# File: Holo.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 10/11/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

struct DontCare
    p
    function DontCare()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi_gain_holo.autd_constraint_dont_care(chandle)
        new(chandle[])
    end
end

struct Normalize
    p
    function Normalize()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi_gain_holo.autd_constraint_normalize(chandle)
        new(chandle[])
    end
end

mutable struct Uniform
    p
    function Uniform(value)
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi_gain_holo.autd_constraint_uniform(chandle, value)
        new(chandle[])
    end
end

struct Clamp
    p
    function Clamp()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi_gain_holo.autd_constraint_clamp(chandle)
        new(chandle[])
    end
end

mutable struct Holo
    _gain::Gain
    add
    set_constraint
    function Holo(ptr::Ptr{Cvoid})
        g = Gain(ptr)
        h = new(g)
        h.add = function (pos::SVector{3,Float64}, amp::Float64)
            x, y, z = pos
            autd3capi_gain_holo.autd_gain_holo_add(ptr, x, y, z, amp)
        end
        h.set_constraint = (constraint) -> autd3capi_gain_holo.autd_set_constraint(ptr, constraint.p)
        h
    end
end
