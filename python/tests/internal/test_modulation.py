'''
File: test_modulation.py
Project: modulation
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 20/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from ..test_autd import create_controller

from pyautd3.modulation import Sine
from pyautd3.link.audit import Audit

import numpy as np


def test_cache():
    autd = create_controller()

    m = Sine(150).with_cache()

    assert autd.send(m)

    for dev in autd.geometry:
        mod = Audit.modulation(autd._ptr, dev.idx)
        mod_expext = [
            85,
            107,
            132,
            157,
            183,
            210,
            236,
            245,
            219,
            192,
            166,
            140,
            115,
            92,
            70,
            50,
            33,
            19,
            8,
            2,
            0,
            2,
            8,
            19,
            33,
            50,
            70,
            92,
            115,
            140,
            166,
            192,
            219,
            245,
            236,
            210,
            183,
            157,
            132,
            107,
            85,
            63,
            44,
            28,
            15,
            6,
            0,
            0,
            3,
            11,
            23,
            39,
            57,
            77,
            100,
            123,
            148,
            174,
            201,
            227,
            255,
            227,
            201,
            174,
            148,
            123,
            100,
            77,
            57,
            39,
            23,
            11,
            3,
            0,
            0,
            6,
            15,
            28,
            44,
            63]
        assert np.array_equal(mod, mod_expext)
        assert Audit.modulation_frequency_division(autd._ptr, dev.idx) == 40960


def test_transform():
    autd = create_controller()

    m = Sine(150).with_transform(lambda i, v: v / 2)

    assert autd.send(m)

    for dev in autd.geometry:
        mod = Audit.modulation(autd._ptr, dev.idx)
        mod_expext = [
            41,
            50,
            60,
            69,
            76,
            81,
            84,
            84,
            82,
            78,
            71,
            63,
            54,
            44,
            34,
            25,
            16,
            9,
            4,
            1,
            0,
            1,
            4,
            9,
            16,
            25,
            34,
            44,
            54,
            63,
            71,
            78,
            82,
            84,
            84,
            81,
            76,
            69,
            60,
            50,
            41,
            31,
            22,
            14,
            7,
            3,
            0,
            0,
            1,
            5,
            11,
            19,
            28,
            37,
            47,
            57,
            66,
            73,
            79,
            83,
            85,
            83,
            79,
            73,
            66,
            57,
            47,
            37,
            28,
            19,
            11,
            5,
            1,
            0,
            0,
            3,
            7,
            14,
            22,
            31]
        assert np.array_equal(mod, mod_expext)
        assert Audit.modulation_frequency_division(autd._ptr, dev.idx) == 40960


def test_radiation_pressure():
    autd = create_controller()

    m = Sine(150).with_radiation_pressure()

    assert autd.send(m)

    for dev in autd.geometry:
        mod = Audit.modulation(autd._ptr, dev.idx)
        mod_expext = [
            127,
            146,
            165,
            184,
            204,
            223,
            242,
            248,
            229,
            210,
            191,
            172,
            153,
            133,
            114,
            95,
            76,
            57,
            38,
            19,
            0,
            19,
            38,
            57,
            76,
            95,
            114,
            133,
            153,
            172,
            191,
            210,
            229,
            248,
            242,
            223,
            204,
            184,
            165,
            146,
            127,
            108,
            89,
            70,
            51,
            31,
            12,
            6,
            25,
            44,
            63,
            82,
            101,
            121,
            140,
            159,
            178,
            197,
            216,
            235,
            255,
            235,
            216,
            197,
            178,
            159,
            140,
            121,
            102,
            82,
            63,
            44,
            25,
            6,
            12,
            31,
            50,
            70,
            89,
            108]
        assert np.array_equal(mod, mod_expext)
        assert Audit.modulation_frequency_division(autd._ptr, dev.idx) == 40960
