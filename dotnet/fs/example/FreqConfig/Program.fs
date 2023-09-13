// File: Program.fs
// Project: FreqConfig
// Created Date: 14/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 14/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 

open AUTD3Sharp.Utils
open AUTD3Sharp
open AUTD3Sharp.Link

let autd = Controller.Builder()
            .Advanced()
            .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
            .OpenWith((new Debug()))

autd.Geometry |> Seq.iter (fun dev -> dev |> Seq.iter (fun tr -> tr.Frequency <- 70e3))

(new Synchronize()) |> autd.Send |> ignore;
