// File: Program.fs
// Project: SOEM
// Created Date: 03/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 03/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 


open AUTD3Sharp.Utils
open AUTD3Sharp
open AUTD3Sharp.Link
open Samples

let geometry = 
    GeometryBuilder()
        .AddDevice(Vector3d.zero, Vector3d.zero)
        .Build()

let onLost (msg:string): unit = 
    printfn $"Unrecoverable error occurred: {msg}"
    System.Environment.Exit(-1)

let link = (new SOEM()).HighPrecision(true).OnLost(new SOEM.OnLostCallbackDelegate(onLost)).Build()

let autd = Controller.Open (geometry, link)

autd.AckCheckTimeoutMs <- 20uL;

SampleRunner.Run autd
