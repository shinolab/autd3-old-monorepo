"""
File: test_amp.py
Project: holo
Created Date: 25/11/2023
Author: Shun Suzuki
-----
Last Modified: 25/11/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


import pytest

from pyautd3.gain.holo import Amplitude, dB, pascal


def test_holo_amp_db():
    amp = 121.5 * dB
    assert amp.pascal == 23.77004454874038
    assert amp.spl == 121.5


def test_holo_amp_pascal():
    amp = 23.77004454874038 * pascal
    assert amp.pascal == 23.77004454874038
    assert amp.spl == 121.5


def test_holo_amp_ctr():
    with pytest.raises(NotImplementedError):
        _ = Amplitude()

    with pytest.raises(NotImplementedError):
        _ = Amplitude._UnitPascal()

    with pytest.raises(NotImplementedError):
        _ = Amplitude._UnitSPL()
