/*
 * File: Program.cs
 * Project: SimulatorClient
 * Created Date: 13/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp.Utils;
using AUTD3Sharp;
using AUTD3Sharp.Link;
using Samples;


var autd = Controller.Builder()
    .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
    .AddDevice(new AUTD3(new Vector3d(AUTD3.DeviceWidth, 0, 0), Vector3d.zero))
    .AdvancedMode()
    .OpenWith(new Simulator(8080));

foreach (var tr in autd.Geometry)
    tr.Frequency = 70e3;

SampleRunner.Run(autd);
