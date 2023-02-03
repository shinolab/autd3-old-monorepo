namespace Samples

open AUTD3Sharp

module AdvancedTest =
    let Test (autd : Controller) = 
        SilencerConfig.None() |> autd.Send |> ignore

        let amp = [| for _ in 1..autd.Geometry.NumTransducers -> 1.0 |]
        let phase = [| for _ in 1..autd.Geometry.NumTransducers -> 0.0 |]
        
        let burst : float array = Array.zeroCreate 4000
        burst[0] <- 1.0;

        let m = new Modulation.Custom(burst, 40960u);
        let g = new Gain.Custom(amp, phase);

        autd.Send(m, g) |> ignore
