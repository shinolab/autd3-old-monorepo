// File: Program.fs
// Project: SOEM
// Created Date: 03/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 11/10/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 


open AUTD3Sharp.Utils
open AUTD3Sharp
open AUTD3Sharp.Link
open Samples

let onLost (msg:string): unit = 
    printfn $"Unrecoverable error occurred: {msg}"
    System.Environment.Exit(-1)

let autd = Controller.Builder()
            .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
            .OpenWith(SOEM.Builder().WithOnLost(new SOEM.OnErrCallbackDelegate(onLost)))

SampleRunner.Run autd
