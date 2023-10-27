"""
File: null.py
Project: gain
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 29/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from pyautd3.geometry import Geometry
from pyautd3.internal.gain import IGain
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import GainPtr


class Null(IGain):
    """Gain to output nothing."""

    def __init__(self: "Null") -> None:
        super().__init__()

    def _gain_ptr(self: "Null", _: Geometry) -> GainPtr:
        return Base().gain_null()
