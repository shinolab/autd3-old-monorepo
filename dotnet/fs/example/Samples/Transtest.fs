// File: Transtest.fs
// Project: Samples
// Created Date: 14/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 24/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 

namespace Samples

open AUTD3Sharp
open AUTD3Sharp.Gain
open AUTD3Sharp.Modulation

module TransTest =
    let Test<'T> (autd : Controller<'T>) = 
        (new Silencer()) |> autd.SendAsync |> Async.AwaitTask |> Async.RunSynchronously |> ignore;

        let m = new Sine(150);
        let g = (new TransducerTest()).Set(autd.Geometry[0][0], 0, EmitIntensity.Max).Set(autd.Geometry[0][248], 0, EmitIntensity.Max);
        (m, g) |> autd.SendAsync |> Async.AwaitTask |> Async.RunSynchronously |> ignore;
