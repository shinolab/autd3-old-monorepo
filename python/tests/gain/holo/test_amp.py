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


from pyautd3.gain.holo import Amplitude


def test_holo_amp_db():
    amp = Amplitude.new_spl(121.5)
    assert amp.pascal == 23.77004454874038
    assert amp.spl == 121.5


def test_holo_amp_pascal():
    amp = Amplitude.new_pascal(23.77004454874038)
    assert amp.pascal == 23.77004454874038
    assert amp.spl == 121.5
