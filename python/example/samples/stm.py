'''
File: stm.py
Project: samples
Created Date: 29/08/2023
Author: Shun Suzuki
-----
Last Modified: 15/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from datetime import timedelta
import threading
from pyautd3 import Controller, Silencer, TimerStrategy
from pyautd3.gain import Focus
from pyautd3.stm import GainSTM, FocusSTM
from pyautd3.modulation import Static
import numpy as np


def stm_focus(autd: Controller):
    config = Silencer.disable()
    autd.send(config)

    m = Static()

    radius = 30.0
    size = 200
    center = autd.geometry.center + np.array([0.0, 0.0, 150.0])
    stm = FocusSTM(1.0).add_foci_from_iter(
        map(
            lambda theta: center + radius * np.array([np.cos(theta), np.sin(theta), 0]),
            map(lambda i: 2.0 * np.pi * i / size, range(size)),
        )
    )

    autd.send((m, stm))


def stm_gain(autd: Controller):
    config = Silencer.disable()
    autd.send(config)

    m = Static()

    radius = 30.0
    size = 50
    center = autd.geometry.center + np.array([0.0, 0.0, 150.0])
    stm = GainSTM(1.0).add_gains_from_iter(
        map(
            lambda theta: Focus(
                center + radius * np.array([np.cos(theta), np.sin(theta), 0])
            ),
            map(lambda i: 2.0 * np.pi * i / size, range(size)),
        )
    )

    autd.send((m, stm))


def stm_software(autd: Controller):
    config = Silencer.disable()
    autd.send(config)

    m = Static()
    autd.send(m)

    fin = False

    def f():
        nonlocal fin
        print('press enter to stop software stm...')
        _ = input()
        fin = True

    th = threading.Thread(target=f)
    th.start()

    freq = 1.0
    radius = 30.0
    size = 100
    center = autd.geometry.center + np.array([0.0, 0.0, 150.0])

    def callback(autd: Controller, i: int, elapsed: timedelta):
        nonlocal fin
        if fin:
            return False

        theta = 2.0 * np.pi * i / size
        try:
            return autd.send(Focus(center + radius * np.array([np.cos(theta), np.sin(theta), 0])))
        except BaseException:
            return False

    autd.software_stm(callback).with_timer_strategy(TimerStrategy.NativeTimer).start(timedelta(seconds=1.0 / freq / size))

    th.join()
