'''
File: holo.py
Project: holo
Created Date: 10/10/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


import functools
import numpy as np
from typing import Iterable, Optional, List, Tuple

from .backend import Backend
from .constraint import AmplitudeConstraint


from pyautd3.gain.gain import IGain


class Holo(IGain):
    _foci: List[float]
    _amps: List[float]
    _backend: Optional[Backend]
    _repeat: Optional[int]
    _constraint: Optional[AmplitudeConstraint]

    def __init__(self, backend: Optional[Backend]):
        self._foci = []
        self._amps = []
        self._backend = backend
        self._repeat = None
        self._constraint = None

    def add_focus(self, focus: np.ndarray, amp: float):
        """Add focus

        Arguments:
        - `focus` - Focus point
        - `amp` - Focus amplitude
        """

        assert len(focus) == 3

        self._foci.append(focus[0])
        self._foci.append(focus[1])
        self._foci.append(focus[2])
        self._amps.append(amp)
        return self

    def add_foci_from_iter(
        self, iterable: Iterable[Tuple[np.ndarray, float]]
    ):
        """Add foci from iterable

        Arguments:
        - `iterable` - Iterable of focus point and amplitude
        """

        return functools.reduce(
            lambda acc, x: acc.add_focus(x[0], x[1]),
            iterable,
            self,
        )

    def with_constraint(self, constraint: AmplitudeConstraint):
        """Set amplitude constraint

        Arguments:
        - `constraint` - Amplitude constraint
        """

        self._constraint = constraint
        return self
