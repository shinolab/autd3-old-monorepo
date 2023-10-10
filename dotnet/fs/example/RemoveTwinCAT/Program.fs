// File: Program.fs
// Project: RemoveTwinCAT
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

let serverAmsNetId = "your TwinCATAUTDServer AMS net id (e.g. 172.16.99.2.1.1)"
 
Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWith(RemoteTwinCAT.Builder serverAmsNetId) |> SampleRunner.Run
