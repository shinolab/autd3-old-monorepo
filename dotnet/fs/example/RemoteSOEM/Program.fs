// File: Program.fs
// Project: RemoteSOEM
// Created Date: 14/09/2023
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
open System.Net

let addr = new IPEndPoint(IPAddress.Parse("127.0.0.1"), 8080);
(new ControllerBuilder()).AddDevice(new AUTD3(Vector3d.zero)).OpenWithAsync(RemoteSOEM.Builder addr) |> Async.AwaitTask |> Async.RunSynchronously |> SampleRunner.Run
