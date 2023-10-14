'''
File: test_simulator.py
Project: link
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from datetime import timedelta
import pytest

from pyautd3 import Controller, AUTD3
from pyautd3.link.simulator import Simulator


@pytest.mark.simulator
def test_simulator():
    autd = Controller.builder()\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))\
        .open_with(Simulator.builder(8080).with_server_ip("127.0.0.1").with_timeout(timedelta(milliseconds=200)))

    autd.close()
