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
from typing import Generic, TypeVar

from pyautd3.emit_intensity import EmitIntensity
from pyautd3.internal.modulation import IModulation
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import ModulationPtr

M = TypeVar("M", bound=IModulation)


class Transform(IModulation, Generic[M]):
    """Modulation to transform modulation data."""

    _m: M

    def __init__(self: "Transform", m: M, f: Callable[[int, EmitIntensity], EmitIntensity]) -> None:
        self._m = m
        self._f_native = ctypes.CFUNCTYPE(ctypes.c_uint8, ctypes.c_void_p, ctypes.c_uint32, ctypes.c_uint8)(
            lambda _, i, d: f(
                int(i),
                EmitIntensity(d),
            ).value,
        )

    def _modulation_ptr(self: "Transform") -> ModulationPtr:
        return Base().modulation_with_transform(self._m._modulation_ptr(), self._f_native, None)  # type: ignore[arg-type]


def __with_transform(self: M, f: Callable[[int, EmitIntensity], EmitIntensity]) -> Transform:
    """Transform modulation data.

    Arguments:
    ---------
        self: Modulation
        f: Transform function. The first argument is the index of the modulation data, and the second is the original data.
    """
    return Transform(self, f)


IModulation.with_transform = __with_transform  # type: ignore[method-assign]
