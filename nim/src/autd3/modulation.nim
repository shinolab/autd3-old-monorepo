# File: modulation.nim
# Project: autd3
# Created Date: 13/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 13/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
#

import header
import native_methods/autd3capi

type Modulation* = object of Header

proc `=destroy`(modulation: var Modulation) =
    if (modulation.p != nil):
        AUTDDeleteModulation(modulation.p)
        modulation.p = pointer(nil)

proc samplingFrequencyDivision*(modulation: Modulation): uint32 =
    AUTDModulationSamplingFrequencyDivision(modulation.p)

proc `samplingFrequencyDivision=`*(modulation: Modulation, value: uint32) =
    AUTDModulationSetSamplingFrequencyDivision(modulation.p, value)

proc samplingFrequency*(modulation: Modulation): float64 =
    AUTDModulationSamplingFrequency(modulation.p)
