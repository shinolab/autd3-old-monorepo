'''
File: simulator.py
Project: extra
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 29/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


from pyautd3.native_methods.autd3capi_extra_simulator import NativeMethods as ExtraSimulator


class Simulator:
    def __init__(self):
        self._settings_path = "settings.json"
        self._vsync = True
        self._gpu_idx = 0

    def settings_path(self, value: str):
        self._settings_path = value
        return self

    def vsync(self, value: bool):
        self._vsync = value
        return self

    def gpu_idx(self, value: int):
        self._gpu_idx = value
        return self

    def run(self):
        ExtraSimulator().init_dll()
        ExtraSimulator().dll.AUTDExtraSimulator(self._settings_path.encode('utf-8'), self._vsync, self._gpu_idx)
