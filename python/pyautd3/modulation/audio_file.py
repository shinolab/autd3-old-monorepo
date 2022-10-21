'''
File: audio_file.py
Project: modulation
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 21/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

from ctypes import byref

from pyautd3.native_methods.autd3capi_modulation_audio_file import NativeMethods as ModulationAudioFile

from .modulation import Modulation


class RawPCM(Modulation):
    def __init__(self, path: str, sampling_freq: float, mod_freq_div: int):
        super().__init__()
        ModulationAudioFile().dll.AUTDModulationRawPCM(byref(self.ptr), path.encode('utf-8'), sampling_freq, mod_freq_div)

    def __del__(self):
        super().__del__()


class Wav(Modulation):
    def __init__(self, path: str, mod_freq_div: int):
        super().__init__()
        ModulationAudioFile().dll.AUTDModulationWav(byref(self.ptr), path.encode('utf-8'), mod_freq_div)

    def __del__(self):
        super().__del__()
