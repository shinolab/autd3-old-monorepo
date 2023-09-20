'''
File: test_simulator.py
Project: link
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 20/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from datetime import timedelta
import pytest

from pyautd3 import Controller, AUTD3
from pyautd3.link import TwinCAT, RemoteTwinCAT


@pytest.mark.twincat
def test_twincat():
    autd = Controller.builder()\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))\
        .open_with(TwinCAT().with_timeout(timedelta(milliseconds=200)))

    autd.close()


@pytest.mark.remote_twincat
def test_remote_twincat():
    autd = Controller.builder()\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))\
        .open_with(
            RemoteTwinCAT("xxx.xxx.xxx.xxx.xxx.xxx").with_server_ip("127.0.0.1").with_client_ams_net_id(
                "127.0.0.1").with_timeout(timedelta(milliseconds=200))
    )

    autd.close()
