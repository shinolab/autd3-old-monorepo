// File: SampleRunner.fs
// Project: Samples
// Created Date: 03/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 21/02/2023
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

        let firmwareList = autd.FirmwareInfoList()
        let inline print_warn msg = 
                System.Console.ForegroundColor <- System.ConsoleColor.Yellow
                printfn "%s" msg
                System.Console.ResetColor()
        if firmwareList |> Seq.exists (fun firm -> not firm.MatchesVersion) then print_warn "WARN: FPGA and CPU firmware version do not match."
        if firmwareList |> Seq.exists (fun firm -> not firm.IsSupported) then print_warn (sprintf "WARN: You are using old firmware. Please consider updating to %s." FirmwareInfo.LatestVersion)
        printfn "==================================== Firmware information ======================================"
        autd.FirmwareInfoList() |> Seq.iter (fun firm -> printfn $"{firm}")
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
