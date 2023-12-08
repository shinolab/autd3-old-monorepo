// File: Flag.fs
// Project: Samples
// Created Date: 14/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 08/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 

namespace Samples

open AUTD3Sharp
open System
open System.Threading.Tasks

module FlagTest =
    let Test<'T> (autd : Controller<'T>) = 
        printfn "press any key to run fan..."
        System.Console.ReadKey true |> ignore;

        (new ConfigureForceFan(fun dev -> true), new ConfigureReadsFPGAInfo(fun dev -> true)) |> autd.SendAsync |> Async.AwaitTask |> Async.RunSynchronously |> ignore;

        let mutable fin = false;
        let th : Task =
            async {
                let prompts = [|'-'; '/'; '|'; '\\'|]
                let mutable promptsIdx = 0;
                while not fin do
                    let states = autd.FPGAInfoAsync() |> Async.AwaitTask |> Async.RunSynchronously
                    printfn "%c FPGA Status..." prompts.[promptsIdx / 1000 % prompts.Length]
                    printfn "%s" (String.Join("\n", states))
                    printf "\x1b[%dA" (states.Length + 1)
                    promptsIdx <- promptsIdx + 1
                done
            } |> Async.StartAsTask :> Task
       
        printfn "press any key stop checking FPGA status..."
        System.Console.ReadKey true |> ignore;

        fin <- true;
        th.Wait();
        
        (new ConfigureForceFan(fun dev -> false), new ConfigureReadsFPGAInfo(fun dev -> false)) |> autd.SendAsync |> Async.AwaitTask |> Async.RunSynchronously |> ignore;

