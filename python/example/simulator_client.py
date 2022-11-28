'''
File: simulator_client.py
Project: example
Created Date: 10/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/11/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

from pyautd3 import Controller, DEVICE_WIDTH
from pyautd3.link import Simulator

from samples import runner


if __name__ == '__main__':
    autd = Controller()

    autd.geometry.add_device([0., 0., 0.], [0., 0., 0.])
    autd.geometry.add_device([DEVICE_WIDTH, 0., 0.], [0., 0., 0.])

    autd.to_normal()
    for tr in autd.geometry:
        tr.frequency = 70e3

    link = Simulator().build()
    if not autd.open(link):
        print('Failed to open Controller')
        exit()

    runner.run(autd)
