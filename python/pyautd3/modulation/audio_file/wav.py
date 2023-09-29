'''
File: audio_file.py
Project: modulation
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 29/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

'''

import ctypes
from datetime import timedelta

from pyautd3.native_methods.autd3capi_modulation_audio_file import (
    NativeMethods as ModulationAudioFile,
)
from pyautd3.native_methods.autd3capi_def import ModulationPtr, FPGA_SUB_CLK_FREQ
from pyautd3.autd_error import AUTDError

from pyautd3.internal.modulation import IModulation

from typing import Optional


class Wav(IModulation):
    """Modulation constructed from a wav file.

    The data is resampled to the sampling frequency of the Modulation.
    """

    _path: str
    _freq_div: Optional[int]

    def __init__(self, path: str):
        """Constructor

        Arguments:
        - `path` - Path to the wav file
        """

        super().__init__()
        self._path = path
        self._freq_div = None

    def with_sampling_frequency_division(self, div: int) -> "Wav":
        """Set sampling frequency division

        Arguments:
        - `div` - Sampling frequency division.
                The sampling frequency will be `pyautd3.AUTD3.fpga_sub_clk_freq()` / `div`.
        """

        self._freq_div = div
        return self

    def with_sampling_frequency(self, freq: float) -> "Wav":
        """Set sampling frequency

        Arguments:
        - `freq` - Sampling frequency.
                The sampling frequency closest to `freq` from the possible sampling frequencies is set.
        """

        div = int(FPGA_SUB_CLK_FREQ / freq)
        return self.with_sampling_frequency_division(div)

    def with_sampling_period(self, period: timedelta) -> "Wav":
        """Set sampling period

        Arguments:
        - `period` - Sampling period.
                The sampling period closest to `period` from the possible sampling periods is set.
        """

        return self.with_sampling_frequency_division(int(FPGA_SUB_CLK_FREQ / 1000000000. * (period.total_seconds() * 1000. * 1000. * 1000.)))

    def modulation_ptr(self) -> ModulationPtr:
        err = ctypes.create_string_buffer(256)
        ptr = ModulationAudioFile().modulation_wav(self._path.encode("utf-8"), err)
        if ptr._0 is None:
            raise AUTDError(err)
        if self._freq_div is not None:
            ptr = ModulationAudioFile().modulation_wav_with_sampling_frequency_division(
                ptr, self._freq_div
            )
        return ptr
