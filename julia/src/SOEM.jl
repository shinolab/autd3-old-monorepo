# File: SOEM.jl
# Project: src
# Created Date: 13/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 20/03/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

@enum SyncMode dc = 0 free_run = 1
@enum TimerStrategy sleep = 0 busy_wait = 1 native_timer = 2

struct SOEM
    _link::Link
    function SOEM(; ifname::String="", buf_size=0, send_cycle=2, sync0_cycle=2, sync_mode::SyncMode=free_run, on_lost=Nothing, timer_strategy::TimerStrategy=sleep, check_interval=500, debug_level=2)
        chandle = Ref(Ptr{Cvoid}(0))
        if on_lost == Nothing
            if ifname == ""
                autd3capi_link_soem.autd_link_soem(chandle, Ptr{Cvoid}(C_NULL), UInt64(buf_size), UInt16(sync0_cycle), UInt16(send_cycle), sync_mode == free_run, Ptr{Cvoid}(C_NULL), UInt8(timer_strategy), UInt64(check_interval), debug_level, Ptr{Cvoid}(C_NULL), Ptr{Cvoid}(C_NULL))
            else
                autd3capi_link_soem.autd_link_soem(chandle, ifname, UInt64(buf_size), UInt16(sync0_cycle), UInt16(send_cycle), sync_mode == free_run, Ptr{Cvoid}(C_NULL), UInt8(timer_strategy), UInt64(check_interval), debug_level, Ptr{Cvoid}(C_NULL), Ptr{Cvoid}(C_NULL))
            end
        else
            f = (x::Cstring) -> on_lost(x)
            p = @cfunction($f, Cvoid, (Cstring,))
            if ifname == ""
                autd3capi_link_soem.autd_link_soem(chandle, Ptr{Cvoid}(C_NULL), UInt64(buf_size), UInt16(sync0_cycle), UInt16(send_cycle), sync_mode == free_run, p, UInt8(timer_strategy), UInt64(check_interval), debug_level, Ptr{Cvoid}(C_NULL), Ptr{Cvoid}(C_NULL))
            else
                autd3capi_link_soem.autd_link_soem(chandle, ifname, UInt64(buf_size), UInt16(sync0_cycle), UInt16(send_cycle), sync_mode == free_run, p, UInt8(timer_strategy), UInt64(check_interval), debug_level, Ptr{Cvoid}(C_NULL), Ptr{Cvoid}(C_NULL))
            end
        end
        new(Link(chandle[]))
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
