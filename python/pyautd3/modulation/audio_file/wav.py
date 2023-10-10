'''
File: audio_file.py
Project: modulation
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
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

from pyautd3.internal.modulation import IModulationWithFreqDiv


class Wav(IModulationWithFreqDiv):
    """Modulation constructed from a wav file.

    The data is resampled to the sampling frequency of the Modulation.
    """

    _path: str

    def __init__(self, path: str):
        """Constructor

        Arguments:
        - `path` - Path to the wav file
        """

        super().__init__()
        self._path = path

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
