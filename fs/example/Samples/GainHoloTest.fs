// File: GainHoloTest.fs
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
open AUTD3Sharp.Gain.Holo
open AUTD3Sharp.Modulation
open AUTD3Sharp.Utils

module GainHoloTest =
    let Test (autd : Controller) = 
        (new SilencerConfig(), TimeSpan.FromMilliseconds(20)) |> autd.Send |> ignore

        let m = new Sine 150;

        let center = autd.Geometry.Center + Vector3d(0, 0, 150);
        let g = new GSPAT();
        g.Add(center + 20.0 * Vector3d.UnitX, 1.0);
        g.Add(center - 20.0 * Vector3d.UnitX, 1.0);
        g.Constraint <- new Uniform(1.0);

        (m, g, TimeSpan.FromMilliseconds(20)) |> autd.Send |> ignore
