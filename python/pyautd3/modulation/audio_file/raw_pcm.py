"""
File: raw_pcm.py
Project: audio_file
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from pathlib import Path

from pyautd3.internal.modulation import IModulationWithSamplingConfig
from pyautd3.internal.utils import _validate_ptr
from pyautd3.native_methods.autd3capi_def import ModulationPtr
from pyautd3.native_methods.autd3capi_modulation_audio_file import (
    NativeMethods as ModulationAudioFile,
)


class RawPCM(IModulationWithSamplingConfig):
    """Modulation constructed from a raw PCM data.

    The data must be 8bit unsinged integer.

    The data is resampled to the sampling frequency of the Modulation.
    """

    _path: Path
    _sample_rate: int

    def __init__(self: "RawPCM", path: Path, sample_rate: int) -> None:
        """Constructor.

        Arguments:
        ---------
            path: Path to the raw PCM data
            sample_rate: Sampling frequency of the raw PCM data
        """
        super().__init__()
        self._path = path
        self._sample_rate = sample_rate

    def _modulation_ptr(self: "RawPCM") -> ModulationPtr:
        ptr = _validate_ptr(ModulationAudioFile().modulation_raw_pcm(str(self._path).encode("utf-8"), self._sample_rate))
        if self._config is not None:
            ptr = ModulationAudioFile().modulation_raw_pcm_with_sampling_config(ptr, self._config._internal)
        return ptr
