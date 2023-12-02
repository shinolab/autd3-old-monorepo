"""
File: group.py
Project: gain
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 27/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from collections.abc import Callable
from ctypes import POINTER, c_int32, c_uint32
from typing import Generic, TypeVar

import numpy as np

from pyautd3.autd_error import UnknownGroupKeyError
from pyautd3.geometry import Device, Geometry, Transducer
from pyautd3.internal.gain import IGain
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import GainPtr

K = TypeVar("K")


class Group(IGain, Generic[K]):
    """Gain to group gains by key."""

    _map: dict[K, IGain]
    _f: Callable[[Device, Transducer], K | None]

    def __init__(self: "Group", f: Callable[[Device, Transducer], K | None]) -> None:
        """Constructor.

        Arguments:
        ---------
            f: Function to get key from device and transducer
        """
        super().__init__()
        self._map = {}
        self._f = f

    def set_gain(self: "Group", key: K, gain: IGain) -> "Group":
        """Set gain.

        Arguments:
        ---------
            key: Key
            gain: Gain
        """
        self._map[key] = gain
        return self

    def _gain_ptr(self: "Group", geometry: Geometry) -> GainPtr:
        keymap: dict[K, int] = {}

        device_indices = np.array([dev.idx for dev in geometry.devices()])

        gain_group_map = Base().gain_group_create_map(np.ctypeslib.as_ctypes(device_indices.astype(c_uint32)), len(device_indices))
        k: int = 0
        for dev in geometry.devices():
            m = np.zeros(dev.num_transducers, dtype=np.int32)
            for tr in dev:
                key = self._f(dev, tr)
                if key is not None:
                    if key not in keymap:
                        keymap[key] = k
                        k += 1
                    m[tr.idx] = keymap[key]
                else:
                    m[tr.idx] = -1
            gain_group_map = Base().gain_group_map_set(gain_group_map, dev.idx, np.ctypeslib.as_ctypes(m.astype(c_int32)))

        keys: np.ndarray = np.ndarray(len(self._map), dtype=np.int32)
        values: np.ndarray = np.ndarray(len(self._map), dtype=GainPtr)
        for i, (key, value) in enumerate(self._map.items()):
            if key not in keymap:
                raise UnknownGroupKeyError
            keys[i] = keymap[key]
            values[i]["_0"] = value._gain_ptr(geometry)._0
        return Base().gain_group(
            gain_group_map,
            np.ctypeslib.as_ctypes(keys.astype(c_int32)),
            values.ctypes.data_as(POINTER(GainPtr)),  # type: ignore[arg-type]
            len(keys),
        )
