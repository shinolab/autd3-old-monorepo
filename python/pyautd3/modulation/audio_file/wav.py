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

from pathlib import Path

from pyautd3.internal.modulation import IModulationWithSamplingConfig
from pyautd3.internal.utils import _validate_ptr
from pyautd3.native_methods.autd3capi_def import ModulationPtr
from pyautd3.native_methods.autd3capi_modulation_audio_file import (
    NativeMethods as ModulationAudioFile,
)


class Wav(IModulationWithSamplingConfig):
    """Modulation constructed from a wav file.

    The data is resampled to the sampling frequency of the Modulation.
    """

    _path: Path

    def __init__(self: "Wav", path: Path) -> None:
        """Constructor.

        Arguments:
        ---------
            path: Path to the wav file
        """
        super().__init__()
        self._path = path

    def _modulation_ptr(self: "Wav") -> ModulationPtr:
        ptr = _validate_ptr(ModulationAudioFile().modulation_wav(str(self._path).encode("utf-8")))
        if self._config is not None:
            ptr = ModulationAudioFile().modulation_wav_with_sampling_config(ptr, self._config._internal)
        return ptr
