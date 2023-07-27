// File: Program.fs
// Project: SimulatorClient
// Created Date: 03/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 18/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 

open AUTD3Sharp.Utils
open AUTD3Sharp
open AUTD3Sharp.Link
open Samples


let autd = Controller.Builder()
                .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
                .AddDevice(new AUTD3(Vector3d(AUTD3.DeviceWidth, 0, 0), Vector3d.zero))
                .AdvancedMode()
                .OpenWith(Simulator 8080us)

for tr in autd.Geometry do
    tr.Frequency <- 70e3

new Synchronize() |> autd.Send |> ignore

SampleRunner.Run autd
