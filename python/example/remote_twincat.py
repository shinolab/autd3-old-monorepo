'''
File: remote_twincat.py
Project: example
Created Date: 23/05/2022
Author: Shun Suzuki
-----
Last Modified: 08/08/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


from pyautd3 import AUTD, RemoteTwinCAT

from samples import runner


if __name__ == '__main__':
    autd = AUTD()

    autd.add_device([0., 0., 0.], [0., 0., 0.])
    # autd.add_device([0., 0., 0.], [0., 0., 0.])

    remote_ip_addr = 'remote ip addr'
    remore_ams_net_id = 'remote ams net id'
    local_ams_net_id = 'local ams net is'
    link = RemoteTwinCAT(remote_ip_addr, remore_ams_net_id).local_ams_net_id(local_ams_net_id).build()
    if not autd.open(link):
        print(AUTD.last_error())
        exit()

    runner.run(autd)
