# File: GSPAT.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 14/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 


mutable struct GSPAT
    _holo::Holo
    _body_ptr
    add
    set_constraint
    function GSPAT(backend; repeat=100)
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi_gain_holo.autd_gain_holo_gspat(chandle, backend._backend_ptr, UInt64(repeat))
        h = Holo(chandle[])
        g = new(h, chandle[])
        g.add = h.add
        g.set_constraint = h.set_constraint
        g
    end
end
