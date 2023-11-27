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


import functools

from pyautd3.emit_intensity import EmitIntensity
from pyautd3.geometry import Geometry, Transducer
from pyautd3.internal.gain import IGain
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import GainPtr


class TransducerTest(IGain):
    """Gain to drive only specified transducers."""

    _data: list[tuple[Transducer, float, EmitIntensity]]

    def __init__(self: "TransducerTest") -> None:
        super().__init__()
        self._data = []

    def set_drive(
        self: "TransducerTest",
        tr: Transducer,
        phase: float,
        intensity: int | EmitIntensity,
    ) -> "TransducerTest":
        """Set drive parameters.

        Arguments:
        ---------
            tr: transducer
            phase: Phase (from 0 to 2π)
            intensity: Emission intensity
        """
        self._data.append((tr, phase, EmitIntensity._cast(intensity)))
        return self

    def _gain_ptr(self: "TransducerTest", _: Geometry) -> GainPtr:
        return functools.reduce(
            lambda acc, v: Base().gain_transducer_test_set(
                acc,
                v[0]._ptr,
                v[1],
                v[2].value,
            ),
            self._data,
            Base().gain_transducer_test(),
        )
