/*
 * File: Program.cs
 * Project: SimulatorClient
 * Created Date: 13/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp.Utils;
using AUTD3Sharp;
using AUTD3Sharp.Link;
using Samples;

var geometry = new Geometry.Builder()
    .AddDevice(Vector3d.zero, Vector3d.zero)
    .AddDevice(new Vector3d(AUTD3.DeviceWidth, 0, 0), Vector3d.zero)
    .AdvancedMode()
    .Build();

var link = new Simulator(8080).Build();
var autd = Controller.Open(geometry, link);

foreach (var tr in autd.Geometry)
    tr.Frequency = 70e3;

SampleRunner.Run(autd);
