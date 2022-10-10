# File: simple.jl
# Project: autd3.jl
# Created Date: 13/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 10/10/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

using Printf

using StaticArrays

include("src/AUTD3.jl")

using .AUTD3

function on_lost(msg::Cstring)
    println(msg)
    exit(-1)
end

const cnt = Controller()
cnt.add_device(SVector(0.0, 0.0, 0.0), SVector(0.0, 0.0, 0.0))

const link = SOEM(on_lost=on_lost)

if !cnt.open(link)
    println(get_last_error())
    exit(-1)
end

cnt.clear()
cnt.synchronize()

firm_info_list = cnt.firmware_info_list()
for firm_info in firm_info_list
    @printf("%s\n", firm_info)
end

const g = Focus(SVector(90.0, 80.0, 150.0))
const m = Sine(150)

cnt.send(m, g)

print("press any key to finish...")
readline()

cnt.stop()
cnt.clear()
cnt.close()
