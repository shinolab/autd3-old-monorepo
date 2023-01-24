# File: CustomModulation.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 24/01/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 


mutable struct CustomModulation
    _m::Modulation
    _header_ptr
    get_sampling_frequency_division
    set_sampling_frequency_division
    get_sampling_frequency
    function CustomModulation(buf::Array{Float64,1}; freq_div=40960)
        len::UInt64 = length(buf)
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_modulation_custom(chandle, buf, len, UInt32(freq_div))
        m = Modulation(chandle[])
        s = new(m, chandle[])
        s.get_sampling_frequency_division = m.get_sampling_frequency_division
        s.set_sampling_frequency_division = m.set_sampling_frequency_division
        s.get_sampling_frequency = m.get_sampling_frequency
        s
    end
end
