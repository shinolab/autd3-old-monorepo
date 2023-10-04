'''
File: test_group.py
Project: gain
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 03/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from ..test_autd import create_controller

from pyautd3.autd_error import AUTDError
from pyautd3.gain import Null, Uniform, Group
from pyautd3.link.audit import Audit

import numpy as np


def test_group():
    autd = create_controller()

    cx = autd.geometry.center[0]

    assert autd.send(Group(lambda _, tr: "uniform" if tr.position[0] < cx else "null")
                     .set("uniform", Uniform(0.5).with_phase(np.pi))
                     .set("null", Null()))

    for dev in autd.geometry:
        duties, phases = Audit.duties_and_phases(autd._ptr, dev.idx, 0)
        for tr in dev:
            if tr.position[0] < cx:
                assert np.all(duties[tr.local_idx] == 680)
                assert np.all(phases[tr.local_idx] == 2048)
            else:
                assert np.all(duties[tr.local_idx] == 8)
                assert np.all(phases[tr.local_idx] == 0)


def test_group_unknown_key():
    autd = create_controller()

    caught_err = False
    try:
        autd.send(Group(lambda _, tr: "null")
                  .set("uniform", Uniform(0.5).with_phase(np.pi))
                  .set("null", Null()))
    except AUTDError as e:
        caught_err = True
        assert e.msg == "Unknown group key"

    assert caught_err


def test_group_unspecified_key():
    autd = create_controller()

    caught_err = False
    try:
        autd.send(Group(lambda _, tr: "null"))
    except AUTDError as e:
        caught_err = True
        assert e.msg == "Unspecified group key"

    assert caught_err


def test_group_check_only_for_enabled():
    autd = create_controller()
    autd.geometry[0].enable = False

    check = np.zeros(autd.geometry.num_devices, dtype=bool)

    def f(dev, tr):
        check[dev.idx] = True
        return 0
    assert autd.send(Group(f).set(0, Uniform(0.5).with_phase(np.pi)))

    assert not check[0]
    assert check[1]

    duties, phases = Audit.duties_and_phases(autd._ptr, 0, 0)
    assert np.all(duties == 0)
    assert np.all(phases == 0)

    duties, phases = Audit.duties_and_phases(autd._ptr, 1, 0)
    assert np.all(duties == 680)
    assert np.all(phases == 2048)
