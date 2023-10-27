"""
File: simulator.py
Project: example
Created Date: 29/08/2023
Author: Shun Suzuki
-----
Last Modified: 11/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from samples import runner

from pyautd3 import AUTD3, Controller
from pyautd3.link.simulator import Simulator

if __name__ == "__main__":
    with (
        Controller.builder()
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))
        .add_device(AUTD3.from_euler_zyz([AUTD3.device_width(), 0.0, 0.0], [0.0, 0.0, 0.0]))
        .open_with(Simulator.builder(8080))
    ) as autd:
        runner.run(autd)
