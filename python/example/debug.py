"""
File: debug.py
Project: example
Created Date: 17/04/2023
Author: Shun Suzuki
-----
Last Modified: 18/04/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from pyautd3 import Controller, DEVICE_WIDTH, Level, AUTD3
from pyautd3.link import Debug

from samples import runner


if __name__ == "__main__":
    autd = (
        Controller.builder()
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))
        .add_device(AUTD3.from_euler_zyz([DEVICE_WIDTH, 0.0, 0.0], [0.0, 0.0, 0.0]))
        .advanced_mode()
        .open_with(Debug().with_log_level(Level.Off))
    )

    for tr in autd.geometry:
        tr.frequency = 70e3

    runner.run(autd)
