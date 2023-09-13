'''
File: gain.py
Project: gain
Created Date: 29/08/2023
Author: Shun Suzuki
-----
Last Modified: 14/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


import functools
import numpy as np
from typing import Callable, Dict
from abc import ABCMeta, abstractmethod
from ctypes import POINTER

from pyautd3.native_methods.autd3capi import Drive
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import GainPtr
from pyautd3.geometry import Transducer, Geometry, Device
from pyautd3.internal.gain import IGain


class Gain(IGain, metaclass=ABCMeta):
    @abstractmethod
    def calc(self, geometry: Geometry) -> Dict[int, np.ndarray]:
        pass

    def gain_ptr(self, geometry: Geometry) -> GainPtr:
        drives = self.calc(geometry)
        return functools.reduce(
            lambda acc, dev: Base().gain_custom_set(acc, dev.idx,
                                                    drives[dev.idx].ctypes.data_as(POINTER(Drive)),  # type: ignore
                                                    len(drives[dev.idx])),
            geometry,
            Base().gain_custom(),
        )

    @staticmethod
    def transform(geometry: Geometry, f: Callable[[Device, Transducer], Drive]) -> Dict[int, np.ndarray]:
        return dict(
            map(
                lambda dev: (dev.idx, np.fromiter(map(lambda tr: np.void(f(dev, tr)), dev), dtype=Drive)),  # type: ignore
                geometry,
            )
        )
