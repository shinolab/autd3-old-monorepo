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

open AUTD3Sharp
open AUTD3Sharp.Gain
open AUTD3Sharp.Modulation

module AdvancedTest =
    type UniformGain () =
        inherit Gain()
        override this.Calc (geometry: Geometry) = Gain.Transform(geometry, fun tr -> new Drive(1., 0.));
        
    type Burst (size: int) =
        inherit Modulation(5120)
        override this.Calc () = 
            let buf: float array = Array.zeroCreate size
            buf[0] <- 1.
            buf
        
    let Test (autd : Controller) = 
        (SilencerConfig.None()) |> autd.Send |> ignore

        let m = new Burst(4000);
        let g = new UniformGain();

        (m, g) |> autd.Send |> ignore
