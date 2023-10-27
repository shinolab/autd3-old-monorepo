"""
File: gain.py
Project: internal
Created Date: 29/08/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from abc import ABCMeta, abstractmethod

from pyautd3.geometry import Geometry
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import DatagramPtr, GainPtr

from .datagram import Datagram

__all__ = []  # type: ignore[var-annotated]


class IGain(Datagram, metaclass=ABCMeta):
    def __init__(self: "IGain") -> None:
        super().__init__()

    def _datagram_ptr(self: "IGain", geometry: Geometry) -> DatagramPtr:
        return Base().gain_into_datagram(self._gain_ptr(geometry))

    @abstractmethod
    def _gain_ptr(self: "IGain", geometry: Geometry) -> GainPtr:
        pass
