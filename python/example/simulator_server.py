'''
File: simulator_client.py
Project: example
Created Date: 10/10/2022
Author: Shun Suzuki
-----
Last Modified: 17/11/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

from pyautd3.extra import Simulator


if __name__ == '__main__':
    Simulator().settings_path("settings.json").vsync(True).gpu_idx(0).run()
