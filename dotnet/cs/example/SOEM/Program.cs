/*
 * File: Program.cs
 * Project: SOEM
 * Created Date: 14/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp;
using AUTD3Sharp.Link;
using AUTD3Sharp.Utils;
using Samples;

var onLost = new SOEM.OnErrCallbackDelegate(msg =>
{
    Console.WriteLine($"Unrecoverable error occurred: {msg}");
    Environment.Exit(-1);
});

var autd = Controller.Builder().
    AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
    .OpenWith(SOEM.Builder()
        .WithOnLost(onLost));

SampleRunner.Run(autd);
