# File: Modulation.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 14/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

mutable struct Modulation
    _ptr::Ptr{Cvoid}
    get_sampling_frequency_division
    set_sampling_frequency_division
    get_sampling_frequency
    function Modulation(ptr::Ptr{Cvoid})
        m = new(ptr)
        m.get_sampling_frequency_division = () -> autd3capi.autd_modulation_sampling_frequency_division(m._ptr)
        m.set_sampling_frequency_division = (freq_div) -> autd3capi.autd_modulation_set_sampling_frequency_division(m._ptr, UInt32(freq_div))
        m.get_sampling_frequency = () -> autd3capi.autd_modulation_sampling_frequency(m._ptr)
        finalizer(m -> autd3capi.autd_delete_modulation(m._ptr), m)
        m
    end
end
