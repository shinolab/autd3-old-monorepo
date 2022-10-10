# File: emulator.nim
# Project: autd3
# Created Date: 13/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 13/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
#


import native_methods/autd3capi_modulation_audio_file
import modulation

type Wav* = object of Modulation

proc initWav*(path: string, freqDiv: uint32): Wav =
  AUTDModulationWav(result.p.addr, path, freqDiv)

type RawPCM* = object of Modulation

proc initRawPCM*(path: string, samplingFreq: float64, freqDiv: uint32): RawPCM =
  AUTDModulationRawPCM(result.p.addr, path, samplingFreq, freqDiv)
