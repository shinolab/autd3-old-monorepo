"""
File: __init__.py
Project: native_methods
Created Date: 09/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""

import os
import os.path
import platform

from .autd3capi import NativeMethods as Base
from .autd3capi_gain_holo import NativeMethods as GainHolo
from .autd3capi_modulation_audio_file import NativeMethods as ModulationAudioFile
from .autd3capi_link_simulator import NativeMethods as LinkSimulator
from .autd3capi_link_twincat import NativeMethods as LinkTwincAT
from .autd3capi_link_soem import NativeMethods as LinkSOEM
from .autd3capi_simulator import NativeMethods as ExtraSimulator
from .autd3capi_geometry_viewer import NativeMethods as ExtraGeometryViewer
from .autd3capi_backend_cuda import NativeMethods as BackendCUDA

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

Base().init_dll(_LIB_PATH, _PREFIX, _BIN_EXT)
GainHolo().init_dll(_LIB_PATH, _PREFIX, _BIN_EXT)
ModulationAudioFile().init_dll(_LIB_PATH, _PREFIX, _BIN_EXT)
LinkSimulator().init_dll(_LIB_PATH, _PREFIX, _BIN_EXT)
LinkSOEM().init_dll(_LIB_PATH, _PREFIX, _BIN_EXT)
LinkTwincAT().init_dll(_LIB_PATH, _PREFIX, _BIN_EXT)
ExtraSimulator().init_dll(_LIB_PATH, _PREFIX, _BIN_EXT)
ExtraGeometryViewer().init_dll(_LIB_PATH, _PREFIX, _BIN_EXT)
BackendCUDA().init_dll(_LIB_PATH, _PREFIX, _BIN_EXT)
