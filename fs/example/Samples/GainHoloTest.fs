namespace Samples

open AUTD3Sharp
open AUTD3Sharp.Gain.Holo
open AUTD3Sharp.Modulation
open AUTD3Sharp.Utils

module GainHoloTest =
    let Test (autd : Controller) = 
        new SilencerConfig() |> autd.Send |> ignore

        let m = new Sine 150;

        let center = autd.Geometry.Center + Vector3d(0, 0, 150);
        let g = new GSPAT();
        g.Add(center + 20.0 * Vector3d.UnitX, 1.0);
        g.Add(center - 20.0 * Vector3d.UnitX, 1.0);
        g.Constraint <- new Uniform(1.0);

        autd.Send(m, g) |> ignore
