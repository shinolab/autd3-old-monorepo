# File: FocusSTM.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 29/11/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

using StaticArrays


mutable struct FocusSTM
    _stm::STM
    _body_ptr::Ptr{Cvoid}
    add
    get_frequency
    set_frequency
    get_sampling_frequency_division
    set_sampling_frequency_division
    get_sampling_frequency
    function FocusSTM(sound_speed::Float64)
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_focus_stm(chandle, sound_speed)
        stm = STM(chandle[])
        p = new(stm, chandle[])
        p.add = function (position::SVector{3,Float64}; shift = 0)
            x, y, z = position
            autd3capi.autd_focus_stm_add(p._body_ptr, x, y, z, UInt8(shift))
        end
        p.get_frequency = stm.get_frequency
        p.set_frequency = stm.set_frequency
        p.get_sampling_frequency_division = stm.get_sampling_frequency_division
        p.set_sampling_frequency_division = stm.set_sampling_frequency_division
        p.get_sampling_frequency = stm.get_sampling_frequency
        p
    end
end
