/*
 * File: Program.cs
 * Project: SOEM
 * Created Date: 14/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp;
using AUTD3Sharp.Link;
using AUTD3Sharp.Utils;
using Samples;

Console.WriteLine("Test with SOEM");

var autd = new Controller();
autd.Geometry.AddDevice(Vector3d.zero, Vector3d.zero);

var onLost = new SOEM.OnLostCallbackDelegate((string msg) =>
{
    Console.WriteLine($"Unrecoverable error occurred: {msg}");
    Environment.Exit(-1);
});
var link = new SOEM()
    .HighPrecision(true)
    .OnLost(onLost)
    .Build();
if (!autd.Open(link))
{
    Console.WriteLine(Controller.LastError);
    return;
}

autd.CheckTrials = 50;

SampleRunner.Run(autd);
