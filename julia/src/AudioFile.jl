# File: Sine.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 14/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 


mutable struct RawPCM
    _m::Modulation
    _header_ptr
    get_sampling_frequency_division
    set_sampling_frequency_division
    get_sampling_frequency
    function RawPCM(filename::String, sampling_freq::Float64, sampling_freq_div)
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi_modulation_audio_file.autd_modulation_raw_pcm(chandle, filename, sampling_freq, UInt32(sampling_freq_div))
        m = Modulation(chandle[])
        s = new(m, chandle[])
        s.get_sampling_frequency_division = m.get_sampling_frequency_division
        s.set_sampling_frequency_division = m.set_sampling_frequency_division
        s.get_sampling_frequency = m.get_sampling_frequency
        s
    end
end

mutable struct Wav
    _m::Modulation
    _header_ptr
    get_sampling_frequency_division
    set_sampling_frequency_division
    get_sampling_frequency
    function Wav(filename::String, sampling_freq_div)
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi_modulation_audio_file.autd_modulation_wav(chandle, filename, UInt32(sampling_freq_div))
        m = Modulation(chandle[])
        s = new(m, chandle[])
        s.get_sampling_frequency_division = m.get_sampling_frequency_division
        s.set_sampling_frequency_division = m.set_sampling_frequency_division
        s.get_sampling_frequency = m.get_sampling_frequency
        s
    end
end
