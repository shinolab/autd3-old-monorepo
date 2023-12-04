"""
File: test_twincat.py
Project: link
Created Date: 17/10/2023
Author: Shun Suzuki
-----
Last Modified: 04/12/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from datetime import timedelta

import pytest

from pyautd3 import AUTD3, Controller
from pyautd3.autd_error import AUTDError
from pyautd3.link.twincat import RemoteTwinCAT, TwinCAT


def test_twincat():
    with pytest.raises(AUTDError):
        _ = Controller.builder().add_device(AUTD3([0.0, 0.0, 0.0])).open_with(TwinCAT.builder().with_timeout(timedelta(milliseconds=200)))


def test_remote_twincat():
    with pytest.raises(AUTDError):
        _ = (
            Controller.builder()
            .add_device(AUTD3([0.0, 0.0, 0.0]))
            .open_with(
                RemoteTwinCAT.builder("xxx.xxx.xxx.xxx.xxx.xxx")
                .with_server_ip("127.0.0.1")
                .with_client_ams_net_id("127.0.0.1")
                .with_timeout(timedelta(milliseconds=200)),
            )
        )
