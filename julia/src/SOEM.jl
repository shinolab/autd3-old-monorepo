# File: SOEM.jl
# Project: src
# Created Date: 13/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 02/02/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

struct SOEM
    _link::Link
    function SOEM(; ifname::String="", send_cycle=2, sync0_cycle=2, freerun::Bool=false, on_lost=Nothing, high_precision::Bool=false, check_interval=500, debug_level=2)
        chandle = Ref(Ptr{Cvoid}(0))
        if on_lost == Nothing
            if ifname == ""
                autd3capi_link_soem.autd_link_soem(chandle, Ptr{Cvoid}(C_NULL), UInt16(sync0_cycle), UInt16(send_cycle), freerun, Ptr{Cvoid}(C_NULL), high_precision, UInt64(check_interval), debug_level, Ptr{Cvoid}(C_NULL), Ptr{Cvoid}(C_NULL))
            else
                autd3capi_link_soem.autd_link_soem(chandle, ifname, UInt16(sync0_cycle), UInt16(send_cycle), freerun, Ptr{Cvoid}(C_NULL), high_precision, UInt64(check_interval), debug_level, Ptr{Cvoid}(C_NULL), Ptr{Cvoid}(C_NULL))
            end
        else
            f = (x::Cstring) -> on_lost(x)
            p = @cfunction($f, Cvoid, (Cstring,))
            if ifname == ""
                autd3capi_link_soem.autd_link_soem(chandle, Ptr{Cvoid}(C_NULL), UInt16(sync0_cycle), UInt16(send_cycle), freerun, p, high_precision, UInt64(check_interval), debug_level, Ptr{Cvoid}(C_NULL), Ptr{Cvoid}(C_NULL))
            else
                autd3capi_link_soem.autd_link_soem(chandle, ifname, UInt16(sync0_cycle), UInt16(send_cycle), freerun, p, high_precision, UInt64(check_interval), debug_level, Ptr{Cvoid}(C_NULL), Ptr{Cvoid}(C_NULL))
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
