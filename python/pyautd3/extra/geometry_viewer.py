"""
File: geometry_viewer.py
Project: extra
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


from pyautd3.autd import Geometry
from pyautd3.native_methods.autd3capi_geometry_viewer import (
    NativeMethods as ExtraGeometryViewer,
)
from pyautd3.native_methods.autd3capi_geometry_viewer import GeometryViewerPtr


class GeometryViewer:
    _handle: GeometryViewerPtr

    def __init__(self):
        self._handle = ExtraGeometryViewer().geometry_viewer()

    def window_size(self, width: int, height: int) -> "GeometryViewer":
        self._handle = ExtraGeometryViewer().geometry_viewer_size(
            self._handle, width, height
        )
        return self

    def vsync(self, value: bool) -> "GeometryViewer":
        self._handle = ExtraGeometryViewer().geometry_viewer_vsync(self._handle, value)
        return self

    def run(self, geometry: Geometry) -> int:
        return int(
            ExtraGeometryViewer().geometry_viewer_run(self._handle, geometry._ptr)
        )
