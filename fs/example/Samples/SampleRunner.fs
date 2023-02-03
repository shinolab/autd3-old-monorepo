// File: SampleRunner.fs
// Project: Samples
// Created Date: 03/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 03/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 

namespace Samples

open AUTD3Sharp

module SampleRunner =
    let Run (autd : Controller) = 
        let examples = [
                (FocusTest.Test, "Single Focal Point Test");
                (BesselBeamTest.Test, "BesselBeam Test");
                (GainHoloTest.Test, "Multiple Focal Points Test");
                (FocusSTMTest.Test, "FocusSTM Test");
                (GainSTMTest.Test, "GainSTM Test");
                (AdvancedTest.Test, "Advanced Test (Custom gain/modulation)");
                (CustomTest.Test, "Custom Test (Custom Focus)")];
        let examples = 
            if autd.Geometry.NumDevices = 2 then examples @ [(GroupTest.Test, "Grouped gain Test")] else examples;

        new Clear() |> autd.Send |> ignore;
        new Synchronize() |> autd.Send |> ignore;

        let print_firm firm = printfn $"{firm}" 
        printfn "==================================== Firmware information ======================================"
        autd.FirmwareInfoList() |> Seq.iter print_firm
        printfn "================================================================================================"

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

                    new Stop() |> autd.Send |> ignore;

                    run_example()
                | _ -> ()

        run_example()

        autd.Close() |> ignore;
        autd.Dispose() |> ignore;
