"""
File: simulator.py
Project: extra
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


from ctypes import c_void_p
import ctypes
from pyautd3.autd_error import AUTDError
from pyautd3.native_methods.autd3capi_simulator import NativeMethods as ExtraSimulator


class Simulator:
    _handle: c_void_p

    def __init__(self):
        self._handle = ExtraSimulator().simulator()

    def port(self, port: int) -> "Simulator":
        self._handle = ExtraSimulator().simulator_port(self._handle, port)
        return self

    def window_size(self, width: int, height: int) -> "Simulator":
        self._handle = ExtraSimulator().simulator_window_size(
            self._handle, width, height
        )
        return self

    def settings_path(self, value: str) -> "Simulator":
        err = ctypes.create_string_buffer(256)
        handle = ExtraSimulator().simulator_settings_path(
            self._handle, value.encode("utf-8"), err
        )
        if handle:
            self._handle = handle
        return self

    def vsync(self, value: bool) -> "Simulator":
        self._handle = ExtraSimulator().simulator_vsync(self._handle, value)
        return self

    def gpu_idx(self, value: int) -> "Simulator":
        self._handle = ExtraSimulator().simulator_gpu_idx(self._handle, value)
        return self

    def run(self) -> int:
        return int(ExtraSimulator().simulator_run(self._handle))

    def save_settings(self, value: str):
        err = ctypes.create_string_buffer(256)
        if not ExtraSimulator().simulator_save_settings(
            self._handle, value.encode("utf-8"), err
        ):
            raise AUTDError(err)
