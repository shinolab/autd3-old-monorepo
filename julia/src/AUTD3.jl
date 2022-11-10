# File: AUTD3.jl
# Project: src
# Created Date: 11/02/2020
# Author: Shun Suzuki
# -----
# Last Modified: 10/11/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2020 Hapis Lab. All rights reserved.
# 

module AUTD3

include("NativeMethods/autd3capi.jl")
include("NativeMethods/autd3capi-gain-holo.jl")
include("NativeMethods/autd3capi-link-soem.jl")
include("NativeMethods/autd3capi-link-simulator.jl")
if Sys.iswindows()
    include("NativeMethods/autd3capi-link-twincat.jl")
end
include("NativeMethods/autd3capi-link-remote-twincat.jl")
include("NativeMethods/autd3capi-modulation-audio-file.jl")
if !Sys.isapple()
    include("NativeMethods/autd3capi-backend-cuda.jl")
end

include("Link.jl")
include("SilencerConfig.jl")
include("Gain.jl")
include("Modulation.jl")
include("STM.jl")
include("PointSTM.jl")
include("Null.jl")
include("Focus.jl")
include("Plane.jl")
include("Bessel.jl")
include("CustomGain.jl")
include("Static.jl")
include("Sine.jl")
include("SineSquared.jl")
include("SineLegacy.jl")
include("Square.jl")
include("CustomModulation.jl")
include("Controller.jl")
include("Amplitudes.jl")
include("Grouped.jl")
include("GainSTM.jl")
include("SOEM.jl")
include("Simulator.jl")
if Sys.iswindows()
    include("TwinCAT.jl")
end
include("RemoteTwinCAT.jl")
include("AudioFile.jl")
if !Sys.isapple()
    include("BackendCUDA.jl")
end
include("BackendEigen.jl")
include("Holo.jl")
include("SDP.jl")
include("EVD.jl")
include("Naive.jl")
include("GS.jl")
include("GSPAT.jl")
include("LM.jl")
include("Greedy.jl")
include("LSSGreedy.jl")
include("APO.jl")

export Controller, get_last_error
export SilencerConfig, SilencerConfigNone, ModDelayConfig, Amplitudes
export Null, Focus, BesselBeam, PlaneWave, CustomGain, Grouped
export Static, Sine, SineSquared, SineLegacy, Square, CustomModulation
export GainSTM, PointSTM
export SOEM, enumerate_adapters
export Simulator
if Sys.iswindows()
    export TwinCAT
end
export RemoteTwinCAT
export RawPCM, Wav
export DontCare, Normalize, Uniform, Clamp
export SDP, EVD, Naive, GS, GSPAT, LM, Greedy, LSSGreedy, APO
export BackendEigen
if !Sys.isapple()
    export BackendCUDA
end
export NUM_TRANS_IN_UNIT, NUM_TRANS_X, NUM_TRANS_Y, TRANS_SPACING, DEVICE_WIDTH, DEVICE_HEIGHT

const NUM_TRANS_IN_UNIT = 249
const NUM_TRANS_X = 18
const NUM_TRANS_Y = 14
const TRANS_SPACING = 10.16
const DEVICE_WIDTH = 192.0
const DEVICE_HEIGHT = 151.4

end
