# File: soem.jl
# Project: example
# Created Date: 30/12/2020
# Author: Shun Suzuki
# -----
# Last Modified: 08/03/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2020 Hapis Lab. All rights reserved.
# 

using Printf

using AUTD3
using StaticArrays
using StaticArrays: size

include("tests/runner.jl")


function main()
    try
        geometry = GeometryBuilder().add_device(SVector(0.0, 0.0, 0.0), SVector(0.0, 0.0, 0.0)).to_advanced().build()

        link = Simulator()

        cnt = Controller(geometry, link)

        for tr in cnt.geometry()
            tr.set_frequency(70e3)
        end

        run(cnt)

    catch e
        println(e)
    end
end

main()
