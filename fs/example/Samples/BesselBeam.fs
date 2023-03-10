// File: BesselBeam.fs
// Project: Samples
// Created Date: 03/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 08/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 

namespace Samples

open System
open AUTD3Sharp
open AUTD3Sharp.Gain
open AUTD3Sharp.Modulation
open AUTD3Sharp.Utils

module BesselBeamTest =
    let Test (autd : Controller) = 
        (new SilencerConfig(), TimeSpan.FromMilliseconds(20)) |> autd.Send |> ignore

        let m = new Sine 150;

        let start = autd.Geometry.Center;
        let dir = Vector3d.UnitZ;

        let g = new BesselBeam(start, dir, 13.0 / 180.0 * AUTD3.Pi);
        (m, g, TimeSpan.FromMilliseconds(20)) |> autd.Send |> ignore
