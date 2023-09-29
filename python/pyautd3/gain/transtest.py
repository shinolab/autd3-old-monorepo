'''
File: transtest.py
Project: gain
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 29/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


import functools
from typing import List, Tuple

from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import GainPtr
from pyautd3.geometry import Geometry
from ..internal.gain import IGain


class TransducerTest(IGain):
    """Gain to drive only specified transducers

    """

    _data: List[Tuple[int, int, float, float]]

    def __init__(self):
        super().__init__()
        self._data = []

    def set(self, dev_idx: int, tr_idx: int, phase: float, amp: float) -> "TransducerTest":
        """Set drive parameters

        Arguments:
        - `dev_idx` - Device index
        - `tr_idx` - Local transducer index
        - `phase` - Phase (from 0 to 2π)
        - `amp` - Normalized amplitude (from 0 to 1)
        """

        self._data.append((dev_idx, tr_idx, phase, amp))
        return self

    def gain_ptr(self, _: Geometry) -> GainPtr:
        return functools.reduce(
            lambda acc, v: Base().gain_transducer_test_set(acc, v[0], v[1], v[2], v[3]),
            self._data,
            Base().gain_transducer_test(),
        )
