"""
File: holo.py
Project: holo
Created Date: 10/10/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


import functools
from collections.abc import Iterable

import numpy as np

from pyautd3.gain.gain import IGain

from .backend import Backend
from .constraint import AmplitudeConstraint

__all__ = []  # type: ignore[var-annotated]


class Holo(IGain):
    _foci: list[float]
    _amps: list[float]
    _repeat: int | None
    _constraint: AmplitudeConstraint | None

    def __init__(self: "Holo") -> None:
        self._foci = []
        self._amps = []
        self._repeat = None
        self._constraint = None

    def add_focus(self: "Holo", focus: np.ndarray, amp: float) -> "Holo":
        """Add focus.

        Arguments:
        ---------
            focus: Focus point
            amp: Focus amplitude
        """
        self._foci.append(focus[0])
        self._foci.append(focus[1])
        self._foci.append(focus[2])
        self._amps.append(amp)
        return self

    def add_foci_from_iter(self: "Holo", iterable: Iterable[tuple[np.ndarray, float]]) -> "Holo":
        """Add foci from iterable.

        Arguments:
        ---------
            iterable: Iterable of focus point and amplitude.
        """
        return functools.reduce(
            lambda acc, x: acc.add_focus(x[0], x[1]),
            iterable,
            self,
        )

    def with_constraint(self: "Holo", constraint: AmplitudeConstraint) -> "Holo":
        """Set amplitude constraint.

        Arguments:
        ---------
            constraint: Amplitude constraint
        """
        self._constraint = constraint
        return self


class HoloWithBackend(Holo):
    _backend: Backend

    def __init__(self: "HoloWithBackend", backend: Backend) -> None:
        super().__init__()
        self._backend = backend
