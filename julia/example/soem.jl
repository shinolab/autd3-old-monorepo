# File: soem.jl
# Project: example
# Created Date: 30/12/2020
# Author: Shun Suzuki
# -----
# Last Modified: 10/10/2022
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
    cnt = Controller()
    cnt.add_device(SVector(0.0, 0.0, 0.0), SVector(0.0, 0.0, 0.0))

    link = SOEM(on_lost=on_lost, high_precision=true)

    if !cnt.open(link)
        println(get_last_error())
        exit(-1)
    end

    run(cnt)
end

main()
