"""
File: holo.py
Project: holo
Created Date: 05/06/2023
Author: Shun Suzuki
-----
Last Modified: 05/06/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""

import numpy as np
from typing import List, Optional

from pyautd3.gain.gain import IGain
from pyautd3.native_methods.autd3capi_def import BackendPtr
from .backend import Backend


class Holo(IGain):
    _foci: List[float]
    _amps: List[float]
    _backend: Backend

    def __init__(self, backend: Optional[Backend] = None):
        self._foci = []
        self._amps = []
        self._backend = backend if backend is not None else Backend(BackendPtr(None))

    def add_focus(self, focus: np.ndarray, amp: float):
        self._foci.append(focus[0])
        self._foci.append(focus[1])
        self._foci.append(focus[2])
        self._amps.append(amp)
