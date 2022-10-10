# File: RemoteTwinCAT.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 08/08/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 


struct RemoteTwinCAT
    _link::Link
    function RemoteTwinCAT(remote_ip_addr::String, remote_ams_net_id::String; local_ams_net_id::String="")
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi_link_remote_twincat.autd_link_remote_twin_cat(chandle, remote_ip_addr, remote_ams_net_id, local_ams_net_id)
        new(Link(chandle[]))
    end
end
