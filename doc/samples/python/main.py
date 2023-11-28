import asyncio
import ctypes
import os
from typing import NoReturn

import numpy as np
from pyautd3 import AUTD3, Controller, Silencer
from pyautd3.gain import Focus
from pyautd3.link.soem import SOEM, OnErrFunc
from pyautd3.modulation import Sine


def on_lost(msg: ctypes.c_char_p) -> NoReturn:
    print(msg.decode("utf-8"), end="")
    os._exit(-1)


def on_err(msg: ctypes.c_char_p) -> None:
    print(msg.decode("utf-8"), end="")


async def main() -> None:
    on_lost_func = OnErrFunc(on_lost)
    on_err_func = OnErrFunc(on_err)

    with await (
        Controller.builder()
        .add_device(AUTD3([0.0, 0.0, 0.0]))
        .open_with_async(
            SOEM.builder().with_on_lost(on_lost_func).with_on_err(on_err_func)
        )
    ) as autd:
        firm_info_list = await autd.firmware_info_list_async()
        print("\n".join([f"[{i}]: {firm}" for i, firm in enumerate(firm_info_list)]))

        await autd.send_async(Silencer())

        g = Focus(autd.geometry.center + np.array([0.0, 0.0, 150.0]))
        m = Sine(150)
        await autd.send_async(m, g)

        _ = input()

        await autd.close_async()


if __name__ == "__main__":
    asyncio.run(main())
