'''
File: wavedrom_navy.py
Project: docs
Created Date: 21/03/2022
Author: Shun Suzuki
-----
Last Modified: 30/05/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

import sys
import re


if __name__ == '__main__':
    path = sys.argv[1]

    pattern_white = '(fill|stroke):#(ffffff|fff)(;|})'
    pattern_black = '(fill|stroke):#(000000|000)(;|})'

    with open(path) as f:
        s = f.read()

    s = re.sub(pattern_white, r'\1:#bcbdd0\3', s)
    s = re.sub(pattern_black, r'\1:#161923\3', s)

    with open(path, mode='w') as f:
        f.write(s)
