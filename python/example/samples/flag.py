'''
File: flag.py
Project: samples
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 14/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from pyautd3 import Controller, UpdateFlags

import threading


def flag(autd: Controller):

    for dev in autd.geometry:
        dev.force_fan = True
        dev.reads_fpga_info = True

    print('press any key to run fan...')
    _ = input()

    autd.send(UpdateFlags())

    fin = False

    def f():
        prompts = ['-', '/', '|', '\\']
        prompts_idx = 0
        while not fin:
            states = autd.fpga_info
            print(f'{prompts[(prompts_idx // 1000) % len(prompts)]} FPGA Status...')
            print("\n".join([str(state) for state in states]))
            print(f'\x1b[{len(states) + 1}A', end='')
            prompts_idx += 1

    th = threading.Thread(target=f)
    th.start()

    print('press any key stop checking FPGA status...')
    _ = input()

    fin = True
    th.join()

    for dev in autd.geometry:
        dev.force_fan = False
        dev.reads_fpga_info = False

    autd.send(UpdateFlags())
