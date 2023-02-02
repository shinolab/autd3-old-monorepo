'''
File: simulator_client.py
Project: example
Created Date: 10/10/2022
Author: Shun Suzuki
-----
Last Modified: 02/02/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

from pyautd3 import Controller, GeometryBuilder, DEVICE_WIDTH
from pyautd3.link import Simulator

from samples import runner


if __name__ == '__main__':
    geometry = GeometryBuilder().add_device([0., 0., 0.], [0., 0., 0.]).add_device([DEVICE_WIDTH, 0., 0.], [0., 0., 0.]).build()

    link = Simulator().build()

    autd = Controller.open(geometry, link)

    autd.to_normal()
    for tr in autd.geometry:
        tr.frequency = 70e3

    autd.ack_check_timeout_ms = 20

    runner.run(autd)
