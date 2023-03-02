// File: Program.fs
// Project: SimulatorClient
// Created Date: 03/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 03/03/2023
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
        .AddDevice(Vector3d(AUTD3.DeviceWidth, 0, 0), Vector3d.zero)
        .Build()

let link = Simulator().Build()

let autd = Controller.Open (geometry, link)

autd.ToAdvanced()
for tr in autd.Geometry do
    tr.Frequency <- 70e3

SampleRunner.Run autd
