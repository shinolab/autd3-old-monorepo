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

from pyautd3 import Device, Transducer
from pyautd3.autd_error import AUTDError
from pyautd3.gain import Group, Null, Uniform
from tests.test_autd import create_controller


def test_group():
    autd = create_controller()

    cx = autd.geometry.center[0]

    assert autd.send(
        Group(lambda _, tr: "uniform" if tr.position[0] < cx else "null")
        .set_gain("uniform", Uniform(0.5).with_phase(np.pi))
        .set_gain("null", Null()),
    )

    for dev in autd.geometry:
        duties, phases = autd.link.duties_and_phases(dev.idx, 0)
        for tr in dev:
            if tr.position[0] < cx:
                assert np.all(duties[tr.local_idx] == 85)
                assert np.all(phases[tr.local_idx] == 256)
            else:
                assert np.all(duties[tr.local_idx] == 0)
                assert np.all(phases[tr.local_idx] == 0)


def test_group_unknown_key():
    autd = create_controller()

    with pytest.raises(AUTDError, match="Unknown group key"):
        autd.send(Group(lambda _, _tr: "null").set_gain("uniform", Uniform(0.5).with_phase(np.pi)).set_gain("null", Null()))


def test_group_unspecified_key():
    autd = create_controller()

    with pytest.raises(AUTDError, match="Unspecified group key"):
        autd.send(Group(lambda _, _tr: "null"))


def test_group_check_only_for_enabled():
    autd = create_controller()
    autd.geometry[0].enable = False

    check = np.zeros(autd.geometry.num_devices, dtype=bool)

    def f(dev: Device, _tr: Transducer) -> int:
        check[dev.idx] = True
        return 0

    assert autd.send(Group(f).set_gain(0, Uniform(0.5).with_phase(np.pi)))

    assert not check[0]
    assert check[1]

    duties, phases = autd.link.duties_and_phases(0, 0)
    assert np.all(duties == 0)
    assert np.all(phases == 0)

    duties, phases = autd.link.duties_and_phases(1, 0)
    assert np.all(duties == 85)
    assert np.all(phases == 256)
