# File: SOEM.jl
# Project: src
# Created Date: 13/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 18/04/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

@enum SyncMode dc = 0 free_run = 1
@enum TimerStrategy sleep = 0 busy_wait = 1 native_timer = 2

mutable struct SOEM
    _soem::Ptr{Cvoid}
    buf_size
    ifname
    send_cycle
    sync0_cycle
    sync_mode
    on_lost
    timer_strategy
    state_check_interval
    debug_level
    debug_func
    timeout
    build
    function SOEM()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi_link_soem.autd_link_soem(chandle)
        soem = new(chandle[])
        soem.buf_size = function (size::UInt64)
            autd3capi_link_soem.autd_link_soem_set_buf_size(soem._soem, size)
            soem
        end
        soem.ifname = function (name::String)
            autd3capi_link_soem.autd_link_soem_ifname(soem._soem, name)
            soem
        end
        soem.send_cycle = function (cycle::UInt16)
            autd3capi_link_soem.autd_link_soem_send_cycle(soem._soem, cycle)
            soem
        end
        soem.sync0_cycle = function (cycle::UInt16)
            autd3capi_link_soem.autd_link_soem_sync0_cycle(soem._soem, cycle)
            soem
        end
        soem.sync_mode = function (mode::SyncMode)
            autd3capi_link_soem.autd_link_soem_freerun(soem._soem, mode == free_run)
            soem
        end
        soem.on_lost = function (f::Function)
            ff = (x::Cstring) -> f(x)
            p = @cfunction($ff, Cvoid, (Cstring,))
            autd3capi_link_soem.autd_link_soem_on_lost(soem._soem, p)
            soem
        end
        soem.timer_strategy = function (strategy::TimerStrategy)
            autd3capi_link_soem.autd_link_soem_timer_strategy(soem._soem, UInt8(strategy))
            soem
        end
        soem.state_check_interval = function (interval::UInt64)
            autd3capi_link_soem.autd_link_soem_state_check_interval(soem._soem, interval)
            soem
        end
        soem.debug_level = function (level::UInt8)
            autd3capi_link_soem.autd_link_soem_log_level(soem._soem, level)
            soem
        end
        soem.debug_func = function (out::Function, flush::Function)
            ffout = (x::Cstring) -> out(x)
            ffflush = () -> flush()
            pout = @cfunction($ffout, Cvoid, (Cstring,))
            pflush = @cfunction($ffflush, Cvoid, ())
            autd3capi_link_soem.autd_link_soem_log_func(soem._soem, pout, pflush)
            soem
        end
        soem.timeout = function (timeout::UInt64)
            autd3capi_link_soem.autd_link_soem_timeout(soem._soem, timeout)
            soem
        end
        soem.build = function ()
            chandle = Ref(Ptr{Cvoid}(0))
            autd3capi_link_soem.autd_link_soem_build(chandle, soem._soem)
            autd3capi_link_soem.autd_link_soem_delete(soem._soem)
            Link(chandle[])
        end
        soem
    end
end

function enumerate_adapters()
    res = []
    phandle = Ref(Ptr{Cvoid}(0))
    size = autd3capi_link_soem.autd_get_adapter_pointer(phandle)
    handle::Ptr{Cvoid} = phandle[]

    for i = 0:size-1
        sb_desc = zeros(UInt8, 128)
        sb_name = zeros(UInt8, 128)
        autd3capi_link_soem.autd_get_adapter(handle, i, sb_desc, sb_name)
        push!(res, [String(strip(String(sb_desc), '\0')), String(strip(String(sb_name), '\0'))])
    end

    autd3capi_link_soem.autd_free_adapter_pointer(handle)
    res
end
