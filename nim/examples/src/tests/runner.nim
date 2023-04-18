# File: runner.nim
# Project: src
# Created Date: 13/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 18/04/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
#

import strformat
import strutils

import autd3
import focus
import bessel
import focus_stm
import gain_stm
import holo
import custom

proc run*(cnt: Controller) =
    let samples = [(focus_test, "Single Focal Point Sample"),
    (bessel_test, "BesselBeam Point Sample"),
    (holo_test, "Holo Gain Sample"),
    (focus_stm_test, "FocusSTM Sample"),
    (gain_stm_test, "GainSTM Sample"),
    (custom_test, "CustomGain Sample")]

    cnt.send(clear())
    cnt.send(synchronize())

    echo "================================== Firmware information =========================================="
    let firmList = cnt.firmwareInfoList()
    for firm in firmList:
        echo firm
    echo "=================================================================================================="

    while true:
        for i, (_, name) in samples:
            echo fmt"[{i}]: {name}"
        echo "[Other]: finish"

        stdout.write "Choose adapter: "
        let input = stdin.readLine

        var idx: int = 0
        try:
            idx = input.parseInt
            if idx >= samples.len:
                break

            let (f, _) = samples[idx]
            f(cnt)

            echo "press enter to finish"
            discard stdin.readLine

            cnt.send(stop())

        except:
            break

    cnt.close()
