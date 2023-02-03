namespace Samples

open AUTD3Sharp
open AUTD3Sharp.Gain
open AUTD3Sharp.Modulation
open AUTD3Sharp.Utils

module BesselBeamTest =
    let Test (autd : Controller) = 
        new SilencerConfig() |> autd.Send |> ignore

        let m = new Sine 150;

        let start = autd.Geometry.Center;
        let dir = Vector3d.UnitZ;

        let g = new BesselBeam(start, dir, 13.0 / 180.0 * AUTD3.Pi);
        autd.Send(m, g) |> ignore
