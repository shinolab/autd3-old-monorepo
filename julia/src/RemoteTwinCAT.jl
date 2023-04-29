# File: RemoteTwinCAT.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 28/04/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

mutable struct RemoteTwinCAT
    _builder::Ptr{Cvoid}
    server_ip
    client_ams_net_id
    log_level
    log_func
    timeout
    build
    function RemoteTwinCAT(remote_ams_net_id::String)
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi_link_remote_twincat.autd_link_remote_twin_cat(chandle, remote_ams_net_id)
        remote_twincat = new(chandle[])
        remote_twincat.server_ip = function (ip::String)
            autd3capi_link_remote_twincat.autd_link_remote_twin_cat_server_ip_addr(remote_twincat._builder, ip)
            remote_twincat
        end
        remote_twincat.client_ams_net_id = function (client_ams_net_id::String)
            autd3capi_link_remote_twincat.autd_link_remote_twin_cat_client_ams_net_id(remote_twincat._builder, client_ams_net_id)
            remote_twincat
        end
        remote_twincat.log_level = function (level::UInt8)
            autd3capi_link_remote_twincat.autd_link_remote_twin_cat_log_level(remote_twincat._builder, level)
            remote_twincat
        end
        remote_twincat.log_func = function (out::Function, flush::Function)
            ffout = (x::Cstring) -> out(x)
            ffflush = () -> flush()
            pout = @cfunction($ffout, Cvoid, (Cstring,))
            pflush = @cfunction($ffflush, Cvoid, ())
            autd3capi_link_remote_twincat.autd_link_remote_twin_cat_log_func(remote_twincat._builder, pout, pflush)
            remote_twincat
        end
        remote_twincat.timeout = function (timeout::UInt64)
            autd3capi_link_remote_twincat.autd_link_remote_twin_cat_timeout(remote_twincat._builder, timeout)
            remote_twincat
        end
        remote_twincat.build = function ()
            chandle = Ref(Ptr{Cvoid}(0))
            autd3capi_link_remote_twincat.autd_link_remote_twin_cat_build(chandle, remote_twincat._builder)
            Link(chandle[])
        end
        remote_twincat
    end
end
