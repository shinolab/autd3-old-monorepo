'''
File: test_geometry_viewer.py
Project: extra
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 20/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from pyautd3 import Controller, AUTD3
from pyautd3.link import Audit
from pyautd3.extra import GeometryViewer

import pytest


@pytest.mark.geometry_viewer
def test_geometry_viewer():
    autd = Controller.builder()\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))\
        .add_device(AUTD3.from_quaternion([0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]))\
        .open_with(Audit())

    GeometryViewer().with_window_size(800, 600).with_vsync(True).run(autd.geometry)
