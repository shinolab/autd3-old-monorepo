"""
File: flag.py
Project: samples
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 14/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


import threading

from pyautd3 import ConfigureForceFan, ConfigureReadsFPGAInfo, Controller


async def flag(autd: Controller) -> None:
    print("press any key to run fan...")
    _ = input()

    await autd.send_async(ConfigureReadsFPGAInfo(lambda _: True), ConfigureForceFan(lambda _: True))

    fin = False

    def f() -> None:
        nonlocal fin
        print("press any key stop checking FPGA status...")
        _ = input()
        fin = True

    th = threading.Thread(target=f)
    th.start()

    prompts = ["-", "/", "|", "\\"]
    prompts_idx = 0
    while not fin:
        states = await autd.fpga_info_async()
        print(f"{prompts[(prompts_idx // 1000) % len(prompts)]} FPGA Status...")
        print("\n".join([f"\x1b[0K[{i}]: thermo = {state.is_thermal_assert()}" for i, state in enumerate(states)]))
        print(f"\x1b[{len(states) + 1}A", end="")
        prompts_idx += 1
    print("\x1b[1F\x1b[0J")

    th.join()
    await autd.send_async(ConfigureReadsFPGAInfo(lambda _: False), ConfigureForceFan(lambda _: False))
