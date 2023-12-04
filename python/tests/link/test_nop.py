"""
File: test_nop.py
Project: link
Created Date: 04/12/2023
Author: Shun Suzuki
-----
Last Modified: 04/12/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from pyautd3 import AUTD3, Controller
from pyautd3.link.nop import Nop


def test_nop():
    autd = Controller.builder().add_device(AUTD3([0.0, 0.0, 0.0])).open_with(Nop.builder())

    autd.close()
