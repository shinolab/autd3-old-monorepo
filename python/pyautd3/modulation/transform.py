"""
File: transform.py
Project: modulation
Created Date: 10/10/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


import ctypes
from collections.abc import Callable

from pyautd3.internal.modulation import IModulation
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import ModulationPtr


class Transform(IModulation):
    """Modulation to transform modulation data."""

    _m: IModulation

    def __init__(self: "Transform", m: IModulation, f: Callable[[int, float], float]) -> None:
        self._m = m
        self._f_native = ctypes.CFUNCTYPE(ctypes.c_double, ctypes.c_void_p, ctypes.c_uint32, ctypes.c_double)(lambda _, i, d: f(int(i), float(d)))

    def _modulation_ptr(self: "Transform") -> ModulationPtr:
        return Base().modulation_with_transform(self._m._modulation_ptr(), self._f_native, None)  # type: ignore[arg-type]


def __with_transform(self: "IModulation", f: Callable[[int, float], float]) -> Transform:
    """Transform modulation data.

    Arguments:
    ---------
        self: Modulation
        f: Transform function. The first argument is the index of the modulation data, and the second is the original data.
    """
    return Transform(self, f)


IModulation.with_transform = __with_transform  # type: ignore[attr-defined]
