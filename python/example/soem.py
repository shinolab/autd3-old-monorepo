"""
File: soem.py
Project: example
Created Date: 30/12/2020
Author: Shun Suzuki
-----
Last Modified: 11/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Shun Suzuki. All rights reserved.

"""

import asyncio
import ctypes
import os
from typing import NoReturn

from samples import runner  # type: ignore[import-not-found]

from pyautd3 import AUTD3, Controller
from pyautd3.link.soem import SOEM, OnErrFunc


def on_lost(msg: ctypes.c_char_p) -> NoReturn:
    if msg.value is not None:
        print(msg.value.decode("utf-8"), end="")
    os._exit(-1)


def on_err(msg: ctypes.c_char_p) -> None:
    if msg.value is not None:
        print(msg.value.decode("utf-8"), end="")


async def main() -> None:
    on_lost_func = OnErrFunc(on_lost)
    on_err_func = OnErrFunc(on_err)
    with await (
        Controller.builder()
        .add_device(AUTD3([0.0, 0.0, 0.0]))
        .open_with_async(
            SOEM.builder().with_on_lost(on_lost_func).with_on_err(on_err_func),
        )
    ) as autd:  # type: Controller
        await runner.run(autd)


if __name__ == "__main__":
    asyncio.run(main())
