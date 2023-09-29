'''
File: group.py
Project: gain
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 29/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


import numpy as np
from typing import Optional, Callable, TypeVar, Generic, Dict
from ctypes import POINTER, c_int32, c_uint32
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import GainPtr
from pyautd3.geometry import Transducer, Geometry, Device
from ..internal.gain import IGain


K = TypeVar("K")


class Group(IGain, Generic[K]):
    _map: Dict[K, IGain]
    _f: Callable[[Device, Transducer], Optional[K]]

    def __init__(self, f: Callable[[Device, Transducer], Optional[K]]):
        """Constructor

        Arguments:
        - `f` - Function to get key from device and transducer
        """

        super().__init__()
        self._map = {}
        self._f = f

    def set(self, key: K, gain: IGain) -> "Group":
        """Set gain

        Arguments:
        - `key` - Key
        - `gain` - Gain
        """

        self._map[key] = gain
        return self

    def gain_ptr(self, geometry: Geometry) -> GainPtr:

        keymap: Dict[K, int] = {}

        device_indices = np.array([dev.idx for dev in geometry.devices()])
        print(device_indices)

        map = Base().gain_group_create_map(np.ctypeslib.as_ctypes(device_indices.astype(c_uint32)), len(device_indices))
        k: int = 0
        for dev in geometry.devices():
            m = np.zeros(dev.num_transducers, dtype=np.int32)
            for tr in dev:
                key = self._f(dev, tr)
                if key is not None:
                    if key not in keymap:
                        keymap[key] = k
                        k += 1
                    m[tr.local_idx] = keymap[key]
                else:
                    m[tr.local_idx] = -1
            map = Base().gain_group_map_set(map, dev.idx, np.ctypeslib.as_ctypes(m.astype(c_int32)))

        keys: np.ndarray = np.ndarray(len(self._map), dtype=np.int32)
        values: np.ndarray = np.ndarray(len(self._map), dtype=GainPtr)
        for i, (key, value) in enumerate(self._map.items()):
            keys[i] = keymap[key]
            values[i]["_0"] = value.gain_ptr(geometry)._0
        return Base().gain_group(
            map,
            np.ctypeslib.as_ctypes(keys.astype(c_int32)),
            values.ctypes.data_as(POINTER(GainPtr)),  # type: ignore
            len(keys),
        )
