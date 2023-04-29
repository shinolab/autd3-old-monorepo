# File: TwinCAT.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 28/04/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

mutable struct TwinCAT
    _builder::Ptr{Cvoid}
    log_level
    log_func
    timeout
    build
    function TwinCAT()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi_link_twincat.autd_link_twin_cat(chandle)
        twincat = new(chandle[])
        twincat.log_level = function (level::UInt8)
            autd3capi_link_twincat.autd_link_twin_cat_log_level(twincat._builder, level)
            twincat
        end
        twincat.log_func = function (out::Function, flush::Function)
            ffout = (x::Cstring) -> out(x)
            ffflush = () -> flush()
            pout = @cfunction($ffout, Cvoid, (Cstring,))
            pflush = @cfunction($ffflush, Cvoid, ())
            autd3capi_link_twincat.autd_link_twin_cat_log_func(twincat._builder, pout, pflush)
            twincat
        end
        twincat.timeout = function (timeout::UInt64)
            autd3capi_link_twincat.autd_link_twin_cat_timeout(twincat._builder, timeout)
            twincat
        end
        twincat.build = function ()
            chandle = Ref(Ptr{Cvoid}(0))
            autd3capi_link_twincat.autd_link_twin_cat_build(chandle, twincat._builder)
            Link(chandle[])
        end
        twincat
    end
end
