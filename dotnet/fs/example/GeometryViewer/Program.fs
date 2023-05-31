// File: Program.fs
// Project: GeometryViewer
// Created Date: 03/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 18/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 

open AUTD3Sharp
open AUTD3Sharp.Extra
open AUTD3Sharp.Utils

Geometry.Builder().AddDevice(Vector3d.zero, Vector3d.zero)
                    .AddDevice(Vector3d(0, 0, AUTD3.DeviceWidth), Vector3d(0, AUTD3.Pi / 2.0, 0))
                    .AddDevice(Vector3d(AUTD3.DeviceWidth, 0, AUTD3.DeviceWidth), Vector3d(0, AUTD3.Pi, 0))
                    .AddDevice(Vector3d(AUTD3.DeviceWidth, 0, 0), Vector3d(0, -AUTD3.Pi / 2.0, 0))
                    .Build() |> GeometryViewer().WindowSize(800u, 600u).Vsync(true).Run |> exit
