"""
File: backend.py
Project: holo
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""

from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo
from pyautd3.native_methods.autd3capi_def import BackendPtr


class Backend:
    _ptr: BackendPtr

    def __init__(self, ptr: BackendPtr):
        self._ptr = ptr

    def ptr(self) -> BackendPtr:
        return self._ptr


class DefaultBackend(Backend):
    def __init__(self):
        super().__init__(GainHolo().default_backend())
