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
from ctypes import c_double

import numpy as np

from pyautd3.internal.modulation import IModulation
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import ModulationPtr


class Modulation(IModulation, metaclass=ABCMeta):
    """Base class of custom Modulation."""

    _freq_div: int

    def __init__(self: "Modulation", freq_div: int) -> None:
        """Constructor.

        Arguments:
        ---------
            freq_div: int
          The sampling frequency will be `pyautd3.AUTD3.fpga_sub_clk_freq()` / `freq_div`.
        """
        super().__init__()
        self._freq_div = freq_div

    @abstractmethod
    def calc(self: "Modulation") -> np.ndarray:
        """Calculate modulation data."""

    def _modulation_ptr(self: "Modulation") -> ModulationPtr:
        data = self.calc()
        size = len(data)
        return Base().modulation_custom(self._freq_div, np.ctypeslib.as_ctypes(data.astype(c_double)), size)
