'''
File: runner.py
Project: samples
Created Date: 30/12/2020
Author: Shun Suzuki
-----
Last Modified: 22/06/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Shun Suzuki. All rights reserved.

'''

from pyautd3 import AUTD

from . import focus, bessel, holo, custom, stm_gain, stm_point


def run(autd: AUTD):
    samples = [
        (focus.simple, "Single Focal Point Sample"),
        (bessel.bessel, "Bessel beam Sample"),
        (holo.holo, "Multiple Focal Points Sample"),
        (stm_point.stm_point, "PointSequence (Hardware STM) Sample"),
        (stm_gain.stm_gain, "GainSequence (Hardware STM with arbitrary Gain) Sample"),
        (custom.custom, "Custom Focus Sample")
    ]

    autd.clear()

    autd.synchronize()

    print('============================ Firmware information ==============================')
    firm_info_list = autd.firmware_info_list()
    for i, firm in enumerate(firm_info_list):
        print(f'[{i}]: {firm}')
    print('================================================================================')

    while True:
        for i, (_, name) in enumerate(samples):
            print(f'[{i}]: {name}')
        print('[Other]: finish')

        idx = input('choose number: ')
        idx = int(idx) if idx.isdigit() else None
        if idx is None or idx >= len(samples):
            break

        (fn, _) = samples[idx]
        fn(autd)

        print('press enter to finish...')

        _ = input()

        print('finish.')
        autd.stop()

    autd.clear()
    autd.dispose()
