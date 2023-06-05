"""
File: geometry_viewer.py
Project: example
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


from pyautd3 import AUTD3, DEVICE_WIDTH, Controller, Level
from pyautd3.link import Debug
from pyautd3.extra import GeometryViewer
from math import pi

if __name__ == "__main__":
    autd = (
        Controller.builder()
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, DEVICE_WIDTH], [0.0, pi / 2, 0.0]))
        .add_device(
            AUTD3.from_euler_zyz([DEVICE_WIDTH, 0.0, DEVICE_WIDTH], [0.0, pi, 0.0])
        )
        .add_device(AUTD3.from_euler_zyz([DEVICE_WIDTH, 0.0, 0.0], [0.0, -pi / 2, 0.0]))
        .open_with(Debug().with_log_level(Level.Off))
    )

    GeometryViewer().window_size(800, 600).vsync(True).run(autd.geometry)
