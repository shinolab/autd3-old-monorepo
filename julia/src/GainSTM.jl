# File: GainSTM.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 14/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

@enum Mode duty_phase_full = 1 phase_full = 2 phase_half = 4

mutable struct GainSTM
    _stm::STM
    _body_ptr::Ptr{Cvoid}
    add
    get_frequency
    set_frequency
    get_sampling_frequency_division
    set_sampling_frequency_division
    get_sampling_frequency
    get_mode
    set_mode
    function GainSTM(cnt::Controller)
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_gain_stm(chandle, cnt._ptr)
        stm = STM(chandle[])
        p = new(stm, chandle[])
        p.add = (gain) -> autd3capi.autd_gain_stm_add(p._body_ptr, gain._body_ptr)
        p.get_frequency = stm.get_frequency
        p.set_frequency = stm.set_frequency
        p.get_sampling_frequency_division = stm.get_sampling_frequency_division
        p.set_sampling_frequency_division = stm.set_sampling_frequency_division
        p.get_sampling_frequency = stm.get_sampling_frequency
        p.get_mode = () -> Mode(autd3capi.autd_get_gain_stm_mode(p._body_ptr))
        p.set_mode = (mode::Mode) -> autd3capi.autd_set_gain_stm_mode(p._body_ptr, UInt16(mode))
        p
    end
end
