/*
 * File: Program.cs
 * Project: TwinCAT
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

var link = new TwinCAT().Build();
if (!autd.Open(link))
{
    Console.WriteLine("Failed to open Controller.");
    return;
}

SampleRunner.Run(autd);
