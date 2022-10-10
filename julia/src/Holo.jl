# File: Holo.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 14/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

struct DontCare
    type
    p
    function DontCare()
        new(0, Ptr{Cvoid}(0))
    end
end


struct Normalize
    type
    p
    function Normalize()
        new(1, Ptr{Cvoid}(0))
    end
end

mutable struct Uniform
    type
    p
    value
    function Uniform(value)
        c = new(2, Ptr{Cvoid}(0), value)
        c.p = Ptr{Cvoid}(c.value)
        c
    end
end

struct Clamp
    type
    p
    function Clamp()
        new(3, Ptr{Cvoid}(0))
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
        h.set_constraint = (constraint) -> autd3capi_gain_holo.autd_set_constraint(ptr, constraint.type, constraint.p)
        h
    end
end
