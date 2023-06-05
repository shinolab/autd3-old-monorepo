"""
File: remote_twincat.py
Project: example
Created Date: 23/05/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


from pyautd3 import Controller, AUTD3
from pyautd3.link import RemoteTwinCAT

from samples import runner


if __name__ == "__main__":
    remote_ip_addr = "remote ip addr"
    remore_ams_net_id = "remote ams net id"
    local_ams_net_id = "local ams net is"

    autd = (
        Controller.builder()
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))
        .open_with(
            RemoteTwinCAT(remore_ams_net_id)
            .with_server_ip(remote_ip_addr)
            .with_client_ams_net_id(local_ams_net_id)
        )
    )

    runner.run(autd)
