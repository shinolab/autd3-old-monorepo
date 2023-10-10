/*
 * File: Program.cs
 * Project: Simulator
 * Created Date: 13/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/09/2023
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
    .Advanced()
    .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
    .AddDevice(new AUTD3(new Vector3d(AUTD3.DeviceWidth, 0, 0), Vector3d.zero))
    .OpenWith(Simulator.Builder(8080));

foreach (var dev in autd.Geometry)
    foreach (var tr in dev)
        tr.Frequency = 70e3;

autd.Send(new Synchronize());

SampleRunner.Run(autd);
