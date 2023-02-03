namespace Samples

open AUTD3Sharp
open AUTD3Sharp.Utils
open AUTD3Sharp.Modulation

module CustomTest =
    let Focus (autd:Controller) (point: Vector3d) =
        let amps = [| for _ in 1..autd.Geometry.NumTransducers -> 1.0 |]
        let phases = 
            autd.Geometry
                |> Seq.map (fun tr -> (2.0 * AUTD3.Pi * (tr.Position - point).L2Norm) / tr.Wavelength)
                |> Seq.toArray

        new Gain.Custom(amps, phases);

    let Test (autd : Controller) = 
        SilencerConfig.None() |> autd.Send |> ignore

        let m = new Sine 150;
        let g = Focus autd (autd.Geometry.Center + Vector3d(0, 0, 150))

        autd.Send(m, g) |> ignore
