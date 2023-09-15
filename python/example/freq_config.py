'''
File: freq_config.py
Project: example
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 15/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from pyautd3 import Controller, Level, AUTD3, Synchronize
from pyautd3.link import Debug


if __name__ == "__main__":
    autd = (
        Controller.builder()
        .advanced()
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))
        .add_device(AUTD3.from_euler_zyz([AUTD3.device_width(), 0.0, 0.0], [0.0, 0.0, 0.0]))
        .open_with(Debug().with_log_level(Level.Off))
    )

    for dev in autd.geometry:
        for tr in dev:
            tr.frequency = 70e3

    autd.send(Synchronize())
