# File: TwinCAT.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 14/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

struct TwinCAT
    _link::Link
    function TwinCAT()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi_link_twincat.autd_link_twin_cat(chandle)
        new(Link(chandle[]))
    end
end
