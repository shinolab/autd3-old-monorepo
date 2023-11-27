// File: GainHoloTest.fs
// Project: Samples
// Created Date: 03/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 27/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 

namespace Samples

open AUTD3Sharp
open AUTD3Sharp.Gain.Holo
open AUTD3Sharp.Modulation
open AUTD3Sharp.Utils

module GainHoloTest =
    let Test<'T> (autd : Controller<'T>) = 
        (new Silencer()) |> autd.SendAsync |> Async.AwaitTask |> Async.RunSynchronously |> ignore;

        let m = new Sine 150;

        let center = autd.Geometry.Center + Vector3d(0, 0, 150);
        let backend = new NalgebraBackend();
        let g = (new GSPAT<NalgebraBackend>(backend)).WithConstraint(new Uniform(EmitIntensity.Max))
                    .AddFocus(center + 20.0 * Vector3d.UnitX, 5e3 * Amplitude.Units.Pascal)
                    .AddFocus(center - 20.0 * Vector3d.UnitX, 5e3 * Amplitude.Units.Pascal);

        (m, g) |> autd.SendAsync |> Async.AwaitTask |> Async.RunSynchronously |> ignore;
