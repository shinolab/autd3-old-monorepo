# File: Simulator.jl
# Project: src
# Created Date: 10/10/2022
# Author: Shun Suzuki
# -----
# Last Modified: 28/04/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 


mutable struct Simulator
    _builder::Ptr{Cvoid}
    log_level
    log_func
    timeout
    build
    function Simulator()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi_link_simulator.autd_link_simulator(chandle)
        simulator = new(chandle[])
        simulator.log_level = function (level::UInt8)
            autd3capi_link_simulator.autd_link_simulator_log_level(simulator._builder, level)
            simulator
        end
        simulator.log_func = function (out::Function, flush::Function)
            ffout = (x::Cstring) -> out(x)
            ffflush = () -> flush()
            pout = @cfunction($ffout, Cvoid, (Cstring,))
            pflush = @cfunction($ffflush, Cvoid, ())
            autd3capi_link_simulator.autd_link_simulator_log_func(simulator._builder, pout, pflush)
            simulator
        end
        simulator.timeout = function (timeout::UInt64)
            autd3capi_link_simulator.autd_link_simulator_timeout(simulator._builder, timeout)
            simulator
        end
        simulator.build = function ()
            chandle = Ref(Ptr{Cvoid}(0))
            autd3capi_link_simulator.autd_link_simulator_build(chandle, simulator._builder)
            Link(chandle[])
        end
        simulator
    end
end
