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


from pyautd3 import Controller, Geometry
from pyautd3.link import RemoteTwinCAT

from samples import runner


if __name__ == "__main__":
    geometry = Geometry.Builder().add_device([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]).build()

    remote_ip_addr = "remote ip addr"
    remore_ams_net_id = "remote ams net id"
    local_ams_net_id = "local ams net is"
    link = (
        RemoteTwinCAT(remote_ip_addr, remore_ams_net_id)
        .local_ams_net_id(local_ams_net_id)
        .build()
    )

    autd = Controller.open(geometry, link)

    runner.run(autd)
