// File: Custom.fs
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
open AUTD3Sharp.Utils
open AUTD3Sharp.Modulation

module CustomTest =
    type Focus (point: Vector3d) =
        inherit Gain()
        let Calc_ (soundSpeed:double) (tr:Transducer) = 
            let mutable drive = new Drive();
            let dist = (tr.Position - point).L2Norm;
            drive.Phase <- dist * tr.Wavenumber(soundSpeed);
            drive.Amp <- 1.0;
            drive
        override this.Calc (geometry: Geometry) = Gain.Transform(geometry, Calc_(geometry.SoundSpeed));
        

    let Test (autd : Controller) = 
        (SilencerConfig.None()) |> autd.Send |> ignore

        let m = new Sine 150;
        let g = new Focus (autd.Geometry.Center + Vector3d(0, 0, 150))

        (m, g) |> autd.Send |> ignore
