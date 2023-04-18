/*
 * File: Program.cs
 * Project: TwinCAT
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

var geometry = new Geometry.Builder()
    .AddDevice(Vector3d.zero, Vector3d.zero)
    .Build();

var link = new TwinCAT().Build();

var autd = Controller.Open(geometry, link);

SampleRunner.Run(autd);
