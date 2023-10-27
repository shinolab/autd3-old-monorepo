"""
File: datagram.py
Project: internal
Created Date: 27/10/2023
Author: Shun Suzuki
-----
Last Modified: 27/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from abc import ABCMeta, abstractmethod

from pyautd3.geometry import Geometry
from pyautd3.native_methods.autd3capi_def import (
    DatagramPtr,
    DatagramSpecialPtr,
)

__all__ = []  # type: ignore[var-annotated]


class SpecialDatagram(metaclass=ABCMeta):
    @abstractmethod
    def __init__(self: "SpecialDatagram") -> None:
        pass

    @abstractmethod
    def _special_datagram_ptr(self: "SpecialDatagram") -> DatagramSpecialPtr:
        pass


class Datagram(metaclass=ABCMeta):
    @abstractmethod
    def __init__(self: "Datagram") -> None:
        pass

    @abstractmethod
    def _datagram_ptr(self: "Datagram", geometry: Geometry) -> DatagramPtr:
        pass
