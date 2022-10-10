# File: CustomGain.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 14/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

using StaticArrays

struct CustomGain
    _gain::Gain
    _body_ptr
    function CustomGain(amps::Array{Float64,1}, phases::Array{Float64,1})
        len::UInt64 = length(amps)
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_gain_custom(chandle, amps, phases, len)
        g = Gain(chandle[])
        new(g, chandle[])
    end
end
