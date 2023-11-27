"""
File: modulation.py
Project: modulation
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 27/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from abc import ABCMeta, abstractmethod
from ctypes import POINTER, c_uint8

import numpy as np

from pyautd3.internal.modulation import IModulation
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import ModulationPtr
from pyautd3.sampling_config import SamplingConfiguration


class Modulation(IModulation, metaclass=ABCMeta):
    """Base class of custom Modulation."""

    _config: SamplingConfiguration

    def __init__(self: "Modulation", config: SamplingConfiguration) -> None:
        """Constructor.

        Arguments:
        ---------
            config: sampling configuration
        """
        super().__init__()
        self._config = config

    @abstractmethod
    def calc(self: "Modulation") -> np.ndarray:
        """Calculate modulation data."""

    def _modulation_ptr(self: "Modulation") -> ModulationPtr:
        data = np.fromiter((m.value for m in self.calc()), dtype=c_uint8)
        size = len(data)
        return Base().modulation_custom(
            self._config._internal,
            data.ctypes.data_as(POINTER(c_uint8)),  # type: ignore[arg-type]
            size,
        )
