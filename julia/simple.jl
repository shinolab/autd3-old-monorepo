# File: simple.jl
# Project: autd3.jl
# Created Date: 13/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 03/03/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

using Printf

using StaticArrays

include("src/AUTD3.jl")

using .AUTD3


try
    geometry = GeometryBuilder().add_device(SVector(0.0, 0.0, 0.0), SVector(0.0, 0.0, 0.0)).build()

    link = Simulator()

    cnt = Controller(geometry, link)

    cnt.to_advanced()
    for tr in cnt.geometry()
        tr.set_frequency(70e3)
    end

    cnt.send(Clear())
    cnt.send(Synchronize())

    firm_info_list = cnt.firmware_info_list()
    for firm_info in firm_info_list
        @printf("%s\n", firm_info)
    end

    g = Focus(cnt.geometry().center() + SVector(0.0, 0.0, 150.0))
    m = Sine(150)

    cnt.send(m, g)

    print("press any key to finish...")
    readline()

    cnt.send(Stop())
    cnt.send(Clear())
    cnt.close()

catch e
    println(e)
end
