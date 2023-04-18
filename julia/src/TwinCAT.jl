# File: TwinCAT.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 18/04/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

struct TwinCAT
    build
    function TwinCAT()
        twincat = new()
        twincat.build = function ()
            chandle = Ref(Ptr{Cvoid}(0))
            autd3capi_link_twincat.autd_link_twin_cat(chandle)
            Link(chandle[])
        end
        twincat
    end
end
