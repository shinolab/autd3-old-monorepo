# File: focus.nim
# Project: tests
# Created Date: 13/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 13/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
#


import autd3
import math

func focus(cnt: Controller, pos: openArray[float64]): CustomGain =
    let n = cnt.deviceNum * 249
    var phases = newSeqOfCap[float64](n)
    var amps = newSeqOfCap[float64](n)
    for i in 0..<cnt.deviceNum:
        for j in 0..<249i32:
            let tp = cnt.transPosition(i, j)
            let dist = ((tp[0] - pos[0]) * (tp[0] - pos[0]) + (tp[1] - pos[1]) *
                    (tp[1] - pos[1]) + (tp[2] - pos[2]) * (tp[2] - pos[2])).sqrt
            let wavenum = 2.0 * PI / cnt.wavelength(i, j)
            phases.add(wavenum * dist / (2.0 * PI))
            amps.add(1.0)
    initCustomGain(amps, phases)

proc custom_test*(cnt: Controller) =
    let config = initSilencerConfig()
    cnt.send(config)

    let f = focus(cnt, [90.0, 80.0, 150.0])
    let m = initSine(150)

    cnt.send(m, f)
