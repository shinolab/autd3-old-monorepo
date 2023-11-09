"""
File: remote_soem.py
Project: example
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


import asyncio

from samples import runner

from pyautd3 import AUTD3, Controller
from pyautd3.link.soem import RemoteSOEM


async def main() -> None:
    with await (
        Controller.builder()
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))
        .open_with(
            RemoteSOEM.builder("127.0.0.1:8080"),
        )
    ) as autd:
        await runner.run(autd)


if __name__ == "__main__":
    asyncio.run(main())
