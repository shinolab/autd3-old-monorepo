'''
File: simulator.py
Project: example
Created Date: 29/08/2023
Author: Shun Suzuki
-----
Last Modified: 14/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from pyautd3 import Controller, AUTD3, DEVICE_WIDTH, Synchronize
from pyautd3.link import Simulator

from samples import runner


if __name__ == "__main__":
    autd = (
        Controller.builder()
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))
        .add_device(AUTD3.from_euler_zyz([DEVICE_WIDTH, 0.0, 0.0], [0.0, 0.0, 0.0]))
        .advanced_mode()
        .open_with(Simulator(8080))
    )

    for tr in autd.geometry:
        tr.frequency = 70e3

    autd.send(Synchronize())

    runner.run(autd)
