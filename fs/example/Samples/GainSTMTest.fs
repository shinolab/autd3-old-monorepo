namespace Samples

open AUTD3Sharp
open AUTD3Sharp.STM
open AUTD3Sharp.Gain
open AUTD3Sharp.Modulation
open AUTD3Sharp.Utils

module GainSTMTest =
    let Test (autd : Controller) = 
        SilencerConfig.None() |> autd.Send |> ignore;

        new Static() |> autd.Send |> ignore;
        
        let center = autd.Geometry.Center + Vector3d(0, 0, 150);
        let stm = new GainSTM();
        [0..199]
            |> List.map (fun i -> (2.0 * AUTD3.Pi * (float)i / 200.0))
            |> List.map (fun theta -> (center + 30.0 * Vector3d(cos(theta), sin(theta), 0.0)))
            |> List.map (fun p -> (new Focus(p)))
            |> List.iter stm.Add

        stm.Frequency <- 1;
        printfn $"Actual frequency is {stm.Frequency}";
        autd.Send stm |> ignore
