# File: Amplitudes.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 08/08/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

mutable struct Amplitudes
    _body_ptr::Ptr{Cvoid}
    function Amplitudes(amps::Array{Float64,1})
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_create_amplitudes(chandle, amps)
        c = new(chandle[])
        finalizer(c -> autd3capi.autd_delete_amplitudes(c._body_ptr), c)
        c
    end
end
