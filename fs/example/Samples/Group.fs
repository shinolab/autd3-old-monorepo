// File: Group.fs
// Project: Samples
// Created Date: 03/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 03/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 

namespace Samples

open AUTD3Sharp
open AUTD3Sharp.Gain
open AUTD3Sharp.Gain.Holo
open AUTD3Sharp.Modulation
open AUTD3Sharp.Utils

module GroupTest =
    let Test (autd : Controller) = 
        new SilencerConfig() |> autd.Send |> ignore

        let g1 = new Focus(autd.Geometry.CenterOf(0) + Vector3d(0, 0, 150));
        let g2 = new GSPAT();
        g2.Add(autd.Geometry.CenterOf(1) + new Vector3d(30.0, 0.0, 150.0), 1.0);
        g2.Add(autd.Geometry.CenterOf(1) - new Vector3d(30.0, 0.0, 150.0), 1.0);

        let gain = new Grouped();
        gain.Add(0, g1);
        gain.Add(1, g2);

        let m = new Sine 150;

        autd.Send(m, gain) |> ignore
