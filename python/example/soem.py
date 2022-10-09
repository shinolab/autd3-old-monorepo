'''
File: soem.py
Project: example
Created Date: 30/12/2020
Author: Shun Suzuki
-----
Last Modified: 14/08/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Shun Suzuki. All rights reserved.

'''


from pyautd3 import AUTD, SOEM

from samples import runner


if __name__ == '__main__':
    autd = AUTD()

    autd.add_device([0., 0., 0.], [0., 0., 0.])
    # autd.add_device([0., 0., 0.], [0., 0., 0.])

    link = SOEM().high_precision(True).build()
    if not autd.open(link):
        print(AUTD.last_error())
        exit()

    autd.check_trials = 50

    runner.run(autd)
