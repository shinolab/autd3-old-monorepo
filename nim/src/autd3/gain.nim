# File: gain.nim
# Project: autd3
# Created Date: 13/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 13/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
#


import body
import native_methods/autd3capi

type Gain* = object of Body

proc `=destroy`(gain: var Gain) =
    if (gain.p != nil):
        AUTDDeleteGain(gain.p)
        gain.p = pointer(nil)
