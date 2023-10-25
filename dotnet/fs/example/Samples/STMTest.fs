// File: STMTest.fs
// Project: Samples
// Created Date: 29/08/2023
// Author: Shun Suzuki
// -----
// Last Modified: 25/10/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 

namespace Samples

open System
open AUTD3Sharp
open AUTD3Sharp.STM
open AUTD3Sharp.Gain
open AUTD3Sharp.Modulation
open AUTD3Sharp.Utils
open System.Threading.Tasks

module STMTest =
    let GainSTMTest (autd : Controller) = 
        (Silencer.Disable()) |> autd.Send |> ignore;

        (new Static()) |> autd.Send |> ignore;
        
        let center = autd.Geometry.Center + Vector3d(0, 0, 150);
        let stm = 
            [0..199]
            |> List.map (fun i -> (2.0 * AUTD3.Pi * (float)i / 200.0))
            |> List.map (fun theta -> (center + 30.0 * Vector3d(cos(theta), sin(theta), 0.0)))
            |> List.map (fun p -> (new Focus(p)))
            |> List.fold (fun (acc: GainSTM) v -> acc.AddGain v) (new GainSTM(1.))

        printfn $"Actual frequency is {stm.Frequency}";
        (stm )|> autd.Send |> ignore

    let FocusSTMTest (autd : Controller) = 
        (Silencer.Disable()) |> autd.Send |> ignore;

        (new Static()) |> autd.Send |> ignore;
        
        let center = autd.Geometry.Center + Vector3d(0, 0, 150);
        let stm = 
            [0..199]
            |> List.map (fun i -> (2.0 * AUTD3.Pi * (float)i / 200.0))
            |> List.map (fun theta -> (center + 30.0 * Vector3d(cos(theta), sin(theta), 0.0)))
            |> List.fold (fun (acc: FocusSTM) v -> acc.AddFocus v) (new FocusSTM(1.))

        printfn $"Actual frequency is {stm.Frequency}";
        (stm)|> autd.Send |> ignore
