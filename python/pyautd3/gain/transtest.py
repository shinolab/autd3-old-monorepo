"""
File: transtest.py
Project: gain
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 29/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


import ctypes
from collections.abc import Callable

from pyautd3.drive import Drive
from pyautd3.geometry import Device, Geometry, Transducer
from pyautd3.internal.gain import IGain
from pyautd3.native_methods.autd3capi import ContextPtr
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import Drive as _Drive
from pyautd3.native_methods.autd3capi_def import GainPtr, GeometryPtr


class TransducerTest(IGain):
    """Gain to drive only specified transducers."""

    def __init__(self: "TransducerTest", f: Callable[[Device, Transducer], Drive | None]) -> None:
        super().__init__()

        def f_native(_context: ContextPtr, geometry_ptr: GeometryPtr, dev_idx: int, tr_idx: int, raw) -> None:  # noqa: ANN001
            dev = Device(dev_idx, Base().device(geometry_ptr, dev_idx))
            tr = Transducer(tr_idx, dev._ptr)
            d = f(dev, tr)
            if d is not None:
                raw[0] = _Drive(d.phase.value, d.intensity.value)

        self._f_native = ctypes.CFUNCTYPE(None, ContextPtr, GeometryPtr, ctypes.c_uint32, ctypes.c_uint8, ctypes.POINTER(_Drive))(f_native)

    def _gain_ptr(self: "TransducerTest", geometry: Geometry) -> GainPtr:
        return Base().gain_transducer_test(self._f_native, ContextPtr(None), geometry._ptr)  # type: ignore[arg-type]
