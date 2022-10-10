# File: Link.jl
# Project: src
# Created Date: 13/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 14/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

struct Link
    _ptr::Ptr{Cvoid}
    function Link(ptr::Ptr{Cvoid})
        new(ptr)
    end
end
