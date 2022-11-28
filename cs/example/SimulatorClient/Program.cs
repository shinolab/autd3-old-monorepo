/*
 * File: Program.cs
 * Project: SimulatorClient
 * Created Date: 13/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp.Utils;
using AUTD3Sharp;
using AUTD3Sharp.Link;
using Samples;

var autd = new Controller();
autd.Geometry.AddDevice(Vector3d.zero, Vector3d.zero);
autd.Geometry.AddDevice(new Vector3d(AUTD3.DeviceWidth, 0, 0), Vector3d.zero);

autd.ToNormal();
foreach (var tr in autd.Geometry)
    tr.Frequency = 70e3;

var link = new Simulator().Build();
if (!autd.Open(link))
{
    Console.WriteLine("Failed to open Controller.");
    return;
}

SampleRunner.Run(autd);
