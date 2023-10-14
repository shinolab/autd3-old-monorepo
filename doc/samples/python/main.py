from pyautd3 import AUTD3, Controller, Silencer
from pyautd3.link.soem import SOEM, OnErrFunc
from pyautd3.gain import Focus
from pyautd3.modulation import Sine

import numpy as np

import os
import ctypes


def on_lost(msg: ctypes.c_char_p):
    print(msg.decode("utf-8"), end="")
    os._exit(-1)


if __name__ == "__main__":
    on_lost_func = OnErrFunc(on_lost)

    autd = (
        Controller.builder()
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))
        .open_with(SOEM.builder().with_on_lost(on_lost_func))
    )

    firm_info_list = autd.firmware_info_list()
    print("\n".join([f"[{i}]: {firm}" for i, firm in enumerate(firm_info_list)]))

    autd.send(Silencer())

    g = Focus(autd.geometry.center + np.array([0.0, 0.0, 150.0]))
    m = Sine(150)
    autd.send((m, g))

    _ = input()

    autd.close()
