'''
File: __init__.py
Project: native_methods
Created Date: 09/10/2022
Author: Shun Suzuki
-----
Last Modified: 21/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

import os
import os.path
import platform

from .autd3capi import NativeMethods as Base
from .autd3capi_gain_holo import NativeMethods as GainHolo
from .autd3capi_modulation_audio_file import NativeMethods as ModulationAudioFile
from .autd3capi_backend_cuda import NativeMethods as BackendCUDA
from .autd3capi_link_simulator import NativeMethods as LinkSimulator
from .autd3capi_link_remote_twincat import NativeMethods as LinkRemoteTwinCAT
from .autd3capi_link_twincat import NativeMethods as LinkTwincAT
from .autd3capi_link_soem import NativeMethods as LinkSOEM
from .autd3capi_extra_simulator import NativeMethods as ExtraSimulator
from .autd3capi_extra_geometry_viewer import NativeMethods as ExtraGeometryViewer

_PLATFORM = platform.system()
_TARGET_OS = ''
_ARCH = ''
_PREFIX = ''
_BIN_EXT = ''
if _PLATFORM == 'Windows':
    _BIN_EXT = '.dll'
    _TARGET_OS = 'win'
    _ARCH = 'x64' if platform.machine().endswith('64') else 'x86'
elif _PLATFORM == 'Darwin':
    _PREFIX = 'lib'
    _BIN_EXT = '.dylib'
    _TARGET_OS = 'macos'
    _ARCH = 'universal'
elif _PLATFORM == 'Linux':
    _PREFIX = 'lib'
    _BIN_EXT = '.so'
    _TARGET_OS = 'linux'
    if platform.machine().startswith('aarch64'):
        _ARCH = 'arm64'
    elif platform.machine().startswith('arm64'):
        _ARCH = 'arm64'
    elif platform.machine().startswith('arm'):
        _ARCH = 'arm32'
    elif platform.machine().endswith('64'):
        _ARCH = 'x64'
    else:
        raise ImportError('Cannot identify CPU architecture')
else:
    raise ImportError('Not supported OS')

_LIB_PATH = os.path.join(os.path.dirname(__file__), '..', 'bin', f'{_TARGET_OS}_{_ARCH}')

Base().init_path(_LIB_PATH, _PREFIX, _BIN_EXT)
GainHolo().init_path(_LIB_PATH, _PREFIX, _BIN_EXT)
ModulationAudioFile().init_path(_LIB_PATH, _PREFIX, _BIN_EXT)
BackendCUDA().init_path(_LIB_PATH, _PREFIX, _BIN_EXT)
Base().init_dll()
GainHolo().init_dll()
ModulationAudioFile().init_dll()

LinkRemoteTwinCAT().init_path(_LIB_PATH, _PREFIX, _BIN_EXT)
LinkSimulator().init_path(_LIB_PATH, _PREFIX, _BIN_EXT)
LinkSOEM().init_path(_LIB_PATH, _PREFIX, _BIN_EXT)
LinkTwincAT().init_path(_LIB_PATH, _PREFIX, _BIN_EXT)
LinkRemoteTwinCAT().init_dll()
LinkSimulator().init_dll()
if platform.system() == 'Windows':
    LinkTwincAT().init_dll()

ExtraSimulator().init_path(_LIB_PATH, _PREFIX, _BIN_EXT)
ExtraGeometryViewer().init_path(_LIB_PATH, _PREFIX, _BIN_EXT)
