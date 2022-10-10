# File: Static.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 14/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 


mutable struct Static
    _m::Modulation
    _header_ptr
    get_sampling_frequency_division
    set_sampling_frequency_division
    get_sampling_frequency
    function Static(amp::Float64=1.0)
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_modulation_static(chandle, amp)
        m = Modulation(chandle[])
        s = new(m, chandle[])
        s.get_sampling_frequency_division = m.get_sampling_frequency_division
        s.set_sampling_frequency_division = m.set_sampling_frequency_division
        s.get_sampling_frequency = m.get_sampling_frequency
        s
    end
end
