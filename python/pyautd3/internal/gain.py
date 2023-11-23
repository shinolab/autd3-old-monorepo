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
from collections.abc import Callable
from typing import TYPE_CHECKING, TypeVar

from pyautd3.drive import Drive
from pyautd3.geometry import Device, Geometry, Transducer
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import DatagramPtr, GainPtr

from .datagram import Datagram

if TYPE_CHECKING:
    from pyautd3.gain.cache import Cache
    from pyautd3.gain.transform import Transform

__all__ = []  # type: ignore[var-annotated]

G = TypeVar("G", bound="IGain")


class IGain(Datagram, metaclass=ABCMeta):
    def __init__(self: "IGain") -> None:
        super().__init__()

    def _datagram_ptr(self: "IGain", geometry: Geometry) -> DatagramPtr:
        return Base().gain_into_datagram(self._gain_ptr(geometry))

    @abstractmethod
    def _gain_ptr(self: "IGain", geometry: Geometry) -> GainPtr:
        pass

    def with_cache(self: G) -> "Cache":  # type: ignore[empty-body]
        # This function is implemented in cache.py
        pass

    def with_transform(self: G, f: Callable[[Device, Transducer, Drive], Drive]) -> "Transform":  # type: ignore[empty-body]
        # This function is implemented in transform.py
        pass
