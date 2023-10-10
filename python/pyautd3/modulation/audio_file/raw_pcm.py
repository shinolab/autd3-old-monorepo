'''
File: raw_pcm.py
Project: audio_file
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


import ctypes
from datetime import timedelta

from pyautd3.native_methods.autd3capi_modulation_audio_file import (
    NativeMethods as ModulationAudioFile,
)
from pyautd3.native_methods.autd3capi_def import ModulationPtr, FPGA_SUB_CLK_FREQ
from pyautd3.autd_error import AUTDError

from pyautd3.internal.modulation import IModulationWithFreqDiv

from typing import Optional


class RawPCM(IModulationWithFreqDiv):
    """Modulation constructed from a raw PCM data.

    The data must be 8bit unsinged integer.

    The data is resampled to the sampling frequency of the Modulation.
    """

    _path: str
    _sample_rate: int

    def __init__(self, path: str, sample_rate: int):
        """Constructor

        Arguments:
        - `path` - Path to the raw PCM data
        - `sample_rate` - Sampling frequency of the raw PCM data
        """

        super().__init__()
        self._path = path
        self._sample_rate = sample_rate

    def modulation_ptr(self) -> ModulationPtr:
        err = ctypes.create_string_buffer(256)
        ptr = ModulationAudioFile().modulation_raw_pcm(self._path.encode("utf-8"), self._sample_rate, err)
        if ptr._0 is None:
            raise AUTDError(err)
        if self._freq_div is not None:
            ptr = ModulationAudioFile().modulation_raw_pcm_with_sampling_frequency_division(
                ptr, self._freq_div
            )
        return ptr
