# File: Null.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 14/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

struct Null
    _gain::Gain
    _body_ptr
    function Null()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_gain_null(chandle)
        g = Gain(chandle[])
        new(g, chandle[])
    end
end
