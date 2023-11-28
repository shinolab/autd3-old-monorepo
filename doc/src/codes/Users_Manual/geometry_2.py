import numpy as np
from pyautd3 import AUTD3, Controller
from pyautd3.geometry import EulerAngles, rad

Controller.builder()\
    .add_device(AUTD3([0.0, 0.0, 0.0]))\
    .add_device(
        AUTD3([AUTD3.device_width(), 0.0, 0.0]).with_rotation(
            EulerAngles.from_zyz(0 * rad, np.pi / 2 * rad, 0 * rad)))
