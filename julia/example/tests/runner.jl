# File: runner.jl
# Project: example
# Created Date: 14/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 08/03/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
# 

include("focus.jl")
include("bessel.jl")
include("holo.jl")
include("focus_stm.jl")
include("gain_stm.jl")

function run(cnt::Controller)
    samples = [
        (focus, "Single Focal Point Sample"),
        (bessel, "BesselBeam Sample"),
        (holo, "Multiple Focal Points Sample"),
        (focus_stm, "FocusSTM Sample"),
        (gain_stm, "GainSTM Sample")
    ]

    firm_info_list = cnt.firmware_info_list()
    for firm_info in firm_info_list
        @printf("%s\n", firm_info)
    end

    cnt.send(Clear(); timeout_ns=UInt64(20 * 1000 * 1000))
    cnt.send(Synchronize(); timeout_ns=UInt64(20 * 1000 * 1000))

    while true
        for (i, (_, name)) in enumerate(samples)
            @printf("[%d]: %s\n", i, name)
        end
        println("[Other]: finish")
        print("Choose number: ")

        idx = tryparse(Int64, readline())
        if idx === nothing || idx > length(samples) || idx < 1
            break
        end

        (fn, _) = samples[idx]
        fn(cnt)

        println("press enter to finish...")

        readline()

        cnt.send(Stop(); timeout_ns=UInt64(20 * 1000 * 1000))
    end

    cnt.close()

end
