"""
File: runner.py
Project: samples
Created Date: 30/12/2020
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Shun Suzuki. All rights reserved.

"""

from pyautd3 import Controller, Stop, FirmwareInfo

from . import focus, bessel, holo, custom, stm_gain, stm_focus, group


def run(autd: Controller):
    samples = [
        (focus.simple, "Single Focal Point Sample"),
        (bessel.bessel, "Bessel beam Sample"),
        (holo.holo, "Multiple Focal Points Sample"),
        (stm_focus.stm_focus, "FocusSTM (Hardware STM) Sample"),
        (stm_gain.stm_gain, "GainSTM (Hardware STM with arbitrary Gain) Sample"),
        (custom.custom, "Custom Focus Sample"),
        (group.group, "Group Sample"),
    ]

    print(
        "========================================= Firmware information ==========================================="
    )
    print("\n".join([str(firm) for firm in autd.firmware_info_list()]))
    print(
        "=========================================================================================================="
    )

    while True:
        print("\n".join([f"[{i}]: {name}" for i, (_, name) in enumerate(samples)]))
        print("[Other]: finish")

        idx = input("choose number: ")
        idx = int(idx) if idx.isdigit() else None
        if idx is None or idx >= len(samples):
            break

        (fn, _) = samples[idx]
        fn(autd)

        print("press enter to finish...")

        _ = input()

        print("finish.")
        autd.send(Stop())

    autd.close()
