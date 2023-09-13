// File: FocusTest.fs
// Project: Samples
// Created Date: 03/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 12/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 

namespace Samples

open AUTD3Sharp
open AUTD3Sharp.Gain
open AUTD3Sharp.Modulation
open AUTD3Sharp.Utils

module FocusTest =
    let Test (autd : Controller) = 
        (new Silencer()) |> autd.Send |> ignore

        let m = new Sine 150;
        let g = new Focus(autd.Geometry.Center + Vector3d(0, 0, 150));
        (m, g) |> autd.Send |> ignore
