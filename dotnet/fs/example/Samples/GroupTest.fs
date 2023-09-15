// File: GroupTest.fs
// Project: Samples
// Created Date: 15/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 15/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 


namespace Samples

open AUTD3Sharp
open AUTD3Sharp.Gain
open AUTD3Sharp.Modulation
open AUTD3Sharp.Utils

module GroupTest =
    let Test (autd : Controller) = 
        (new Silencer()) |> autd.Send |> ignore

        autd.Group(fun dev -> match dev.Idx with
                                | 0 ->  "null"
                                | 1 ->  "focus"
                                | _ ->  null
             )
            .Set("null", new Static(), new Null())
            .Set("focus", new Sine(150), new Focus(autd.Geometry.Center + new Vector3d(0,0,150)))
            .Send();
