# File: Gain.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 14/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

mutable struct Gain
    _ptr::Ptr{Cvoid}
    function Gain(ptr::Ptr{Cvoid})
        g = new(ptr)
        finalizer(g -> autd3capi.autd_delete_gain(g._ptr), g)
        g
    end
end
