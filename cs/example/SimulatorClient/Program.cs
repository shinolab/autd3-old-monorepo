/*
 * File: Program.cs
 * Project: SimulatorClient
 * Created Date: 13/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/02/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp.Utils;
using AUTD3Sharp;
using AUTD3Sharp.Link;
using Samples;

var geometry = new GeometryBuilder()
    .AddDevice(Vector3d.zero, Vector3d.zero)
    .AddDevice(new Vector3d(AUTD3.DeviceWidth, 0, 0), Vector3d.zero).Build();

var link = new Simulator().Build();
var autd = Controller.Open(geometry, link);

autd.ToNormal();
foreach (var tr in autd.Geometry)
    tr.Frequency = 70e3;

autd.AckCheckTimeoutMs = 20;

SampleRunner.Run(autd);
