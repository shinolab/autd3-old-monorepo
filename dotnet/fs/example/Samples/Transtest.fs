// File: Transtest.fs
// Project: Samples
// Created Date: 14/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 14/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 

namespace Samples

open AUTD3Sharp
open AUTD3Sharp.Gain
open AUTD3Sharp.Modulation

module TransTest =
    let Test (autd : Controller) = 
        (new Silencer()) |> autd.Send |> ignore

        let m = new Sine(150);
        let g = (new TransducerTest()).Set(0, 0, 0, 1).Set(0, 248, 0, 1);
        (m, g) |> autd.Send |> ignore
