"""
File: audio_file.py
Project: modulation
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""

import ctypes

from pyautd3.autd_error import AUTDError
from pyautd3.internal.modulation import IModulationWithFreqDiv
from pyautd3.native_methods.autd3capi_def import ModulationPtr
from pyautd3.native_methods.autd3capi_modulation_audio_file import (
    NativeMethods as ModulationAudioFile,
)


class Wav(IModulationWithFreqDiv):
    """Modulation constructed from a wav file.

    The data is resampled to the sampling frequency of the Modulation.
    """

    _path: str

    def __init__(self: "Wav", path: str) -> None:
        """Constructor.

        Arguments:
        ---------
            path: Path to the wav file
        """
        super().__init__()
        self._path = path

    def _modulation_ptr(self: "Wav") -> ModulationPtr:
        err = ctypes.create_string_buffer(256)
        ptr = ModulationAudioFile().modulation_wav(self._path.encode("utf-8"), err)
        if ptr._0 is None:
            raise AUTDError(err)
        if self._freq_div is not None:
            ptr = ModulationAudioFile().modulation_wav_with_sampling_frequency_division(ptr, self._freq_div)
        return ptr
