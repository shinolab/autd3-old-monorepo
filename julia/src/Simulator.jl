# File: Simulator.jl
# Project: src
# Created Date: 10/10/2022
# Author: Shun Suzuki
# -----
# Last Modified: 18/04/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 


mutable struct Simulator
    _timeout::UInt64
    timeout
    build
    function Simulator()
        simulator = new(20 * 1000 * 1000)
        simulator.timeout = function (t::UInt64)
            simulator._timeout = t
            simulator
        end
        simulator.build = function ()
            chandle = Ref(Ptr{Cvoid}(0))
            autd3capi_link_simulator.autd_link_simulator(chandle, simulator._timeout)
            Link(chandle[])
        end
        simulator
    end
end
