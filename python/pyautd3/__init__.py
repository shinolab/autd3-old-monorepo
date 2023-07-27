"""
File: __init__.py
Project: pyautd3
Created Date: 24/05/2021
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


from pyautd3.autd import SilencerConfig
from pyautd3.autd import Controller, FirmwareInfo
from pyautd3.geometry import AUTD3, Geometry, Transducer
from pyautd3.autd import Amplitudes
from pyautd3.native_methods.autd3capi_def import (
    NUM_TRANS_IN_UNIT,
    NUM_TRANS_IN_X,
    NUM_TRANS_IN_Y,
    FPGA_CLK_FREQ,
    FPGA_SUB_CLK_FREQ,
    Level,
    TimerStrategy,
)
from pyautd3.native_methods.autd3capi_def import TRANS_SPACING_MM as TRANS_SPACING
from pyautd3.native_methods.autd3capi_def import DEVICE_WIDTH_MM as DEVICE_WIDTH
from pyautd3.native_methods.autd3capi_def import DEVICE_HEIGHT_MM as DEVICE_HEIGHT
from pyautd3.autd import Clear, UpdateFlags, Synchronize, ModDelayConfig, Stop

__all__ = [
    "SilencerConfig",
    "Controller",
    "AUTD3",
    "Geometry",
    "Transducer",
    "FirmwareInfo",
    "Amplitudes",
    "Clear",
    "UpdateFlags",
    "Synchronize",
    "ModDelayConfig",
    "Stop",
    "NUM_TRANS_IN_UNIT",
    "NUM_TRANS_IN_X",
    "NUM_TRANS_IN_Y",
    "TRANS_SPACING",
    "DEVICE_WIDTH",
    "DEVICE_HEIGHT",
    "FPGA_CLK_FREQ",
    "FPGA_SUB_CLK_FREQ",
    "Level",
    "TimerStrategy",
]

__version__ = "14.1.0"
