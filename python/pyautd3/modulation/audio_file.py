"""
File: audio_file.py
Project: modulation
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""

import ctypes

from pyautd3.native_methods.autd3capi_modulation_audio_file import (
    NativeMethods as ModulationAudioFile,
)
from pyautd3.native_methods.autd3capi_def import ModulationPtr
from pyautd3.autd_error import AUTDError

from .modulation import IModulation


class Wav(IModulation):
    _path: str

    def __init__(self, path: str):
        super().__init__()
        self._path = path

    def modulation_ptr(self) -> ModulationPtr:
        err = ctypes.create_string_buffer(256)
        ptr = ModulationAudioFile().modulation_wav(self._path.encode("utf-8"), err)
        if ptr._0 is None:
            raise AUTDError(err)
        return ptr
