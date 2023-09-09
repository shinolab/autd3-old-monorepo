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
from functools import reduce
import numpy as np
from typing import Iterable, Dict
from ctypes import POINTER, create_string_buffer

from pyautd3.autd import Datagram
from pyautd3.geometry import Device
from pyautd3.autd_error import AUTDError

from pyautd3.native_methods.autd3capi import Drive
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import DatagramPtr, DevicePtr, GainPtr, AUTD3_ERR


class IGain(Datagram, metaclass=ABCMeta):
    def __init__(self):
        super().__init__()

    def ptr(self, devices: Iterable[Device]) -> DatagramPtr:
        return Base().gain_into_datagram(self.gain_ptr(devices))

    @abstractmethod
    def gain_ptr(self, devices: Iterable[Device]) -> GainPtr:
        pass

    def with_cache(self):
        return Cache(self)


class Cache(IGain):
    _g: IGain
    _cache: Dict[int, np.ndarray]

    def __init__(self, g: IGain):
        super().__init__()
        self._g = g
        self._cache = {}

    def gain_ptr(self, devices: Iterable[Device]) -> GainPtr:
        devices = list(devices)

        device_indices = [dev.idx for dev in devices]

        if len(self._cache) != len(devices) or any([idx not in self._cache for idx in device_indices]):
            device_ptrs = np.array([dev.ptr() for dev in devices])
            device_ptrs_ = device_ptrs.ctypes.data_as(POINTER(DevicePtr))

            drives = [np.zeros(dev.num_transducers, dtype=Drive) for dev in devices]
            drives_ptrs = np.array([np.ctypeslib.as_ctypes(d) for d in drives])
            drives_ptrs_ = np.ctypeslib.as_ctypes(drives_ptrs)

            err = create_string_buffer(256)
            if Base().gain_calc(self._g.gain_ptr(devices),
                                device_ptrs_,  # type: ignore
                                drives_ptrs_, len(device_ptrs), err) == AUTD3_ERR:
                raise AUTDError(err)

            for i, dev in enumerate(devices):
                self._cache[dev.idx] = drives_ptrs[i]

        return reduce(
            lambda acc, dev: Base().gain_custom_set(acc, dev.idx,
                                                    self._cache[dev.idx],  # type: ignore
                                                    len(self._cache[dev.idx])),
            devices,
            Base().gain_custom(),
        )
