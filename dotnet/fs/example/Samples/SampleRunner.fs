// File: SampleRunner.fs
// Project: Samples
// Created Date: 03/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 12/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 

namespace Samples

open AUTD3Sharp

module SampleRunner =
    let Run (autd : Controller) = 
        let examples = [
                (FocusTest.Test, "Single focus test");
                (BesselBeamTest.Test, "Bessel beam test");
                (PlaneTest.Test, "Plane wave test");
                (WavTest.Test, "Wav modulation test");
                (STMTest.FocusSTMTest, "FocusSTM test");
                (STMTest.GainSTMTest, "GainSTM test");
                (STMTest.SoftwareSTMTest, "SoftwareSTM test");
                (GainHoloTest.Test, "Multiple foci test");
                (CustomTest.Test, "Custom Gain & Modulation test");
                (FlagTest.Test, "Flag test");
                (TransTest.Test, "TransducerTest test")];

        let examples = 
            if autd.Geometry.NumDevices >= 2 then examples @ [(GroupTest.Test, "Group test")] else examples;


        printfn "======== AUTD3 firmware information ========"
        autd.FirmwareInfoList() |> Seq.iter (fun firm -> printfn $"{firm}")
        printfn "============================================"

        let rec run_example () =
            let print_example i =
                let _, name = examples[i] in
                printfn $"[{i}]: {name}"
            [0..examples.Length-1] |> List.iter print_example
            printfn "[Others]: finish"
            
            printf "Choose number: "
            let input = stdin.ReadLine()
            match System.Int32.TryParse input with
                | true,i -> 
                    let f, _ = examples[i] in
                    f(autd)

                    printfn "press any key to finish..."
                    System.Console.ReadKey true |> ignore;

                    printfn "finish."

                    (new Stop()) |> autd.Send |> ignore;

                    run_example()
                | _ -> ()

        run_example()

        autd.Close() |> ignore;
        autd.Dispose() |> ignore;
