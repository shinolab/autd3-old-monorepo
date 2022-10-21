'''
File: simulator_client.py
Project: example
Created Date: 10/10/2022
Author: Shun Suzuki
-----
Last Modified: 21/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

from pyautd3.extra import Simulator


if __name__ == '__main__':
    Simulator().settings_path("settings.json").ip("127.0.0.1").port(50632).vsync(True).gpu_idx(0).run()
