# File: soem.jl
# Project: example
# Created Date: 30/12/2020
# Author: Shun Suzuki
# -----
# Last Modified: 18/04/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2020 Hapis Lab. All rights reserved.
# 

using Printf

using AUTD3
using StaticArrays
using StaticArrays: size

include("tests/runner.jl")

function on_lost(msg::Cstring)
    println(msg)
    exit(-1)
end

function main()
    try
        geometry = GeometryBuilder().add_device(SVector(0.0, 0.0, 0.0), SVector(0.0, 0.0, 0.0)).build()

        link = SOEM().on_lost(on_lost).build()

        cnt = Controller(geometry, link)

        run(cnt)

    catch e
        println(e)
    end
end

main()
