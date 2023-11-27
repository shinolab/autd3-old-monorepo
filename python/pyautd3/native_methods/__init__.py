'''
File: __init__.py
Project: native_methods
Created Date: 09/10/2022
Author: Shun Suzuki
-----
Last Modified: 27/11/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

'''

import os
import os.path
import platform
import sys

from .autd3capi import NativeMethods as Base
from .autd3capi_def import NativeMethods as Def
from .autd3capi_gain_holo import NativeMethods as GainHolo
from .autd3capi_modulation_audio_file import NativeMethods as ModulationAudioFile
from .autd3capi_link_simulator import NativeMethods as LinkSimulator
from .autd3capi_link_twincat import NativeMethods as LinkTwincAT
from .autd3capi_link_soem import NativeMethods as LinkSOEM

_PLATFORM = platform.system()
_PREFIX = ""
_BIN_EXT = ""
if _PLATFORM == "Windows":
    _BIN_EXT = ".dll"
elif _PLATFORM == "Darwin":
    _PREFIX = "lib"
    _BIN_EXT = ".dylib"
elif _PLATFORM == "Linux":
    _PREFIX = "lib"
    _BIN_EXT = ".so"
else:
    raise ImportError("Not supported OS")

_LIB_PATH = os.path.join(os.path.dirname(__file__), "..", "bin")

Def().init_dll(_LIB_PATH, _PREFIX, _BIN_EXT)
Base().init_dll(_LIB_PATH, _PREFIX, _BIN_EXT)
GainHolo().init_dll(_LIB_PATH, _PREFIX, _BIN_EXT)
ModulationAudioFile().init_dll(_LIB_PATH, _PREFIX, _BIN_EXT)
LinkSimulator().init_dll(_LIB_PATH, _PREFIX, _BIN_EXT)
LinkSOEM().init_dll(_LIB_PATH, _PREFIX, _BIN_EXT)
if sys.platform == "win32":
    try:
        os.add_dll_directory("C:\\TwinCAT\\Common64")
    except FileNotFoundError:
        pass
LinkTwincAT().init_dll(_LIB_PATH, _PREFIX, _BIN_EXT)
