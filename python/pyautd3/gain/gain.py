"""
File: gain.py
Project: gain
Created Date: 29/08/2023
Author: Shun Suzuki
-----
Last Modified: 29/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


import functools
from abc import ABCMeta, abstractmethod
from collections.abc import Callable
from ctypes import POINTER

import numpy as np

from pyautd3.drive import Drive
from pyautd3.geometry import Device, Geometry, Transducer
from pyautd3.internal.gain import IGain
from pyautd3.native_methods.autd3capi import Drive as _Drive
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import GainPtr


class Gain(IGain, metaclass=ABCMeta):
    """Base class of custom Gain."""

    @abstractmethod
    def calc(self: "Gain", geometry: Geometry) -> dict[int, np.ndarray]:
        """Calculate amp and phase for all transducers."""

    def _gain_ptr(self: "Gain", geometry: Geometry) -> GainPtr:
        drives = self.calc(geometry)
        return functools.reduce(
            lambda acc, dev: Base().gain_custom_set(
                acc,
                dev.idx,
                drives[dev.idx].ctypes.data_as(POINTER(_Drive)),  # type: ignore[arg-type]
                len(drives[dev.idx]),
            ),
            geometry.devices(),
            Base().gain_custom(),
        )

    @staticmethod
    def _transform(geometry: Geometry, f: Callable[[Device, Transducer], Drive]) -> dict[int, np.ndarray]:
        return {
            dev.idx: np.fromiter(
                (np.void(_Drive(d.phase._value, d.intensity._value)) for d in (f(dev, tr) for tr in dev)),  # type: ignore[call-overload]
                dtype=_Drive,
            )
            for dev in geometry.devices()
        }
