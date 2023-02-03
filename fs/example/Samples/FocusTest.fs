namespace Samples

open AUTD3Sharp
open AUTD3Sharp.Gain
open AUTD3Sharp.Modulation
open AUTD3Sharp.Utils

module FocusTest =
    let Test (autd : Controller) = 
        new SilencerConfig() |> autd.Send |> ignore

        let m = new Sine 150;
        let g = new Focus(autd.Geometry.Center + Vector3d(0, 0, 150));
        autd.Send(m, g) |> ignore
