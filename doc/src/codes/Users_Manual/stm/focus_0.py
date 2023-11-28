import numpy as np
from pyautd3.stm import FocusSTM

center = autd.geometry.center + np.array([0.0, 0.0, 150.0])
point_num = 200
radius = 30.0
stm = FocusSTM(1.0).add_foci_from_iter(
    map(
        lambda theta: center + radius * np.array([np.cos(theta), np.sin(theta), 0]),
        map(lambda i: 2.0 * np.pi * i / point_num, range(point_num)),
    )
)
