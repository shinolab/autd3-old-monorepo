'''
File: group_gain.py
Project: example
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 15/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from pyautd3 import Controller, Level, AUTD3
from pyautd3.link import Debug

from pyautd3.gain import Focus, Null, Group
from pyautd3.modulation import Sine

import numpy as np

if __name__ == "__main__":
    autd = (
        Controller.builder()
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))
        .open_with(Debug().with_log_level(Level.Off))
    )

    cx = autd.geometry.center[0]
    g1 = Focus(autd.geometry.center + np.array([0, 0, 150]))
    g2 = Null()

    g = Group(lambda _, tr: "focus" if tr.position[0] < cx else "null").set("focus", g1).set("null", g2)

    m = Sine(150)

    autd.send((m, g))

    autd.close()
