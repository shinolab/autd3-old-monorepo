'''
File: emulator.py
Project: example
Created Date: 10/07/2021
Author: Shun Suzuki
-----
Last Modified: 14/08/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2021 Shun Suzuki. All rights reserved.

'''


from pyautd3 import AUTD, Emulator

from samples import runner


if __name__ == '__main__':
    autd = AUTD()

    autd.add_device([0., 0., 0.], [0., 0., 0.])
    # autd.add_device([0., 0., 0.], [0., 0., 0.])

    link = Emulator().port(50632).build()
    if not autd.open(link):
        print(AUTD.last_error())
        exit()

    runner.run(autd)
