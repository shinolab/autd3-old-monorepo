from datetime import timedelta
from pyautd3 import GeometryBuilder, Controller, SilencerConfig, Clear, Synchronize
from pyautd3.link import SOEM
from pyautd3.gain import Focus
from pyautd3.modulation import Sine

import numpy as np

if __name__ == '__main__':
    geometry = GeometryBuilder().add_device([0., 0., 0.], [0., 0., 0.]).build()

    link = SOEM().build()

    autd = Controller.open(geometry, link)

    autd.send(Clear(), timeout=timedelta(milliseconds=20))
    autd.send(Synchronize(), timeout=timedelta(milliseconds=20))

    firm_info_list = autd.firmware_info_list()
    for i, firm in enumerate(firm_info_list):
        print(f'[{i}]: {firm}')

    config = SilencerConfig()
    autd.send(config, timeout=timedelta(milliseconds=20))

    g = Focus(autd.geometry.center + np.array([0., 0., 150.]))
    m = Sine(150)
    autd.send(m, g, timeout=timedelta(milliseconds=20))

    _ = input()

    autd.close()
