/*
 * File: Program.cs
 * Project: SOEM
 * Created Date: 14/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/03/2023
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

var geometry = new Geometry.Builder()
    .AddDevice(Vector3d.zero, Vector3d.zero)
    .Build();

var onLost = new SOEM.OnLostCallbackDelegate((string msg) =>
{
    Console.WriteLine($"Unrecoverable error occurred: {msg}");
    Environment.Exit(-1);
});
var link = new SOEM()
    .OnLost(onLost)
    .Build();

var autd = Controller.Open(geometry, link);

SampleRunner.Run(autd);
