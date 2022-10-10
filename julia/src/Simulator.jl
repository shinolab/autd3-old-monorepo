# File: Simulator.jl
# Project: src
# Created Date: 10/10/2022
# Author: Shun Suzuki
# -----
# Last Modified: 10/10/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 


struct Simulator
    _link::Link
    function Simulator(; port=50632; ip_addr::String="")
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi_link_simulator.autd_link_simulator(chandle, UInt16(port), ip_addr)
        new(Link(chandle[]))
    end
end
