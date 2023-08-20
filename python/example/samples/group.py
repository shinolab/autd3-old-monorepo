"""
File: group.py
Project: samples
Created Date: 28/05/2023
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from pyautd3 import Controller, SilencerConfig
from pyautd3.gain import Focus, Group, Bessel, Null
from pyautd3.modulation import Sine
import numpy as np


def group(autd: Controller):
    config = SilencerConfig()
    autd.send(config)

    m = Sine(150)
    if autd.geometry.num_devices > 1:
        f = (
            Group.by_device(lambda dev: "focus" if dev == 0 else "bessel")
            .set(
                "focus", Focus(autd.geometry.center_of(0) + np.array([0.0, 0.0, 150.0]))
            )
            .set(
                "bessel",
                Bessel(
                    autd.geometry.center_of(1),
                    np.array([0.0, 0.0, 1.0]),
                    13 / 180 * np.pi,
                ),
            )
        )
        autd.send((m, f))
    else:
        cx = autd.geometry.center[0]
        f = (
            Group.by_transducer(lambda tr: "focus" if tr.position[0] < cx else "null")
            .set("focus", Focus(autd.geometry.center + np.array([0.0, 0.0, 150.0])))
            .set("null", Null())
        )
        autd.send((m, f))
