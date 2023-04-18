'''
File: soem.py
Project: example
Created Date: 30/12/2020
Author: Shun Suzuki
-----
Last Modified: 17/04/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Shun Suzuki. All rights reserved.

'''


from pyautd3 import Controller, Geometry
from pyautd3.link import TwinCAT

from samples import runner


if __name__ == '__main__':
    geometry = Geometry.Builder().add_device([0., 0., 0.], [0., 0., 0.]).build()

    link = TwinCAT().build()

    autd = Controller.open(geometry, link)

    runner.run(autd)
