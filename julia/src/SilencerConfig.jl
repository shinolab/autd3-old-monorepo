# File: SilencerConfig.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 14/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

mutable struct SilencerConfig
    _header_ptr
    function SilencerConfig(step=10, freq_div=4096)
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_create_silencer(chandle, UInt16(step), UInt16(freq_div))
        c = new(chandle[])
        finalizer(c -> autd3capi.autd_delete_silencer(c._header_ptr), c)
        c
    end
end

function SilencerConfigNone()
    SilencerConfig(0xFFFF, 4096)
end
