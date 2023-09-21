'''
File: geometry_viewer.py
Project: extra
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 22/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

'''


import ctypes

from pyautd3.autd import Geometry
from pyautd3.native_methods.autd3capi_geometry_viewer import (
    NativeMethods as ExtraGeometryViewer,
)
from pyautd3.native_methods.autd3capi_geometry_viewer import GeometryViewerPtr
from pyautd3.autd_error import AUTDError


class GeometryViewer:
    _handle: GeometryViewerPtr

    def __init__(self):
        self._handle = ExtraGeometryViewer().geometry_viewer()

    def with_window_size(self, width: int, height: int) -> "GeometryViewer":
        self._handle = ExtraGeometryViewer().geometry_viewer_with_size(
            self._handle, width, height
        )
        return self

    def with_vsync(self, value: bool) -> "GeometryViewer":
        self._handle = ExtraGeometryViewer().geometry_viewer_with_vsync(self._handle, value)
        return self

    def run(self, geometry: Geometry) -> int:
        err = ctypes.create_string_buffer(256)
        res = int(ExtraGeometryViewer().geometry_viewer_run(self._handle, geometry._ptr, err))
        if res < 0:
            raise AUTDError(err)
        return res
