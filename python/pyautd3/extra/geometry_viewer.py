'''
File: geometry_viewer.py
Project: extra
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 21/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


from pyautd3.autd import Geometry
from pyautd3.native_methods.autd3capi_extra_geometry_viewer import NativeMethods as ExtraGeometryViewer


class GeometryViewer:
    def __init__(self):
        self._width = 800
        self._height = 600
        self._vsync = True
        self._gpu_idx = 0

    def window_size(self, width: int, height: int):
        self._width = width
        self._height = height
        return self

    def vsync(self, value: bool):
        self._vsync = value
        return self

    def gpu_idx(self, value: int):
        self._gpu_idx = value
        return self

    def view(self, geometry: Geometry):
        ExtraGeometryViewer().init_dll()
        ExtraGeometryViewer().dll.AUTDExtraGeometryViewer(geometry._cnt, self._width, self._height, self._vsync, self._gpu_idx)
