# File: RemoteTwinCAT.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 18/04/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 


struct RemoteTwinCAT
    _local_ams_net_id::String
    _remote_ip_addr::String
    _remote_ams_net_id::String
    local_ams_net_id
    remote_ip_addr
    remote_ams_net_id
    build
    function RemoteTwinCAT()
        remote = new("")
        remote.local_ams_net_id = function (local_ams_net_id::String)
            remote._local_ams_net_id = local_ams_net_id
            remote
        end
        remote.remote_ip_addr = function (remote_ip_addr::String)
            remote._remote_ip_addr = remote_ip_addr
            remote
        end
        remote.remote_ams_net_id = function (remote_ams_net_id::String)
            remote._remote_ams_net_id = remote_ams_net_id
            remote
        end
        remote.build = function ()
            chandle = Ref(Ptr{Cvoid}(0))
            autd3capi_link_remote_twincat.autd_link_remote_twin_cat(chandle, remote._remote_ip_addr, remote._remote_ams_net_id, remote._local_ams_net_id)
            Link(chandle[])
        end
        remote
    end
end
