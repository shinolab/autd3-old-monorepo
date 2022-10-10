# File: ModDelayConfig.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 14/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 


mutable struct ModDelayConfig
    _header_ptr
    function ModDelayConfig()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi.autd_create_mod_delay_config(chandle)
        c = new(chandle[])
        finalizer(c -> autd3capi.autd_delete_mod_delay_config(c._header_ptr), c)
        c
    end
end
