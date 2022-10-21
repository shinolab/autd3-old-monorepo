'''
File: remote_twincat.py
Project: example
Created Date: 23/05/2022
Author: Shun Suzuki
-----
Last Modified: 21/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


from pyautd3 import Controller
from pyautd3.link import RemoteTwinCAT

from samples import runner


if __name__ == '__main__':
    autd = Controller()

    autd.geometry.add_device([0., 0., 0.], [0., 0., 0.])

    remote_ip_addr = 'remote ip addr'
    remore_ams_net_id = 'remote ams net id'
    local_ams_net_id = 'local ams net is'
    link = RemoteTwinCAT(remote_ip_addr, remore_ams_net_id).local_ams_net_id(local_ams_net_id).build()
    if not autd.open(link):
        print(Controller.last_error())
        exit()

    runner.run(autd)
