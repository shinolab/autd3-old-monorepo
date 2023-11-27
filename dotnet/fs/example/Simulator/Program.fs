// File: Program.fs
// Project: SimulatorClient
// Created Date: 03/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 26/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 

open AUTD3Sharp.Utils
open AUTD3Sharp
open AUTD3Sharp.Link
open Samples


let autd = (new ControllerBuilder())
                .AddDevice(new AUTD3(Vector3d.zero))
                .AddDevice(new AUTD3(Vector3d(AUTD3.DeviceWidth, 0, 0)))
                .OpenWithAsync(Simulator.Builder 8080us) |> Async.AwaitTask |> Async.RunSynchronously 

SampleRunner.Run autd
