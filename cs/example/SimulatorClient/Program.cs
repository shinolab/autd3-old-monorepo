/*
 * File: Program.cs
 * Project: SimulatorClient
 * Created Date: 13/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/10/2022
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
foreach (var device in autd.Geometry)
    foreach (var tr in device)
        tr.Frequency = 70e3;

var link = new Simulator().Port(50632).Build();
if (!autd.Open(link))
{
    Console.WriteLine(Controller.LastError);
    return;
}

SampleRunner.Run(autd);
