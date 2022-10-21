'''
File: soem.py
Project: example
Created Date: 30/12/2020
Author: Shun Suzuki
-----
Last Modified: 21/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Shun Suzuki. All rights reserved.

'''


from pyautd3 import Controller
from pyautd3.link import SOEM

from samples import runner


if __name__ == '__main__':
    autd = Controller()

    autd.geometry.add_device([0., 0., 0.], [0., 0., 0.])

    link = SOEM().high_precision(True).build()
    if not autd.open(link):
        print(Controller.last_error())
        exit()

    autd.check_trials = 50

    runner.run(autd)
