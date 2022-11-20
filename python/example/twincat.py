'''
File: soem.py
Project: example
Created Date: 30/12/2020
Author: Shun Suzuki
-----
Last Modified: 20/11/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Shun Suzuki. All rights reserved.

'''


from pyautd3 import Controller
from pyautd3.link import TwinCAT

from samples import runner


if __name__ == '__main__':
    autd = Controller()

    autd.geometry.add_device([0., 0., 0.], [0., 0., 0.])

    link = TwinCAT().build()
    if not autd.open(link):
        print('Failed to open Controller')
        exit()

    runner.run(autd)
