# File: Gain.jl
# Project: src
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 14/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

mutable struct BackendEigen
    _backend_ptr::Ptr{Cvoid}
    function BackendEigen()
        chandle = Ref(Ptr{Cvoid}(0))
        autd3capi_gain_holo.autd_eigen_backend(chandle)
        g = new(chandle[])
        finalizer(g -> autd3capi_gain_holo.autd_delete_backend(g._backend_ptr), g)
        g
    end
end
