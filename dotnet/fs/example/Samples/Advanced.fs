// File: Advanced.fs
// Project: Samples
// Created Date: 03/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 18/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 

namespace Samples

open System
open AUTD3Sharp

module AdvancedTest =
    let Test (autd : Controller) = 
        (SilencerConfig.None()) |> autd.Send |> ignore

        let amp = [| for _ in 1u..autd.Geometry.NumTransducers -> 1.0 |]
        let phase = [| for _ in 1u..autd.Geometry.NumTransducers -> 0.0 |]
        
        let burst : float array = Array.zeroCreate 4000
        burst[0] <- 1.0;

        let m = new Modulation.Custom(burst, 40960u);
        let g = new Gain.Custom(amp, phase);

        (m, g) |> autd.Send |> ignore
