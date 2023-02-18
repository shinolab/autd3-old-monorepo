'''
File: runner.py
Project: samples
Created Date: 30/12/2020
Author: Shun Suzuki
-----
Last Modified: 18/02/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Shun Suzuki. All rights reserved.

'''

from pyautd3 import Controller, Clear, Synchronize, Stop, FirmwareInfo

from . import focus, bessel, holo, custom, stm_gain, stm_focus


def run(autd: Controller):
    samples = [
        (focus.simple, "Single Focal Point Sample"),
        (bessel.bessel, "Bessel beam Sample"),
        (holo.holo, "Multiple Focal Points Sample"),
        (stm_focus.stm_focus, "PointSequence (Hardware STM) Sample"),
        (stm_gain.stm_gain, "GainSequence (Hardware STM with arbitrary Gain) Sample"),
        (custom.custom, "Custom Focus Sample")
    ]

    autd.send(Clear())
    autd.send(Synchronize())

    firm_info_list = autd.firmware_info_list()
    if not all([firm.matches_version for firm in firm_info_list]):
        print('\033[93mWARN: FPGA and CPU firmware version do not match.\033[0m')
    if not all([firm.is_latest for firm in firm_info_list]):
        print(f'\033[93mWARN: You are using old firmware. Please consider updating to {FirmwareInfo.latest_version()}.\033[0m')
    print('================================== Firmware information ====================================')
    for firm in firm_info_list:
        print(firm)
    print('============================================================================================')

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
        autd.send(Stop())

    autd.dispose()
