"""
File: greedy.py
Project: holo
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


from .holo import Holo


from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo


class Greedy(Holo):
    def __init__(self):
        super().__init__()
        self.ptr = GainHolo().gain_holo_greedy()

    def phase_div(self, value: int):
        GainHolo().gain_holo_greedy_phase_div(self.ptr, value)

    def __del__(self):
        super().__del__()
