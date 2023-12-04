"""
File: test_group.py
Project: gain
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 11/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


import numpy as np
import pytest

from pyautd3 import Device, Phase, Transducer
from pyautd3.autd_error import AUTDError
from pyautd3.gain import Group, Null, Uniform
from tests.test_autd import create_controller


@pytest.mark.asyncio()
async def test_group():
    autd = await create_controller()

    cx = autd.geometry.center[0]

    assert await autd.send_async(
        Group(lambda _, tr: "uniform" if tr.position[0] < cx else "null")
        .set_gain("uniform", Uniform(0x80).with_phase(Phase(0x90)))
        .set_gain("null", Null()),
    )
    for dev in autd.geometry:
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 0)
        for tr in dev:
            if tr.position[0] < cx:
                assert np.all(intensities[tr.idx] == 0x80)
                assert np.all(phases[tr.idx] == 0x90)
            else:
                assert np.all(intensities[tr.idx] == 0)
                assert np.all(phases[tr.idx] == 0)

    assert await autd.send_async(
        Group(lambda _, tr: "uniform" if tr.position[0] < cx else None).set_gain("uniform", Uniform(0x80).with_phase(Phase(0x90)))
    )
    for dev in autd.geometry:
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 0)
        for tr in dev:
            if tr.position[0] < cx:
                assert np.all(intensities[tr.idx] == 0x80)
                assert np.all(phases[tr.idx] == 0x90)
            else:
                assert np.all(intensities[tr.idx] == 0)
                assert np.all(phases[tr.idx] == 0)


@pytest.mark.asyncio()
async def test_group_unknown_key():
    autd = await create_controller()

    with pytest.raises(AUTDError, match="Unknown group key"):
        await autd.send_async(Group(lambda _, _tr: "null").set_gain("uniform", Uniform(0x80).with_phase(Phase(0x90))).set_gain("null", Null()))


@pytest.mark.asyncio()
async def test_group_unspecified_key():
    autd = await create_controller()

    with pytest.raises(AUTDError, match="Unspecified group key"):
        await autd.send_async(Group(lambda _, _tr: "null"))


@pytest.mark.asyncio()
async def test_group_check_only_for_enabled():
    autd = await create_controller()
    autd.geometry[0].enable = False

    check = np.zeros(autd.geometry.num_devices, dtype=bool)

    def f(dev: Device, _tr: Transducer) -> int:
        check[dev.idx] = True
        return 0

    assert await autd.send_async(Group(f).set_gain(0, Uniform(0x80).with_phase(Phase(0x90))))

    assert not check[0]
    assert check[1]

    intensities, phases = autd.link.intensities_and_phases(0, 0)
    assert np.all(intensities == 0)
    assert np.all(phases == 0)

    intensities, phases = autd.link.intensities_and_phases(1, 0)
    assert np.all(intensities == 0x80)
    assert np.all(phases == 0x90)
