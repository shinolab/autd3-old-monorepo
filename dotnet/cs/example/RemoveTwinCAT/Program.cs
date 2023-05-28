/*
 * File: Program.cs
 * Project: RemoveTwinCAT
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
    .Build();

const string serverAmsNetId = "your TwinCATAUTDServer AMS net id (e.g. 172.16.99.2.1.1)";

var link = new RemoteTwinCAT(serverAmsNetId).Build();

var autd = Controller.Open(geometry, link);

SampleRunner.Run(autd);
