'''
File: geometry_viewer.py
Project: example
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 02/02/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


from pyautd3 import GeometryBuilder, DEVICE_WIDTH
from pyautd3.extra import GeometryViewer
from math import pi

if __name__ == '__main__':
    geometry = GeometryBuilder()\
        .add_device([0., 0., 0.], [0., 0., 0.])\
        .add_device([0., 0., DEVICE_WIDTH], [0., pi / 2, 0.])\
        .add_device([DEVICE_WIDTH, 0., DEVICE_WIDTH], [0., pi, 0.])\
        .add_device([DEVICE_WIDTH, 0., 0.], [0., -pi / 2, 0.])\
        .build()

    GeometryViewer().window_size(800, 600).vsync(True).view(geometry)
