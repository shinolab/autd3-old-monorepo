from pyautd3 import Controller, SilencerConfig, Clear, Synchronize
from pyautd3.link import SOEM
from pyautd3.gain import Focus
from pyautd3.modulation import Sine

import numpy as np
import sys

if __name__ == '__main__':
    autd = Controller()

    autd.geometry.add_device([0., 0., 0.], [0., 0., 0.])

    link = SOEM().high_precision(True).build()
    if not autd.open(link):
        sys.exit(-1)

    autd.ack_check_timeout_ms = 20

    autd.send(Clear())

    autd.send(Synchronize())

    firm_info_list = autd.firmware_info_list()
    for i, firm in enumerate(firm_info_list):
        print(f'[{i}]: {firm}')

    config = SilencerConfig()
    autd.send(config)

    g = Focus(autd.geometry.center + np.array([0., 0., 150.]))
    m = Sine(150)
    autd.send(m, g)

    _ = input()

    autd.close()
