"""
File: custom.py
Project: samples
Created Date: 11/10/2021
Author: Shun Suzuki
-----
Last Modified: 21/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


import numpy as np
from numpy.typing import ArrayLike

from pyautd3 import Controller, Drive, EmitIntensity, Geometry, Phase, SamplingConfiguration, Silencer
from pyautd3.gain import Gain
from pyautd3.modulation import Modulation


class Focus(Gain):
    def __init__(self: "Focus", point: ArrayLike) -> None:
        self.point = np.array(point)

    def calc(self: "Focus", geometry: Geometry) -> dict[int, np.ndarray]:
        return Gain._transform(
            geometry,
            lambda dev, tr: Drive(
                Phase.from_rad(float(np.linalg.norm(tr.position - self.point)) * tr.wavenumber(dev.sound_speed)),
                EmitIntensity.maximum(),
            ),
        )


class Burst(Modulation):
    _length: int

    def __init__(self: "Burst", length: int, config: SamplingConfiguration | None = None) -> None:
        super().__init__(config if config is not None else SamplingConfiguration.from_frequency(4e3))
        self._length = length

    def calc(self: "Burst") -> np.ndarray:
        buf = np.array([EmitIntensity.minimum()] * self._length)
        buf[0] = EmitIntensity.maximum()
        return buf


async def custom(autd: Controller) -> None:
    config = Silencer()
    await autd.send_async(config)

    f = Focus(autd.geometry.center + np.array([0.0, 0.0, 150.0]))
    m = Burst(4000)

    await autd.send_async(m, f)
