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


from pyautd3 import Controller, Geometry, DEVICE_WIDTH, Level
from pyautd3.link import Debug

from samples import runner


if __name__ == "__main__":
    geometry = (
        Geometry.Builder()
        .add_device([0.0, 0.0, 0.0], [0.0, 0.0, 0.0])
        .add_device([DEVICE_WIDTH, 0.0, 0.0], [0.0, 0.0, 0.0])
        .advanced_mode()
        .build()
    )

    link = Debug().log_level(Level.Off).build()

    autd = Controller.open(geometry, link)

    for tr in autd.geometry:
        tr.frequency = 70e3

    runner.run(autd)
