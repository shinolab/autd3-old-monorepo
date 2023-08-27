"""
File: gain.py
Project: gain
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""

from abc import ABCMeta, abstractmethod
import numpy as np
from typing import Iterator
from ctypes import create_string_buffer

from pyautd3.autd import Body, Geometry
from pyautd3.autd_error import AUTDError

from pyautd3.native_methods.autd3capi import Drive
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import DatagramBodyPtr, GainPtr, AUTD3_ERR


class IGain(Body, metaclass=ABCMeta):
    def __init__(self):
        super().__init__()

    def ptr(self, geometry: Geometry) -> DatagramBodyPtr:
        return Base().gain_into_datagram(self.gain_ptr(geometry))

    @abstractmethod
    def gain_ptr(self, geometry: Geometry) -> GainPtr:
        pass

    def with_cache(self, geometry: Geometry):
        return Cache(self, geometry)


class Cache(IGain):
    _drives: np.ndarray

    def __init__(self, g: IGain, geometry: Geometry):
        super().__init__()
        self._drives = np.zeros(geometry.num_transducers, dtype=Drive)
        dp = np.ctypeslib.as_ctypes(self._drives)
        err = create_string_buffer(256)
        if Base().gain_calc(g.gain_ptr(geometry), geometry.ptr(), dp, err) == AUTD3_ERR:
            raise AUTDError(err)

    def gain_ptr(self, _: Geometry) -> GainPtr:
        size = len(self._drives)
        dp = np.ctypeslib.as_ctypes(self._drives)
        return Base().gain_custom(dp, size)

    @property
    def drives(self) -> np.ndarray:
        return self._drives

    def __getitem__(self, key: int) -> Drive:
        return self._drives[key]

    def __iter__(self) -> Iterator[Drive]:
        return iter(self._drives)
